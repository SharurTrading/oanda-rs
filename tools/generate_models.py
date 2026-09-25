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
    if not clean: clean = 'OANDA v20 field.'
    out=[]
    while clean:
        chunk=clean[:105]
        if len(clean)>105 and ' ' in chunk: chunk=chunk.rsplit(' ',1)[0]
        out.append(pad+'/// '+chunk)
        clean=clean[len(chunk):].strip()
    return out


def fields(pre) -> list[tuple[str, str, bool, str]]:
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
        required=any(x.strip()=='required' for x in parts[1:])
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
    lines=docs(desc)
    lines+=['#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]', '#[serde(rename_all = "camelCase")]','pub struct '+name+' {']
    seen=set()
    for fname,t,required,fielddesc in fields(pre):
        fid=ident(fname)
        if fid in seen: continue
        seen.add(fid)
        lines+=docs(fielddesc, '    ')
        lines+=['    #[serde(rename = '+repr(fname).replace("'",'"') + ('' if required else ', default, skip_serializing_if = "Option::is_none"')+')]']
        lines+=['    pub '+fid+': '+(t if required else 'Option<'+t+'>')+',']
    lines+=['}','']
    return lines


def emit_enum(name, desc, table):
    rows=table.xpath('.//tr')
    values=[]
    for row in rows[1:]:
        val=' '.join(row.xpath('./td[1]//text()')).strip()
        description=' '.join(row.xpath('./td[2]//text()')).strip()
        if val: values.append((val,description))
    if not values: return []
    lines=docs(desc)+['#[derive(Debug, Clone, PartialEq, Eq, Hash)]','#[non_exhaustive]','pub enum '+name+' {']
    used=set()
    for value,description in values:
        var=variant(value)
        if var in used: var+='Value'
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


def main():
    DEST.mkdir(parents=True,exist_ok=True)
    mods=[]
    names=[]
    for n in NAMES:
        path=SOURCE / f'oanda-{n}-df.html'
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
                if name=='CandleSpecification':
                    lines+=docs(desc)+['pub type CandleSpecification = String;',''];continue
                if name=='PricingComponent':
                    lines+=docs(desc)+['pub type PricingComponent = String;',''];continue
            lines+=docs(desc)+['pub type '+name+' = String;','']
        (DEST/f'{mod}.rs').write_text('\n'.join(lines)+'\n')
    modlines=['//! Provider-native OANDA v20 models.','pub use crate::ids::*;', 'pub use crate::timestamp::Timestamp as DateTime;', '/// Exact OANDA decimal number.','pub type DecimalNumber = rust_decimal::Decimal;', '/// Exact account-currency units.','pub type AccountUnits = rust_decimal::Decimal;', '/// Exact provider price.','pub type PriceValue = rust_decimal::Decimal;', 'mod variants;', 'pub use variants::*;', '']
    for mod in mods: modlines += [f'mod {mod};',f'pub use {mod}::*;']
    (DEST/'mod.rs').write_text('\n'.join(modlines)+'\n')
    print('generated',len(names),'definitions')

if __name__=='__main__':main()
