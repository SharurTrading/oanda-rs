"""Generate tagged provider Order, OrderRequest, and Transaction unions from website definitions."""
from pathlib import Path
from lxml import html
import sys
ROOT=Path(__file__).resolve().parents[1]
SOURCE=Path(sys.argv[1]) if len(sys.argv)>1 else Path('/private/tmp')

def definitions(page):
 root=html.fromstring((SOURCE/f'oanda-{page}-df.html').read_bytes())
 return {''.join(h.xpath('.//span[contains(@class,"method")]/text()')).strip() for h in root.xpath('//div[contains(@class,"endpoint_header")]')}

def values(page,name):
 root=html.fromstring((SOURCE/f'oanda-{page}-df.html').read_bytes())
 for h in root.xpath('//div[contains(@class,"endpoint_header")]'):
  n=''.join(h.xpath('.//span[contains(@class,"method")]/text()')).strip()
  if n!=name:continue
  href=h.xpath('./a/@href')[0].lstrip('#')
  body=root.xpath(f'//div[@id="{href}"]')[0]
  return [' '.join(r.xpath('./td[1]//text()')).strip() for r in body.xpath('.//table[1]//tr[td]')]
 return []

def camel(s):return ''.join(p.capitalize() for p in s.lower().split('_'))
def classify(vals,suffix,defs):
 out=[]
 for v in vals:
  expected=camel(v)+suffix
  candidates=[d for d in defs if d.lower()==expected.lower()]
  if len(candidates)==1:out.append((v,candidates[0]))
  else:print('missing',v,expected)
 return out

def emit(name,pairs,allow_unknown):
 lines=['/// Documented '+name+' variants identified by OANDA’s `type` field.','#[derive(Debug, Clone, PartialEq)]','#[non_exhaustive]','pub enum '+name+' {']
 for v,t in pairs:lines+=['    /// `'+v+'` provider variant.','    '+t+'('+t+'),']
 if allow_unknown:lines+=['    /// Future provider variant with its bounded raw value.','    Unknown {','        /// Provider discriminator.','        kind: String,','        /// Bounded provider object.','        raw: serde_json::Value,','    },']
 lines+=['}','']
 lines+=["impl<'de> Deserialize<'de> for "+name+' {',"    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self,D::Error> {",'        let value = serde_json::Value::deserialize(deserializer)?;','        let kind = value.get("type").and_then(serde_json::Value::as_str).ok_or_else(|| serde::de::Error::custom("missing OANDA type discriminator"))?;','        match kind {']
 for v,t in pairs:lines+=['            "'+v+'" => serde_json::from_value(value).map(Self::'+t+').map_err(serde::de::Error::custom),']
 if allow_unknown:lines+=['            other => Ok(Self::Unknown { kind: other.to_owned(), raw: value }),']
 else:lines+=['            _ => Err(serde::de::Error::custom("unsupported OANDA request type")),']
 lines+=['        }','    }','}','']
 lines+=['impl Serialize for '+name+' {','    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok,S::Error> {','        let (kind, mut value) = match self {']
 for v,t in pairs:lines+=['            Self::'+t+'(body) => ("'+v+'", serde_json::to_value(body).map_err(serde::ser::Error::custom)?),']
 if allow_unknown:lines+=['            Self::Unknown { kind, raw } => (kind.as_str(), raw.clone()),']
 lines+=['        };','        let object = value.as_object_mut().ok_or_else(|| serde::ser::Error::custom("OANDA variant must be an object"))?;','        object.insert("type".to_owned(), serde_json::Value::String(kind.to_owned()));','        value.serialize(serializer)','    }','}','']
 return lines

orders=definitions('order');transactions=definitions('transaction')
order_pairs=classify(values('order','OrderType'),'Order',orders)
request_pairs=[(v,t+'Request') for v,t in order_pairs if t+'Request' in orders]
transaction_pairs=classify(values('transaction','TransactionType'),'Transaction',transactions)
lines=['//! Tagged provider unions for orders and transactions.','use super::*;','use serde::{Deserialize, Serialize};','']
lines+=emit('Order',order_pairs,True)
lines+=emit('OrderRequest',request_pairs,False)
lines+=emit('Transaction',transaction_pairs,True)
(ROOT/'src/models/variants.rs').write_text('\n'.join(lines)+'\n')
print(len(order_pairs),len(request_pairs),len(transaction_pairs))
