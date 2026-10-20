//! OANDA v20 order definitions.
use super::{
    ClientExtensions, GuaranteedStopLossDetails, GuaranteedStopLossOrderLevelRestriction,
    MarketOrderDelayedTradeClose, MarketOrderMarginCloseout, MarketOrderPositionCloseout,
    MarketOrderTradeClose, StopLossDetails, TakeProfitDetails, TradeState, TrailingStopLossDetails,
};
use crate::ids::{ClientID, InstrumentName, OrderID, TradeID, TransactionID};
use crate::timestamp::Timestamp;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/// A `MarketOrder` is an order that is filled immediately upon creation using the current market price.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarketOrder {
    /// The Order’s identifier, unique within the Order’s Account.
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<OrderID>,
    /// The time when the Order was created.
    #[serde(
        rename = "createTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub create_time: Option<Timestamp>,
    /// The current state of the Order.
    #[serde(rename = "state", default, skip_serializing_if = "Option::is_none")]
    pub state: Option<OrderState>,
    /// The client extensions of the Order. Do not set, modify, or delete clientExtensions if your account is
    /// associated with MT4.
    #[serde(
        rename = "clientExtensions",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub client_extensions: Option<ClientExtensions>,
    /// The type of the Order. Always set to “MARKET” for Market Orders.
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    #[serde(serialize_with = "serialize_market_order_type")]
    pub r#type: Option<OrderType>,
    /// The Market Order’s Instrument.
    #[serde(rename = "instrument")]
    pub instrument: InstrumentName,
    /// The quantity requested to be filled by the Market Order. A positive number of units results in a long
    /// Order, and a negative number of units results in a short Order.
    #[serde(rename = "units")]
    #[serde(deserialize_with = "crate::decimal_wire::number_or_string")]
    pub units: Decimal,
    /// The time-in-force requested for the Market Order. Restricted to FOK or IOC for a `MarketOrder`.
    #[serde(rename = "timeInForce")]
    pub time_in_force: TimeInForce,
    /// The worst price that the client is willing to have the Market Order filled at.
    #[serde(
        rename = "priceBound",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    #[serde(deserialize_with = "crate::decimal_wire::optional_number_or_string")]
    pub price_bound: Option<Decimal>,
    /// Specification of how Positions in the Account are modified when the Order is filled.
    #[serde(rename = "positionFill")]
    pub position_fill: OrderPositionFill,
    /// Details of the Trade requested to be closed, only provided when the Market Order is being used to
    /// explicitly close a Trade.
    #[serde(
        rename = "tradeClose",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub trade_close: Option<MarketOrderTradeClose>,
    /// Details of the long Position requested to be closed out, only provided when a Market Order is being used
    /// to explicitly closeout a long Position.
    #[serde(
        rename = "longPositionCloseout",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub long_position_closeout: Option<MarketOrderPositionCloseout>,
    /// Details of the short Position requested to be closed out, only provided when a Market Order is being
    /// used to explicitly closeout a short Position.
    #[serde(
        rename = "shortPositionCloseout",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub short_position_closeout: Option<MarketOrderPositionCloseout>,
    /// Details of the Margin Closeout that this Market Order was created for
    #[serde(
        rename = "marginCloseout",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub margin_closeout: Option<MarketOrderMarginCloseout>,
    /// Details of the delayed Trade close that this Market Order was created for
    #[serde(
        rename = "delayedTradeClose",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub delayed_trade_close: Option<MarketOrderDelayedTradeClose>,
    /// `TakeProfitDetails` specifies the details of a Take Profit Order to be created on behalf of a client. This
    /// may happen when an Order is filled that opens a Trade requiring a Take Profit, or when a Trade’s
    /// dependent Take Profit Order is modified directly through the Trade.
    #[serde(
        rename = "takeProfitOnFill",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub take_profit_on_fill: Option<TakeProfitDetails>,
    /// `StopLossDetails` specifies the details of a Stop Loss Order to be created on behalf of a client. This may
    /// happen when an Order is filled that opens a Trade requiring a Stop Loss, or when a Trade’s dependent
    /// Stop Loss Order is modified directly through the Trade.
    #[serde(
        rename = "stopLossOnFill",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub stop_loss_on_fill: Option<StopLossDetails>,
    /// `GuaranteedStopLossDetails` specifies the details of a Guaranteed Stop Loss Order to be created on behalf
    /// of a client. This may happen when an Order is filled that opens a Trade requiring a Guaranteed Stop
    /// Loss, or when a Trade’s dependent Guaranteed Stop Loss Order is modified directly through the Trade.
    #[serde(
        rename = "guaranteedStopLossOnFill",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub guaranteed_stop_loss_on_fill: Option<GuaranteedStopLossDetails>,
    /// `TrailingStopLossDetails` specifies the details of a Trailing Stop Loss Order to be created on behalf of a
    /// client. This may happen when an Order is filled that opens a Trade requiring a Trailing Stop Loss, or
    /// when a Trade’s dependent Trailing Stop Loss Order is modified directly through the Trade.
    #[serde(
        rename = "trailingStopLossOnFill",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub trailing_stop_loss_on_fill: Option<TrailingStopLossDetails>,
    /// Client Extensions to add to the Trade created when the Order is filled (if such a Trade is created). Do
    /// not set, modify, or delete tradeClientExtensions if your account is associated with MT4.
    #[serde(
        rename = "tradeClientExtensions",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub trade_client_extensions: Option<ClientExtensions>,
    /// ID of the Transaction that filled this Order (only provided when the Order’s state is FILLED)
    #[serde(
        rename = "fillingTransactionID",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub filling_transaction_id: Option<TransactionID>,
    /// Date/time when the Order was filled (only provided when the Order’s state is FILLED)
    #[serde(
        rename = "filledTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub filled_time: Option<Timestamp>,
    /// Trade ID of Trade opened when the Order was filled (only provided when the Order’s state is FILLED and a
    /// Trade was opened as a result of the fill)
    #[serde(
        rename = "tradeOpenedID",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub trade_opened_id: Option<TradeID>,
    /// Trade ID of Trade reduced when the Order was filled (only provided when the Order’s state is FILLED and
    /// a Trade was reduced as a result of the fill)
    #[serde(
        rename = "tradeReducedID",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub trade_reduced_id: Option<TradeID>,
    /// Trade IDs of Trades closed when the Order was filled (only provided when the Order’s state is FILLED and
    /// one or more Trades were closed as a result of the fill)
    #[serde(
        rename = "tradeClosedIDs",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub trade_closed_ids: Option<Vec<TradeID>>,
    /// ID of the Transaction that cancelled the Order (only provided when the Order’s state is CANCELLED)
    #[serde(
        rename = "cancellingTransactionID",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub cancelling_transaction_id: Option<TransactionID>,
    /// Date/time when the Order was cancelled (only provided when the state of the Order is CANCELLED)
    #[serde(
        rename = "cancelledTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub cancelled_time: Option<Timestamp>,
}

/// A `FixedPriceOrder` is an order that is filled immediately upon creation using a fixed price.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FixedPriceOrder {
    /// The Order’s identifier, unique within the Order’s Account.
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<OrderID>,
    /// The time when the Order was created.
    #[serde(
        rename = "createTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub create_time: Option<Timestamp>,
    /// The current state of the Order.
    #[serde(rename = "state", default, skip_serializing_if = "Option::is_none")]
    pub state: Option<OrderState>,
    /// The client extensions of the Order. Do not set, modify, or delete clientExtensions if your account is
    /// associated with MT4.
    #[serde(
        rename = "clientExtensions",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub client_extensions: Option<ClientExtensions>,
    /// The type of the Order. Always set to “`FIXED_PRICE`” for Fixed Price Orders.
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    #[serde(serialize_with = "serialize_fixed_price_order_type")]
    pub r#type: Option<OrderType>,
    /// The Fixed Price Order’s Instrument.
    #[serde(rename = "instrument")]
    pub instrument: InstrumentName,
    /// The quantity requested to be filled by the Fixed Price Order. A positive number of units results in a
    /// long Order, and a negative number of units results in a short Order.
    #[serde(rename = "units")]
    #[serde(deserialize_with = "crate::decimal_wire::number_or_string")]
    pub units: Decimal,
    /// The price specified for the Fixed Price Order. This price is the exact price that the Fixed Price Order
    /// will be filled at.
    #[serde(rename = "price")]
    #[serde(deserialize_with = "crate::decimal_wire::number_or_string")]
    pub price: Decimal,
    /// Specification of how Positions in the Account are modified when the Order is filled.
    #[serde(rename = "positionFill")]
    pub position_fill: OrderPositionFill,
    /// The state that the trade resulting from the Fixed Price Order should be set to.
    #[serde(rename = "tradeState")]
    pub trade_state: TradeState,
    /// `TakeProfitDetails` specifies the details of a Take Profit Order to be created on behalf of a client. This
    /// may happen when an Order is filled that opens a Trade requiring a Take Profit, or when a Trade’s
    /// dependent Take Profit Order is modified directly through the Trade.
    #[serde(
        rename = "takeProfitOnFill",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub take_profit_on_fill: Option<TakeProfitDetails>,
    /// `StopLossDetails` specifies the details of a Stop Loss Order to be created on behalf of a client. This may
    /// happen when an Order is filled that opens a Trade requiring a Stop Loss, or when a Trade’s dependent
    /// Stop Loss Order is modified directly through the Trade.
    #[serde(
        rename = "stopLossOnFill",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub stop_loss_on_fill: Option<StopLossDetails>,
    /// `GuaranteedStopLossDetails` specifies the details of a Guaranteed Stop Loss Order to be created on behalf
    /// of a client. This may happen when an Order is filled that opens a Trade requiring a Guaranteed Stop
    /// Loss, or when a Trade’s dependent Guaranteed Stop Loss Order is modified directly through the Trade.
    #[serde(
        rename = "guaranteedStopLossOnFill",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub guaranteed_stop_loss_on_fill: Option<GuaranteedStopLossDetails>,
    /// `TrailingStopLossDetails` specifies the details of a Trailing Stop Loss Order to be created on behalf of a
    /// client. This may happen when an Order is filled that opens a Trade requiring a Trailing Stop Loss, or
    /// when a Trade’s dependent Trailing Stop Loss Order is modified directly through the Trade.
    #[serde(
        rename = "trailingStopLossOnFill",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub trailing_stop_loss_on_fill: Option<TrailingStopLossDetails>,
    /// Client Extensions to add to the Trade created when the Order is filled (if such a Trade is created). Do
    /// not set, modify, or delete tradeClientExtensions if your account is associated with MT4.
    #[serde(
        rename = "tradeClientExtensions",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub trade_client_extensions: Option<ClientExtensions>,
    /// ID of the Transaction that filled this Order (only provided when the Order’s state is FILLED)
    #[serde(
        rename = "fillingTransactionID",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub filling_transaction_id: Option<TransactionID>,
    /// Date/time when the Order was filled (only provided when the Order’s state is FILLED)
    #[serde(
        rename = "filledTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub filled_time: Option<Timestamp>,
    /// Trade ID of Trade opened when the Order was filled (only provided when the Order’s state is FILLED and a
    /// Trade was opened as a result of the fill)
    #[serde(
        rename = "tradeOpenedID",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub trade_opened_id: Option<TradeID>,
    /// Trade ID of Trade reduced when the Order was filled (only provided when the Order’s state is FILLED and
    /// a Trade was reduced as a result of the fill)
    #[serde(
        rename = "tradeReducedID",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub trade_reduced_id: Option<TradeID>,
    /// Trade IDs of Trades closed when the Order was filled (only provided when the Order’s state is FILLED and
    /// one or more Trades were closed as a result of the fill)
    #[serde(
        rename = "tradeClosedIDs",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub trade_closed_ids: Option<Vec<TradeID>>,
    /// ID of the Transaction that cancelled the Order (only provided when the Order’s state is CANCELLED)
    #[serde(
        rename = "cancellingTransactionID",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub cancelling_transaction_id: Option<TransactionID>,
    /// Date/time when the Order was cancelled (only provided when the state of the Order is CANCELLED)
    #[serde(
        rename = "cancelledTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub cancelled_time: Option<Timestamp>,
}

/// A `LimitOrder` is an order that is created with a price threshold, and will only be filled by a price that
/// is equal to or better than the threshold.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LimitOrder {
    /// The Order’s identifier, unique within the Order’s Account.
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<OrderID>,
    /// The time when the Order was created.
    #[serde(
        rename = "createTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub create_time: Option<Timestamp>,
    /// The current state of the Order.
    #[serde(rename = "state", default, skip_serializing_if = "Option::is_none")]
    pub state: Option<OrderState>,
    /// The client extensions of the Order. Do not set, modify, or delete clientExtensions if your account is
    /// associated with MT4.
    #[serde(
        rename = "clientExtensions",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub client_extensions: Option<ClientExtensions>,
    /// The type of the Order. Always set to “LIMIT” for Limit Orders.
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    #[serde(serialize_with = "serialize_limit_order_type")]
    pub r#type: Option<OrderType>,
    /// The Limit Order’s Instrument.
    #[serde(rename = "instrument")]
    pub instrument: InstrumentName,
    /// The quantity requested to be filled by the Limit Order. A positive number of units results in a long
    /// Order, and a negative number of units results in a short Order.
    #[serde(rename = "units")]
    #[serde(deserialize_with = "crate::decimal_wire::number_or_string")]
    pub units: Decimal,
    /// The price threshold specified for the Limit Order. The Limit Order will only be filled by a market price
    /// that is equal to or better than this price.
    #[serde(rename = "price")]
    #[serde(deserialize_with = "crate::decimal_wire::number_or_string")]
    pub price: Decimal,
    /// The time-in-force requested for the Limit Order.
    #[serde(rename = "timeInForce")]
    pub time_in_force: TimeInForce,
    /// The date/time when the Limit Order will be cancelled if its timeInForce is “GTD”.
    #[serde(rename = "gtdTime", default, skip_serializing_if = "Option::is_none")]
    pub gtd_time: Option<Timestamp>,
    /// Specification of how Positions in the Account are modified when the Order is filled.
    #[serde(rename = "positionFill")]
    pub position_fill: OrderPositionFill,
    /// Specification of which price component should be used when determining if an Order should be triggered
    /// and filled. This allows Orders to be triggered based on the bid, ask, mid, default (ask for buy, bid for
    /// sell) or inverse (ask for sell, bid for buy) price depending on the desired behaviour. Orders are always
    /// filled using their default price component. This feature is only provided through the REST API. Clients
    /// who choose to specify a non-default trigger condition will not see it reflected in any of OANDA’s
    /// proprietary or partner trading platforms, their transaction history or their account statements. OANDA
    /// platforms always assume that an Order’s trigger condition is set to the default value when indicating
    /// the distance from an Order’s trigger price, and will always provide the default trigger condition when
    /// creating or modifying an Order. A special restriction applies when creating a Guaranteed Stop Loss
    /// Order. In this case the `TriggerCondition` value must either be “DEFAULT”, or the “natural” trigger side
    /// “DEFAULT” results in. So for a Guaranteed Stop Loss Order for a long trade valid values are “DEFAULT”
    /// and “BID”, and for short trades “DEFAULT” and “ASK” are valid.
    #[serde(rename = "triggerCondition")]
    pub trigger_condition: OrderTriggerCondition,
    /// `TakeProfitDetails` specifies the details of a Take Profit Order to be created on behalf of a client. This
    /// may happen when an Order is filled that opens a Trade requiring a Take Profit, or when a Trade’s
    /// dependent Take Profit Order is modified directly through the Trade.
    #[serde(
        rename = "takeProfitOnFill",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub take_profit_on_fill: Option<TakeProfitDetails>,
    /// `StopLossDetails` specifies the details of a Stop Loss Order to be created on behalf of a client. This may
    /// happen when an Order is filled that opens a Trade requiring a Stop Loss, or when a Trade’s dependent
    /// Stop Loss Order is modified directly through the Trade.
    #[serde(
        rename = "stopLossOnFill",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub stop_loss_on_fill: Option<StopLossDetails>,
    /// `GuaranteedStopLossDetails` specifies the details of a Guaranteed Stop Loss Order to be created on behalf
    /// of a client. This may happen when an Order is filled that opens a Trade requiring a Guaranteed Stop
    /// Loss, or when a Trade’s dependent Guaranteed Stop Loss Order is modified directly through the Trade.
    #[serde(
        rename = "guaranteedStopLossOnFill",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub guaranteed_stop_loss_on_fill: Option<GuaranteedStopLossDetails>,
    /// `TrailingStopLossDetails` specifies the details of a Trailing Stop Loss Order to be created on behalf of a
    /// client. This may happen when an Order is filled that opens a Trade requiring a Trailing Stop Loss, or
    /// when a Trade’s dependent Trailing Stop Loss Order is modified directly through the Trade.
    #[serde(
        rename = "trailingStopLossOnFill",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub trailing_stop_loss_on_fill: Option<TrailingStopLossDetails>,
    /// Client Extensions to add to the Trade created when the Order is filled (if such a Trade is created). Do
    /// not set, modify, or delete tradeClientExtensions if your account is associated with MT4.
    #[serde(
        rename = "tradeClientExtensions",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub trade_client_extensions: Option<ClientExtensions>,
    /// ID of the Transaction that filled this Order (only provided when the Order’s state is FILLED)
    #[serde(
        rename = "fillingTransactionID",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub filling_transaction_id: Option<TransactionID>,
    /// Date/time when the Order was filled (only provided when the Order’s state is FILLED)
    #[serde(
        rename = "filledTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub filled_time: Option<Timestamp>,
    /// Trade ID of Trade opened when the Order was filled (only provided when the Order’s state is FILLED and a
    /// Trade was opened as a result of the fill)
    #[serde(
        rename = "tradeOpenedID",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub trade_opened_id: Option<TradeID>,
    /// Trade ID of Trade reduced when the Order was filled (only provided when the Order’s state is FILLED and
    /// a Trade was reduced as a result of the fill)
    #[serde(
        rename = "tradeReducedID",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub trade_reduced_id: Option<TradeID>,
    /// Trade IDs of Trades closed when the Order was filled (only provided when the Order’s state is FILLED and
    /// one or more Trades were closed as a result of the fill)
    #[serde(
        rename = "tradeClosedIDs",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub trade_closed_ids: Option<Vec<TradeID>>,
    /// ID of the Transaction that cancelled the Order (only provided when the Order’s state is CANCELLED)
    #[serde(
        rename = "cancellingTransactionID",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub cancelling_transaction_id: Option<TransactionID>,
    /// Date/time when the Order was cancelled (only provided when the state of the Order is CANCELLED)
    #[serde(
        rename = "cancelledTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub cancelled_time: Option<Timestamp>,
    /// The ID of the Order that was replaced by this Order (only provided if this Order was created as part of
    /// a cancel/replace).
    #[serde(
        rename = "replacesOrderID",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub replaces_order_id: Option<OrderID>,
    /// The ID of the Order that replaced this Order (only provided if this Order was cancelled as part of a
    /// cancel/replace).
    #[serde(
        rename = "replacedByOrderID",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub replaced_by_order_id: Option<OrderID>,
}

/// A `StopOrder` is an order that is created with a price threshold, and will only be filled by a price that
/// is equal to or worse than the threshold.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StopOrder {
    /// The Order’s identifier, unique within the Order’s Account.
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<OrderID>,
    /// The time when the Order was created.
    #[serde(
        rename = "createTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub create_time: Option<Timestamp>,
    /// The current state of the Order.
    #[serde(rename = "state", default, skip_serializing_if = "Option::is_none")]
    pub state: Option<OrderState>,
    /// The client extensions of the Order. Do not set, modify, or delete clientExtensions if your account is
    /// associated with MT4.
    #[serde(
        rename = "clientExtensions",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub client_extensions: Option<ClientExtensions>,
    /// The type of the Order. Always set to “STOP” for Stop Orders.
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    #[serde(serialize_with = "serialize_stop_order_type")]
    pub r#type: Option<OrderType>,
    /// The Stop Order’s Instrument.
    #[serde(rename = "instrument")]
    pub instrument: InstrumentName,
    /// The quantity requested to be filled by the Stop Order. A positive number of units results in a long
    /// Order, and a negative number of units results in a short Order.
    #[serde(rename = "units")]
    #[serde(deserialize_with = "crate::decimal_wire::number_or_string")]
    pub units: Decimal,
    /// The price threshold specified for the Stop Order. The Stop Order will only be filled by a market price
    /// that is equal to or worse than this price.
    #[serde(rename = "price")]
    #[serde(deserialize_with = "crate::decimal_wire::number_or_string")]
    pub price: Decimal,
    /// The worst market price that may be used to fill this Stop Order. If the market gaps and crosses through
    /// both the price and the priceBound, the Stop Order will be cancelled instead of being filled.
    #[serde(
        rename = "priceBound",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    #[serde(deserialize_with = "crate::decimal_wire::optional_number_or_string")]
    pub price_bound: Option<Decimal>,
    /// The time-in-force requested for the Stop Order.
    #[serde(rename = "timeInForce")]
    pub time_in_force: TimeInForce,
    /// The date/time when the Stop Order will be cancelled if its timeInForce is “GTD”.
    #[serde(rename = "gtdTime", default, skip_serializing_if = "Option::is_none")]
    pub gtd_time: Option<Timestamp>,
    /// Specification of how Positions in the Account are modified when the Order is filled.
    #[serde(rename = "positionFill")]
    pub position_fill: OrderPositionFill,
    /// Specification of which price component should be used when determining if an Order should be triggered
    /// and filled. This allows Orders to be triggered based on the bid, ask, mid, default (ask for buy, bid for
    /// sell) or inverse (ask for sell, bid for buy) price depending on the desired behaviour. Orders are always
    /// filled using their default price component. This feature is only provided through the REST API. Clients
    /// who choose to specify a non-default trigger condition will not see it reflected in any of OANDA’s
    /// proprietary or partner trading platforms, their transaction history or their account statements. OANDA
    /// platforms always assume that an Order’s trigger condition is set to the default value when indicating
    /// the distance from an Order’s trigger price, and will always provide the default trigger condition when
    /// creating or modifying an Order. A special restriction applies when creating a Guaranteed Stop Loss
    /// Order. In this case the `TriggerCondition` value must either be “DEFAULT”, or the “natural” trigger side
    /// “DEFAULT” results in. So for a Guaranteed Stop Loss Order for a long trade valid values are “DEFAULT”
    /// and “BID”, and for short trades “DEFAULT” and “ASK” are valid.
    #[serde(rename = "triggerCondition")]
    pub trigger_condition: OrderTriggerCondition,
    /// `TakeProfitDetails` specifies the details of a Take Profit Order to be created on behalf of a client. This
    /// may happen when an Order is filled that opens a Trade requiring a Take Profit, or when a Trade’s
    /// dependent Take Profit Order is modified directly through the Trade.
    #[serde(
        rename = "takeProfitOnFill",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub take_profit_on_fill: Option<TakeProfitDetails>,
    /// `StopLossDetails` specifies the details of a Stop Loss Order to be created on behalf of a client. This may
    /// happen when an Order is filled that opens a Trade requiring a Stop Loss, or when a Trade’s dependent
    /// Stop Loss Order is modified directly through the Trade.
    #[serde(
        rename = "stopLossOnFill",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub stop_loss_on_fill: Option<StopLossDetails>,
    /// `GuaranteedStopLossDetails` specifies the details of a Guaranteed Stop Loss Order to be created on behalf
    /// of a client. This may happen when an Order is filled that opens a Trade requiring a Guaranteed Stop
    /// Loss, or when a Trade’s dependent Guaranteed Stop Loss Order is modified directly through the Trade.
    #[serde(
        rename = "guaranteedStopLossOnFill",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub guaranteed_stop_loss_on_fill: Option<GuaranteedStopLossDetails>,
    /// `TrailingStopLossDetails` specifies the details of a Trailing Stop Loss Order to be created on behalf of a
    /// client. This may happen when an Order is filled that opens a Trade requiring a Trailing Stop Loss, or
    /// when a Trade’s dependent Trailing Stop Loss Order is modified directly through the Trade.
    #[serde(
        rename = "trailingStopLossOnFill",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub trailing_stop_loss_on_fill: Option<TrailingStopLossDetails>,
    /// Client Extensions to add to the Trade created when the Order is filled (if such a Trade is created). Do
    /// not set, modify, or delete tradeClientExtensions if your account is associated with MT4.
    #[serde(
        rename = "tradeClientExtensions",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub trade_client_extensions: Option<ClientExtensions>,
    /// ID of the Transaction that filled this Order (only provided when the Order’s state is FILLED)
    #[serde(
        rename = "fillingTransactionID",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub filling_transaction_id: Option<TransactionID>,
    /// Date/time when the Order was filled (only provided when the Order’s state is FILLED)
    #[serde(
        rename = "filledTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub filled_time: Option<Timestamp>,
    /// Trade ID of Trade opened when the Order was filled (only provided when the Order’s state is FILLED and a
    /// Trade was opened as a result of the fill)
    #[serde(
        rename = "tradeOpenedID",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub trade_opened_id: Option<TradeID>,
    /// Trade ID of Trade reduced when the Order was filled (only provided when the Order’s state is FILLED and
    /// a Trade was reduced as a result of the fill)
    #[serde(
        rename = "tradeReducedID",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub trade_reduced_id: Option<TradeID>,
    /// Trade IDs of Trades closed when the Order was filled (only provided when the Order’s state is FILLED and
    /// one or more Trades were closed as a result of the fill)
    #[serde(
        rename = "tradeClosedIDs",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub trade_closed_ids: Option<Vec<TradeID>>,
    /// ID of the Transaction that cancelled the Order (only provided when the Order’s state is CANCELLED)
    #[serde(
        rename = "cancellingTransactionID",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub cancelling_transaction_id: Option<TransactionID>,
    /// Date/time when the Order was cancelled (only provided when the state of the Order is CANCELLED)
    #[serde(
        rename = "cancelledTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub cancelled_time: Option<Timestamp>,
    /// The ID of the Order that was replaced by this Order (only provided if this Order was created as part of
    /// a cancel/replace).
    #[serde(
        rename = "replacesOrderID",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub replaces_order_id: Option<OrderID>,
    /// The ID of the Order that replaced this Order (only provided if this Order was cancelled as part of a
    /// cancel/replace).
    #[serde(
        rename = "replacedByOrderID",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub replaced_by_order_id: Option<OrderID>,
}

/// A `MarketIfTouchedOrder` is an order that is created with a price threshold, and will only be filled by a
/// market price that touches or crosses the threshold.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarketIfTouchedOrder {
    /// The Order’s identifier, unique within the Order’s Account.
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<OrderID>,
    /// The time when the Order was created.
    #[serde(
        rename = "createTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub create_time: Option<Timestamp>,
    /// The current state of the Order.
    #[serde(rename = "state", default, skip_serializing_if = "Option::is_none")]
    pub state: Option<OrderState>,
    /// The client extensions of the Order. Do not set, modify, or delete clientExtensions if your account is
    /// associated with MT4.
    #[serde(
        rename = "clientExtensions",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub client_extensions: Option<ClientExtensions>,
    /// The type of the Order. Always set to “`MARKET_IF_TOUCHED`” for Market If Touched Orders.
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    #[serde(serialize_with = "serialize_market_if_touched_order_type")]
    pub r#type: Option<OrderType>,
    /// The `MarketIfTouched` Order’s Instrument.
    #[serde(rename = "instrument")]
    pub instrument: InstrumentName,
    /// The quantity requested to be filled by the `MarketIfTouched` Order. A positive number of units results in
    /// a long Order, and a negative number of units results in a short Order.
    #[serde(rename = "units")]
    #[serde(deserialize_with = "crate::decimal_wire::number_or_string")]
    pub units: Decimal,
    /// The price threshold specified for the `MarketIfTouched` Order. The `MarketIfTouched` Order will only be
    /// filled by a market price that crosses this price from the direction of the market price at the time when
    /// the Order was created (the initialMarketPrice). Depending on the value of the Order’s price and
    /// initialMarketPrice, the `MarketIfTouchedOrder` will behave like a Limit or a Stop Order.
    #[serde(rename = "price")]
    #[serde(deserialize_with = "crate::decimal_wire::number_or_string")]
    pub price: Decimal,
    /// The worst market price that may be used to fill this `MarketIfTouched` Order.
    #[serde(
        rename = "priceBound",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    #[serde(deserialize_with = "crate::decimal_wire::optional_number_or_string")]
    pub price_bound: Option<Decimal>,
    /// The time-in-force requested for the `MarketIfTouched` Order. Restricted to “GTC”, “GFD” and “GTD” for
    /// `MarketIfTouched` Orders.
    #[serde(rename = "timeInForce")]
    pub time_in_force: TimeInForce,
    /// The date/time when the `MarketIfTouched` Order will be cancelled if its timeInForce is “GTD”.
    #[serde(rename = "gtdTime", default, skip_serializing_if = "Option::is_none")]
    pub gtd_time: Option<Timestamp>,
    /// Specification of how Positions in the Account are modified when the Order is filled.
    #[serde(rename = "positionFill")]
    pub position_fill: OrderPositionFill,
    /// Specification of which price component should be used when determining if an Order should be triggered
    /// and filled. This allows Orders to be triggered based on the bid, ask, mid, default (ask for buy, bid for
    /// sell) or inverse (ask for sell, bid for buy) price depending on the desired behaviour. Orders are always
    /// filled using their default price component. This feature is only provided through the REST API. Clients
    /// who choose to specify a non-default trigger condition will not see it reflected in any of OANDA’s
    /// proprietary or partner trading platforms, their transaction history or their account statements. OANDA
    /// platforms always assume that an Order’s trigger condition is set to the default value when indicating
    /// the distance from an Order’s trigger price, and will always provide the default trigger condition when
    /// creating or modifying an Order. A special restriction applies when creating a Guaranteed Stop Loss
    /// Order. In this case the `TriggerCondition` value must either be “DEFAULT”, or the “natural” trigger side
    /// “DEFAULT” results in. So for a Guaranteed Stop Loss Order for a long trade valid values are “DEFAULT”
    /// and “BID”, and for short trades “DEFAULT” and “ASK” are valid.
    #[serde(rename = "triggerCondition")]
    pub trigger_condition: OrderTriggerCondition,
    /// The Market price at the time when the `MarketIfTouched` Order was created.
    #[serde(
        rename = "initialMarketPrice",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    #[serde(deserialize_with = "crate::decimal_wire::optional_number_or_string")]
    pub initial_market_price: Option<Decimal>,
    /// `TakeProfitDetails` specifies the details of a Take Profit Order to be created on behalf of a client. This
    /// may happen when an Order is filled that opens a Trade requiring a Take Profit, or when a Trade’s
    /// dependent Take Profit Order is modified directly through the Trade.
    #[serde(
        rename = "takeProfitOnFill",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub take_profit_on_fill: Option<TakeProfitDetails>,
    /// `StopLossDetails` specifies the details of a Stop Loss Order to be created on behalf of a client. This may
    /// happen when an Order is filled that opens a Trade requiring a Stop Loss, or when a Trade’s dependent
    /// Stop Loss Order is modified directly through the Trade.
    #[serde(
        rename = "stopLossOnFill",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub stop_loss_on_fill: Option<StopLossDetails>,
    /// `GuaranteedStopLossDetails` specifies the details of a Guaranteed Stop Loss Order to be created on behalf
    /// of a client. This may happen when an Order is filled that opens a Trade requiring a Guaranteed Stop
    /// Loss, or when a Trade’s dependent Guaranteed Stop Loss Order is modified directly through the Trade.
    #[serde(
        rename = "guaranteedStopLossOnFill",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub guaranteed_stop_loss_on_fill: Option<GuaranteedStopLossDetails>,
    /// `TrailingStopLossDetails` specifies the details of a Trailing Stop Loss Order to be created on behalf of a
    /// client. This may happen when an Order is filled that opens a Trade requiring a Trailing Stop Loss, or
    /// when a Trade’s dependent Trailing Stop Loss Order is modified directly through the Trade.
    #[serde(
        rename = "trailingStopLossOnFill",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub trailing_stop_loss_on_fill: Option<TrailingStopLossDetails>,
    /// Client Extensions to add to the Trade created when the Order is filled (if such a Trade is created). Do
    /// not set, modify, or delete tradeClientExtensions if your account is associated with MT4.
    #[serde(
        rename = "tradeClientExtensions",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub trade_client_extensions: Option<ClientExtensions>,
    /// ID of the Transaction that filled this Order (only provided when the Order’s state is FILLED)
    #[serde(
        rename = "fillingTransactionID",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub filling_transaction_id: Option<TransactionID>,
    /// Date/time when the Order was filled (only provided when the Order’s state is FILLED)
    #[serde(
        rename = "filledTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub filled_time: Option<Timestamp>,
    /// Trade ID of Trade opened when the Order was filled (only provided when the Order’s state is FILLED and a
    /// Trade was opened as a result of the fill)
    #[serde(
        rename = "tradeOpenedID",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub trade_opened_id: Option<TradeID>,
    /// Trade ID of Trade reduced when the Order was filled (only provided when the Order’s state is FILLED and
    /// a Trade was reduced as a result of the fill)
    #[serde(
        rename = "tradeReducedID",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub trade_reduced_id: Option<TradeID>,
    /// Trade IDs of Trades closed when the Order was filled (only provided when the Order’s state is FILLED and
    /// one or more Trades were closed as a result of the fill)
    #[serde(
        rename = "tradeClosedIDs",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub trade_closed_ids: Option<Vec<TradeID>>,
    /// ID of the Transaction that cancelled the Order (only provided when the Order’s state is CANCELLED)
    #[serde(
        rename = "cancellingTransactionID",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub cancelling_transaction_id: Option<TransactionID>,
    /// Date/time when the Order was cancelled (only provided when the state of the Order is CANCELLED)
    #[serde(
        rename = "cancelledTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub cancelled_time: Option<Timestamp>,
    /// The ID of the Order that was replaced by this Order (only provided if this Order was created as part of
    /// a cancel/replace).
    #[serde(
        rename = "replacesOrderID",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub replaces_order_id: Option<OrderID>,
    /// The ID of the Order that replaced this Order (only provided if this Order was cancelled as part of a
    /// cancel/replace).
    #[serde(
        rename = "replacedByOrderID",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub replaced_by_order_id: Option<OrderID>,
}

/// A `TakeProfitOrder` is an order that is linked to an open Trade and created with a price threshold. The
/// Order will be filled (closing the Trade) by the first price that is equal to or better than the
/// threshold. A `TakeProfitOrder` cannot be used to open a new Position.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TakeProfitOrder {
    /// The Order’s identifier, unique within the Order’s Account.
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<OrderID>,
    /// The time when the Order was created.
    #[serde(
        rename = "createTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub create_time: Option<Timestamp>,
    /// The current state of the Order.
    #[serde(rename = "state", default, skip_serializing_if = "Option::is_none")]
    pub state: Option<OrderState>,
    /// The client extensions of the Order. Do not set, modify, or delete clientExtensions if your account is
    /// associated with MT4.
    #[serde(
        rename = "clientExtensions",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub client_extensions: Option<ClientExtensions>,
    /// The type of the Order. Always set to “`TAKE_PROFIT`” for Take Profit Orders.
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    #[serde(serialize_with = "serialize_take_profit_order_type")]
    pub r#type: Option<OrderType>,
    /// The ID of the Trade to close when the price threshold is breached.
    #[serde(rename = "tradeID")]
    pub trade_id: TradeID,
    /// The client ID of the Trade to be closed when the price threshold is breached.
    #[serde(
        rename = "clientTradeID",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub client_trade_id: Option<ClientID>,
    /// The price threshold specified for the `TakeProfit` Order. The associated Trade will be closed by a market
    /// price that is equal to or better than this threshold.
    #[serde(rename = "price")]
    #[serde(deserialize_with = "crate::decimal_wire::number_or_string")]
    pub price: Decimal,
    /// The time-in-force requested for the `TakeProfit` Order. Restricted to “GTC”, “GFD” and “GTD” for
    /// `TakeProfit` Orders.
    #[serde(rename = "timeInForce")]
    pub time_in_force: TimeInForce,
    /// The date/time when the `TakeProfit` Order will be cancelled if its timeInForce is “GTD”.
    #[serde(rename = "gtdTime", default, skip_serializing_if = "Option::is_none")]
    pub gtd_time: Option<Timestamp>,
    /// Specification of which price component should be used when determining if an Order should be triggered
    /// and filled. This allows Orders to be triggered based on the bid, ask, mid, default (ask for buy, bid for
    /// sell) or inverse (ask for sell, bid for buy) price depending on the desired behaviour. Orders are always
    /// filled using their default price component. This feature is only provided through the REST API. Clients
    /// who choose to specify a non-default trigger condition will not see it reflected in any of OANDA’s
    /// proprietary or partner trading platforms, their transaction history or their account statements. OANDA
    /// platforms always assume that an Order’s trigger condition is set to the default value when indicating
    /// the distance from an Order’s trigger price, and will always provide the default trigger condition when
    /// creating or modifying an Order. A special restriction applies when creating a Guaranteed Stop Loss
    /// Order. In this case the `TriggerCondition` value must either be “DEFAULT”, or the “natural” trigger side
    /// “DEFAULT” results in. So for a Guaranteed Stop Loss Order for a long trade valid values are “DEFAULT”
    /// and “BID”, and for short trades “DEFAULT” and “ASK” are valid.
    #[serde(rename = "triggerCondition")]
    pub trigger_condition: OrderTriggerCondition,
    /// ID of the Transaction that filled this Order (only provided when the Order’s state is FILLED)
    #[serde(
        rename = "fillingTransactionID",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub filling_transaction_id: Option<TransactionID>,
    /// Date/time when the Order was filled (only provided when the Order’s state is FILLED)
    #[serde(
        rename = "filledTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub filled_time: Option<Timestamp>,
    /// Trade ID of Trade opened when the Order was filled (only provided when the Order’s state is FILLED and a
    /// Trade was opened as a result of the fill)
    #[serde(
        rename = "tradeOpenedID",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub trade_opened_id: Option<TradeID>,
    /// Trade ID of Trade reduced when the Order was filled (only provided when the Order’s state is FILLED and
    /// a Trade was reduced as a result of the fill)
    #[serde(
        rename = "tradeReducedID",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub trade_reduced_id: Option<TradeID>,
    /// Trade IDs of Trades closed when the Order was filled (only provided when the Order’s state is FILLED and
    /// one or more Trades were closed as a result of the fill)
    #[serde(
        rename = "tradeClosedIDs",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub trade_closed_ids: Option<Vec<TradeID>>,
    /// ID of the Transaction that cancelled the Order (only provided when the Order’s state is CANCELLED)
    #[serde(
        rename = "cancellingTransactionID",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub cancelling_transaction_id: Option<TransactionID>,
    /// Date/time when the Order was cancelled (only provided when the state of the Order is CANCELLED)
    #[serde(
        rename = "cancelledTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub cancelled_time: Option<Timestamp>,
    /// The ID of the Order that was replaced by this Order (only provided if this Order was created as part of
    /// a cancel/replace).
    #[serde(
        rename = "replacesOrderID",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub replaces_order_id: Option<OrderID>,
    /// The ID of the Order that replaced this Order (only provided if this Order was cancelled as part of a
    /// cancel/replace).
    #[serde(
        rename = "replacedByOrderID",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub replaced_by_order_id: Option<OrderID>,
}

/// A `StopLossOrder` is an order that is linked to an open Trade and created with a price threshold. The
/// Order will be filled (closing the Trade) by the first price that is equal to or worse than the
/// threshold. A `StopLossOrder` cannot be used to open a new Position.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StopLossOrder {
    /// The Order’s identifier, unique within the Order’s Account.
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<OrderID>,
    /// The time when the Order was created.
    #[serde(
        rename = "createTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub create_time: Option<Timestamp>,
    /// The current state of the Order.
    #[serde(rename = "state", default, skip_serializing_if = "Option::is_none")]
    pub state: Option<OrderState>,
    /// The client extensions of the Order. Do not set, modify, or delete clientExtensions if your account is
    /// associated with MT4.
    #[serde(
        rename = "clientExtensions",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub client_extensions: Option<ClientExtensions>,
    /// The type of the Order. Always set to “`STOP_LOSS`” for Stop Loss Orders.
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    #[serde(serialize_with = "serialize_stop_loss_order_type")]
    pub r#type: Option<OrderType>,
    /// The premium that will be charged if the Stop Loss Order is guaranteed and the Order is filled at the
    /// guaranteed price. It is in price units and is charged for each unit of the Trade. Deprecated: Will be
    /// removed in a future API update.
    #[serde(
        rename = "guaranteedExecutionPremium",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    #[serde(deserialize_with = "crate::decimal_wire::optional_number_or_string")]
    pub guaranteed_execution_premium: Option<Decimal>,
    /// The ID of the Trade to close when the price threshold is breached.
    #[serde(rename = "tradeID")]
    pub trade_id: TradeID,
    /// The client ID of the Trade to be closed when the price threshold is breached.
    #[serde(
        rename = "clientTradeID",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub client_trade_id: Option<ClientID>,
    /// The price threshold specified for the Stop Loss Order. The associated Trade will be closed by a market
    /// price that is equal to or worse than this threshold.
    #[serde(rename = "price")]
    #[serde(deserialize_with = "crate::decimal_wire::number_or_string")]
    pub price: Decimal,
    /// Specifies the distance (in price units) from the Account’s current price to use as the Stop Loss Order
    /// price. If the Trade is short the Instrument’s bid price is used, and for long Trades the ask is used.
    #[serde(rename = "distance", default, skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "crate::decimal_wire::optional_number_or_string")]
    pub distance: Option<Decimal>,
    /// The time-in-force requested for the `StopLoss` Order. Restricted to “GTC”, “GFD” and “GTD” for `StopLoss`
    /// Orders.
    #[serde(rename = "timeInForce")]
    pub time_in_force: TimeInForce,
    /// The date/time when the `StopLoss` Order will be cancelled if its timeInForce is “GTD”.
    #[serde(rename = "gtdTime", default, skip_serializing_if = "Option::is_none")]
    pub gtd_time: Option<Timestamp>,
    /// Specification of which price component should be used when determining if an Order should be triggered
    /// and filled. This allows Orders to be triggered based on the bid, ask, mid, default (ask for buy, bid for
    /// sell) or inverse (ask for sell, bid for buy) price depending on the desired behaviour. Orders are always
    /// filled using their default price component. This feature is only provided through the REST API. Clients
    /// who choose to specify a non-default trigger condition will not see it reflected in any of OANDA’s
    /// proprietary or partner trading platforms, their transaction history or their account statements. OANDA
    /// platforms always assume that an Order’s trigger condition is set to the default value when indicating
    /// the distance from an Order’s trigger price, and will always provide the default trigger condition when
    /// creating or modifying an Order. A special restriction applies when creating a Guaranteed Stop Loss
    /// Order. In this case the `TriggerCondition` value must either be “DEFAULT”, or the “natural” trigger side
    /// “DEFAULT” results in. So for a Guaranteed Stop Loss Order for a long trade valid values are “DEFAULT”
    /// and “BID”, and for short trades “DEFAULT” and “ASK” are valid.
    #[serde(rename = "triggerCondition")]
    pub trigger_condition: OrderTriggerCondition,
    /// Flag indicating that the Stop Loss Order is guaranteed. The default value depends on the
    /// `GuaranteedStopLossOrderMode` of the account, if it is REQUIRED, the default will be true, for DISABLED or
    /// ENABLED the default is false. Deprecated: Will be removed in a future API update.
    #[serde(
        rename = "guaranteed",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub guaranteed: Option<bool>,
    /// ID of the Transaction that filled this Order (only provided when the Order’s state is FILLED)
    #[serde(
        rename = "fillingTransactionID",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub filling_transaction_id: Option<TransactionID>,
    /// Date/time when the Order was filled (only provided when the Order’s state is FILLED)
    #[serde(
        rename = "filledTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub filled_time: Option<Timestamp>,
    /// Trade ID of Trade opened when the Order was filled (only provided when the Order’s state is FILLED and a
    /// Trade was opened as a result of the fill)
    #[serde(
        rename = "tradeOpenedID",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub trade_opened_id: Option<TradeID>,
    /// Trade ID of Trade reduced when the Order was filled (only provided when the Order’s state is FILLED and
    /// a Trade was reduced as a result of the fill)
    #[serde(
        rename = "tradeReducedID",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub trade_reduced_id: Option<TradeID>,
    /// Trade IDs of Trades closed when the Order was filled (only provided when the Order’s state is FILLED and
    /// one or more Trades were closed as a result of the fill)
    #[serde(
        rename = "tradeClosedIDs",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub trade_closed_ids: Option<Vec<TradeID>>,
    /// ID of the Transaction that cancelled the Order (only provided when the Order’s state is CANCELLED)
    #[serde(
        rename = "cancellingTransactionID",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub cancelling_transaction_id: Option<TransactionID>,
    /// Date/time when the Order was cancelled (only provided when the state of the Order is CANCELLED)
    #[serde(
        rename = "cancelledTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub cancelled_time: Option<Timestamp>,
    /// The ID of the Order that was replaced by this Order (only provided if this Order was created as part of
    /// a cancel/replace).
    #[serde(
        rename = "replacesOrderID",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub replaces_order_id: Option<OrderID>,
    /// The ID of the Order that replaced this Order (only provided if this Order was cancelled as part of a
    /// cancel/replace).
    #[serde(
        rename = "replacedByOrderID",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub replaced_by_order_id: Option<OrderID>,
}

/// A `GuaranteedStopLossOrder` is an order that is linked to an open Trade and created with a price threshold
/// which is guaranteed against slippage that may occur as the market crosses the price set for that order.
/// The Order will be filled (closing the Trade) by the first price that is equal to or worse than the
/// threshold. The price level specified for the `GuaranteedStopLossOrder` must be at least the configured
/// minimum distance (in price units) away from the entry price for the traded instrument. A
/// `GuaranteedStopLossOrder` cannot be used to open a new Position.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GuaranteedStopLossOrder {
    /// The Order’s identifier, unique within the Order’s Account.
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<OrderID>,
    /// The time when the Order was created.
    #[serde(
        rename = "createTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub create_time: Option<Timestamp>,
    /// The current state of the Order.
    #[serde(rename = "state", default, skip_serializing_if = "Option::is_none")]
    pub state: Option<OrderState>,
    /// The client extensions of the Order. Do not set, modify, or delete clientExtensions if your account is
    /// associated with MT4.
    #[serde(
        rename = "clientExtensions",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub client_extensions: Option<ClientExtensions>,
    /// The type of the Order. Always set to “`GUARANTEED_STOP_LOSS`” for Guaranteed Stop Loss Orders.
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    #[serde(serialize_with = "serialize_guaranteed_stop_loss_order_type")]
    pub r#type: Option<OrderType>,
    /// The premium that will be charged if the Guaranteed Stop Loss Order is filled at the guaranteed price. It
    /// is in price units and is charged for each unit of the Trade.
    #[serde(
        rename = "guaranteedExecutionPremium",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    #[serde(deserialize_with = "crate::decimal_wire::optional_number_or_string")]
    pub guaranteed_execution_premium: Option<Decimal>,
    /// The ID of the Trade to close when the price threshold is breached.
    #[serde(rename = "tradeID")]
    pub trade_id: TradeID,
    /// The client ID of the Trade to be closed when the price threshold is breached.
    #[serde(
        rename = "clientTradeID",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub client_trade_id: Option<ClientID>,
    /// The price threshold specified for the Guaranteed Stop Loss Order. The associated Trade will be closed at
    /// this price.
    #[serde(rename = "price")]
    #[serde(deserialize_with = "crate::decimal_wire::number_or_string")]
    pub price: Decimal,
    /// Specifies the distance (in price units) from the Account’s current price to use as the Guaranteed Stop
    /// Loss Order price. If the Trade is short the Instrument’s bid price is used, and for long Trades the ask
    /// is used.
    #[serde(rename = "distance", default, skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "crate::decimal_wire::optional_number_or_string")]
    pub distance: Option<Decimal>,
    /// The time-in-force requested for the `GuaranteedStopLoss` Order. Restricted to “GTC”, “GFD” and “GTD” for
    /// `GuaranteedStopLoss` Orders.
    #[serde(rename = "timeInForce")]
    pub time_in_force: TimeInForce,
    /// The date/time when the `GuaranteedStopLoss` Order will be cancelled if its timeInForce is “GTD”.
    #[serde(rename = "gtdTime", default, skip_serializing_if = "Option::is_none")]
    pub gtd_time: Option<Timestamp>,
    /// Specification of which price component should be used when determining if an Order should be triggered
    /// and filled. This allows Orders to be triggered based on the bid, ask, mid, default (ask for buy, bid for
    /// sell) or inverse (ask for sell, bid for buy) price depending on the desired behaviour. Orders are always
    /// filled using their default price component. This feature is only provided through the REST API. Clients
    /// who choose to specify a non-default trigger condition will not see it reflected in any of OANDA’s
    /// proprietary or partner trading platforms, their transaction history or their account statements. OANDA
    /// platforms always assume that an Order’s trigger condition is set to the default value when indicating
    /// the distance from an Order’s trigger price, and will always provide the default trigger condition when
    /// creating or modifying an Order. A special restriction applies when creating a Guaranteed Stop Loss
    /// Order. In this case the `TriggerCondition` value must either be “DEFAULT”, or the “natural” trigger side
    /// “DEFAULT” results in. So for a Guaranteed Stop Loss Order for a long trade valid values are “DEFAULT”
    /// and “BID”, and for short trades “DEFAULT” and “ASK” are valid.
    #[serde(rename = "triggerCondition")]
    pub trigger_condition: OrderTriggerCondition,
    /// ID of the Transaction that filled this Order (only provided when the Order’s state is FILLED)
    #[serde(
        rename = "fillingTransactionID",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub filling_transaction_id: Option<TransactionID>,
    /// Date/time when the Order was filled (only provided when the Order’s state is FILLED)
    #[serde(
        rename = "filledTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub filled_time: Option<Timestamp>,
    /// Trade ID of Trade opened when the Order was filled (only provided when the Order’s state is FILLED and a
    /// Trade was opened as a result of the fill)
    #[serde(
        rename = "tradeOpenedID",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub trade_opened_id: Option<TradeID>,
    /// Trade ID of Trade reduced when the Order was filled (only provided when the Order’s state is FILLED and
    /// a Trade was reduced as a result of the fill)
    #[serde(
        rename = "tradeReducedID",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub trade_reduced_id: Option<TradeID>,
    /// Trade IDs of Trades closed when the Order was filled (only provided when the Order’s state is FILLED and
    /// one or more Trades were closed as a result of the fill)
    #[serde(
        rename = "tradeClosedIDs",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub trade_closed_ids: Option<Vec<TradeID>>,
    /// ID of the Transaction that cancelled the Order (only provided when the Order’s state is CANCELLED)
    #[serde(
        rename = "cancellingTransactionID",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub cancelling_transaction_id: Option<TransactionID>,
    /// Date/time when the Order was cancelled (only provided when the state of the Order is CANCELLED)
    #[serde(
        rename = "cancelledTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub cancelled_time: Option<Timestamp>,
    /// The ID of the Order that was replaced by this Order (only provided if this Order was created as part of
    /// a cancel/replace).
    #[serde(
        rename = "replacesOrderID",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub replaces_order_id: Option<OrderID>,
    /// The ID of the Order that replaced this Order (only provided if this Order was cancelled as part of a
    /// cancel/replace).
    #[serde(
        rename = "replacedByOrderID",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub replaced_by_order_id: Option<OrderID>,
}

/// A `TrailingStopLossOrder` is an order that is linked to an open Trade and created with a price distance.
/// The price distance is used to calculate a trailing stop value for the order that is in the losing
/// direction from the market price at the time of the order’s creation. The trailing stop value will follow
/// the market price as it moves in the winning direction, and the order will be filled (closing the Trade)
/// by the first price that is equal to or worse than the trailing stop value. A `TrailingStopLossOrder`
/// cannot be used to open a new Position.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrailingStopLossOrder {
    /// The Order’s identifier, unique within the Order’s Account.
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<OrderID>,
    /// The time when the Order was created.
    #[serde(
        rename = "createTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub create_time: Option<Timestamp>,
    /// The current state of the Order.
    #[serde(rename = "state", default, skip_serializing_if = "Option::is_none")]
    pub state: Option<OrderState>,
    /// The client extensions of the Order. Do not set, modify, or delete clientExtensions if your account is
    /// associated with MT4.
    #[serde(
        rename = "clientExtensions",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub client_extensions: Option<ClientExtensions>,
    /// The type of the Order. Always set to “`TRAILING_STOP_LOSS`” for Trailing Stop Loss Orders.
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    #[serde(serialize_with = "serialize_trailing_stop_loss_order_type")]
    pub r#type: Option<OrderType>,
    /// The ID of the Trade to close when the price threshold is breached.
    #[serde(rename = "tradeID")]
    pub trade_id: TradeID,
    /// The client ID of the Trade to be closed when the price threshold is breached.
    #[serde(
        rename = "clientTradeID",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub client_trade_id: Option<ClientID>,
    /// The price distance (in price units) specified for the `TrailingStopLoss` Order.
    #[serde(rename = "distance")]
    #[serde(deserialize_with = "crate::decimal_wire::number_or_string")]
    pub distance: Decimal,
    /// The time-in-force requested for the `TrailingStopLoss` Order. Restricted to “GTC”, “GFD” and “GTD” for
    /// `TrailingStopLoss` Orders.
    #[serde(rename = "timeInForce")]
    pub time_in_force: TimeInForce,
    /// The date/time when the `StopLoss` Order will be cancelled if its timeInForce is “GTD”.
    #[serde(rename = "gtdTime", default, skip_serializing_if = "Option::is_none")]
    pub gtd_time: Option<Timestamp>,
    /// Specification of which price component should be used when determining if an Order should be triggered
    /// and filled. This allows Orders to be triggered based on the bid, ask, mid, default (ask for buy, bid for
    /// sell) or inverse (ask for sell, bid for buy) price depending on the desired behaviour. Orders are always
    /// filled using their default price component. This feature is only provided through the REST API. Clients
    /// who choose to specify a non-default trigger condition will not see it reflected in any of OANDA’s
    /// proprietary or partner trading platforms, their transaction history or their account statements. OANDA
    /// platforms always assume that an Order’s trigger condition is set to the default value when indicating
    /// the distance from an Order’s trigger price, and will always provide the default trigger condition when
    /// creating or modifying an Order. A special restriction applies when creating a Guaranteed Stop Loss
    /// Order. In this case the `TriggerCondition` value must either be “DEFAULT”, or the “natural” trigger side
    /// “DEFAULT” results in. So for a Guaranteed Stop Loss Order for a long trade valid values are “DEFAULT”
    /// and “BID”, and for short trades “DEFAULT” and “ASK” are valid.
    #[serde(rename = "triggerCondition")]
    pub trigger_condition: OrderTriggerCondition,
    /// The trigger price for the Trailing Stop Loss Order. The trailing stop value will trail (follow) the
    /// market price by the TSL order’s configured “distance” as the market price moves in the winning
    /// direction. If the market price moves to a level that is equal to or worse than the trailing stop value,
    /// the order will be filled and the Trade will be closed.
    #[serde(
        rename = "trailingStopValue",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    #[serde(deserialize_with = "crate::decimal_wire::optional_number_or_string")]
    pub trailing_stop_value: Option<Decimal>,
    /// ID of the Transaction that filled this Order (only provided when the Order’s state is FILLED)
    #[serde(
        rename = "fillingTransactionID",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub filling_transaction_id: Option<TransactionID>,
    /// Date/time when the Order was filled (only provided when the Order’s state is FILLED)
    #[serde(
        rename = "filledTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub filled_time: Option<Timestamp>,
    /// Trade ID of Trade opened when the Order was filled (only provided when the Order’s state is FILLED and a
    /// Trade was opened as a result of the fill)
    #[serde(
        rename = "tradeOpenedID",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub trade_opened_id: Option<TradeID>,
    /// Trade ID of Trade reduced when the Order was filled (only provided when the Order’s state is FILLED and
    /// a Trade was reduced as a result of the fill)
    #[serde(
        rename = "tradeReducedID",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub trade_reduced_id: Option<TradeID>,
    /// Trade IDs of Trades closed when the Order was filled (only provided when the Order’s state is FILLED and
    /// one or more Trades were closed as a result of the fill)
    #[serde(
        rename = "tradeClosedIDs",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub trade_closed_ids: Option<Vec<TradeID>>,
    /// ID of the Transaction that cancelled the Order (only provided when the Order’s state is CANCELLED)
    #[serde(
        rename = "cancellingTransactionID",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub cancelling_transaction_id: Option<TransactionID>,
    /// Date/time when the Order was cancelled (only provided when the state of the Order is CANCELLED)
    #[serde(
        rename = "cancelledTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub cancelled_time: Option<Timestamp>,
    /// The ID of the Order that was replaced by this Order (only provided if this Order was created as part of
    /// a cancel/replace).
    #[serde(
        rename = "replacesOrderID",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub replaces_order_id: Option<OrderID>,
    /// The ID of the Order that replaced this Order (only provided if this Order was cancelled as part of a
    /// cancel/replace).
    #[serde(
        rename = "replacedByOrderID",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub replaced_by_order_id: Option<OrderID>,
}

/// A `MarketOrderRequest` specifies the parameters that may be set when creating a Market Order.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarketOrderRequest {
    /// The type of the Order to Create. Must be set to “MARKET” when creating a Market Order.
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    #[serde(serialize_with = "serialize_market_order_request_type")]
    pub r#type: Option<OrderType>,
    /// The Market Order’s Instrument.
    #[serde(rename = "instrument")]
    pub instrument: InstrumentName,
    /// The quantity requested to be filled by the Market Order. A positive number of units results in a long
    /// Order, and a negative number of units results in a short Order.
    #[serde(rename = "units")]
    #[serde(deserialize_with = "crate::decimal_wire::number_or_string")]
    pub units: Decimal,
    /// The time-in-force requested for the Market Order. Restricted to FOK or IOC for a `MarketOrder`.
    #[serde(rename = "timeInForce")]
    pub time_in_force: TimeInForce,
    /// The worst price that the client is willing to have the Market Order filled at.
    #[serde(
        rename = "priceBound",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    #[serde(deserialize_with = "crate::decimal_wire::optional_number_or_string")]
    pub price_bound: Option<Decimal>,
    /// Specification of how Positions in the Account are modified when the Order is filled.
    #[serde(rename = "positionFill")]
    pub position_fill: OrderPositionFill,
    /// The client extensions to add to the Order. Do not set, modify, or delete clientExtensions if your
    /// account is associated with MT4.
    #[serde(
        rename = "clientExtensions",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub client_extensions: Option<ClientExtensions>,
    /// `TakeProfitDetails` specifies the details of a Take Profit Order to be created on behalf of a client. This
    /// may happen when an Order is filled that opens a Trade requiring a Take Profit, or when a Trade’s
    /// dependent Take Profit Order is modified directly through the Trade.
    #[serde(
        rename = "takeProfitOnFill",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub take_profit_on_fill: Option<TakeProfitDetails>,
    /// `StopLossDetails` specifies the details of a Stop Loss Order to be created on behalf of a client. This may
    /// happen when an Order is filled that opens a Trade requiring a Stop Loss, or when a Trade’s dependent
    /// Stop Loss Order is modified directly through the Trade.
    #[serde(
        rename = "stopLossOnFill",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub stop_loss_on_fill: Option<StopLossDetails>,
    /// `GuaranteedStopLossDetails` specifies the details of a Guaranteed Stop Loss Order to be created on behalf
    /// of a client. This may happen when an Order is filled that opens a Trade requiring a Guaranteed Stop
    /// Loss, or when a Trade’s dependent Guaranteed Stop Loss Order is modified directly through the Trade.
    #[serde(
        rename = "guaranteedStopLossOnFill",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub guaranteed_stop_loss_on_fill: Option<GuaranteedStopLossDetails>,
    /// `TrailingStopLossDetails` specifies the details of a Trailing Stop Loss Order to be created on behalf of a
    /// client. This may happen when an Order is filled that opens a Trade requiring a Trailing Stop Loss, or
    /// when a Trade’s dependent Trailing Stop Loss Order is modified directly through the Trade.
    #[serde(
        rename = "trailingStopLossOnFill",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub trailing_stop_loss_on_fill: Option<TrailingStopLossDetails>,
    /// Client Extensions to add to the Trade created when the Order is filled (if such a Trade is created). Do
    /// not set, modify, or delete tradeClientExtensions if your account is associated with MT4.
    #[serde(
        rename = "tradeClientExtensions",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub trade_client_extensions: Option<ClientExtensions>,
}

/// A `LimitOrderRequest` specifies the parameters that may be set when creating a Limit Order.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LimitOrderRequest {
    /// The type of the Order to Create. Must be set to “LIMIT” when creating a Market Order.
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    #[serde(serialize_with = "serialize_limit_order_request_type")]
    pub r#type: Option<OrderType>,
    /// The Limit Order’s Instrument.
    #[serde(rename = "instrument")]
    pub instrument: InstrumentName,
    /// The quantity requested to be filled by the Limit Order. A positive number of units results in a long
    /// Order, and a negative number of units results in a short Order.
    #[serde(rename = "units")]
    #[serde(deserialize_with = "crate::decimal_wire::number_or_string")]
    pub units: Decimal,
    /// The price threshold specified for the Limit Order. The Limit Order will only be filled by a market price
    /// that is equal to or better than this price.
    #[serde(rename = "price")]
    #[serde(deserialize_with = "crate::decimal_wire::number_or_string")]
    pub price: Decimal,
    /// The time-in-force requested for the Limit Order.
    #[serde(rename = "timeInForce")]
    pub time_in_force: TimeInForce,
    /// The date/time when the Limit Order will be cancelled if its timeInForce is “GTD”.
    #[serde(rename = "gtdTime", default, skip_serializing_if = "Option::is_none")]
    pub gtd_time: Option<Timestamp>,
    /// Specification of how Positions in the Account are modified when the Order is filled.
    #[serde(rename = "positionFill")]
    pub position_fill: OrderPositionFill,
    /// Specification of which price component should be used when determining if an Order should be triggered
    /// and filled. This allows Orders to be triggered based on the bid, ask, mid, default (ask for buy, bid for
    /// sell) or inverse (ask for sell, bid for buy) price depending on the desired behaviour. Orders are always
    /// filled using their default price component. This feature is only provided through the REST API. Clients
    /// who choose to specify a non-default trigger condition will not see it reflected in any of OANDA’s
    /// proprietary or partner trading platforms, their transaction history or their account statements. OANDA
    /// platforms always assume that an Order’s trigger condition is set to the default value when indicating
    /// the distance from an Order’s trigger price, and will always provide the default trigger condition when
    /// creating or modifying an Order. A special restriction applies when creating a Guaranteed Stop Loss
    /// Order. In this case the `TriggerCondition` value must either be “DEFAULT”, or the “natural” trigger side
    /// “DEFAULT” results in. So for a Guaranteed Stop Loss Order for a long trade valid values are “DEFAULT”
    /// and “BID”, and for short trades “DEFAULT” and “ASK” are valid.
    #[serde(rename = "triggerCondition")]
    pub trigger_condition: OrderTriggerCondition,
    /// The client extensions to add to the Order. Do not set, modify, or delete clientExtensions if your
    /// account is associated with MT4.
    #[serde(
        rename = "clientExtensions",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub client_extensions: Option<ClientExtensions>,
    /// `TakeProfitDetails` specifies the details of a Take Profit Order to be created on behalf of a client. This
    /// may happen when an Order is filled that opens a Trade requiring a Take Profit, or when a Trade’s
    /// dependent Take Profit Order is modified directly through the Trade.
    #[serde(
        rename = "takeProfitOnFill",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub take_profit_on_fill: Option<TakeProfitDetails>,
    /// `StopLossDetails` specifies the details of a Stop Loss Order to be created on behalf of a client. This may
    /// happen when an Order is filled that opens a Trade requiring a Stop Loss, or when a Trade’s dependent
    /// Stop Loss Order is modified directly through the Trade.
    #[serde(
        rename = "stopLossOnFill",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub stop_loss_on_fill: Option<StopLossDetails>,
    /// `GuaranteedStopLossDetails` specifies the details of a Guaranteed Stop Loss Order to be created on behalf
    /// of a client. This may happen when an Order is filled that opens a Trade requiring a Guaranteed Stop
    /// Loss, or when a Trade’s dependent Guaranteed Stop Loss Order is modified directly through the Trade.
    #[serde(
        rename = "guaranteedStopLossOnFill",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub guaranteed_stop_loss_on_fill: Option<GuaranteedStopLossDetails>,
    /// `TrailingStopLossDetails` specifies the details of a Trailing Stop Loss Order to be created on behalf of a
    /// client. This may happen when an Order is filled that opens a Trade requiring a Trailing Stop Loss, or
    /// when a Trade’s dependent Trailing Stop Loss Order is modified directly through the Trade.
    #[serde(
        rename = "trailingStopLossOnFill",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub trailing_stop_loss_on_fill: Option<TrailingStopLossDetails>,
    /// Client Extensions to add to the Trade created when the Order is filled (if such a Trade is created). Do
    /// not set, modify, or delete tradeClientExtensions if your account is associated with MT4.
    #[serde(
        rename = "tradeClientExtensions",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub trade_client_extensions: Option<ClientExtensions>,
}

/// A `StopOrderRequest` specifies the parameters that may be set when creating a Stop Order.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StopOrderRequest {
    /// The type of the Order to Create. Must be set to “STOP” when creating a Stop Order.
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    #[serde(serialize_with = "serialize_stop_order_request_type")]
    pub r#type: Option<OrderType>,
    /// The Stop Order’s Instrument.
    #[serde(rename = "instrument")]
    pub instrument: InstrumentName,
    /// The quantity requested to be filled by the Stop Order. A positive number of units results in a long
    /// Order, and a negative number of units results in a short Order.
    #[serde(rename = "units")]
    #[serde(deserialize_with = "crate::decimal_wire::number_or_string")]
    pub units: Decimal,
    /// The price threshold specified for the Stop Order. The Stop Order will only be filled by a market price
    /// that is equal to or worse than this price.
    #[serde(rename = "price")]
    #[serde(deserialize_with = "crate::decimal_wire::number_or_string")]
    pub price: Decimal,
    /// The worst market price that may be used to fill this Stop Order. If the market gaps and crosses through
    /// both the price and the priceBound, the Stop Order will be cancelled instead of being filled.
    #[serde(
        rename = "priceBound",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    #[serde(deserialize_with = "crate::decimal_wire::optional_number_or_string")]
    pub price_bound: Option<Decimal>,
    /// The time-in-force requested for the Stop Order.
    #[serde(rename = "timeInForce")]
    pub time_in_force: TimeInForce,
    /// The date/time when the Stop Order will be cancelled if its timeInForce is “GTD”.
    #[serde(rename = "gtdTime", default, skip_serializing_if = "Option::is_none")]
    pub gtd_time: Option<Timestamp>,
    /// Specification of how Positions in the Account are modified when the Order is filled.
    #[serde(rename = "positionFill")]
    pub position_fill: OrderPositionFill,
    /// Specification of which price component should be used when determining if an Order should be triggered
    /// and filled. This allows Orders to be triggered based on the bid, ask, mid, default (ask for buy, bid for
    /// sell) or inverse (ask for sell, bid for buy) price depending on the desired behaviour. Orders are always
    /// filled using their default price component. This feature is only provided through the REST API. Clients
    /// who choose to specify a non-default trigger condition will not see it reflected in any of OANDA’s
    /// proprietary or partner trading platforms, their transaction history or their account statements. OANDA
    /// platforms always assume that an Order’s trigger condition is set to the default value when indicating
    /// the distance from an Order’s trigger price, and will always provide the default trigger condition when
    /// creating or modifying an Order. A special restriction applies when creating a Guaranteed Stop Loss
    /// Order. In this case the `TriggerCondition` value must either be “DEFAULT”, or the “natural” trigger side
    /// “DEFAULT” results in. So for a Guaranteed Stop Loss Order for a long trade valid values are “DEFAULT”
    /// and “BID”, and for short trades “DEFAULT” and “ASK” are valid.
    #[serde(rename = "triggerCondition")]
    pub trigger_condition: OrderTriggerCondition,
    /// The client extensions to add to the Order. Do not set, modify, or delete clientExtensions if your
    /// account is associated with MT4.
    #[serde(
        rename = "clientExtensions",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub client_extensions: Option<ClientExtensions>,
    /// `TakeProfitDetails` specifies the details of a Take Profit Order to be created on behalf of a client. This
    /// may happen when an Order is filled that opens a Trade requiring a Take Profit, or when a Trade’s
    /// dependent Take Profit Order is modified directly through the Trade.
    #[serde(
        rename = "takeProfitOnFill",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub take_profit_on_fill: Option<TakeProfitDetails>,
    /// `StopLossDetails` specifies the details of a Stop Loss Order to be created on behalf of a client. This may
    /// happen when an Order is filled that opens a Trade requiring a Stop Loss, or when a Trade’s dependent
    /// Stop Loss Order is modified directly through the Trade.
    #[serde(
        rename = "stopLossOnFill",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub stop_loss_on_fill: Option<StopLossDetails>,
    /// `GuaranteedStopLossDetails` specifies the details of a Guaranteed Stop Loss Order to be created on behalf
    /// of a client. This may happen when an Order is filled that opens a Trade requiring a Guaranteed Stop
    /// Loss, or when a Trade’s dependent Guaranteed Stop Loss Order is modified directly through the Trade.
    #[serde(
        rename = "guaranteedStopLossOnFill",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub guaranteed_stop_loss_on_fill: Option<GuaranteedStopLossDetails>,
    /// `TrailingStopLossDetails` specifies the details of a Trailing Stop Loss Order to be created on behalf of a
    /// client. This may happen when an Order is filled that opens a Trade requiring a Trailing Stop Loss, or
    /// when a Trade’s dependent Trailing Stop Loss Order is modified directly through the Trade.
    #[serde(
        rename = "trailingStopLossOnFill",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub trailing_stop_loss_on_fill: Option<TrailingStopLossDetails>,
    /// Client Extensions to add to the Trade created when the Order is filled (if such a Trade is created). Do
    /// not set, modify, or delete tradeClientExtensions if your account is associated with MT4.
    #[serde(
        rename = "tradeClientExtensions",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub trade_client_extensions: Option<ClientExtensions>,
}

/// A `MarketIfTouchedOrderRequest` specifies the parameters that may be set when creating a Market-if-Touched
/// Order.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarketIfTouchedOrderRequest {
    /// The type of the Order to Create. Must be set to “`MARKET_IF_TOUCHED`” when creating a Market If Touched
    /// Order.
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    #[serde(serialize_with = "serialize_market_if_touched_order_request_type")]
    pub r#type: Option<OrderType>,
    /// The `MarketIfTouched` Order’s Instrument.
    #[serde(rename = "instrument")]
    pub instrument: InstrumentName,
    /// The quantity requested to be filled by the `MarketIfTouched` Order. A positive number of units results in
    /// a long Order, and a negative number of units results in a short Order.
    #[serde(rename = "units")]
    #[serde(deserialize_with = "crate::decimal_wire::number_or_string")]
    pub units: Decimal,
    /// The price threshold specified for the `MarketIfTouched` Order. The `MarketIfTouched` Order will only be
    /// filled by a market price that crosses this price from the direction of the market price at the time when
    /// the Order was created (the initialMarketPrice). Depending on the value of the Order’s price and
    /// initialMarketPrice, the `MarketIfTouchedOrder` will behave like a Limit or a Stop Order.
    #[serde(rename = "price")]
    #[serde(deserialize_with = "crate::decimal_wire::number_or_string")]
    pub price: Decimal,
    /// The worst market price that may be used to fill this `MarketIfTouched` Order.
    #[serde(
        rename = "priceBound",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    #[serde(deserialize_with = "crate::decimal_wire::optional_number_or_string")]
    pub price_bound: Option<Decimal>,
    /// The time-in-force requested for the `MarketIfTouched` Order. Restricted to “GTC”, “GFD” and “GTD” for
    /// `MarketIfTouched` Orders.
    #[serde(rename = "timeInForce")]
    pub time_in_force: TimeInForce,
    /// The date/time when the `MarketIfTouched` Order will be cancelled if its timeInForce is “GTD”.
    #[serde(rename = "gtdTime", default, skip_serializing_if = "Option::is_none")]
    pub gtd_time: Option<Timestamp>,
    /// Specification of how Positions in the Account are modified when the Order is filled.
    #[serde(rename = "positionFill")]
    pub position_fill: OrderPositionFill,
    /// Specification of which price component should be used when determining if an Order should be triggered
    /// and filled. This allows Orders to be triggered based on the bid, ask, mid, default (ask for buy, bid for
    /// sell) or inverse (ask for sell, bid for buy) price depending on the desired behaviour. Orders are always
    /// filled using their default price component. This feature is only provided through the REST API. Clients
    /// who choose to specify a non-default trigger condition will not see it reflected in any of OANDA’s
    /// proprietary or partner trading platforms, their transaction history or their account statements. OANDA
    /// platforms always assume that an Order’s trigger condition is set to the default value when indicating
    /// the distance from an Order’s trigger price, and will always provide the default trigger condition when
    /// creating or modifying an Order. A special restriction applies when creating a Guaranteed Stop Loss
    /// Order. In this case the `TriggerCondition` value must either be “DEFAULT”, or the “natural” trigger side
    /// “DEFAULT” results in. So for a Guaranteed Stop Loss Order for a long trade valid values are “DEFAULT”
    /// and “BID”, and for short trades “DEFAULT” and “ASK” are valid.
    #[serde(rename = "triggerCondition")]
    pub trigger_condition: OrderTriggerCondition,
    /// The client extensions to add to the Order. Do not set, modify, or delete clientExtensions if your
    /// account is associated with MT4.
    #[serde(
        rename = "clientExtensions",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub client_extensions: Option<ClientExtensions>,
    /// `TakeProfitDetails` specifies the details of a Take Profit Order to be created on behalf of a client. This
    /// may happen when an Order is filled that opens a Trade requiring a Take Profit, or when a Trade’s
    /// dependent Take Profit Order is modified directly through the Trade.
    #[serde(
        rename = "takeProfitOnFill",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub take_profit_on_fill: Option<TakeProfitDetails>,
    /// `StopLossDetails` specifies the details of a Stop Loss Order to be created on behalf of a client. This may
    /// happen when an Order is filled that opens a Trade requiring a Stop Loss, or when a Trade’s dependent
    /// Stop Loss Order is modified directly through the Trade.
    #[serde(
        rename = "stopLossOnFill",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub stop_loss_on_fill: Option<StopLossDetails>,
    /// `GuaranteedStopLossDetails` specifies the details of a Guaranteed Stop Loss Order to be created on behalf
    /// of a client. This may happen when an Order is filled that opens a Trade requiring a Guaranteed Stop
    /// Loss, or when a Trade’s dependent Guaranteed Stop Loss Order is modified directly through the Trade.
    #[serde(
        rename = "guaranteedStopLossOnFill",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub guaranteed_stop_loss_on_fill: Option<GuaranteedStopLossDetails>,
    /// `TrailingStopLossDetails` specifies the details of a Trailing Stop Loss Order to be created on behalf of a
    /// client. This may happen when an Order is filled that opens a Trade requiring a Trailing Stop Loss, or
    /// when a Trade’s dependent Trailing Stop Loss Order is modified directly through the Trade.
    #[serde(
        rename = "trailingStopLossOnFill",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub trailing_stop_loss_on_fill: Option<TrailingStopLossDetails>,
    /// Client Extensions to add to the Trade created when the Order is filled (if such a Trade is created). Do
    /// not set, modify, or delete tradeClientExtensions if your account is associated with MT4.
    #[serde(
        rename = "tradeClientExtensions",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub trade_client_extensions: Option<ClientExtensions>,
}

/// A `TakeProfitOrderRequest` specifies the parameters that may be set when creating a Take Profit Order.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TakeProfitOrderRequest {
    /// The type of the Order to Create. Must be set to “`TAKE_PROFIT`” when creating a Take Profit Order.
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    #[serde(serialize_with = "serialize_take_profit_order_request_type")]
    pub r#type: Option<OrderType>,
    /// The ID of the Trade to close when the price threshold is breached.
    #[serde(rename = "tradeID")]
    pub trade_id: TradeID,
    /// The client ID of the Trade to be closed when the price threshold is breached.
    #[serde(
        rename = "clientTradeID",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub client_trade_id: Option<ClientID>,
    /// The price threshold specified for the `TakeProfit` Order. The associated Trade will be closed by a market
    /// price that is equal to or better than this threshold.
    #[serde(rename = "price")]
    #[serde(deserialize_with = "crate::decimal_wire::number_or_string")]
    pub price: Decimal,
    /// The time-in-force requested for the `TakeProfit` Order. Restricted to “GTC”, “GFD” and “GTD” for
    /// `TakeProfit` Orders.
    #[serde(rename = "timeInForce")]
    pub time_in_force: TimeInForce,
    /// The date/time when the `TakeProfit` Order will be cancelled if its timeInForce is “GTD”.
    #[serde(rename = "gtdTime", default, skip_serializing_if = "Option::is_none")]
    pub gtd_time: Option<Timestamp>,
    /// Specification of which price component should be used when determining if an Order should be triggered
    /// and filled. This allows Orders to be triggered based on the bid, ask, mid, default (ask for buy, bid for
    /// sell) or inverse (ask for sell, bid for buy) price depending on the desired behaviour. Orders are always
    /// filled using their default price component. This feature is only provided through the REST API. Clients
    /// who choose to specify a non-default trigger condition will not see it reflected in any of OANDA’s
    /// proprietary or partner trading platforms, their transaction history or their account statements. OANDA
    /// platforms always assume that an Order’s trigger condition is set to the default value when indicating
    /// the distance from an Order’s trigger price, and will always provide the default trigger condition when
    /// creating or modifying an Order. A special restriction applies when creating a Guaranteed Stop Loss
    /// Order. In this case the `TriggerCondition` value must either be “DEFAULT”, or the “natural” trigger side
    /// “DEFAULT” results in. So for a Guaranteed Stop Loss Order for a long trade valid values are “DEFAULT”
    /// and “BID”, and for short trades “DEFAULT” and “ASK” are valid.
    #[serde(rename = "triggerCondition")]
    pub trigger_condition: OrderTriggerCondition,
    /// The client extensions to add to the Order. Do not set, modify, or delete clientExtensions if your
    /// account is associated with MT4.
    #[serde(
        rename = "clientExtensions",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub client_extensions: Option<ClientExtensions>,
}

/// A `StopLossOrderRequest` specifies the parameters that may be set when creating a Stop Loss Order. Only
/// one of the price and distance fields may be specified.
///
/// The published definition requires `price` while allowing `distance` only in
/// its absence. This DTO follows that requiredness: outbound admission rejects
/// an explicit distance alongside the required price. Distance-only creation is
/// unavailable under this published contract; received fields remain inspectable.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StopLossOrderRequest {
    /// The type of the Order to Create. Must be set to “`STOP_LOSS`” when creating a Stop Loss Order.
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    #[serde(serialize_with = "serialize_stop_loss_order_request_type")]
    pub r#type: Option<OrderType>,
    /// The ID of the Trade to close when the price threshold is breached.
    #[serde(rename = "tradeID")]
    pub trade_id: TradeID,
    /// The client ID of the Trade to be closed when the price threshold is breached.
    #[serde(
        rename = "clientTradeID",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub client_trade_id: Option<ClientID>,
    /// The price threshold specified for the Stop Loss Order. The associated Trade will be closed by a market
    /// price that is equal to or worse than this threshold.
    #[serde(rename = "price")]
    #[serde(deserialize_with = "crate::decimal_wire::number_or_string")]
    pub price: Decimal,
    /// Specifies the distance (in price units) from the Account’s current price to use as the Stop Loss Order
    /// price. If the Trade is short the Instrument’s bid price is used, and for long Trades the ask is used.
    #[serde(rename = "distance", default, skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "crate::decimal_wire::optional_number_or_string")]
    pub distance: Option<Decimal>,
    /// The time-in-force requested for the `StopLoss` Order. Restricted to “GTC”, “GFD” and “GTD” for `StopLoss`
    /// Orders.
    #[serde(rename = "timeInForce")]
    pub time_in_force: TimeInForce,
    /// The date/time when the `StopLoss` Order will be cancelled if its timeInForce is “GTD”.
    #[serde(rename = "gtdTime", default, skip_serializing_if = "Option::is_none")]
    pub gtd_time: Option<Timestamp>,
    /// Specification of which price component should be used when determining if an Order should be triggered
    /// and filled. This allows Orders to be triggered based on the bid, ask, mid, default (ask for buy, bid for
    /// sell) or inverse (ask for sell, bid for buy) price depending on the desired behaviour. Orders are always
    /// filled using their default price component. This feature is only provided through the REST API. Clients
    /// who choose to specify a non-default trigger condition will not see it reflected in any of OANDA’s
    /// proprietary or partner trading platforms, their transaction history or their account statements. OANDA
    /// platforms always assume that an Order’s trigger condition is set to the default value when indicating
    /// the distance from an Order’s trigger price, and will always provide the default trigger condition when
    /// creating or modifying an Order. A special restriction applies when creating a Guaranteed Stop Loss
    /// Order. In this case the `TriggerCondition` value must either be “DEFAULT”, or the “natural” trigger side
    /// “DEFAULT” results in. So for a Guaranteed Stop Loss Order for a long trade valid values are “DEFAULT”
    /// and “BID”, and for short trades “DEFAULT” and “ASK” are valid.
    #[serde(rename = "triggerCondition")]
    pub trigger_condition: OrderTriggerCondition,
    /// Flag indicating that the Stop Loss Order is guaranteed. The default value depends on the
    /// `GuaranteedStopLossOrderMode` of the account, if it is REQUIRED, the default will be true, for DISABLED or
    /// ENABLED the default is false. Deprecated: Will be removed in a future API update.
    #[serde(
        rename = "guaranteed",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub guaranteed: Option<bool>,
    /// The client extensions to add to the Order. Do not set, modify, or delete clientExtensions if your
    /// account is associated with MT4.
    #[serde(
        rename = "clientExtensions",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub client_extensions: Option<ClientExtensions>,
}

/// A `GuaranteedStopLossOrderRequest` specifies the parameters that may be set when creating a Guaranteed
/// Stop Loss Order. Only one of the price and distance fields may be specified.
///
/// The published definition requires `price` while allowing `distance` only in
/// its absence. This DTO follows that requiredness: outbound admission rejects
/// an explicit distance alongside the required price. Distance-only creation is
/// unavailable under this published contract; received fields remain inspectable.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GuaranteedStopLossOrderRequest {
    /// The type of the Order to Create. Must be set to “`GUARANTEED_STOP_LOSS`” when creating a Guaranteed Stop
    /// Loss Order.
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    #[serde(serialize_with = "serialize_guaranteed_stop_loss_order_request_type")]
    pub r#type: Option<OrderType>,
    /// The ID of the Trade to close when the price threshold is breached.
    #[serde(rename = "tradeID")]
    pub trade_id: TradeID,
    /// The client ID of the Trade to be closed when the price threshold is breached.
    #[serde(
        rename = "clientTradeID",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub client_trade_id: Option<ClientID>,
    /// The price threshold specified for the Guaranteed Stop Loss Order. The associated Trade will be closed at
    /// this price.
    #[serde(rename = "price")]
    #[serde(deserialize_with = "crate::decimal_wire::number_or_string")]
    pub price: Decimal,
    /// Specifies the distance (in price units) from the Account’s current price to use as the Guaranteed Stop
    /// Loss Order price. If the Trade is short the Instrument’s bid price is used, and for long Trades the ask
    /// is used.
    #[serde(rename = "distance", default, skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "crate::decimal_wire::optional_number_or_string")]
    pub distance: Option<Decimal>,
    /// The time-in-force requested for the `GuaranteedStopLoss` Order. Restricted to “GTC”, “GFD” and “GTD” for
    /// `GuaranteedStopLoss` Orders.
    #[serde(rename = "timeInForce")]
    pub time_in_force: TimeInForce,
    /// The date/time when the `GuaranteedStopLoss` Order will be cancelled if its timeInForce is “GTD”.
    #[serde(rename = "gtdTime", default, skip_serializing_if = "Option::is_none")]
    pub gtd_time: Option<Timestamp>,
    /// Specification of which price component should be used when determining if an Order should be triggered
    /// and filled. This allows Orders to be triggered based on the bid, ask, mid, default (ask for buy, bid for
    /// sell) or inverse (ask for sell, bid for buy) price depending on the desired behaviour. Orders are always
    /// filled using their default price component. This feature is only provided through the REST API. Clients
    /// who choose to specify a non-default trigger condition will not see it reflected in any of OANDA’s
    /// proprietary or partner trading platforms, their transaction history or their account statements. OANDA
    /// platforms always assume that an Order’s trigger condition is set to the default value when indicating
    /// the distance from an Order’s trigger price, and will always provide the default trigger condition when
    /// creating or modifying an Order. A special restriction applies when creating a Guaranteed Stop Loss
    /// Order. In this case the `TriggerCondition` value must either be “DEFAULT”, or the “natural” trigger side
    /// “DEFAULT” results in. So for a Guaranteed Stop Loss Order for a long trade valid values are “DEFAULT”
    /// and “BID”, and for short trades “DEFAULT” and “ASK” are valid.
    #[serde(rename = "triggerCondition")]
    pub trigger_condition: OrderTriggerCondition,
    /// The client extensions to add to the Order. Do not set, modify, or delete clientExtensions if your
    /// account is associated with MT4.
    #[serde(
        rename = "clientExtensions",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub client_extensions: Option<ClientExtensions>,
}

/// A `TrailingStopLossOrderRequest` specifies the parameters that may be set when creating a Trailing Stop
/// Loss Order.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrailingStopLossOrderRequest {
    /// The type of the Order to Create. Must be set to “`TRAILING_STOP_LOSS`” when creating a Trailing Stop Loss
    /// Order.
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    #[serde(serialize_with = "serialize_trailing_stop_loss_order_request_type")]
    pub r#type: Option<OrderType>,
    /// The ID of the Trade to close when the price threshold is breached.
    #[serde(rename = "tradeID")]
    pub trade_id: TradeID,
    /// The client ID of the Trade to be closed when the price threshold is breached.
    #[serde(
        rename = "clientTradeID",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub client_trade_id: Option<ClientID>,
    /// The price distance (in price units) specified for the `TrailingStopLoss` Order.
    #[serde(rename = "distance")]
    #[serde(deserialize_with = "crate::decimal_wire::number_or_string")]
    pub distance: Decimal,
    /// The time-in-force requested for the `TrailingStopLoss` Order. Restricted to “GTC”, “GFD” and “GTD” for
    /// `TrailingStopLoss` Orders.
    #[serde(rename = "timeInForce")]
    pub time_in_force: TimeInForce,
    /// The date/time when the `StopLoss` Order will be cancelled if its timeInForce is “GTD”.
    #[serde(rename = "gtdTime", default, skip_serializing_if = "Option::is_none")]
    pub gtd_time: Option<Timestamp>,
    /// Specification of which price component should be used when determining if an Order should be triggered
    /// and filled. This allows Orders to be triggered based on the bid, ask, mid, default (ask for buy, bid for
    /// sell) or inverse (ask for sell, bid for buy) price depending on the desired behaviour. Orders are always
    /// filled using their default price component. This feature is only provided through the REST API. Clients
    /// who choose to specify a non-default trigger condition will not see it reflected in any of OANDA’s
    /// proprietary or partner trading platforms, their transaction history or their account statements. OANDA
    /// platforms always assume that an Order’s trigger condition is set to the default value when indicating
    /// the distance from an Order’s trigger price, and will always provide the default trigger condition when
    /// creating or modifying an Order. A special restriction applies when creating a Guaranteed Stop Loss
    /// Order. In this case the `TriggerCondition` value must either be “DEFAULT”, or the “natural” trigger side
    /// “DEFAULT” results in. So for a Guaranteed Stop Loss Order for a long trade valid values are “DEFAULT”
    /// and “BID”, and for short trades “DEFAULT” and “ASK” are valid.
    #[serde(rename = "triggerCondition")]
    pub trigger_condition: OrderTriggerCondition,
    /// The client extensions to add to the Order. Do not set, modify, or delete clientExtensions if your
    /// account is associated with MT4.
    #[serde(
        rename = "clientExtensions",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub client_extensions: Option<ClientExtensions>,
}

provider_enum! {
    /// The type of the Order.
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    #[non_exhaustive]
    pub enum OrderType {
        /// A Market Order
        Market => "MARKET",
        /// A Limit Order
        Limit => "LIMIT",
        /// A Stop Order
        Stop => "STOP",
        /// A Market-if-touched Order
        MarketIfTouched => "MARKET_IF_TOUCHED",
        /// A Take Profit Order
        TakeProfit => "TAKE_PROFIT",
        /// A Stop Loss Order
        StopLoss => "STOP_LOSS",
        /// A Guaranteed Stop Loss Order
        GuaranteedStopLoss => "GUARANTEED_STOP_LOSS",
        /// A Trailing Stop Loss Order
        TrailingStopLoss => "TRAILING_STOP_LOSS",
        /// A Fixed Price Order
        FixedPrice => "FIXED_PRICE",
        ;
        /// An unrecognized provider value, preserved for forward compatibility.
        Unknown(String),
    }
}

provider_enum! {
    /// The type of the Order.
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    #[non_exhaustive]
    pub enum CancellableOrderType {
        /// A Limit Order
        Limit => "LIMIT",
        /// A Stop Order
        Stop => "STOP",
        /// A Market-if-touched Order
        MarketIfTouched => "MARKET_IF_TOUCHED",
        /// A Take Profit Order
        TakeProfit => "TAKE_PROFIT",
        /// A Stop Loss Order
        StopLoss => "STOP_LOSS",
        /// A Guaranteed Stop Loss Order
        GuaranteedStopLoss => "GUARANTEED_STOP_LOSS",
        /// A Trailing Stop Loss Order
        TrailingStopLoss => "TRAILING_STOP_LOSS",
        ;
        /// An unrecognized provider value, preserved for forward compatibility.
        Unknown(String),
    }
}

provider_enum! {
    /// The current state of the Order.
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    #[non_exhaustive]
    pub enum OrderState {
        /// The Order is currently pending execution
        Pending => "PENDING",
        /// The Order has been filled
        Filled => "FILLED",
        /// The Order has been triggered
        Triggered => "TRIGGERED",
        /// The Order has been cancelled
        Cancelled => "CANCELLED",
        ;
        /// An unrecognized provider value, preserved for forward compatibility.
        Unknown(String),
    }
}

provider_enum! {
    /// Unrecognized spellings are retained and serialized unchanged for provider
    /// compatibility; endpoint admission applies any operation-specific restrictions.
    /// The state to filter the requested Orders by.
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    #[non_exhaustive]
    pub enum OrderStateFilter {
        /// The Orders that are currently pending execution
        Pending => "PENDING",
        /// The Orders that have been filled
        Filled => "FILLED",
        /// The Orders that have been triggered
        Triggered => "TRIGGERED",
        /// The Orders that have been cancelled
        Cancelled => "CANCELLED",
        /// The Orders that are in any of the possible states listed above
        All => "ALL",
        ;
        /// An unrecognized provider value, preserved for forward compatibility.
        Unknown(String),
    }
}

/// An `OrderIdentifier` is used to refer to an Order, and contains both the `OrderID` and the `ClientOrderID`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderIdentifier {
    /// The OANDA-assigned Order ID
    #[serde(rename = "orderID", default, skip_serializing_if = "Option::is_none")]
    pub order_id: Option<OrderID>,
    /// The client-provided client Order ID
    #[serde(
        rename = "clientOrderID",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub client_order_id: Option<ClientID>,
}

provider_enum! {
    /// The time-in-force of an Order. `TimeInForce` describes how long an Order should remain pending before
    /// being automatically cancelled by the execution system.
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    #[non_exhaustive]
    pub enum TimeInForce {
        /// The Order is “Good unTil Cancelled”
        Gtc => "GTC",
        /// The Order is “Good unTil Date” and will be cancelled at the provided time
        Gtd => "GTD",
        /// The Order is “Good For Day” and will be cancelled at 5pm New York time
        Gfd => "GFD",
        /// The Order must be immediately “Filled Or Killed”
        Fok => "FOK",
        /// The Order must be “Immediately partially filled Or Cancelled”
        Ioc => "IOC",
        ;
        /// An unrecognized provider value, preserved for forward compatibility.
        Unknown(String),
    }
}

provider_enum! {
    /// Specification of how Positions in the Account are modified when the Order is filled.
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    #[non_exhaustive]
    pub enum OrderPositionFill {
        /// When the Order is filled, only allow Positions to be opened or extended.
        OpenOnly => "OPEN_ONLY",
        /// When the Order is filled, always fully reduce an existing Position before opening a new Position.
        ReduceFirst => "REDUCE_FIRST",
        /// When the Order is filled, only reduce an existing Position.
        ReduceOnly => "REDUCE_ONLY",
        /// When the Order is filled, use `REDUCE_FIRST` behaviour for non-client hedging Accounts, and `OPEN_ONLY`
        /// behaviour for client hedging Accounts.
        Default => "DEFAULT",
        ;
        /// An unrecognized provider value, preserved for forward compatibility.
        Unknown(String),
    }
}

provider_enum! {
    /// Specification of which price component should be used when determining if an Order should be triggered
    /// and filled. This allows Orders to be triggered based on the bid, ask, mid, default (ask for buy, bid for
    /// sell) or inverse (ask for sell, bid for buy) price depending on the desired behaviour. Orders are always
    /// filled using their default price component. This feature is only provided through the REST API. Clients
    /// who choose to specify a non-default trigger condition will not see it reflected in any of OANDA’s
    /// proprietary or partner trading platforms, their transaction history or their account statements. OANDA
    /// platforms always assume that an Order’s trigger condition is set to the default value when indicating
    /// the distance from an Order’s trigger price, and will always provide the default trigger condition when
    /// creating or modifying an Order. A special restriction applies when creating a Guaranteed Stop Loss
    /// Order. In this case the `TriggerCondition` value must either be “DEFAULT”, or the “natural” trigger side
    /// “DEFAULT” results in. So for a Guaranteed Stop Loss Order for a long trade valid values are “DEFAULT”
    /// and “BID”, and for short trades “DEFAULT” and “ASK” are valid.
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    #[non_exhaustive]
    pub enum OrderTriggerCondition {
        /// Trigger an Order the “natural” way: compare its price to the ask for long Orders and bid for short
        /// Orders.
        Default => "DEFAULT",
        /// Trigger an Order the opposite of the “natural” way: compare its price the bid for long Orders and ask
        /// for short Orders.
        Inverse => "INVERSE",
        /// Trigger an Order by comparing its price to the bid regardless of whether it is long or short.
        Bid => "BID",
        /// Trigger an Order by comparing its price to the ask regardless of whether it is long or short.
        Ask => "ASK",
        /// Trigger an Order by comparing its price to the midpoint regardless of whether it is long or short.
        Mid => "MID",
        ;
        /// An unrecognized provider value, preserved for forward compatibility.
        Unknown(String),
    }
}

/// The dynamic state of an Order. This is only relevant to `TrailingStopLoss` Orders, as no other Order type
/// has dynamic state.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DynamicOrderState {
    /// The Order’s ID.
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<OrderID>,
    /// The Order’s calculated trailing stop value.
    #[serde(
        rename = "trailingStopValue",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    #[serde(deserialize_with = "crate::decimal_wire::optional_number_or_string")]
    pub trailing_stop_value: Option<Decimal>,
    /// The distance between the Trailing Stop Loss Order’s trailingStopValue and the current Market Price. This
    /// represents the distance (in price units) of the Order from a triggering price. If the distance could not
    /// be determined, this value will not be set.
    #[serde(
        rename = "triggerDistance",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    #[serde(deserialize_with = "crate::decimal_wire::optional_number_or_string")]
    pub trigger_distance: Option<Decimal>,
    /// True if an exact trigger distance could be calculated. If false, it means the provided trigger distance
    /// is a best estimate. If the distance could not be determined, this value will not be set.
    #[serde(
        rename = "isTriggerDistanceExact",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_trigger_distance_exact: Option<bool>,
}

/// Representation of many units of an Instrument are available to be traded for both long and short Orders.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UnitsAvailableDetails {
    /// The units available for long Orders.
    #[serde(rename = "long", default, skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "crate::decimal_wire::optional_number_or_string")]
    pub long: Option<Decimal>,
    /// The units available for short Orders.
    #[serde(rename = "short", default, skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "crate::decimal_wire::optional_number_or_string")]
    pub short: Option<Decimal>,
}

/// Representation of how many units of an Instrument are available to be traded by an Order depending on
/// its positionFill option.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UnitsAvailable {
    /// The number of units that are available to be traded using an Order with a positionFill option of
    /// “DEFAULT”. For an Account with hedging enabled, this value will be the same as the “`OPEN_ONLY`” value.
    /// For an Account without hedging enabled, this value will be the same as the “`REDUCE_FIRST`” value.
    #[serde(rename = "default", default, skip_serializing_if = "Option::is_none")]
    pub default: Option<UnitsAvailableDetails>,
    /// The number of units that may are available to be traded with an Order with a positionFill option of
    /// “`REDUCE_FIRST`”.
    #[serde(
        rename = "reduceFirst",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub reduce_first: Option<UnitsAvailableDetails>,
    /// The number of units that may are available to be traded with an Order with a positionFill option of
    /// “`REDUCE_ONLY`”.
    #[serde(
        rename = "reduceOnly",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub reduce_only: Option<UnitsAvailableDetails>,
    /// The number of units that may are available to be traded with an Order with a positionFill option of
    /// “`OPEN_ONLY`”.
    #[serde(rename = "openOnly", default, skip_serializing_if = "Option::is_none")]
    pub open_only: Option<UnitsAvailableDetails>,
}

/// Details required by clients creating a Guaranteed Stop Loss Order
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GuaranteedStopLossOrderEntryData {
    /// The minimum distance allowed between the Trade’s fill price and the configured price for guaranteed Stop
    /// Loss Orders created for this instrument. Specified in price units.
    #[serde(
        rename = "minimumDistance",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    #[serde(deserialize_with = "crate::decimal_wire::optional_number_or_string")]
    pub minimum_distance: Option<Decimal>,
    /// The amount that is charged to the account if a guaranteed Stop Loss Order is triggered and filled. The
    /// value is in price units and is charged for each unit of the Trade.
    #[serde(rename = "premium", default, skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "crate::decimal_wire::optional_number_or_string")]
    pub premium: Option<Decimal>,
    /// The guaranteed Stop Loss Order level restriction for this instrument.
    #[serde(
        rename = "levelRestriction",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub level_restriction: Option<GuaranteedStopLossOrderLevelRestriction>,
}

// A concrete DTO may omit its documented default, but cannot contradict its own type.
macro_rules! concrete_type_serializer {
    ($name:ident, $expected:literal) => {
        #[expect(
            clippy::ref_option,
            reason = "Serde serialize_with passes the optional field by reference"
        )]
        fn $name<S: serde::Serializer>(
            value: &Option<OrderType>,
            serializer: S,
        ) -> Result<S::Ok, S::Error> {
            if let Some(observed) = value
                && observed.as_str() != $expected
            {
                return Err(serde::ser::Error::custom(format!(
                    "OANDA type discriminator mismatch: expected {:?}, observed {:?}",
                    $expected,
                    observed.as_str()
                )));
            }
            value.serialize(serializer)
        }
    };
}
concrete_type_serializer!(serialize_market_order_type, "MARKET");
concrete_type_serializer!(serialize_limit_order_type, "LIMIT");
concrete_type_serializer!(serialize_stop_order_type, "STOP");
concrete_type_serializer!(serialize_market_if_touched_order_type, "MARKET_IF_TOUCHED");
concrete_type_serializer!(serialize_take_profit_order_type, "TAKE_PROFIT");
concrete_type_serializer!(serialize_stop_loss_order_type, "STOP_LOSS");
concrete_type_serializer!(
    serialize_guaranteed_stop_loss_order_type,
    "GUARANTEED_STOP_LOSS"
);
concrete_type_serializer!(
    serialize_trailing_stop_loss_order_type,
    "TRAILING_STOP_LOSS"
);
concrete_type_serializer!(serialize_fixed_price_order_type, "FIXED_PRICE");
concrete_type_serializer!(serialize_market_order_request_type, "MARKET");
concrete_type_serializer!(serialize_limit_order_request_type, "LIMIT");
concrete_type_serializer!(serialize_stop_order_request_type, "STOP");
concrete_type_serializer!(
    serialize_market_if_touched_order_request_type,
    "MARKET_IF_TOUCHED"
);
concrete_type_serializer!(serialize_take_profit_order_request_type, "TAKE_PROFIT");
concrete_type_serializer!(serialize_stop_loss_order_request_type, "STOP_LOSS");
concrete_type_serializer!(
    serialize_guaranteed_stop_loss_order_request_type,
    "GUARANTEED_STOP_LOSS"
);
concrete_type_serializer!(
    serialize_trailing_stop_loss_order_request_type,
    "TRAILING_STOP_LOSS"
);

pub(super) fn validate_time_in_force(
    value: &TimeInForce,
    market: bool,
    gtd_time: Option<&Timestamp>,
    require_gtd_time: bool,
) -> crate::Result<()> {
    match value {
        TimeInForce::Unknown(_) => {
            return Err(crate::Error::InvalidInput("unsupported timeInForce".into()));
        }
        TimeInForce::Fok | TimeInForce::Ioc if !market => {
            return Err(crate::Error::InvalidInput(
                "non-market order cannot use FOK or IOC".into(),
            ));
        }
        TimeInForce::Gtc | TimeInForce::Gtd | TimeInForce::Gfd if market => {
            return Err(crate::Error::InvalidInput(
                "market order requires FOK or IOC".into(),
            ));
        }
        TimeInForce::Gtd if require_gtd_time && gtd_time.is_none() => {
            return Err(crate::Error::InvalidInput(
                "GTD order requires gtdTime".into(),
            ));
        }
        TimeInForce::Fok
        | TimeInForce::Ioc
        | TimeInForce::Gtc
        | TimeInForce::Gtd
        | TimeInForce::Gfd => {}
    }
    Ok(())
}

// These details create dependent orders. Unlike a partial modification, a
// supplied GTD instruction cannot inherit its cancellation time from an order.
struct OnFill<'a> {
    take_profit: Option<&'a TakeProfitDetails>,
    stop_loss: Option<&'a StopLossDetails>,
    guaranteed_stop_loss: Option<&'a GuaranteedStopLossDetails>,
    trailing_stop_loss: Option<&'a TrailingStopLossDetails>,
}

impl crate::validation::Validate for OnFill<'_> {
    fn validate(&self) -> crate::Result<()> {
        if let Some(details) = self.take_profit {
            details.validate()?;
            if let Some(time_in_force) = &details.time_in_force {
                validate_time_in_force(time_in_force, false, details.gtd_time.as_ref(), true)?;
            }
        }
        if let Some(details) = self.stop_loss {
            details.validate()?;
            if let Some(time_in_force) = &details.time_in_force {
                validate_time_in_force(time_in_force, false, details.gtd_time.as_ref(), true)?;
            }
        }
        if let Some(details) = self.guaranteed_stop_loss {
            details.validate()?;
            if let Some(time_in_force) = &details.time_in_force {
                validate_time_in_force(time_in_force, false, details.gtd_time.as_ref(), true)?;
            }
        }
        if let Some(details) = self.trailing_stop_loss {
            details.validate()?;
            if let Some(time_in_force) = &details.time_in_force {
                validate_time_in_force(time_in_force, false, details.gtd_time.as_ref(), true)?;
            }
        }
        Ok(())
    }
}

impl crate::validation::Validate for MarketOrderRequest {
    fn validate(&self) -> crate::Result<()> {
        if self.units.is_zero() {
            return Err(crate::Error::InvalidInput(
                "order units must not be zero".into(),
            ));
        }
        validate_time_in_force(&self.time_in_force, true, None, true)?;
        if matches!(self.position_fill, OrderPositionFill::Unknown(_)) {
            return Err(crate::Error::InvalidInput(
                "unsupported positionFill".into(),
            ));
        }
        OnFill {
            take_profit: self.take_profit_on_fill.as_ref(),
            stop_loss: self.stop_loss_on_fill.as_ref(),
            guaranteed_stop_loss: self.guaranteed_stop_loss_on_fill.as_ref(),
            trailing_stop_loss: self.trailing_stop_loss_on_fill.as_ref(),
        }
        .validate()?;
        Ok(())
    }
}

impl crate::validation::Validate for LimitOrderRequest {
    fn validate(&self) -> crate::Result<()> {
        if self.units.is_zero() {
            return Err(crate::Error::InvalidInput(
                "order units must not be zero".into(),
            ));
        }
        validate_time_in_force(&self.time_in_force, false, self.gtd_time.as_ref(), true)?;
        if matches!(self.position_fill, OrderPositionFill::Unknown(_)) {
            return Err(crate::Error::InvalidInput(
                "unsupported positionFill".into(),
            ));
        }
        if matches!(self.trigger_condition, OrderTriggerCondition::Unknown(_)) {
            return Err(crate::Error::InvalidInput(
                "unsupported triggerCondition".into(),
            ));
        }
        OnFill {
            take_profit: self.take_profit_on_fill.as_ref(),
            stop_loss: self.stop_loss_on_fill.as_ref(),
            guaranteed_stop_loss: self.guaranteed_stop_loss_on_fill.as_ref(),
            trailing_stop_loss: self.trailing_stop_loss_on_fill.as_ref(),
        }
        .validate()?;
        Ok(())
    }
}

impl crate::validation::Validate for StopOrderRequest {
    fn validate(&self) -> crate::Result<()> {
        if self.units.is_zero() {
            return Err(crate::Error::InvalidInput(
                "order units must not be zero".into(),
            ));
        }
        validate_time_in_force(&self.time_in_force, false, self.gtd_time.as_ref(), true)?;
        if matches!(self.position_fill, OrderPositionFill::Unknown(_)) {
            return Err(crate::Error::InvalidInput(
                "unsupported positionFill".into(),
            ));
        }
        if matches!(self.trigger_condition, OrderTriggerCondition::Unknown(_)) {
            return Err(crate::Error::InvalidInput(
                "unsupported triggerCondition".into(),
            ));
        }
        OnFill {
            take_profit: self.take_profit_on_fill.as_ref(),
            stop_loss: self.stop_loss_on_fill.as_ref(),
            guaranteed_stop_loss: self.guaranteed_stop_loss_on_fill.as_ref(),
            trailing_stop_loss: self.trailing_stop_loss_on_fill.as_ref(),
        }
        .validate()?;
        Ok(())
    }
}

impl crate::validation::Validate for MarketIfTouchedOrderRequest {
    fn validate(&self) -> crate::Result<()> {
        if self.units.is_zero() {
            return Err(crate::Error::InvalidInput(
                "order units must not be zero".into(),
            ));
        }
        validate_time_in_force(&self.time_in_force, false, self.gtd_time.as_ref(), true)?;
        if matches!(self.position_fill, OrderPositionFill::Unknown(_)) {
            return Err(crate::Error::InvalidInput(
                "unsupported positionFill".into(),
            ));
        }
        if matches!(self.trigger_condition, OrderTriggerCondition::Unknown(_)) {
            return Err(crate::Error::InvalidInput(
                "unsupported triggerCondition".into(),
            ));
        }
        OnFill {
            take_profit: self.take_profit_on_fill.as_ref(),
            stop_loss: self.stop_loss_on_fill.as_ref(),
            guaranteed_stop_loss: self.guaranteed_stop_loss_on_fill.as_ref(),
            trailing_stop_loss: self.trailing_stop_loss_on_fill.as_ref(),
        }
        .validate()?;
        Ok(())
    }
}

impl crate::validation::Validate for TakeProfitOrderRequest {
    fn validate(&self) -> crate::Result<()> {
        validate_time_in_force(&self.time_in_force, false, self.gtd_time.as_ref(), true)?;
        if matches!(self.trigger_condition, OrderTriggerCondition::Unknown(_)) {
            return Err(crate::Error::InvalidInput(
                "unsupported triggerCondition".into(),
            ));
        }
        Ok(())
    }
}

impl crate::validation::Validate for StopLossOrderRequest {
    fn validate(&self) -> crate::Result<()> {
        validate_time_in_force(&self.time_in_force, false, self.gtd_time.as_ref(), true)?;
        if matches!(self.trigger_condition, OrderTriggerCondition::Unknown(_)) {
            return Err(crate::Error::InvalidInput(
                "unsupported triggerCondition".into(),
            ));
        }
        if self.distance.is_some() {
            return Err(crate::Error::InvalidInput(
                "price and distance are mutually exclusive".into(),
            ));
        }
        Ok(())
    }
}

impl crate::validation::Validate for GuaranteedStopLossOrderRequest {
    fn validate(&self) -> crate::Result<()> {
        validate_time_in_force(&self.time_in_force, false, self.gtd_time.as_ref(), true)?;
        if matches!(self.trigger_condition, OrderTriggerCondition::Unknown(_)) {
            return Err(crate::Error::InvalidInput(
                "unsupported triggerCondition".into(),
            ));
        }
        if self.distance.is_some() {
            return Err(crate::Error::InvalidInput(
                "price and distance are mutually exclusive".into(),
            ));
        }
        Ok(())
    }
}

impl crate::validation::Validate for TrailingStopLossOrderRequest {
    fn validate(&self) -> crate::Result<()> {
        validate_time_in_force(&self.time_in_force, false, self.gtd_time.as_ref(), true)?;
        if matches!(self.trigger_condition, OrderTriggerCondition::Unknown(_)) {
            return Err(crate::Error::InvalidInput(
                "unsupported triggerCondition".into(),
            ));
        }
        Ok(())
    }
}
