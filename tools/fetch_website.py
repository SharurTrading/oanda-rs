"""Fetch current OANDA v20 contract pages for a deliberate coverage review.

Usage: python3 tools/fetch_website.py /tmp/oanda-review
The fetched HTML is not committed; generated Rust and coverage decisions are.
"""
from pathlib import Path
from urllib.request import Request, urlopen
import sys

pages = (
    'account-ep', 'order-ep', 'trade-ep', 'position-ep', 'transaction-ep', 'pricing-ep',
    'account-df', 'instrument-df', 'order-df', 'trade-df', 'position-df',
    'transaction-df', 'pricing-df', 'pricing-common-df', 'primitives-df',
)
destination = Path(sys.argv[1]) if len(sys.argv) > 1 else Path('/private/tmp')
destination.mkdir(parents=True, exist_ok=True)
for page in pages:
    url = f'https://developer.oanda.com/rest-live-v20/{page}/'
    request = Request(url, headers={'User-Agent': 'oanda-rs-contract-review/0.1'})
    with urlopen(request, timeout=30) as response:
        data = response.read(8 * 1024 * 1024 + 1)
    if len(data) > 8 * 1024 * 1024:
        raise ValueError(f'OANDA page exceeds review bound: {url}')
    (destination / f'oanda-{page}.html').write_bytes(data)
    print(url)
