<!--
SPDX-FileCopyrightText: 2026 Kevin Monaghan
SPDX-License-Identifier: MIT-0
-->

# Close of a flat hedged side

> Synthetic OANDA transactions in the shape OANDA writes for a `close_position` whose long
> side is already flat, decoded by `tests/close_flat_side.rs`.

| File | Transaction |
| --- | --- |
| `market-order.json` | 41, `MARKET_ORDER`, reason `POSITION_CLOSEOUT`, the closeout of the open long side |
| `order-fill.json` | 42, `ORDER_FILL` of 41, closing trade 39 |
| `order-cancel.json` | 43, `ORDER_CANCEL` of resting order 40, `CLIENT_REQUEST` |
| `market-order-reject.json` | 44, `MARKET_ORDER_REJECT`, reason `POSITION_CLOSEOUT`, reject reason `CLOSEOUT_POSITION_DOESNT_EXIST` |

Each file is a response body of `GET /v3/accounts/{accountID}/transactions/{id}`. The shape
was observed on an OANDA Practice hedged account on 2026-10-16: a `close_position` with
`longUnits: ALL`, sent after the long side had already been closed, was rejected, and the
reject carries no `units` field. The requested size is only in `longPositionCloseout`.

> [!NOTE]
> Every value is synthetic: ids, times, prices, liquidity, amounts, units, batch, request,
> user, client and account ids. The field set, field order and JSON types are the ones OANDA
> sent, and enum values and the closeout `units: ALL` are as observed.
