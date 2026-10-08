"""Generate provider DTOs from saved OANDA definition HTML pages.

Usage: python3 tools/generate_models.py /directory/containing/oanda-*-df.html
Requires lxml. Generated Rust is checked in and reviewed before use.
"""
from __future__ import annotations
import re
import sys
from pathlib import Path
from lxml import html

NAMES = ('account', 'instrument', 'order', 'trade', 'position', 'transaction', 'pricing', 'pricing-common', 'primitives')
DEST = Path(__file__).resolve().parents[1] / 'src' / 'models'
SOURCE = Path(sys.argv[1]) if len(sys.argv) > 1 else Path('/private/tmp')
SCALARS = {'DecimalNumber': 'Decimal', 'AccountUnits': 'Decimal', 'PriceValue': 'Decimal', 'DateTime': 'Timestamp', 'string': 'String', 'integer': 'i64', 'boolean': 'bool', 'integer or decimal if available': 'Decimal'}
IDS = {'AccountID', 'OrderID', 'TradeID', 'TransactionID', 'InstrumentName', 'OrderSpecifier', 'TradeSpecifier', 'ClientID', 'ClientTag', 'ClientComment', 'RequestID', 'ClientRequestID', 'Currency', 'CandleSpecification', 'PricingComponent'}
SKIP = {'DecimalNumber', 'AccountUnits', 'PriceValue', 'DateTime', 'Order', 'OrderRequest', 'Transaction'} | IDS


def fail(message: str):
    """A generator must never substitute a plausible default for provider content.

    Every parse miss is a hard failure: non-zero exit, nothing written, the
    offending definition named on stderr.
    """
    raise SystemExit(f'generate_models.py: {message}')


def ident(s: str) -> str:
    s = re.sub(r'IDs\b', 'Ids', s)
    s = re.sub(r'ID\b', 'Id', s)
    s = re.sub(r'(?<=[a-z0-9])(?=[A-Z])', '_', s)
    s = re.sub(r'(?<=[A-Z])(?=[A-Z][a-z])', '_', s)
    s = re.sub(r'[^A-Za-z0-9]+', '_', s).strip('_').lower()
    if s in {'type', 'ref', 'match', 'self', 'use', 'mod', 'where', 'in', 'loop', 'move', 'final', 'as'}:
        return 'r#' + s
    if s and s[0].isdigit():
        return 'v_' + s
    return s


def variant(s: str) -> str:
    parts = re.split(r'[^A-Za-z0-9]+', s)
    name = ''.join(p[:1].upper() + p[1:].lower() for p in parts if p)
    if not name or name[0].isdigit(): name = 'V' + name
    return name


def typ(s: str) -> str:
    s = s.strip()
    if s.startswith('Array[') and s.endswith(']'):
        return 'Vec<' + typ(s[6:-1]) + '>'
    return SCALARS.get(s, s)


def docs(text: str, pad='') -> list[str]:
    clean = ' '.join(text.split()).replace('*/', '').replace('[', r'\[').replace(']', r'\]')
    out=[]
    while clean:
        chunk=clean[:105]
        if len(clean)>105 and ' ' in chunk: chunk=chunk.rsplit(' ',1)[0]
        out.append(pad+'/// '+chunk)
        clean=clean[len(chunk):].strip()
    return out


def fields(pre, label: str) -> list[tuple[str, str, bool, str]]:
    out=[]; comments=[]
    for raw in pre.text_content().splitlines():
        line=raw.strip()
        if line.startswith('#'):
            comments.append(line.lstrip('#').strip())
            continue
        m=re.match(r'^([A-Za-z_][A-Za-z0-9_]*)\s*:\s*\((.*)\),?$', line)
        if not m: continue
        fname, rhs = m.groups()
        parts=rhs.split(',')
        t=typ(parts[0])
        required=False
        for part in parts[1:]:
            token=part.strip()
            if token=='required':
                required=True
            elif token=='deprecated' or token.startswith('default='):
                pass
            else:
                fail(f'{label}.{fname}: unrecognized schema token {token!r}; refusing to guess requiredness')
        desc=' '.join(x for x in comments if x).strip()
        comments=[]
        out.append((fname,t,required,desc))
    return out


def extract(root):
    for header in root.xpath('//div[contains(concat(" ",normalize-space(@class)," ")," endpoint_header ")]'):
        name=''.join(header.xpath('.//span[contains(@class,"method")]/text()')).strip()
        if not name: continue
        desc=' '.join(header.xpath('.//span[contains(@class,"definition")]//text()')).strip()
        href=header.xpath('.//a/@href')
        if not href: continue
        body=root.xpath(f'//div[@id="{href[0].lstrip("#")}"]')
        if not body: continue
        yield name, desc, body[0]


def emit_object(name, desc, pre):
    if not desc.strip():
        fail(f'{name}: definition has no provider description')
    lines=docs(desc)
    lines+=['#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]', '#[serde(rename_all = "camelCase")]','pub struct '+name+' {']
    seen=set()
    for fname,t,required,fielddesc in fields(pre, name):
        if not fielddesc.strip():
            fail(f'{name}.{fname}: field has no provider description')
        fid=ident(fname)
        if fid in seen: continue
        seen.add(fid)
        lines+=docs(fielddesc, '    ')
        serde_args = 'rename = '+repr(fname).replace("'",'"') + ('' if required else ', default, skip_serializing_if = "Option::is_none"')
        if t == 'Decimal':
            decoder = 'number_or_string' if required else 'optional_number_or_string'
            serde_args += ', deserialize_with = "crate::decimal_wire::' + decoder + '"'
        lines+=['    #[serde('+serde_args+')]']
        lines+=['    pub '+fid+': '+(t if required else 'Option<'+t+'>')+',']
    lines+=['}','']
    return lines


def emit_enum(name, desc, table):
    if not desc.strip():
        fail(f'{name}: definition has no provider description')
    rows=table.xpath('.//tr')
    values=[]
    for row in rows[1:]:
        val=' '.join(row.xpath('./td[1]//text()')).strip()
        description=' '.join(row.xpath('./td[2]//text()')).strip()
        if not val:
            # Same rule as generate_variants.values: any row without a value
            # cell is a failure, so a table-layout change cannot mean two
            # different things in the two tools.
            cells=[' '.join(td.text_content().split()) for td in row.xpath('./td')]
            fail(f'{name}: enum row has an empty value cell: {cells[:3]}')
        values.append((val,description))
    if not values:
        fail(f'{name}: value table has no parseable rows')
    for value,description in values:
        if not description.strip():
            fail(f'{name}.{value}: enum value has no provider description')
    lines=docs(desc)+['#[derive(Debug, Clone, PartialEq, Eq, Hash)]','#[non_exhaustive]','pub enum '+name+' {']
    used=set()
    for value,description in values:
        var=variant(value)
        if var in used:
            # Two provider values normalizing to one Rust ident stay lossless
            # (as_str keeps the exact spellings), but the rename is a decision,
            # not a silence: it is printed for the review of the diff.
            print(f'{name}: value {value!r} collides with an earlier variant; '
                  f'rust ident becomes {var}')
            var+='Value'
        used.add(var)
        lines+=docs(description, '    ')+['    '+var+',']
    lines+=['    /// An unrecognized provider value, preserved for forward compatibility.','    Unknown(String),','}', '']
    lines+=['impl '+name+' {', '    /// Return the exact provider spelling.', '    pub fn as_str(&self) -> &str {', '        match self {']
    used=set()
    for value,_ in values:
        var=variant(value)
        if var in used: var+='Value'
        used.add(var)
        lines+=['            Self::'+var+' => '+repr(value).replace("'",'"')+',']
    lines+=['            Self::Unknown(value) => value,','        }','    }','}', '']
    lines+=['impl Serialize for '+name+' {','    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {','        serializer.serialize_str(self.as_str())','    }','}', '']
    lines+=["impl<'de> Deserialize<'de> for "+name+' {',"    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {",'        let value = String::deserialize(deserializer)?;','        Ok(match value.as_str() {']
    used=set()
    for value,_ in values:
        var=variant(value)
        if var in used: var+='Value'
        used.add(var)
        lines+=['            '+repr(value).replace("'",'"')+' => Self::'+var+',']
    lines+=['            _ => Self::Unknown(value),','        })','    }','}', '']
    return lines


def main(source=None, dest=None):
    source = Path(source) if source else SOURCE
    dest = Path(dest) if dest else DEST
    mods=[]
    names=[]
    outputs={}
    for n in NAMES:
        path=source / f'oanda-{n}-df.html'
        root=html.fromstring(path.read_bytes())
        mod=n.replace('-','_')
        mods.append(mod)
        lines=['//! OANDA v20 '+n+' definitions.', '// Generated shared imports vary by definition family.', '#[allow(unused_imports)]','use serde::{Deserialize, Serialize};', '#[allow(unused_imports)]','use rust_decimal::Decimal;', '#[allow(unused_imports)]','use crate::ids::*;', '#[allow(unused_imports)]','use crate::timestamp::Timestamp;', '#[allow(unused_imports)]','use super::*;', '']
        for name,desc,body in extract(root):
            names.append((n,name))
            if name in SKIP: continue
            pre=body.xpath('.//pre[contains(@class,"json_schema")]')
            if pre:
                lines+=emit_object(name,desc,pre[0]);continue
            table=body.xpath('.//table[1]')
            if table:
                header=' '.join(table[0].xpath('.//tr[1]/th//text()')).strip()
                if 'Value' in header:
                    lines+=emit_enum(name,desc,table[0]);continue
            fail(f'{n}-df {name}: definition has neither a schema block nor a value table; '
                 f'refusing to emit a String alias the provider did not write')
        outputs[mod]=lines
    modlines=['//! Provider-native OANDA v20 models.','pub use crate::ids::*;', 'pub use crate::timestamp::Timestamp as DateTime;', '/// Exact OANDA decimal number.','pub type DecimalNumber = rust_decimal::Decimal;', '/// Exact account-currency units.','pub type AccountUnits = rust_decimal::Decimal;', '/// Exact provider price.','pub type PriceValue = rust_decimal::Decimal;', 'mod variants;', 'pub use variants::*;', '']
    for mod in mods: modlines += [f'mod {mod};',f'pub use {mod}::*;']
    outputs['__mod__']=modlines
    dest.mkdir(parents=True,exist_ok=True)
    for mod,lines in outputs.items():
        name='mod.rs' if mod=='__mod__' else f'{mod}.rs'
        (dest/name).write_text('\n'.join(lines)+'\n')
    print('generated',len(names),'definitions')

if __name__=='__main__':main()
