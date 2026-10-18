<!--
SPDX-FileCopyrightText: 2026 Kevin Monaghan
SPDX-License-Identifier: MIT-0
-->

# Fill that opens a trade

> A synthetic OANDA `ORDER_FILL` in the shape OANDA writes for a market order that opens a
> trade, decoded by `tests/open_fill.rs`.

| File | Transaction |
| --- | --- |
| `order-fill.json` | 52, `ORDER_FILL` of 51, reason `MARKET_ORDER`, opening trade 52 |

The file is a response body of `GET /v3/accounts/{accountID}/transactions/{id}`. The shape
is that of the opening fills in a Practice hedged-account recording from 2026-10-12. Their
`tradeOpened` entries carry `guaranteedExecutionFeeHomeConversionCost`, a field OANDA does not
document, as a decimal string. The fill itself carries `requestedUnits` and the four
home-conversion costs.

> [!NOTE]
> Every value is synthetic: ids, times, prices, liquidity, amounts, units, batch, request,
> client and account ids. The field set and the JSON types are the ones OANDA sent; the
> `fullPrice` depth is shortened to one level per side.
