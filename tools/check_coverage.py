"""Check the checked-in website coverage ledger without network access.

Every check must be able to fail for the artifact it claims to verify:
definitions count only as typed contracts carrying fields (never a bare
String alias in any spelling, never a laundered re-export), test markers must
resolve to a real #[test] function whose body decodes the contract, and every
operation whose path ends in /stream needs a typed stream event — not two
hardcoded method names. `tools/test_check_coverage.py` fabricates the artifact
each check exists to reject and asserts the failure.
"""
from pathlib import Path
import hashlib
import json
import re
import sys

root = Path(__file__).resolve().parents[1]

# An alias counts as a contract only when it names one of the exact types the
# crate's own rules require; every other target — String in any spelling,
# serde_json::Value, (), scalars — is the generator's removed fabrication
# fallback and fails the gate.
TYPED_ALIAS_TARGETS = {'rust_decimal::Decimal', 'crate::timestamp::Timestamp',
                       'Decimal', 'Timestamp'}


def test_functions(tests_text):
    """Map each #[test]/#[tokio::test] function name to its body.

    A marker resolves only against a real test function, so a helper, a
    constant, or a comment satisfies nothing. Generic functions cannot be
    tests and are not captured.
    """
    functions = {}
    current = None
    body = []
    pending_attribute = False
    for line in tests_text.splitlines():
        if current is None:
            stripped = line.strip()
            if re.fullmatch(r'#\[(test|tokio::test(?:\([^)]*\))?)\]', stripped):
                pending_attribute = True
                continue
            started = re.match(r'(?:pub )?(?:async )?fn (\w+)\(', line)
            if started and pending_attribute and not re.match(r'(?:pub )?(?:async )?fn \w+<', line):
                current = started.group(1)
                body = []
            pending_attribute = False
            continue
        if line == '}':
            functions[current] = '\n'.join(body)
            current = None
        else:
            body.append(line)
    return functions


def struct_or_enum(name, source):
    """A nominal contract that carries fields or variants."""
    escaped = re.escape(name)
    struct = re.search(r'pub struct ' + escaped + r' \{(.*?)\}', source, re.S)
    if struct:
        return bool(re.search(r'\n?\s*pub (r\#)?\w+:', struct.group(1)))
    # A tuple struct with a non-empty body carries its field as its payload.
    tuple_struct = re.search(r'pub struct ' + escaped + r'\s*\((.+)\);', source, re.S)
    if tuple_struct:
        return bool(tuple_struct.group(1).strip())
    enum = re.search(r'pub enum ' + escaped + r' \{(.*?)\}', source, re.S)
    if enum:
        return bool(re.search(r'\n?\s*[A-Z]\w*\s*(?:\(|,|\{|$)', enum.group(1)))
    return False


def definition_implemented(name, source):
    """True only for a typed contract, never for an untyped alias."""
    if struct_or_enum(name, source):
        return True
    if re.search(r'id_type!\(' + re.escape(name) + r'\)', source):
        return True
    alias = re.search(r'pub type\s+' + re.escape(name) + r'\s*=\s*([^;]+);', source)
    if alias:
        return alias.group(1).strip() in TYPED_ALIAS_TARGETS
    reexport = re.search(r'pub use\s+((?:crate|super)::[^;]+)\s+as\s+' + re.escape(name) + r'\s*;', source)
    if reexport:
        # A re-export launders whatever it points at, so the target must
        # itself be a nominal contract, not another alias or re-export.
        target = reexport.group(1).rsplit('::', 1)[-1]
        return target != name and struct_or_enum(target, source)
    return False


def signature_text(public_method, source):
    """The text of a method's signature, from `pub async fn` to its body.

    Parens are counted so a parameter carrying a nested paren does not end
    the parameter list, and block comments are stripped so a `/* HttpStream */
    */` before the body cannot satisfy a type check on text alone.
    """
    start = re.search(r'pub async fn\s+' + re.escape(public_method) + r'\s*\(', source)
    if not start:
        return ''
    depth = 0
    index = start.end() - 1
    while index < len(source):
        character = source[index]
        if character == '(':
            depth += 1
        elif character == ')':
            depth -= 1
            if depth == 0:
                break
        index += 1
    remainder = source[index:len(source)].split('{', 1)[0]
    remainder = re.sub(r'/\*.*?\*/', '', remainder, flags=re.S)
    return remainder


def stream_contract(public_method, source):
    """A stream operation's element type must be a typed event enum."""
    signature = signature_text(public_method, source)
    element = re.search(r'HttpStream<\s*([A-Za-z_][A-Za-z0-9_]*)\s*>', signature)
    if not element:
        return False
    return struct_or_enum(element.group(1), source) and bool(
        re.search(r'pub enum\s+' + re.escape(element.group(1)) + r'\b', source))


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
        is_stream = op['path'].endswith('/stream')
        kind = op.get('kind')
        if kind not in {'query', 'mutation', 'stream'}:
            errors.append(f'invalid kind: {key}: {kind!r}')
        elif (kind == 'stream') != is_stream:
            errors.append(f'kind {kind!r} disagrees with path: {key}')
        allowed_rejections = {'stream'} if is_stream else {'endpoint', 'generic'}
        if op.get('rejection') not in allowed_rejections:
            errors.append(f'operation records no rejection decision: {key}: {op.get("rejection")!r}')
        if status in {'implemented', 'tested'}:
            if not re.search(r'pub async fn\s+' + re.escape(op['public_method']) + r'\s*\(', source):
                errors.append(f'missing public method: {key}')
            if is_stream:
                if not stream_contract(op['public_method'], source):
                    errors.append(f'missing typed stream contract: {key}')
            else:
                rejection = op.get('rejection')
                stem = ''.join(word.capitalize() for word in op['public_method'].split('_'))
                if not re.search(r'pub struct\s+' + stem + 'Response' + r'\b', source):
                    errors.append(f'missing typed response: {key}')
                if rejection == 'endpoint':
                    if not re.search(r'pub struct\s+' + stem + 'Rejection' + r'\b', source):
                        errors.append(f'missing typed rejection: {key}')
                elif rejection == 'generic':
                    if 'OperationError<crate::GenericRejection>' not in signature_text(op['public_method'], source):
                        errors.append(f'generic rejection not typed in method: {key}')
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
            elif not re.search(r'(?:decode_fixture::<oanda_client::models::|round_trip::<models::|round_trip::<oanda_client::models::|round_trip::<)' + re.escape(definition['name']) + r'>', functions[marker]):
                errors.append(f'test does not decode the definition: {definition["name"]}')
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


def real_state():
    """The checked-in ledger, spec, source, and tests, for a clean-run proof."""
    ledger = json.loads((root / 'docs' / 'coverage.json').read_text())
    spec_bytes = (root / 'spec' / 'official' / 'v20-openapi.json').read_bytes()
    source = '\n'.join(path.read_text() for path in (root / 'src').rglob('*.rs'))
    tests = '\n'.join(path.read_text() for path in (root / 'tests').rglob('*.rs'))
    return ledger, spec_bytes, source, tests


def main():
    ledger, spec_bytes, source, tests = real_state()
    errors = check(ledger, spec_bytes, source, tests)
    print('Operations:', ', '.join(f'{s}={sum(o["status"] == s for o in ledger["operations"])}' for s in ('inventoried','implemented','tested','blocked')))
    print('Definitions:', ', '.join(f'{s}={sum(d["status"] == s for d in ledger["definitions"])}' for s in ('inventoried','implemented','tested','blocked')))
    for error in errors: print('ERROR:', error, file=sys.stderr)
    return bool(errors)


if __name__ == '__main__':
    sys.exit(main())
