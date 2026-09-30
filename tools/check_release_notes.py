"""Check OANDA release-note versions against the manually reviewed baseline.

Uses only the Python standard library. Network access is reserved for scheduled
or manually dispatched CI; --html replays a saved page without network access.
Exit codes: 0 = current, 1 = review needed, 2 = check could not be completed.

A release row whose version cell is empty is a continuation row only when it
structurally continues the release above it: empty version, date, and
compatibility cells with detail text below a release that named a version.
Any other row without a readable version is reported as exit 2 — "could not be
completed" — with the offending row named, never silently dropped from the
compared set.
"""

import argparse
from datetime import date
from html.parser import HTMLParser
from http.client import HTTPException
import json
import os
from pathlib import Path
import re
import sys
from urllib.error import URLError
from urllib.request import Request, urlopen

URL = "https://developer.oanda.com/rest-live-v20/release-notes/"
BASELINE = Path(__file__).resolve().parents[1] / "docs" / "release-notes.json"
MAX_BYTES = 8 * 1024 * 1024
HEADERS = ["Version", "Date", "Compatibility Changes", "Details"]


def version_tuple(value):
    """Compare numeric components, so 3.0.10 advances past 3.0.9."""
    if not isinstance(value, str) or not re.fullmatch(
        r"(?:0|[1-9][0-9]*)\.(?:0|[1-9][0-9]*)\.(?:0|[1-9][0-9]*)", value
    ):
        raise ValueError("expected a release version with three numeric components")
    return tuple(int(part) for part in value.split("."))


class ReleaseTables(HTMLParser):
    """Collect table cells, including text wrapped in links or emphasis."""

    def __init__(self):
        super().__init__()
        self.tables = []
        self.table = None
        self.row = None
        self.cell = None

    def handle_starttag(self, tag, attrs):
        if tag == "table":
            if self.table is not None:
                raise ValueError("unexpected nested release-note table")
            self.table = []
        elif self.table is not None:
            if tag == "tr":
                if self.row is not None:
                    raise ValueError("unclosed release-note row")
                self.row = []
            elif tag in {"th", "td"} and self.row is not None:
                if self.cell is not None:
                    raise ValueError("unclosed release-note cell")
                self.cell = []

    def handle_data(self, data):
        if self.cell is not None:
            self.cell.append(data)

    def handle_endtag(self, tag):
        if tag in {"th", "td"} and self.cell is not None:
            self.row.append(" ".join("".join(self.cell).split()))
            self.cell = None
        elif tag == "tr" and self.row is not None:
            if self.cell is not None:
                raise ValueError("unclosed release-note cell")
            self.table.append(self.row)
            self.row = None
        elif tag == "table" and self.table is not None:
            if self.row is not None:
                raise ValueError("unclosed release-note row")
            self.tables.append(self.table)
            self.table = None


def release_versions(html):
    parser = ReleaseTables()
    parser.feed(html)
    parser.close()
    if parser.table is not None:
        raise ValueError("unclosed release-note table")
    tables = [table for table in parser.tables if table and table[0] == HEADERS]
    if len(tables) != 1:
        raise ValueError("expected exactly one OANDA release-note table")
    versions = set()
    continuations = []
    for row in tables[0][1:]:
        if len(row) != len(HEADERS):
            raise ValueError("unexpected release-note row shape")
        if row[0]:
            version_tuple(row[0])
            versions.add(row[0])
        elif row[1] or row[2] or not row[3] or not versions:
            # Only a row that structurally continues the release above it (empty
            # version, date, and compatibility cells, detail text, and a named
            # release before it) may be skipped; any other empty version cell is
            # a release this checker cannot identify and must not vanish from
            # the compared set.
            quoted = "; ".join(repr(cell[:120]) for cell in row)
            raise ValueError(
                f"release-note row with no readable version: [{quoted}]")
        else:
            # The shape is indistinguishable offline from a brand-new release
            # whose version cell this checker cannot read, so the row is
            # reported rather than vanishing: every message below names each
            # continuation row this check treated as one.
            continuations.append(row[3])
    if not versions:
        raise ValueError("release-note table contains no versions")
    return sorted(versions, key=version_tuple), continuations


def fetch_html():
    request = Request(URL, headers={"User-Agent": "oanda-rs-release-note-check/0.1"})
    with urlopen(request, timeout=30) as response:
        body = response.read(MAX_BYTES + 1)
    if len(body) > MAX_BYTES:
        raise ValueError("OANDA release-note page exceeds the 8 MiB limit")
    return body.decode("utf-8")


def check(html, reviewed):
    baseline = version_tuple(reviewed)
    versions, continuations = release_versions(html)
    skipped = ""
    if continuations:
        quoted = "; ".join(repr(row[:60]) for row in continuations[:3])
        more = f" (+{len(continuations) - 3} more)" if len(continuations) > 3 else ""
        skipped = f" Skipped {len(continuations)} structural continuation rows: {quoted}{more}."
    newer = [version for version in versions if version_tuple(version) > baseline]
    if newer:
        return 1, (
            f"OANDA release-note review needed: last reviewed {reviewed}; "
            f"newer versions: {', '.join(newer)}. Review {URL} and the authoritative "
            "endpoint/definition pages, then update docs/release-notes.json in a reviewed PR."
            f"{skipped}"
        )
    if version_tuple(versions[-1]) < baseline:
        raise ValueError("latest published version is below the reviewed baseline")
    return 0, (
        f"OANDA release notes are current: latest {versions[-1]}, last reviewed {reviewed}."
        f"{skipped}"
    )


def main(argv=None):
    arguments = argparse.ArgumentParser(description=__doc__)
    arguments.add_argument("--html", type=Path, help="check a saved HTML page offline")
    arguments.add_argument("--baseline", type=Path, default=BASELINE)
    args = arguments.parse_args(argv)
    try:
        baseline = json.loads(args.baseline.read_text(encoding="utf-8"))
        reviewed = baseline["last_reviewed_version"]
        version_tuple(reviewed)
        date.fromisoformat(baseline["reviewed_at"])
        html = args.html.read_text(encoding="utf-8") if args.html else fetch_html()
        status, message = check(html, reviewed)
    except (OSError, URLError, HTTPException, ValueError, KeyError, TypeError) as error:
        status, message = 2, f"OANDA release-note check could not be completed: {error}"
    print(message)
    if os.environ.get("GITHUB_ACTIONS") == "true" and status:
        escaped = message.replace("%", "%25").replace("\r", "%0D").replace("\n", "%0A")
        print(f"::error::{escaped}")
    summary = os.environ.get("GITHUB_STEP_SUMMARY")
    if summary:
        with open(summary, "a", encoding="utf-8") as output:
            output.write(f"### OANDA release notes\n\n{message}\n")
    return status


if __name__ == "__main__":
    sys.exit(main())
