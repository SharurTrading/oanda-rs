"""Check the checked-in website coverage ledger without network access.

Every check must be able to fail for the artifact it claims to verify:
definitions count only as typed contracts that carry fields (never a bare
String alias), test markers must resolve to a real test function whose body
uses the contract, and every stream operation — not two hardcoded names —
needs a typed stream contract. `tools/test_check_coverage.py` proves each
check fails on a fabricated artifact.
"""
from pathlib import Path
import hashlib
import json
import re
import sys

root = Path(__file__).resolve().parents[1]


def test_functions(tests_text):
    """Map each top-level test function name to its body.

    Markers and method calls are anchored to the function that must contain
    them, so a name in a comment, a string, or an unrelated file satisfies
    nothing.
    """
    functions = {}
    current = None
    body = []
    for line in tests_text.splitlines():
        started = re.match(r'(?:pub )?(?:async )?fn (\w+)\(', line)
        if started and current is None:
            current = started.group(1)
            body = []
            continue
        if current is not None:
            if line == '}':
                functions[current] = '\n'.join(body)
                current = None
            else:
                body.append(line)
    return functions


def definition_implemented(name, source):
    """A definition counts only as a typed contract, never a bare String alias.

    `pub type X = String` was the generator's removed fabrication fallback;
    aliases to real types (the exact `Decimal` aliases OA-DECIMAL-01 requires)
    and `pub use ... as X` re-exports of validated types remain contracts.
    """
    escaped = re.escape(name)
    if re.search(r'pub (?:struct|enum)\s+' + escaped + r'\b', source):
        return True
    if re.search(r'id_type!\(' + escaped + r'\)', source):
        return True
    if re.search(r'pub use [^;]+ as ' + escaped + r'\s*;', source):
        return True
    alias = re.search(r'pub type\s+' + escaped + r'\s*=\s*([^;]+);', source)
    return bool(alias and alias.group(1).strip() != 'String')


def stream_contract(public_method, source):
    """A stream operation's typed contract is a method returning HttpStream<E>."""
    signature = re.search(
        r'pub async fn\s+' + re.escape(public_method) + r'\s*\([^)]*\)\s*'
        r'->\s*[^{;]*?HttpStream<', source, re.S)
    return bool(signature)


def check(ledger, spec_bytes, source, tests_text):
    """Return the list of gate errors for one ledger/source/tests state."""
    spec = json.loads(spec_bytes)
    errors = []
    if hashlib.sha256(spec_bytes).hexdigest() != '5856fab076e3bc6c40fb06ecaa78d85cc8a828e4f065a95806cace3ac1dea212':
        errors.append('pinned OpenAPI checksum changed; review and update drift decisions')
    functions = test_functions(tests_text)
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
                rejection = op.get('rejection')
                if rejection not in {'endpoint', 'generic'}:
                    errors.append(f'operation records no rejection decision: {key}: {rejection!r}')
                stem = ''.join(word.capitalize() for word in op['public_method'].split('_'))
                if not re.search(r'pub struct\s+' + stem + 'Response' + r'\b', source):
                    errors.append(f'missing typed response: {key}')
                if rejection == 'endpoint':
                    if not re.search(r'pub struct\s+' + stem + 'Rejection' + r'\b', source):
                        errors.append(f'missing typed rejection: {key}')
                elif not re.search(r'OperationError<crate::GenericRejection>', source):
                    errors.append(f'generic rejection not typed: {key}')
            elif not stream_contract(op['public_method'], source):
                errors.append(f'missing typed stream contract: {key}')
        if status == 'tested':
            marker = op.get('test')
            if not marker or marker not in functions:
                errors.append(f'missing test marker: {key}')
            elif not re.search(r'\bclient\s*\.\s*' + re.escape(op['public_method']) + r'\s*\(', functions[marker]):
                errors.append(f'test does not call public method: {key}')
    for definition in ledger['definitions']:
        status = definition['status']
        if status not in {'inventoried', 'implemented', 'tested', 'blocked'}:
            errors.append(f'invalid definition status: {definition["name"]}: {status}')
        if status != 'tested':
            errors.append(f'definition lacks a checked typed contract: {definition["name"]}: {status}')
        if status in {'implemented', 'tested'}:
            if not definition_implemented(definition['name'], source):
                errors.append(f'missing public definition: {definition["name"]}')
        if status == 'tested':
            marker = definition.get('test')
            if not marker or marker not in functions:
                errors.append(f'missing definition test marker: {definition["name"]}')
            elif not re.search(r'(?:typed_contract::<oanda_client::models::|round_trip::<models::|round_trip::<)' + re.escape(definition['name']) + r'>', functions[marker]):
                errors.append(f'test does not use typed definition: {definition["name"]}')
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
    return errors


def main():
    ledger = json.loads((root / 'docs' / 'coverage.json').read_text())
    spec_bytes = (root / 'spec' / 'official' / 'v20-openapi.json').read_bytes()
    source = '\n'.join(path.read_text() for path in (root / 'src').rglob('*.rs'))
    tests = '\n'.join(path.read_text() for path in (root / 'tests').rglob('*.rs'))
    errors = check(ledger, spec_bytes, source, tests)
    print('Operations:', ', '.join(f'{s}={sum(o["status"] == s for o in ledger["operations"])}' for s in ('inventoried','implemented','tested','blocked')))
    print('Definitions:', ', '.join(f'{s}={sum(d["status"] == s for d in ledger["definitions"])}' for s in ('inventoried','implemented','tested','blocked')))
    for error in errors: print('ERROR:', error, file=sys.stderr)
    return bool(errors)


if __name__ == '__main__':
    sys.exit(main())
