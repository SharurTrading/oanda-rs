"""Check the checked-in website coverage ledger without network access."""
from pathlib import Path
import hashlib
import json
import re
import sys

root = Path(__file__).resolve().parents[1]
ledger = json.loads((root / 'docs' / 'coverage.json').read_text())
spec_bytes = (root / 'spec' / 'official' / 'v20-openapi.json').read_bytes()
spec = json.loads(spec_bytes)
source = '\n'.join(path.read_text() for path in (root / 'src').rglob('*.rs'))
tests = '\n'.join(path.read_text() for path in (root / 'tests').rglob('*.rs'))
errors = []
if hashlib.sha256(spec_bytes).hexdigest() != '5856fab076e3bc6c40fb06ecaa78d85cc8a828e4f065a95806cace3ac1dea212':
    errors.append('pinned OpenAPI checksum changed; review and update drift decisions')
seen = set()
for op in ledger['operations']:
    key = (op['method'], op['path'])
    if key in seen: errors.append(f'duplicate operation: {key}')
    seen.add(key)
    status = op['status']
    if status not in {'inventoried', 'implemented', 'tested', 'blocked'}:
        errors.append(f'invalid status: {key}: {status}')
    if status != 'tested':
        errors.append(f'operation lacks a completed typed fixture: {key}: {status}')
    if status in {'implemented', 'tested'}:
        if not re.search(r'pub async fn\s+' + re.escape(op['public_method']) + r'\s*\(', source):
            errors.append(f'missing public method: {key}')
        if op['kind'] != 'stream':
            stem = ''.join(word.capitalize() for word in op['public_method'].split('_'))
            for suffix in ('Response', 'Rejection'):
                if not re.search(r'pub struct\s+' + stem + suffix + r'\b', source):
                    errors.append(f'missing typed {suffix.lower()}: {key}')
        elif op['public_method'] == 'stream_pricing' and 'pub enum PriceStreamEvent' not in source:
            errors.append(f'missing typed stream event: {key}')
        elif op['public_method'] == 'stream_transactions' and 'pub enum TransactionStreamEvent' not in source:
            errors.append(f'missing typed stream event: {key}')
    if status == 'tested':
        marker = op.get('test')
        if not marker or marker not in tests: errors.append(f'missing test marker: {key}')
        if not re.search(r'\bclient\s*\.\s*' + re.escape(op['public_method']) + r'\s*\(', tests):
            errors.append(f'test does not call public method: {key}')
for definition in ledger['definitions']:
    status = definition['status']
    if status not in {'inventoried', 'implemented', 'tested', 'blocked'}:
        errors.append(f'invalid definition status: {definition["name"]}: {status}')
    if status != 'tested':
        errors.append(f'definition lacks a checked typed contract: {definition["name"]}: {status}')
    if status in {'implemented', 'tested'}:
        name = re.escape(definition['name'])
        if not re.search(r'(?:pub (?:struct|enum|type)\s+|id_type!\()' + name + r'\b|pub use [^;]+ as ' + name + r'\s*;', source):
            errors.append(f'missing public definition: {definition["name"]}')
    if status == 'tested':
        marker = definition.get('test')
        if not marker or marker not in tests: errors.append(f'missing definition test marker: {definition["name"]}')
        if not re.search(r'(?:typed_contract::<oanda_client::models::|round_trip::<models::|round_trip::<)' + re.escape(definition['name']) + r'>', tests):
            errors.append(f'test does not mention typed definition: {definition["name"]}')
website_ops = {(o['method'], o['path']) for o in ledger['operations']}
openapi_ops = {(verb.upper(), '/v3' + path) for path, item in spec['paths'].items() for verb in item if verb.lower() in {'get','post','put','patch','delete'}}
website_only = {('GET', '/v3/accounts/{accountID}/candles/latest')}
openapi_only = {('GET', '/v3/instruments/{instrument}/candles'), ('GET', '/v3/instruments/{instrument}/orderBook'), ('GET', '/v3/instruments/{instrument}/positionBook'), ('GET', '/v3/instruments/{instrument}/price'), ('GET', '/v3/instruments/{instrument}/price/range'), ('GET', '/v3/pricing'), ('GET', '/v3/pricing/range'), ('GET', '/v3/users/{userSpecifier}'), ('GET', '/v3/users/{userSpecifier}/externalInfo')}
if website_ops - openapi_ops != website_only or openapi_ops - website_ops != openapi_only:
    errors.append('endpoint drift changed; review the website and OpenAPI decisions')
website_definitions = {d['name'] for d in ledger['definitions']}
openapi_definitions = set(spec['definitions'])
website_only_definitions = {'AccumulatedAccountState','CandleSpecification','CandlestickResponse','ConversionFactor','DayOfWeek','DividendAdjustmentTransaction','FinancingDayOfWeek','GuaranteedStopLossDetails','GuaranteedStopLossOrder','GuaranteedStopLossOrderModeForInstrument','GuaranteedStopLossOrderMutability','GuaranteedStopLossOrderParameters','GuaranteedStopLossOrderReason','GuaranteedStopLossOrderRejectTransaction','GuaranteedStopLossOrderRequest','GuaranteedStopLossOrderTransaction','HomeConversionFactors','InstrumentFinancing','OpenTradeDividendAdjustment','PricingComponent','Tag','UserAttributes'}
openapi_only_definitions = {'MT4TransactionHeartbeat','OrderBook','OrderBookBucket','PositionBook','PositionBookBucket','Price','StatementYear','UserInfo','UserInfoExternal','UserSpecifier'}
if website_definitions - openapi_definitions != website_only_definitions or openapi_definitions - website_definitions != openapi_only_definitions:
    errors.append('definition drift changed; review the website and OpenAPI decisions')
print('Operations:', ', '.join(f'{s}={sum(o["status"] == s for o in ledger["operations"])}' for s in ('inventoried','implemented','tested','blocked')))
print('Definitions:', ', '.join(f'{s}={sum(o["status"] == s for o in ledger["definitions"])}' for s in ('inventoried','implemented','tested','blocked')))
for error in errors: print('ERROR:', error, file=sys.stderr)
sys.exit(bool(errors))
