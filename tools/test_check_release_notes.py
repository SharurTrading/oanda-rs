"""Credential-free release-note checks using synthetic HTML and mocked HTTP."""

from contextlib import redirect_stdout
from http.client import IncompleteRead
import io
import json
import os
from pathlib import Path
import tempfile
import unittest
from unittest.mock import patch
from urllib.error import HTTPError, URLError

import check_release_notes as checker


def page(*versions):
    header = "<tr>" + "".join(f"<th>{value}</th>" for value in checker.HEADERS) + "</tr>"
    rows = []
    for version in versions:
        if version:
            rows.append(
                f"<tr><td><strong>{version}</strong></td><td>Unreleased</td>"
                "<td></td><td>Example change</td></tr>"
            )
        else:
            # A continuation row on OANDA's page carries text only in Details;
            # the version, date, and compatibility cells stay empty.
            rows.append("<tr><td></td><td></td><td></td><td>Example change</td></tr>")
    return f"<html><table>{header}{''.join(rows)}</table></html>"


def unreadable_version_row(date="September 28, 2018"):
    """A row whose version cell is empty but which is not a continuation."""
    return (
        f"<tr><td></td><td>{date}</td><td></td>"
        "<td>Unreadable release detail</td></tr>"
    )


class ReleaseNoteTests(unittest.TestCase):
    def test_current_page_ignores_other_versions_and_continuation_rows(self):
        html = "<p>Unrelated 99.0.0</p><table><tr><td>88.0.0</td></tr></table>"
        html += page("3.0.25", "", "3.0.24", "3.0.9")
        self.assertEqual(checker.check(html, "3.0.25")[0], 0)

    def test_newer_versions_are_numeric_sorted_and_deduplicated(self):
        status, message = checker.check(
            page("4.0.0", "3.0.10", "3.1.0", "3.0.9", "3.0.10"), "3.0.9"
        )
        self.assertEqual(status, 1)
        self.assertIn("newer versions: 3.0.10, 3.1.0, 4.0.0.", message)
        self.assertIn("docs/release-notes.json", message)

    def test_unreleased_version_also_requires_review(self):
        self.assertEqual(checker.check(page("3.0.26", "3.0.25"), "3.0.25")[0], 1)

    def test_regressed_page_is_not_reported_as_current(self):
        with self.assertRaisesRegex(ValueError, "below the reviewed baseline"):
            checker.check(page("3.0.24"), "3.0.25")

    def test_missing_empty_duplicate_or_malformed_tables_fail(self):
        examples = [
            "<html>Service unavailable</html>",
            page(),
            page("3.0.25") + page("3.0.25"),
            page("3.0.25").replace("<th>Version</th>", "<th>Release</th>"),
            page("3.0.25").replace("</table>", ""),
            page("3.0.25").replace("</td>", "", 1),
            page("3.0.25").replace("</tr>", "", 1),
            page("3.0.25").replace("<td></td>", ""),
            page("3.0.25").replace("Example change", "<table></table>"),
        ]
        for html in examples:
            with self.subTest(html=html), self.assertRaises(ValueError):
                checker.release_versions(html)

    def test_empty_version_row_with_a_date_is_reported_not_skipped(self):
        # A release whose version cell is empty for any reason other than a
        # structural continuation must never vanish from the compared set: the
        # checker reports it and exits 2 instead of judging "current".
        html = "<html><table>" + (
            "<tr><th>Version</th><th>Date</th><th>Compatibility Changes</th><th>Details</th></tr>"
            "<tr><td><strong>3.0.25</strong></td><td>Unreleased</td><td></td><td>Example change</td></tr>"
            + unreadable_version_row()
            + "</table></html>"
        )
        with self.assertRaisesRegex(ValueError, "no readable version"):
            checker.release_versions(html)

    def test_cli_reports_an_unreadable_version_row_as_incomplete(self):
        with tempfile.TemporaryDirectory() as directory:
            baseline = Path(directory) / "baseline.json"
            baseline.write_text(json.dumps({"last_reviewed_version": "3.0.25", "reviewed_at": "2026-09-27"}))
            html = Path(directory) / "page.html"
            html.write_text(
                "<html><table>"
                "<tr><th>Version</th><th>Date</th><th>Compatibility Changes</th><th>Details</th></tr>"
                "<tr><td><strong>3.0.25</strong></td><td>Unreleased</td><td></td><td>Example change</td></tr>"
                + unreadable_version_row()
                + "</table></html>"
            )
            with patch.dict(os.environ, {}, clear=True):
                with redirect_stdout(io.StringIO()) as output:
                    self.assertEqual(
                        checker.main(["--html", str(html), "--baseline", str(baseline)]), 2
                    )
            self.assertIn("could not be completed", output.getvalue())
            self.assertIn("Unreadable release detail", output.getvalue())

    def test_first_row_cannot_be_a_continuation(self):
        html = (
            "<html><table>"
            "<tr><th>Version</th><th>Date</th><th>Compatibility Changes</th><th>Details</th></tr>"
            "<tr><td></td><td></td><td></td><td>Example change</td></tr>"
            "<tr><td><strong>3.0.25</strong></td><td>Unreleased</td><td></td><td>Example change</td></tr>"
            "</table></html>"
        )
        with self.assertRaisesRegex(ValueError, "no readable version"):
            checker.release_versions(html)

    def test_unsupported_versions_fail_instead_of_being_skipped(self):
        for version in ["v3.0.26", "3.0.26-rc1", "3.0", "3.0.26.1", "3.00.26", "unknown"]:
            with self.subTest(version=version), self.assertRaises(ValueError):
                checker.release_versions(page(version, "3.0.25"))

    def test_fetch_uses_bounded_read_and_timeout(self):
        with patch.object(checker, "urlopen") as fetch:
            fetch.return_value.__enter__.return_value.read.return_value = page("3.0.25").encode()
            self.assertEqual(checker.release_versions(checker.fetch_html()), ["3.0.25"])
            self.assertEqual(fetch.call_args.args[0].full_url, checker.URL)
            self.assertEqual(fetch.call_args.kwargs["timeout"], 30)
            fetch.return_value.__enter__.return_value.read.assert_called_once_with(checker.MAX_BYTES + 1)

    def test_oversized_and_non_utf8_responses_fail(self):
        for body in [b"x" * (checker.MAX_BYTES + 1), b"\xff"]:
            with self.subTest(size=len(body)), patch.object(checker, "urlopen") as fetch:
                fetch.return_value.__enter__.return_value.read.return_value = body
                with self.assertRaises(ValueError):
                    checker.fetch_html()

    def test_cli_status_summary_and_baseline_stays_unchanged(self):
        with tempfile.TemporaryDirectory() as directory:
            baseline = Path(directory) / "baseline.json"
            original = json.dumps({"last_reviewed_version": "3.0.25", "reviewed_at": "2026-09-27"})
            baseline.write_text(original)
            html = Path(directory) / "page.html"
            summary = Path(directory) / "summary.md"
            for version, expected in [("3.0.25", 0), ("3.0.26", 1), ("unknown", 2)]:
                html.write_text(page(version))
                output = io.StringIO()
                with patch.dict(os.environ, {"GITHUB_ACTIONS": "true", "GITHUB_STEP_SUMMARY": str(summary)}):
                    with redirect_stdout(output), patch.object(checker, "urlopen") as fetch:
                        status = checker.main(["--html", str(html), "--baseline", str(baseline)])
                        fetch.assert_not_called()
                self.assertEqual(status, expected)
                self.assertEqual("::error::" in output.getvalue(), bool(expected))
                self.assertIn(output.getvalue().splitlines()[0], summary.read_text())
                self.assertEqual(baseline.read_text(), original)

    def test_cli_fetch_failures_are_errors(self):
        for error in [
            URLError("unavailable"), TimeoutError("timed out"),
            HTTPError(checker.URL, 503, "unavailable", {}, None),
            IncompleteRead(b"partial HTML", 100),
        ]:
            with self.subTest(error=error), patch.dict(os.environ, {}, clear=True):
                with patch.object(checker, "urlopen", side_effect=error), redirect_stdout(io.StringIO()) as output:
                    self.assertEqual(checker.main([]), 2)
                self.assertIn("could not be completed", output.getvalue())

    def test_invalid_baseline_fails_before_network_access(self):
        baselines = [
            "{", "[]", "{}",
            '{"last_reviewed_version":"3.0.25","reviewed_at":"bad-date"}',
            '{"last_reviewed_version":"latest","reviewed_at":"2026-09-27"}',
        ]
        with tempfile.TemporaryDirectory() as directory:
            path = Path(directory) / "baseline.json"
            for baseline in baselines:
                path.write_text(baseline)
                with self.subTest(baseline=baseline), patch.dict(os.environ, {}, clear=True):
                    with patch.object(checker, "urlopen") as fetch, redirect_stdout(io.StringIO()):
                        self.assertEqual(checker.main(["--baseline", str(path)]), 2)
                        fetch.assert_not_called()


if __name__ == "__main__":
    unittest.main()
