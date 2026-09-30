"""Generate website-inventoried v20 operation DTOs and methods.

Requires lxml and saved current OANDA endpoint HTML pages in the named directory.
Outputs are checked in and reviewed, with website URLs in docs/coverage.json.
"""
from __future__ import annotations
import json, re, sys
from pathlib import Path
from lxml import html
from generate_models import fields, docs, ident, typ, fail

ROOT=Path(__file__).resolve().parents[1]
SOURCE=Path(sys.argv[1]) if len(sys.argv)>1 else Path('/private/tmp')
CAPS=('account','order','trade','position','transaction','pricing')
METHODS={
'account':['list_accounts','account_details','account_summary','account_instruments','configure_account','account_changes'],
'order':['create_order','list_orders','list_pending_orders','order_details','replace_order','cancel_order','update_order_client_extensions'],
'trade':['list_trades','list_open_trades','trade_details','close_trade','update_trade_client_extensions','set_trade_dependent_orders'],
'position':['list_positions','list_open_positions','position_details','close_position'],
'transaction':['list_transactions','transaction_details','transactions_by_id_range','transactions_since_id','stream_transactions'],
'pricing':['latest_candles','pricing','stream_pricing','instrument_candles'],
}
PATH_TYPES={'accountID':'AccountID','orderSpecifier':'OrderSpecifier','tradeSpecifier':'TradeSpecifier','transactionID':'TransactionID','instrument':'InstrumentName'}

def camel(s): return ''.join(x.capitalize() for x in s.split('_'))
def simple_type(s):
    s=' '.join(s.split())
    if s.startswith('List of '):
        x=s.removeprefix('List of ').removesuffix(' (csv)').strip()
        return 'Vec<'+typ(x)+'>'
    return typ(s)

def make_struct(name, desc, fs, all_optional=False):
    out=docs(desc)+['#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]', '#[serde(rename_all = "camelCase")]','pub struct '+name+' {']
    for fname,t,required,fd in fs:
        field=ident(fname)
        out+=docs(fd, '    ')
        if name=='SetTradeDependentOrdersBody' and fname in {'takeProfit','stopLoss','guaranteedStopLoss','trailingStopLoss'}:
            out+=['    #[serde(rename = "'+fname+'", default, skip_serializing_if = "crate::Patch::is_unchanged")]']
            out+=['    pub '+field+': crate::Patch<'+t+'>,']
            continue
        optional=all_optional or not required
        out+=['    #[serde(rename = "'+fname+'"'+(', default, skip_serializing_if = "Option::is_none"' if optional else '')+')]']
        out+=['    pub '+field+': '+('Option<'+t+'>' if optional else t)+',']
    out+=['}','']
    if all_optional: out.insert(len(docs(desc))+1,'#[derive(Default)]')
    return out

def endpoint_chunks(page):
    raw=page.read_text()
    starts=[m.start() for m in re.finditer(r'<div class="endpoint_header method_',raw)]
    for i,start in enumerate(starts):
        part=raw[start:starts[i+1] if i+1<len(starts) else len(raw)]
        yield html.fromstring(part)

def parse_params(root, label):
    table=root.xpath('.//div[contains(@id,"_parameters")]//table[contains(@class,"parameter_table")][1]')
    if not table:
        fail(f'{label}: no parameter table; OANDA documents Authorization on every endpoint')
    out=[]
    for tr in table[0].xpath('.//tr[td]'):
        tds=tr.xpath('./td')
        if len(tds)<4: continue
        name=tds[0].text_content().strip(); loc=tds[1].text_content().strip()
        type_=simple_type(tds[2].text_content())
        desc=' '.join(tds[3].text_content().split())
        out.append((name,loc,type_,'[required]' in desc,desc))
    if not any(p[3] for p in out):
        fail(f'{label}: no parameter is marked [required]; requiredness cannot be read from this page')
    return out

def first_body(root, label):
    body=root.xpath('.//div[contains(@class,"body_schema")]//pre[contains(@class,"json_schema")]')
    return fields(body[0], label+' request body') if body else []

def response(root, label):
    for el in root.xpath('.//div[@id]'):
        id_=el.get('id','')
        if re.match(r'collapse_\d+_2\d\d$',id_):
            pre=el.xpath('.//pre[contains(@class,"json_schema")]')
            if pre:return fields(pre[0], label+' success response')
    return []

def rejections(root):
    """Fields of the endpoint's documented rejection schema, or None.

    None means the page documents no rejection body for this endpoint; the
    generated method then rejects with the reviewed GenericRejection type.
    Nothing is invented here.
    """
    found={}
    for el in root.xpath('.//div[@id]'):
        id_=el.get('id','')
        if not re.match(r'collapse_\d+_[345]\d\d$',id_): continue
        pre=el.xpath('.//pre[contains(@class,"json_schema")]')
        if not pre:continue
        for fname,t,required,desc in fields(pre[0], id_):
            found.setdefault(fname,(fname,t,False,desc))
    return list(found.values()) if found else None

def emit_method(name, verb, path, params, has_body, has_query, rejection_type):
    class_name=camel(name)
    path_params=[p for p in params if p[1]=='path']
    has_header=any(p[0]=='ClientRequestID' for p in params)
    args=[ident(p[0])+': &'+PATH_TYPES.get(p[0],p[2]) for p in path_params]
    if has_query:
        required=any(p[3] for p in params if p[1]=='query')
        args+=['query: '+('&' if required else 'Option<&')+class_name+'Query'+('' if required else '>')]
    if has_body:args+=['body: &'+class_name+'Body']
    if has_header:args+=['client_request_id: Option<&ClientRequestID>']
    path_expr='"'+re.sub(r'\{([^}]+)\}',lambda m:'{'+ident(m.group(1))+'_encoded}',path)+'"'
    out=['    /// '+name.replace('_',' ').capitalize()+'.', '    pub async fn '+name+'(&self, '+', '.join(args)+') -> std::result::Result<crate::ApiResponse<'+class_name+'Response>, crate::OperationError<'+rejection_type+'>> {']
    if path_params:
        for p in path_params:out+=['        let '+ident(p[0])+'_encoded = crate::client::segment('+ident(p[0])+'.as_str());']
        out+=['        let path = format!('+path_expr+');']
    else:out+=['        let path = '+path_expr+'.to_owned();']
    q=('Some(query)' if has_query and any(p[3] for p in params if p[1]=='query') else 'query' if has_query else 'None::<&()>')
    b='Some(body)' if has_body else 'None::<&()>'
    account='Some(account_id)' if verb in ('POST','PUT','PATCH','DELETE') else 'None'
    cid='client_request_id' if has_header else 'None'
    out+=['        self.execute(Method::'+verb+', &path, '+q+', '+b+', '+account+', '+cid+').await','    }','']
    return out

def main(source=None, root_dir=None):
    source = Path(source) if source else SOURCE
    root_dir = Path(root_dir) if root_dir else ROOT
    previous_path=root_dir/'docs'/'coverage.json'
    previous=json.loads(previous_path.read_text()) if previous_path.exists() else {'operations':[],'definitions':[]}
    old_operations={(o['method'],o['path']):o for o in previous['operations']}
    old_definitions={d['name']:d for d in previous['definitions']}
    manifest={'reviewed_at':'2026-09-25','authority':'https://developer.oanda.com/rest-live-v20/introduction/','openapi_commit':'70324cfee31ff0074ed0bf1f93e67d8ee6c84444','operations':[],'definitions':[]}
    outputs={}
    for cap in CAPS:
        lines=['//! OANDA '+cap+' endpoint contracts.', '// Generated shared imports vary by capability; unused ones are deliberately allowed.', '#[allow(unused_imports)]','use crate::{Client, Result};','#[allow(unused_imports)]','use crate::ids::*;','#[allow(unused_imports)]','use crate::models::*;','#[allow(unused_imports)]','use crate::Timestamp;','#[allow(unused_imports)]','use reqwest::Method;','#[allow(unused_imports)]','use rust_decimal::Decimal;','#[allow(unused_imports)]','use serde::{Serialize, Deserialize};','']
        chunks=list(endpoint_chunks(source/f'oanda-{cap}-ep.html'))
        for i,root in enumerate(chunks):
            name=METHODS[cap][i]
            method=''.join(root.xpath('.//span[contains(@class,"method")]/text()')).strip()
            path=''.join(root.xpath('.//span[contains(@class,"path")]/text()')).strip()
            summary=' '.join(root.xpath('.//span[contains(@class,"path")]/p//text()')).strip()
            label=f'{cap}-ep {method} {path}'
            params=parse_params(root, label)
            query=[(p[0],p[2],p[3],p[4]) for p in params if p[1]=='query']
            body=first_body(root, label)
            reject=rejections(root)
            stream=path.endswith('/stream')
            resp=None if stream else response(root, label)
            entry={'method':method,'path':path,'public_method':name,'source':f'https://developer.oanda.com/rest-live-v20/{cap}-ep/','kind':'stream' if stream else ('mutation' if method!='GET' else 'query'),'rejection':'stream' if stream else ('endpoint' if reject else 'generic'),'status':'inventoried','test':None}
            old=old_operations.get((method,path),{})
            entry['status']=old.get('status','inventoried')
            entry['test']=old.get('test')
            manifest['operations'].append(entry)
            cname=camel(name)
            if stream:
                if query:lines+=make_struct(cname+'Query',name+' stream query parameters.',query,all_optional=not any(p[2] for p in query))
                continue
            if query:lines+=make_struct(cname+'Query',name+' query parameters.',query,all_optional=not any(p[2] for p in query))
            if body:lines+=make_struct(cname+'Body',name+' request body.',body)
            if not resp:
                fail(f'{label}: no success response schema; refusing to emit a struct that accepts any payload')
            lines+=make_struct(cname+'Response',name+' successful response.',resp)
            if reject:
                lines+=make_struct(cname+'Rejection',name+' documented rejection fields.',reject,all_optional=True)
                rejection_type=cname+'Rejection'
            else:
                rejection_type='crate::GenericRejection'
            lines+=['impl Client {']+emit_method(name,method,path,params,bool(body),bool(query),rejection_type)+['}','']
        outputs[cap]=lines
    for cap in ('account','instrument','order','trade','position','transaction','pricing','pricing-common','primitives'):
        root=html.fromstring((source/f'oanda-{cap}-df.html').read_bytes())
        for h in root.xpath('//div[contains(concat(" ",normalize-space(@class)," ")," endpoint_header ")]'):
            name=''.join(h.xpath('.//span[contains(@class,"method")]/text()')).strip()
            if name:
                old=old_definitions.get(name,{})
                manifest['definitions'].append({'name':name,'source':f'https://developer.oanda.com/rest-live-v20/{cap}-df/','status':old.get('status','inventoried'),'test':old.get('test')})
    for cap,lines in outputs.items():
        (root_dir/'src'/f'{cap}.rs').write_text('\n'.join(lines)+'\n')
    (root_dir/'docs'/'coverage.json').write_text(json.dumps(manifest,indent=2)+'\n')
    print('generated',len(manifest['operations']),'operations and',len(manifest['definitions']),'definitions')
if __name__=='__main__':main()
