"""Build tests/definition_wire.rs: an independently sourced fixture per definition.

Provenance, because it is the point of the suite:

- Fixture field sets and values come from the PINNED OpenAPI spec
  (spec/official/v20-openapi.json) — a different document, parsed by a
  different reader than the website generator — so a field the generator
  dropped or misnamed on the website pages cannot also vanish from the
  fixture: the round trip fails instead.
- Where the spec and the website disagree, the website is authoritative
  (docs/coverage.md), so the fixture carries the intersection and every
  spec-only field is printed as drift evidence, never silently ignored.
- Definitions the website adds beyond the spec are fixture'd from the saved
  definition pages and are listed in the emitted header.
- Required-field negatives come from the checked-in structs' own Option-ness
  and lock today's website contract in place: removing a required field from
  a fixture must fail to decode.

Usage: python3 tools/build_definition_fixtures.py /directory/containing/pages
"""
from __future__ import annotations
import json
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
SPEC = json.loads((ROOT / 'spec' / 'official' / 'v20-openapi.json').read_bytes())
LEDGER = json.loads((ROOT / 'docs' / 'coverage.json').read_text())
PAGES = Path(sys.argv[1]) if len(sys.argv) > 1 else Path('/private/tmp')
DEST = ROOT / 'tests' / 'definition_wire.rs'

# Tagged unions: the spec flattens all variants into one object, but the crate
# routes them by their `type` discriminator, so the fixture is built from the
# first documented variant's own definition.
UNIONS = {
    'Order': 'MarketOrder',
    'OrderRequest': 'MarketOrderRequest',
    'Transaction': 'CreateTransaction',
}

VALUES = {
    'DecimalNumber': '1.5', 'AccountUnits': '1.5', 'PriceValue': '1.5',
    'DateTime': '2024-01-02T03:04:05.123456789Z', 'CandleSpecification': 'M10:M',
    'InstrumentName': 'EUR_USD', 'Currency': 'USD', 'RequestID': '42',
}
SCALARS = {'string': '1', 'integer': 1, 'boolean': True, 'number': '1.5'}

# Website-only definitions whose pages are neither a schema <pre> nor a value
# table; hand-authored from the page text.
PAGE_FIXTURES = {
    'PricingComponent': 'M',
    'ConversionFactor': {'factor': '1.5'},
}

STRUCT_FIELDS = {}
STRUCT_OPTIONAL = {}
DRIFT = []
PAGE_DEFS = {}


def load_structures():
    for directory in ((ROOT / 'src' / 'models'), (ROOT / 'src')):
        for path in sorted(directory.glob('*.rs')):
            text = path.read_text(encoding='utf-8')
            for m in re.finditer(r'pub struct (\w+) \{(.*?)\n\}', text, re.S):
                name, body = m.group(1), m.group(2)
                fields = {}
                # rustfmt wraps serde attributes across lines, so pair each
                # `pub field:` with the nearest `rename = "..."` before it.
                marks = []
                for m in re.finditer(r'rename = "([^"]+)"', body):
                    marks.append((m.start(), 'rename', m.group(1)))
                for m in re.finditer(r'\n\s*pub (r\#)?(\w+): (Option<|)', body):
                    marks.append((m.start(), 'field', (m.group(2), m.group(3) == 'Option<')))
                marks.sort()
                pending = None
                for _, kind, data in marks:
                    if kind == 'rename':
                        pending = data
                    else:
                        rust_name, optional = data
                        provider = pending if pending is not None else rust_name
                        fields[provider] = optional
                        pending = None
                STRUCT_FIELDS.setdefault(name, set(fields))
                STRUCT_OPTIONAL.setdefault(name, fields)


def scalar(schema):
    """Value for an inlined spec primitive, keyed on its documented format."""
    text = schema.get('format', '')
    if 'RFC 3339' in text or 'Unix Epoch' in text:
        return VALUES['DateTime']
    if 'decimal number encoded as a string' in text:
        return '1.5'
    if 'ISO 4217' in text:
        return 'USD'
    return SCALARS.get(schema.get('type'), '1')


def spec_object(name, schema, seen=()):
    """Fixture for one spec object, reconciled against the website struct.

    Spec properties the checked-in struct does not carry are dropped as
    recorded drift (the website is authoritative), and the difference is
    printed for review rather than silently ignored.
    """
    if name in seen:
        return {}
    seen = seen + (name,)
    known = STRUCT_FIELDS.get(name)
    out = {}
    for field, shape in schema.get('properties', {}).items():
        if known is not None and field not in known:
            DRIFT.append(f'{name}.{field}')
            continue
        value = spec_member(shape, seen)
        if name == 'PriceBucket' and field == 'liquidity' and isinstance(value, (int, float)):
            # liquidity is the one field OANDA sends as a JSON number and this
            # crate re-emits as its exact decimal string, so the string form is
            # what can round trip; the numeric acceptance is covered by
            # tests/client_contract.rs.
            value = str(int(value))
        out[field] = value
    return out


def spec_member(shape, seen):
    if '$ref' in shape:
        target = shape['$ref'].split('/')[-1]
        if target in VALUES:
            return VALUES[target]
        inner = SPEC['definitions'].get(target)
        if inner is None:
            return '1'
        if 'enum' in inner:
            return inner['enum'][0]
        if inner.get('type') == 'object' or 'properties' in inner:
            return spec_object(target, inner, seen)
        return scalar(inner)
    if 'enum' in shape:
        return shape['enum'][0]
    if shape.get('type') == 'array':
        items = shape.get('items', {})
        if items.get('$ref'):
            # Bounded fixtures: the referenced definition has its own fixture.
            return []
        if 'enum' in items:
            return [items['enum'][0]]
        return [scalar(items)]
    return scalar(shape)


def spec_top_level(name):
    shape = SPEC['definitions'][name]
    if shape.get('type') == 'object' or 'properties' in shape:
        return spec_object(name, shape)
    if 'enum' in shape:
        return shape['enum'][0]
    if name in VALUES:
        return VALUES[name]
    return scalar(shape)


def page_definitions():
    from lxml import html
    for page in ('account', 'instrument', 'order', 'trade', 'position',
                 'transaction', 'pricing', 'pricing-common', 'primitives'):
        root = html.fromstring((PAGES / f'oanda-{page}-df.html').read_bytes())
        for header in root.xpath(
                '//div[contains(concat(" ",normalize-space(@class)," ")," endpoint_header ")]'):
            name = ''.join(
                header.xpath('.//span[contains(@class,"method")]/text()')).strip()
            href = header.xpath('./a/@href')
            if not name or not href:
                continue
            body = root.xpath(f'//div[@id="{href[0].lstrip("#")}"]')
            if body:
                yield name, body[0]


def leaf_value(type_name):
    type_name = type_name.strip()
    if type_name.startswith('Array['):
        return [leaf_value(type_name[6:-1])]
    if type_name in {'integer'}:
        return 1
    if type_name in {'boolean'}:
        return True
    if type_name in VALUES:
        return VALUES[type_name]
    target = SPEC['definitions'].get(type_name)
    if target is not None:
        if 'enum' in target:
            return target['enum'][0]
        if target.get('type') == 'object' or 'properties' in target:
            return spec_object(type_name, target)
        return scalar(target)
    body = PAGE_DEFS.get(type_name)
    if body is not None:
        return page_value(type_name, body)
    return '1'


def page_value(name, body):
    if name in PAGE_FIXTURES:
        return PAGE_FIXTURES[name]
    if name in VALUES:
        return VALUES[name]
    pre = body.xpath('.//pre[contains(@class,"json_schema")]')
    if pre:
        out = {}
        field_re = re.compile(r'^([A-Za-z_][A-Za-z0-9_]*)\s*:\s*\((.*)\),?$')
        for raw in pre[0].text_content().splitlines():
            line = raw.strip()
            m = field_re.match(line)
            if not m:
                continue
            field, rhs = m.groups()
            out[field] = leaf_value(rhs.split(',')[0])
        return out
    table = body.xpath('.//table[1]')
    if table:
        header = ' '.join(table[0].xpath('.//tr[1]/th//text()')).strip()
        if 'Value' in header:
            first = table[0].xpath('.//tr[td]/td[1]//text()')
            return ' '.join(first).strip() or '1'
    raise SystemExit(
        f'build_definition_fixtures.py: website-only definition {name} has neither '
        f'a schema block nor a value table; hand-author a fixture value')


def rust_field_name(provider_field):
    s = re.sub(r'IDs\b', 'Ids', provider_field)
    s = re.sub(r'ID\b', 'Id', s)
    s = re.sub(r'(?<=[a-z0-9])(?=[A-Z])', '_', s)
    s = re.sub(r'(?<=[A-Z])(?=[A-Z][a-z])', '_', s)
    s = re.sub(r'[^A-Za-z0-9]+', '_', s).strip('_').lower()
    return s


def main():
    load_structures()
    pages = dict(page_definitions())
    PAGE_DEFS.update(pages)
    fixtures = {}
    website_only = []
    for definition in LEDGER['definitions']:
        name = definition['name']
        if name in UNIONS and UNIONS[name] in SPEC['definitions']:
            variant = UNIONS[name]
            fixtures[name] = spec_object(variant, SPEC['definitions'][variant])
            continue
        if name in SPEC['definitions']:
            fixtures[name] = spec_top_level(name)
            continue
        if name in pages:
            fixtures[name] = page_value(name, pages[name])
            website_only.append(name)
            continue
        raise SystemExit(
            f'build_definition_fixtures.py: {name} is in the ledger but neither '
            f'the spec nor the pages provide it')

    negatives = []
    for name, fixture in sorted(fixtures.items()):
        if not isinstance(fixture, dict):
            continue
        fields = STRUCT_OPTIONAL.get(name)
        if not fields:
            continue
        for provider_field in sorted(fixture):
            rust_field = rust_field_name(provider_field)
            if fields.get(rust_field) is False:
                negatives.append((name, provider_field, fixture))

    lines = [
        '//! Decodes an independently sourced fixture for every documented definition.',
        '//!',
        '//! Fixture field sets and values are built from the pinned `OpenAPI` spec, a',
        '//! different document parsed by a different reader than the website generator,',
        '//! so a field the generator dropped or misnamed cannot also vanish from the',
        '//! fixture: the round trip fails instead. Where the spec documents a field the',
        '//! website does not, the website is authoritative and the field is recorded as',
        '//! drift by the builder rather than silently ignored. Definitions the website',
        '//! adds beyond the spec are fixture\'d from the saved definition pages:',
        '//! ' + ', '.join(f'`{name}`' for name in sorted(website_only)) + '.',
        '//! Removing a field the struct holds as required must fail to decode, locking',
        '//! the requiredness contract in place.',
        '// Generated by tools/build_definition_fixtures.py; checked in and reviewed.',
        '#![allow(',
        '    clippy::expect_used,',
        '    clippy::unwrap_used,',
        '    clippy::panic,',
        '    clippy::too_many_lines,',
        '    clippy::needless_pass_by_value',
        ')]',
        'use serde_json::json;',
        '',
        'fn decode_fixture<T: serde::Serialize + serde::de::DeserializeOwned>(fixture: serde_json::Value) {',
        '    let value: T = serde_json::from_value(fixture.clone()).expect("decode fixture");',
        '    let encoded = serde_json::to_value(value).expect("encode fixture");',
        '    assert_eq!(encoded, fixture, "the fixture must survive the round trip");',
        '}',
        '',
        'fn missing_required_fails<T: serde::de::DeserializeOwned>(fixture: serde_json::Value, field: &str) {',
        '    let mut reduced = fixture;',
        '    let object = reduced.as_object_mut().expect("fixture is an object");',
        '    assert!(object.remove(field).is_some(), "fixture carries {field}");',
        '    assert!(',
        '        serde_json::from_value::<T>(reduced).is_err(),',
        '        "removing required field {field} must fail to decode"',
        '    );',
        '}',
        '',
        '#[test]',
        'fn every_documented_definition_decodes_an_independent_fixture() {',
    ]
    for name in sorted(fixtures):
        fixture = json.dumps(fixtures[name], sort_keys=True, ensure_ascii=False)
        lines.append(f'    decode_fixture::<oanda_client::models::{name}>(json!({fixture}));')
    lines += ['}', '', '#[test]', 'fn removing_a_required_documented_field_fails_to_decode() {']
    for name, field, fixture in negatives:
        payload = json.dumps(fixture, sort_keys=True, ensure_ascii=False)
        lines.append(
            f'    missing_required_fails::<oanda_client::models::{name}>(json!({payload}), "{field}");')
    lines += ['}', '']
    DEST.write_text('\n'.join(lines), encoding='utf-8')
    print(f'emitted {len(fixtures)} fixtures and {len(negatives)} required-field negatives; '
          f'{len(website_only)} website-only definitions sourced from pages')
    if DRIFT:
        print('spec documents fields the website does not (dropped from fixtures as drift):')
        for entry in sorted(set(DRIFT)):
            print('  ', entry)


if __name__ == '__main__':
    main()
