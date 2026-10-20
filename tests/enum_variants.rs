// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Round-trip every scalar enum value listed on OANDA's definition pages.
// Assertions and fixture setup intentionally fail fast in tests; production paths return errors.
#![allow(
    clippy::expect_used,
    clippy::unwrap_used,
    clippy::panic,
    reason = "test setup and assertions fail immediately with fixture context"
)]
use oanda_client::models;
use serde::{Serialize, de::DeserializeOwned};
use std::fmt::Debug;
fn round_trip<T: Serialize + DeserializeOwned + Debug + PartialEq>(wire: &str, expected: &T) {
    let decoded: T = serde_json::from_value(serde_json::Value::String(wire.to_owned()))
        .expect("decode documented enum");
    assert_eq!(
        &decoded, expected,
        "{wire:?} must decode to its documented variant, not to the Unknown arm"
    );
    let encoded = serde_json::to_value(decoded).expect("encode documented enum");
    assert_eq!(encoded, serde_json::Value::String(wire.to_owned()));
}
#[allow(
    clippy::too_many_lines,
    reason = "the reviewed fixture table is one completeness contract"
)]
#[test]
fn all_documented_scalar_enum_variants_round_trip() {
    let mut tested: std::collections::BTreeMap<&str, std::collections::BTreeSet<&str>> =
        std::collections::BTreeMap::new();
    macro_rules! fixture {
        ($model:ident, $wire:literal, $variant:ident) => {{
            round_trip::<models::$model>($wire, &models::$model::$variant);
            assert!(
                tested.entry(stringify!($model)).or_default().insert($wire),
                "duplicate enum fixture"
            );
        }};
    }
    fixture!(GuaranteedStopLossOrderMode, "DISABLED", Disabled);
    fixture!(GuaranteedStopLossOrderMode, "ALLOWED", Allowed);
    fixture!(GuaranteedStopLossOrderMode, "REQUIRED", Required);
    fixture!(GuaranteedStopLossOrderMutability, "FIXED", Fixed);
    fixture!(
        GuaranteedStopLossOrderMutability,
        "REPLACEABLE",
        Replaceable
    );
    fixture!(GuaranteedStopLossOrderMutability, "CANCELABLE", Cancelable);
    fixture!(
        GuaranteedStopLossOrderMutability,
        "PRICE_WIDEN_ONLY",
        PriceWidenOnly
    );
    fixture!(AccountFinancingMode, "NO_FINANCING", NoFinancing);
    fixture!(AccountFinancingMode, "SECOND_BY_SECOND", SecondBySecond);
    fixture!(AccountFinancingMode, "DAILY", Daily);
    fixture!(PositionAggregationMode, "ABSOLUTE_SUM", AbsoluteSum);
    fixture!(PositionAggregationMode, "MAXIMAL_SIDE", MaximalSide);
    fixture!(PositionAggregationMode, "NET_SUM", NetSum);
    fixture!(CandlestickGranularity, "S5", S5);
    fixture!(CandlestickGranularity, "S10", S10);
    fixture!(CandlestickGranularity, "S15", S15);
    fixture!(CandlestickGranularity, "S30", S30);
    fixture!(CandlestickGranularity, "M1", M1);
    fixture!(CandlestickGranularity, "M2", M2);
    fixture!(CandlestickGranularity, "M4", M4);
    fixture!(CandlestickGranularity, "M5", M5);
    fixture!(CandlestickGranularity, "M10", M10);
    fixture!(CandlestickGranularity, "M15", M15);
    fixture!(CandlestickGranularity, "M30", M30);
    fixture!(CandlestickGranularity, "H1", H1);
    fixture!(CandlestickGranularity, "H2", H2);
    fixture!(CandlestickGranularity, "H3", H3);
    fixture!(CandlestickGranularity, "H4", H4);
    fixture!(CandlestickGranularity, "H6", H6);
    fixture!(CandlestickGranularity, "H8", H8);
    fixture!(CandlestickGranularity, "H12", H12);
    fixture!(CandlestickGranularity, "D", D);
    fixture!(CandlestickGranularity, "W", W);
    fixture!(CandlestickGranularity, "M", M);
    fixture!(WeeklyAlignment, "Monday", Monday);
    fixture!(WeeklyAlignment, "Tuesday", Tuesday);
    fixture!(WeeklyAlignment, "Wednesday", Wednesday);
    fixture!(WeeklyAlignment, "Thursday", Thursday);
    fixture!(WeeklyAlignment, "Friday", Friday);
    fixture!(WeeklyAlignment, "Saturday", Saturday);
    fixture!(WeeklyAlignment, "Sunday", Sunday);
    fixture!(OrderType, "MARKET", Market);
    fixture!(OrderType, "LIMIT", Limit);
    fixture!(OrderType, "STOP", Stop);
    fixture!(OrderType, "MARKET_IF_TOUCHED", MarketIfTouched);
    fixture!(OrderType, "TAKE_PROFIT", TakeProfit);
    fixture!(OrderType, "STOP_LOSS", StopLoss);
    fixture!(OrderType, "GUARANTEED_STOP_LOSS", GuaranteedStopLoss);
    fixture!(OrderType, "TRAILING_STOP_LOSS", TrailingStopLoss);
    fixture!(OrderType, "FIXED_PRICE", FixedPrice);
    fixture!(CancellableOrderType, "LIMIT", Limit);
    fixture!(CancellableOrderType, "STOP", Stop);
    fixture!(CancellableOrderType, "MARKET_IF_TOUCHED", MarketIfTouched);
    fixture!(CancellableOrderType, "TAKE_PROFIT", TakeProfit);
    fixture!(CancellableOrderType, "STOP_LOSS", StopLoss);
    fixture!(
        CancellableOrderType,
        "GUARANTEED_STOP_LOSS",
        GuaranteedStopLoss
    );
    fixture!(CancellableOrderType, "TRAILING_STOP_LOSS", TrailingStopLoss);
    fixture!(OrderState, "PENDING", Pending);
    fixture!(OrderState, "FILLED", Filled);
    fixture!(OrderState, "TRIGGERED", Triggered);
    fixture!(OrderState, "CANCELLED", Cancelled);
    fixture!(OrderStateFilter, "PENDING", Pending);
    fixture!(OrderStateFilter, "FILLED", Filled);
    fixture!(OrderStateFilter, "TRIGGERED", Triggered);
    fixture!(OrderStateFilter, "CANCELLED", Cancelled);
    fixture!(OrderStateFilter, "ALL", All);
    fixture!(TimeInForce, "GTC", Gtc);
    fixture!(TimeInForce, "GTD", Gtd);
    fixture!(TimeInForce, "GFD", Gfd);
    fixture!(TimeInForce, "FOK", Fok);
    fixture!(TimeInForce, "IOC", Ioc);
    fixture!(OrderPositionFill, "OPEN_ONLY", OpenOnly);
    fixture!(OrderPositionFill, "REDUCE_FIRST", ReduceFirst);
    fixture!(OrderPositionFill, "REDUCE_ONLY", ReduceOnly);
    fixture!(OrderPositionFill, "DEFAULT", Default);
    fixture!(OrderTriggerCondition, "DEFAULT", Default);
    fixture!(OrderTriggerCondition, "INVERSE", Inverse);
    fixture!(OrderTriggerCondition, "BID", Bid);
    fixture!(OrderTriggerCondition, "ASK", Ask);
    fixture!(OrderTriggerCondition, "MID", Mid);
    fixture!(TradeState, "OPEN", Open);
    fixture!(TradeState, "CLOSED", Closed);
    fixture!(TradeState, "CLOSE_WHEN_TRADEABLE", CloseWhenTradeable);
    fixture!(TradeStateFilter, "OPEN", Open);
    fixture!(TradeStateFilter, "CLOSED", Closed);
    fixture!(TradeStateFilter, "CLOSE_WHEN_TRADEABLE", CloseWhenTradeable);
    fixture!(TradeStateFilter, "ALL", All);
    fixture!(TradePL, "POSITIVE", Positive);
    fixture!(TradePL, "NEGATIVE", Negative);
    fixture!(TradePL, "ZERO", Zero);
    fixture!(TransactionType, "CREATE", Create);
    fixture!(TransactionType, "CLOSE", Close);
    fixture!(TransactionType, "REOPEN", Reopen);
    fixture!(TransactionType, "CLIENT_CONFIGURE", ClientConfigure);
    fixture!(
        TransactionType,
        "CLIENT_CONFIGURE_REJECT",
        ClientConfigureReject
    );
    fixture!(TransactionType, "TRANSFER_FUNDS", TransferFunds);
    fixture!(
        TransactionType,
        "TRANSFER_FUNDS_REJECT",
        TransferFundsReject
    );
    fixture!(TransactionType, "MARKET_ORDER", MarketOrder);
    fixture!(TransactionType, "MARKET_ORDER_REJECT", MarketOrderReject);
    fixture!(TransactionType, "FIXED_PRICE_ORDER", FixedPriceOrder);
    fixture!(TransactionType, "LIMIT_ORDER", LimitOrder);
    fixture!(TransactionType, "LIMIT_ORDER_REJECT", LimitOrderReject);
    fixture!(TransactionType, "STOP_ORDER", StopOrder);
    fixture!(TransactionType, "STOP_ORDER_REJECT", StopOrderReject);
    fixture!(
        TransactionType,
        "MARKET_IF_TOUCHED_ORDER",
        MarketIfTouchedOrder
    );
    fixture!(
        TransactionType,
        "MARKET_IF_TOUCHED_ORDER_REJECT",
        MarketIfTouchedOrderReject
    );
    fixture!(TransactionType, "TAKE_PROFIT_ORDER", TakeProfitOrder);
    fixture!(
        TransactionType,
        "TAKE_PROFIT_ORDER_REJECT",
        TakeProfitOrderReject
    );
    fixture!(TransactionType, "STOP_LOSS_ORDER", StopLossOrder);
    fixture!(
        TransactionType,
        "STOP_LOSS_ORDER_REJECT",
        StopLossOrderReject
    );
    fixture!(
        TransactionType,
        "GUARANTEED_STOP_LOSS_ORDER",
        GuaranteedStopLossOrder
    );
    fixture!(
        TransactionType,
        "GUARANTEED_STOP_LOSS_ORDER_REJECT",
        GuaranteedStopLossOrderReject
    );
    fixture!(
        TransactionType,
        "TRAILING_STOP_LOSS_ORDER",
        TrailingStopLossOrder
    );
    fixture!(
        TransactionType,
        "TRAILING_STOP_LOSS_ORDER_REJECT",
        TrailingStopLossOrderReject
    );
    fixture!(TransactionType, "ORDER_FILL", OrderFill);
    fixture!(TransactionType, "ORDER_CANCEL", OrderCancel);
    fixture!(TransactionType, "ORDER_CANCEL_REJECT", OrderCancelReject);
    fixture!(
        TransactionType,
        "ORDER_CLIENT_EXTENSIONS_MODIFY",
        OrderClientExtensionsModify
    );
    fixture!(
        TransactionType,
        "ORDER_CLIENT_EXTENSIONS_MODIFY_REJECT",
        OrderClientExtensionsModifyReject
    );
    fixture!(
        TransactionType,
        "TRADE_CLIENT_EXTENSIONS_MODIFY",
        TradeClientExtensionsModify
    );
    fixture!(
        TransactionType,
        "TRADE_CLIENT_EXTENSIONS_MODIFY_REJECT",
        TradeClientExtensionsModifyReject
    );
    fixture!(TransactionType, "MARGIN_CALL_ENTER", MarginCallEnter);
    fixture!(TransactionType, "MARGIN_CALL_EXTEND", MarginCallExtend);
    fixture!(TransactionType, "MARGIN_CALL_EXIT", MarginCallExit);
    fixture!(
        TransactionType,
        "DELAYED_TRADE_CLOSURE",
        DelayedTradeClosure
    );
    fixture!(TransactionType, "DAILY_FINANCING", DailyFinancing);
    fixture!(TransactionType, "DIVIDEND_ADJUSTMENT", DividendAdjustment);
    fixture!(TransactionType, "RESET_RESETTABLE_PL", ResetResettablePl);
    fixture!(FundingReason, "CLIENT_FUNDING", ClientFunding);
    fixture!(FundingReason, "ACCOUNT_TRANSFER", AccountTransfer);
    fixture!(FundingReason, "DIVISION_MIGRATION", DivisionMigration);
    fixture!(FundingReason, "SITE_MIGRATION", SiteMigration);
    fixture!(FundingReason, "ADJUSTMENT", Adjustment);
    fixture!(MarketOrderReason, "CLIENT_ORDER", ClientOrder);
    fixture!(MarketOrderReason, "TRADE_CLOSE", TradeClose);
    fixture!(MarketOrderReason, "POSITION_CLOSEOUT", PositionCloseout);
    fixture!(MarketOrderReason, "MARGIN_CLOSEOUT", MarginCloseout);
    fixture!(MarketOrderReason, "DELAYED_TRADE_CLOSE", DelayedTradeClose);
    fixture!(
        FixedPriceOrderReason,
        "PLATFORM_ACCOUNT_MIGRATION",
        PlatformAccountMigration
    );
    fixture!(
        FixedPriceOrderReason,
        "TRADE_CLOSE_DIVISION_ACCOUNT_MIGRATION",
        TradeCloseDivisionAccountMigration
    );
    fixture!(
        FixedPriceOrderReason,
        "TRADE_CLOSE_ADMINISTRATIVE_ACTION",
        TradeCloseAdministrativeAction
    );
    fixture!(LimitOrderReason, "CLIENT_ORDER", ClientOrder);
    fixture!(LimitOrderReason, "REPLACEMENT", Replacement);
    fixture!(StopOrderReason, "CLIENT_ORDER", ClientOrder);
    fixture!(StopOrderReason, "REPLACEMENT", Replacement);
    fixture!(MarketIfTouchedOrderReason, "CLIENT_ORDER", ClientOrder);
    fixture!(MarketIfTouchedOrderReason, "REPLACEMENT", Replacement);
    fixture!(TakeProfitOrderReason, "CLIENT_ORDER", ClientOrder);
    fixture!(TakeProfitOrderReason, "REPLACEMENT", Replacement);
    fixture!(TakeProfitOrderReason, "ON_FILL", OnFill);
    fixture!(StopLossOrderReason, "CLIENT_ORDER", ClientOrder);
    fixture!(StopLossOrderReason, "REPLACEMENT", Replacement);
    fixture!(StopLossOrderReason, "ON_FILL", OnFill);
    fixture!(GuaranteedStopLossOrderReason, "CLIENT_ORDER", ClientOrder);
    fixture!(GuaranteedStopLossOrderReason, "REPLACEMENT", Replacement);
    fixture!(GuaranteedStopLossOrderReason, "ON_FILL", OnFill);
    fixture!(TrailingStopLossOrderReason, "CLIENT_ORDER", ClientOrder);
    fixture!(TrailingStopLossOrderReason, "REPLACEMENT", Replacement);
    fixture!(TrailingStopLossOrderReason, "ON_FILL", OnFill);
    fixture!(OrderFillReason, "LIMIT_ORDER", LimitOrder);
    fixture!(OrderFillReason, "STOP_ORDER", StopOrder);
    fixture!(
        OrderFillReason,
        "MARKET_IF_TOUCHED_ORDER",
        MarketIfTouchedOrder
    );
    fixture!(OrderFillReason, "TAKE_PROFIT_ORDER", TakeProfitOrder);
    fixture!(OrderFillReason, "STOP_LOSS_ORDER", StopLossOrder);
    fixture!(
        OrderFillReason,
        "GUARANTEED_STOP_LOSS_ORDER",
        GuaranteedStopLossOrder
    );
    fixture!(
        OrderFillReason,
        "TRAILING_STOP_LOSS_ORDER",
        TrailingStopLossOrder
    );
    fixture!(OrderFillReason, "MARKET_ORDER", MarketOrder);
    fixture!(
        OrderFillReason,
        "MARKET_ORDER_TRADE_CLOSE",
        MarketOrderTradeClose
    );
    fixture!(
        OrderFillReason,
        "MARKET_ORDER_POSITION_CLOSEOUT",
        MarketOrderPositionCloseout
    );
    fixture!(
        OrderFillReason,
        "MARKET_ORDER_MARGIN_CLOSEOUT",
        MarketOrderMarginCloseout
    );
    fixture!(
        OrderFillReason,
        "MARKET_ORDER_DELAYED_TRADE_CLOSE",
        MarketOrderDelayedTradeClose
    );
    fixture!(OrderFillReason, "FIXED_PRICE_ORDER", FixedPriceOrder);
    fixture!(
        OrderFillReason,
        "FIXED_PRICE_ORDER_PLATFORM_ACCOUNT_MIGRATION",
        FixedPriceOrderPlatformAccountMigration
    );
    fixture!(
        OrderFillReason,
        "FIXED_PRICE_ORDER_DIVISION_ACCOUNT_MIGRATION",
        FixedPriceOrderDivisionAccountMigration
    );
    fixture!(
        OrderFillReason,
        "FIXED_PRICE_ORDER_ADMINISTRATIVE_ACTION",
        FixedPriceOrderAdministrativeAction
    );
    fixture!(
        OrderCancelReason,
        "INTERNAL_SERVER_ERROR",
        InternalServerError
    );
    fixture!(OrderCancelReason, "ACCOUNT_LOCKED", AccountLocked);
    fixture!(
        OrderCancelReason,
        "ACCOUNT_NEW_POSITIONS_LOCKED",
        AccountNewPositionsLocked
    );
    fixture!(
        OrderCancelReason,
        "ACCOUNT_ORDER_CREATION_LOCKED",
        AccountOrderCreationLocked
    );
    fixture!(
        OrderCancelReason,
        "ACCOUNT_ORDER_FILL_LOCKED",
        AccountOrderFillLocked
    );
    fixture!(OrderCancelReason, "CLIENT_REQUEST", ClientRequest);
    fixture!(OrderCancelReason, "MIGRATION", Migration);
    fixture!(OrderCancelReason, "MARKET_HALTED", MarketHalted);
    fixture!(OrderCancelReason, "LINKED_TRADE_CLOSED", LinkedTradeClosed);
    fixture!(
        OrderCancelReason,
        "TIME_IN_FORCE_EXPIRED",
        TimeInForceExpired
    );
    fixture!(OrderCancelReason, "INSUFFICIENT_MARGIN", InsufficientMargin);
    fixture!(OrderCancelReason, "FIFO_VIOLATION", FifoViolation);
    fixture!(OrderCancelReason, "BOUNDS_VIOLATION", BoundsViolation);
    fixture!(
        OrderCancelReason,
        "CLIENT_REQUEST_REPLACED",
        ClientRequestReplaced
    );
    fixture!(
        OrderCancelReason,
        "DIVIDEND_ADJUSTMENT_REPLACED",
        DividendAdjustmentReplaced
    );
    fixture!(
        OrderCancelReason,
        "INSUFFICIENT_LIQUIDITY",
        InsufficientLiquidity
    );
    fixture!(
        OrderCancelReason,
        "TAKE_PROFIT_ON_FILL_GTD_TIMESTAMP_IN_PAST",
        TakeProfitOnFillGtdTimestampInPast
    );
    fixture!(
        OrderCancelReason,
        "TAKE_PROFIT_ON_FILL_LOSS",
        TakeProfitOnFillLoss
    );
    fixture!(OrderCancelReason, "LOSING_TAKE_PROFIT", LosingTakeProfit);
    fixture!(
        OrderCancelReason,
        "STOP_LOSS_ON_FILL_GTD_TIMESTAMP_IN_PAST",
        StopLossOnFillGtdTimestampInPast
    );
    fixture!(
        OrderCancelReason,
        "STOP_LOSS_ON_FILL_LOSS",
        StopLossOnFillLoss
    );
    fixture!(
        OrderCancelReason,
        "STOP_LOSS_ON_FILL_PRICE_DISTANCE_MAXIMUM_EXCEEDED",
        StopLossOnFillPriceDistanceMaximumExceeded
    );
    fixture!(
        OrderCancelReason,
        "STOP_LOSS_ON_FILL_REQUIRED",
        StopLossOnFillRequired
    );
    fixture!(
        OrderCancelReason,
        "STOP_LOSS_ON_FILL_GUARANTEED_REQUIRED",
        StopLossOnFillGuaranteedRequired
    );
    fixture!(
        OrderCancelReason,
        "STOP_LOSS_ON_FILL_GUARANTEED_NOT_ALLOWED",
        StopLossOnFillGuaranteedNotAllowed
    );
    fixture!(
        OrderCancelReason,
        "STOP_LOSS_ON_FILL_GUARANTEED_MINIMUM_DISTANCE_NOT_MET",
        StopLossOnFillGuaranteedMinimumDistanceNotMet
    );
    fixture!(
        OrderCancelReason,
        "STOP_LOSS_ON_FILL_GUARANTEED_LEVEL_RESTRICTION_EXCEEDED",
        StopLossOnFillGuaranteedLevelRestrictionExceeded
    );
    fixture!(
        OrderCancelReason,
        "STOP_LOSS_ON_FILL_GUARANTEED_HEDGING_NOT_ALLOWED",
        StopLossOnFillGuaranteedHedgingNotAllowed
    );
    fixture!(
        OrderCancelReason,
        "STOP_LOSS_ON_FILL_TIME_IN_FORCE_INVALID",
        StopLossOnFillTimeInForceInvalid
    );
    fixture!(
        OrderCancelReason,
        "STOP_LOSS_ON_FILL_TRIGGER_CONDITION_INVALID",
        StopLossOnFillTriggerConditionInvalid
    );
    fixture!(
        OrderCancelReason,
        "GUARANTEED_STOP_LOSS_ON_FILL_GTD_TIMESTAMP_IN_PAST",
        GuaranteedStopLossOnFillGtdTimestampInPast
    );
    fixture!(
        OrderCancelReason,
        "GUARANTEED_STOP_LOSS_ON_FILL_LOSS",
        GuaranteedStopLossOnFillLoss
    );
    fixture!(
        OrderCancelReason,
        "GUARANTEED_STOP_LOSS_ON_FILL_PRICE_DISTANCE_MAXIMUM_EXCEEDED",
        GuaranteedStopLossOnFillPriceDistanceMaximumExceeded
    );
    fixture!(
        OrderCancelReason,
        "GUARANTEED_STOP_LOSS_ON_FILL_REQUIRED",
        GuaranteedStopLossOnFillRequired
    );
    fixture!(
        OrderCancelReason,
        "GUARANTEED_STOP_LOSS_ON_FILL_NOT_ALLOWED",
        GuaranteedStopLossOnFillNotAllowed
    );
    fixture!(
        OrderCancelReason,
        "GUARANTEED_STOP_LOSS_ON_FILL_MINIMUM_DISTANCE_NOT_MET",
        GuaranteedStopLossOnFillMinimumDistanceNotMet
    );
    fixture!(
        OrderCancelReason,
        "GUARANTEED_STOP_LOSS_ON_FILL_LEVEL_RESTRICTION_VOLUME_EXCEEDED",
        GuaranteedStopLossOnFillLevelRestrictionVolumeExceeded
    );
    fixture!(
        OrderCancelReason,
        "GUARANTEED_STOP_LOSS_ON_FILL_LEVEL_RESTRICTION_PRICE_RANGE_EXCEEDED",
        GuaranteedStopLossOnFillLevelRestrictionPriceRangeExceeded
    );
    fixture!(
        OrderCancelReason,
        "GUARANTEED_STOP_LOSS_ON_FILL_HEDGING_NOT_ALLOWED",
        GuaranteedStopLossOnFillHedgingNotAllowed
    );
    fixture!(
        OrderCancelReason,
        "GUARANTEED_STOP_LOSS_ON_FILL_TIME_IN_FORCE_INVALID",
        GuaranteedStopLossOnFillTimeInForceInvalid
    );
    fixture!(
        OrderCancelReason,
        "GUARANTEED_STOP_LOSS_ON_FILL_TRIGGER_CONDITION_INVALID",
        GuaranteedStopLossOnFillTriggerConditionInvalid
    );
    fixture!(
        OrderCancelReason,
        "TAKE_PROFIT_ON_FILL_PRICE_DISTANCE_MAXIMUM_EXCEEDED",
        TakeProfitOnFillPriceDistanceMaximumExceeded
    );
    fixture!(
        OrderCancelReason,
        "TRAILING_STOP_LOSS_ON_FILL_GTD_TIMESTAMP_IN_PAST",
        TrailingStopLossOnFillGtdTimestampInPast
    );
    fixture!(
        OrderCancelReason,
        "CLIENT_TRADE_ID_ALREADY_EXISTS",
        ClientTradeIdAlreadyExists
    );
    fixture!(
        OrderCancelReason,
        "POSITION_CLOSEOUT_FAILED",
        PositionCloseoutFailed
    );
    fixture!(
        OrderCancelReason,
        "OPEN_TRADES_ALLOWED_EXCEEDED",
        OpenTradesAllowedExceeded
    );
    fixture!(
        OrderCancelReason,
        "PENDING_ORDERS_ALLOWED_EXCEEDED",
        PendingOrdersAllowedExceeded
    );
    fixture!(
        OrderCancelReason,
        "TAKE_PROFIT_ON_FILL_CLIENT_ORDER_ID_ALREADY_EXISTS",
        TakeProfitOnFillClientOrderIdAlreadyExists
    );
    fixture!(
        OrderCancelReason,
        "STOP_LOSS_ON_FILL_CLIENT_ORDER_ID_ALREADY_EXISTS",
        StopLossOnFillClientOrderIdAlreadyExists
    );
    fixture!(
        OrderCancelReason,
        "GUARANTEED_STOP_LOSS_ON_FILL_CLIENT_ORDER_ID_ALREADY_EXISTS",
        GuaranteedStopLossOnFillClientOrderIdAlreadyExists
    );
    fixture!(
        OrderCancelReason,
        "TRAILING_STOP_LOSS_ON_FILL_CLIENT_ORDER_ID_ALREADY_EXISTS",
        TrailingStopLossOnFillClientOrderIdAlreadyExists
    );
    fixture!(
        OrderCancelReason,
        "POSITION_SIZE_EXCEEDED",
        PositionSizeExceeded
    );
    fixture!(
        OrderCancelReason,
        "HEDGING_GSLO_VIOLATION",
        HedgingGsloViolation
    );
    fixture!(
        OrderCancelReason,
        "ACCOUNT_POSITION_VALUE_LIMIT_EXCEEDED",
        AccountPositionValueLimitExceeded
    );
    fixture!(
        OrderCancelReason,
        "INSTRUMENT_BID_REDUCE_ONLY",
        InstrumentBidReduceOnly
    );
    fixture!(
        OrderCancelReason,
        "INSTRUMENT_ASK_REDUCE_ONLY",
        InstrumentAskReduceOnly
    );
    fixture!(
        OrderCancelReason,
        "INSTRUMENT_BID_HALTED",
        InstrumentBidHalted
    );
    fixture!(
        OrderCancelReason,
        "INSTRUMENT_ASK_HALTED",
        InstrumentAskHalted
    );
    fixture!(
        OrderCancelReason,
        "STOP_LOSS_ON_FILL_GUARANTEED_BID_HALTED",
        StopLossOnFillGuaranteedBidHalted
    );
    fixture!(
        OrderCancelReason,
        "STOP_LOSS_ON_FILL_GUARANTEED_ASK_HALTED",
        StopLossOnFillGuaranteedAskHalted
    );
    fixture!(
        OrderCancelReason,
        "GUARANTEED_STOP_LOSS_ON_FILL_BID_HALTED",
        GuaranteedStopLossOnFillBidHalted
    );
    fixture!(
        OrderCancelReason,
        "GUARANTEED_STOP_LOSS_ON_FILL_ASK_HALTED",
        GuaranteedStopLossOnFillAskHalted
    );
    fixture!(
        OrderCancelReason,
        "FIFO_VIOLATION_SAFEGUARD_VIOLATION",
        FifoViolationSafeguardViolation
    );
    fixture!(
        OrderCancelReason,
        "FIFO_VIOLATION_SAFEGUARD_PARTIAL_CLOSE_VIOLATION",
        FifoViolationSafeguardPartialCloseViolation
    );
    fixture!(
        OrderCancelReason,
        "ORDERS_ON_FILL_RMO_MUTUAL_EXCLUSIVITY_MUTUALLY_EXCLUSIVE_VIOLATION",
        OrdersOnFillRmoMutualExclusivityMutuallyExclusiveViolation
    );
    fixture!(
        MarketOrderMarginCloseoutReason,
        "MARGIN_CHECK_VIOLATION",
        MarginCheckViolation
    );
    fixture!(
        MarketOrderMarginCloseoutReason,
        "REGULATORY_MARGIN_CALL_VIOLATION",
        RegulatoryMarginCallViolation
    );
    fixture!(
        MarketOrderMarginCloseoutReason,
        "REGULATORY_MARGIN_CHECK_VIOLATION",
        RegulatoryMarginCheckViolation
    );
    fixture!(
        TransactionRejectReason,
        "INTERNAL_SERVER_ERROR",
        InternalServerError
    );
    fixture!(
        TransactionRejectReason,
        "INSTRUMENT_PRICE_UNKNOWN",
        InstrumentPriceUnknown
    );
    fixture!(
        TransactionRejectReason,
        "ACCOUNT_NOT_ACTIVE",
        AccountNotActive
    );
    fixture!(TransactionRejectReason, "ACCOUNT_LOCKED", AccountLocked);
    fixture!(
        TransactionRejectReason,
        "ACCOUNT_ORDER_CREATION_LOCKED",
        AccountOrderCreationLocked
    );
    fixture!(
        TransactionRejectReason,
        "ACCOUNT_CONFIGURATION_LOCKED",
        AccountConfigurationLocked
    );
    fixture!(
        TransactionRejectReason,
        "ACCOUNT_DEPOSIT_LOCKED",
        AccountDepositLocked
    );
    fixture!(
        TransactionRejectReason,
        "ACCOUNT_WITHDRAWAL_LOCKED",
        AccountWithdrawalLocked
    );
    fixture!(
        TransactionRejectReason,
        "ACCOUNT_ORDER_CANCEL_LOCKED",
        AccountOrderCancelLocked
    );
    fixture!(
        TransactionRejectReason,
        "INSTRUMENT_NOT_TRADEABLE",
        InstrumentNotTradeable
    );
    fixture!(
        TransactionRejectReason,
        "PENDING_ORDERS_ALLOWED_EXCEEDED",
        PendingOrdersAllowedExceeded
    );
    fixture!(
        TransactionRejectReason,
        "ORDER_ID_UNSPECIFIED",
        OrderIdUnspecified
    );
    fixture!(
        TransactionRejectReason,
        "ORDER_DOESNT_EXIST",
        OrderDoesntExist
    );
    fixture!(
        TransactionRejectReason,
        "ORDER_IDENTIFIER_INCONSISTENCY",
        OrderIdentifierInconsistency
    );
    fixture!(
        TransactionRejectReason,
        "TRADE_ID_UNSPECIFIED",
        TradeIdUnspecified
    );
    fixture!(
        TransactionRejectReason,
        "TRADE_DOESNT_EXIST",
        TradeDoesntExist
    );
    fixture!(
        TransactionRejectReason,
        "TRADE_IDENTIFIER_INCONSISTENCY",
        TradeIdentifierInconsistency
    );
    fixture!(
        TransactionRejectReason,
        "INSUFFICIENT_MARGIN",
        InsufficientMargin
    );
    fixture!(
        TransactionRejectReason,
        "INSTRUMENT_MISSING",
        InstrumentMissing
    );
    fixture!(
        TransactionRejectReason,
        "INSTRUMENT_UNKNOWN",
        InstrumentUnknown
    );
    fixture!(TransactionRejectReason, "UNITS_MISSING", UnitsMissing);
    fixture!(TransactionRejectReason, "UNITS_INVALID", UnitsInvalid);
    fixture!(
        TransactionRejectReason,
        "UNITS_PRECISION_EXCEEDED",
        UnitsPrecisionExceeded
    );
    fixture!(
        TransactionRejectReason,
        "UNITS_LIMIT_EXCEEDED",
        UnitsLimitExceeded
    );
    fixture!(
        TransactionRejectReason,
        "UNITS_MINIMUM_NOT_MET",
        UnitsMinimumNotMet
    );
    fixture!(TransactionRejectReason, "PRICE_MISSING", PriceMissing);
    fixture!(TransactionRejectReason, "PRICE_INVALID", PriceInvalid);
    fixture!(
        TransactionRejectReason,
        "PRICE_PRECISION_EXCEEDED",
        PricePrecisionExceeded
    );
    fixture!(
        TransactionRejectReason,
        "PRICE_DISTANCE_MISSING",
        PriceDistanceMissing
    );
    fixture!(
        TransactionRejectReason,
        "PRICE_DISTANCE_INVALID",
        PriceDistanceInvalid
    );
    fixture!(
        TransactionRejectReason,
        "PRICE_DISTANCE_PRECISION_EXCEEDED",
        PriceDistancePrecisionExceeded
    );
    fixture!(
        TransactionRejectReason,
        "PRICE_DISTANCE_MAXIMUM_EXCEEDED",
        PriceDistanceMaximumExceeded
    );
    fixture!(
        TransactionRejectReason,
        "PRICE_DISTANCE_MINIMUM_NOT_MET",
        PriceDistanceMinimumNotMet
    );
    fixture!(
        TransactionRejectReason,
        "TIME_IN_FORCE_MISSING",
        TimeInForceMissing
    );
    fixture!(
        TransactionRejectReason,
        "TIME_IN_FORCE_INVALID",
        TimeInForceInvalid
    );
    fixture!(
        TransactionRejectReason,
        "TIME_IN_FORCE_GTD_TIMESTAMP_MISSING",
        TimeInForceGtdTimestampMissing
    );
    fixture!(
        TransactionRejectReason,
        "TIME_IN_FORCE_GTD_TIMESTAMP_IN_PAST",
        TimeInForceGtdTimestampInPast
    );
    fixture!(
        TransactionRejectReason,
        "PRICE_BOUND_INVALID",
        PriceBoundInvalid
    );
    fixture!(
        TransactionRejectReason,
        "PRICE_BOUND_PRECISION_EXCEEDED",
        PriceBoundPrecisionExceeded
    );
    fixture!(
        TransactionRejectReason,
        "ORDERS_ON_FILL_DUPLICATE_CLIENT_ORDER_IDS",
        OrdersOnFillDuplicateClientOrderIds
    );
    fixture!(
        TransactionRejectReason,
        "TRADE_ON_FILL_CLIENT_EXTENSIONS_NOT_SUPPORTED",
        TradeOnFillClientExtensionsNotSupported
    );
    fixture!(
        TransactionRejectReason,
        "CLIENT_ORDER_ID_INVALID",
        ClientOrderIdInvalid
    );
    fixture!(
        TransactionRejectReason,
        "CLIENT_ORDER_ID_ALREADY_EXISTS",
        ClientOrderIdAlreadyExists
    );
    fixture!(
        TransactionRejectReason,
        "CLIENT_ORDER_TAG_INVALID",
        ClientOrderTagInvalid
    );
    fixture!(
        TransactionRejectReason,
        "CLIENT_ORDER_COMMENT_INVALID",
        ClientOrderCommentInvalid
    );
    fixture!(
        TransactionRejectReason,
        "CLIENT_TRADE_ID_INVALID",
        ClientTradeIdInvalid
    );
    fixture!(
        TransactionRejectReason,
        "CLIENT_TRADE_ID_ALREADY_EXISTS",
        ClientTradeIdAlreadyExists
    );
    fixture!(
        TransactionRejectReason,
        "CLIENT_TRADE_TAG_INVALID",
        ClientTradeTagInvalid
    );
    fixture!(
        TransactionRejectReason,
        "CLIENT_TRADE_COMMENT_INVALID",
        ClientTradeCommentInvalid
    );
    fixture!(
        TransactionRejectReason,
        "ORDER_FILL_POSITION_ACTION_MISSING",
        OrderFillPositionActionMissing
    );
    fixture!(
        TransactionRejectReason,
        "ORDER_FILL_POSITION_ACTION_INVALID",
        OrderFillPositionActionInvalid
    );
    fixture!(
        TransactionRejectReason,
        "TRIGGER_CONDITION_MISSING",
        TriggerConditionMissing
    );
    fixture!(
        TransactionRejectReason,
        "TRIGGER_CONDITION_INVALID",
        TriggerConditionInvalid
    );
    fixture!(
        TransactionRejectReason,
        "ORDER_PARTIAL_FILL_OPTION_MISSING",
        OrderPartialFillOptionMissing
    );
    fixture!(
        TransactionRejectReason,
        "ORDER_PARTIAL_FILL_OPTION_INVALID",
        OrderPartialFillOptionInvalid
    );
    fixture!(
        TransactionRejectReason,
        "INVALID_REISSUE_IMMEDIATE_PARTIAL_FILL",
        InvalidReissueImmediatePartialFill
    );
    fixture!(
        TransactionRejectReason,
        "ORDERS_ON_FILL_RMO_MUTUAL_EXCLUSIVITY_MUTUALLY_EXCLUSIVE_VIOLATION",
        OrdersOnFillRmoMutualExclusivityMutuallyExclusiveViolation
    );
    fixture!(
        TransactionRejectReason,
        "ORDERS_ON_FILL_RMO_MUTUAL_EXCLUSIVITY_GSLO_EXCLUDES_OTHERS_VIOLATION",
        OrdersOnFillRmoMutualExclusivityGsloExcludesOthersViolation
    );
    fixture!(
        TransactionRejectReason,
        "TAKE_PROFIT_ORDER_ALREADY_EXISTS",
        TakeProfitOrderAlreadyExists
    );
    fixture!(
        TransactionRejectReason,
        "TAKE_PROFIT_ORDER_WOULD_VIOLATE_FIFO_VIOLATION_SAFEGUARD",
        TakeProfitOrderWouldViolateFifoViolationSafeguard
    );
    fixture!(
        TransactionRejectReason,
        "TAKE_PROFIT_ON_FILL_PRICE_MISSING",
        TakeProfitOnFillPriceMissing
    );
    fixture!(
        TransactionRejectReason,
        "TAKE_PROFIT_ON_FILL_PRICE_INVALID",
        TakeProfitOnFillPriceInvalid
    );
    fixture!(
        TransactionRejectReason,
        "TAKE_PROFIT_ON_FILL_PRICE_PRECISION_EXCEEDED",
        TakeProfitOnFillPricePrecisionExceeded
    );
    fixture!(
        TransactionRejectReason,
        "TAKE_PROFIT_ON_FILL_TIME_IN_FORCE_MISSING",
        TakeProfitOnFillTimeInForceMissing
    );
    fixture!(
        TransactionRejectReason,
        "TAKE_PROFIT_ON_FILL_TIME_IN_FORCE_INVALID",
        TakeProfitOnFillTimeInForceInvalid
    );
    fixture!(
        TransactionRejectReason,
        "TAKE_PROFIT_ON_FILL_GTD_TIMESTAMP_MISSING",
        TakeProfitOnFillGtdTimestampMissing
    );
    fixture!(
        TransactionRejectReason,
        "TAKE_PROFIT_ON_FILL_GTD_TIMESTAMP_IN_PAST",
        TakeProfitOnFillGtdTimestampInPast
    );
    fixture!(
        TransactionRejectReason,
        "TAKE_PROFIT_ON_FILL_CLIENT_ORDER_ID_INVALID",
        TakeProfitOnFillClientOrderIdInvalid
    );
    fixture!(
        TransactionRejectReason,
        "TAKE_PROFIT_ON_FILL_CLIENT_ORDER_TAG_INVALID",
        TakeProfitOnFillClientOrderTagInvalid
    );
    fixture!(
        TransactionRejectReason,
        "TAKE_PROFIT_ON_FILL_CLIENT_ORDER_COMMENT_INVALID",
        TakeProfitOnFillClientOrderCommentInvalid
    );
    fixture!(
        TransactionRejectReason,
        "TAKE_PROFIT_ON_FILL_TRIGGER_CONDITION_MISSING",
        TakeProfitOnFillTriggerConditionMissing
    );
    fixture!(
        TransactionRejectReason,
        "TAKE_PROFIT_ON_FILL_TRIGGER_CONDITION_INVALID",
        TakeProfitOnFillTriggerConditionInvalid
    );
    fixture!(
        TransactionRejectReason,
        "STOP_LOSS_ORDER_ALREADY_EXISTS",
        StopLossOrderAlreadyExists
    );
    fixture!(
        TransactionRejectReason,
        "STOP_LOSS_ORDER_GUARANTEED_REQUIRED",
        StopLossOrderGuaranteedRequired
    );
    fixture!(
        TransactionRejectReason,
        "STOP_LOSS_ORDER_GUARANTEED_PRICE_WITHIN_SPREAD",
        StopLossOrderGuaranteedPriceWithinSpread
    );
    fixture!(
        TransactionRejectReason,
        "STOP_LOSS_ORDER_GUARANTEED_NOT_ALLOWED",
        StopLossOrderGuaranteedNotAllowed
    );
    fixture!(
        TransactionRejectReason,
        "STOP_LOSS_ORDER_GUARANTEED_HALTED_CREATE_VIOLATION",
        StopLossOrderGuaranteedHaltedCreateViolation
    );
    fixture!(
        TransactionRejectReason,
        "STOP_LOSS_ORDER_GUARANTEED_HALTED_TIGHTEN_VIOLATION",
        StopLossOrderGuaranteedHaltedTightenViolation
    );
    fixture!(
        TransactionRejectReason,
        "STOP_LOSS_ORDER_GUARANTEED_HEDGING_NOT_ALLOWED",
        StopLossOrderGuaranteedHedgingNotAllowed
    );
    fixture!(
        TransactionRejectReason,
        "STOP_LOSS_ORDER_GUARANTEED_MINIMUM_DISTANCE_NOT_MET",
        StopLossOrderGuaranteedMinimumDistanceNotMet
    );
    fixture!(
        TransactionRejectReason,
        "STOP_LOSS_ORDER_NOT_CANCELABLE",
        StopLossOrderNotCancelable
    );
    fixture!(
        TransactionRejectReason,
        "STOP_LOSS_ORDER_NOT_REPLACEABLE",
        StopLossOrderNotReplaceable
    );
    fixture!(
        TransactionRejectReason,
        "STOP_LOSS_ORDER_GUARANTEED_LEVEL_RESTRICTION_EXCEEDED",
        StopLossOrderGuaranteedLevelRestrictionExceeded
    );
    fixture!(
        TransactionRejectReason,
        "STOP_LOSS_ORDER_PRICE_AND_DISTANCE_BOTH_SPECIFIED",
        StopLossOrderPriceAndDistanceBothSpecified
    );
    fixture!(
        TransactionRejectReason,
        "STOP_LOSS_ORDER_PRICE_AND_DISTANCE_BOTH_MISSING",
        StopLossOrderPriceAndDistanceBothMissing
    );
    fixture!(
        TransactionRejectReason,
        "STOP_LOSS_ORDER_WOULD_VIOLATE_FIFO_VIOLATION_SAFEGUARD",
        StopLossOrderWouldViolateFifoViolationSafeguard
    );
    fixture!(
        TransactionRejectReason,
        "STOP_LOSS_ORDER_RMO_MUTUAL_EXCLUSIVITY_MUTUALLY_EXCLUSIVE_VIOLATION",
        StopLossOrderRmoMutualExclusivityMutuallyExclusiveViolation
    );
    fixture!(
        TransactionRejectReason,
        "STOP_LOSS_ORDER_RMO_MUTUAL_EXCLUSIVITY_GSLO_EXCLUDES_OTHERS_VIOLATION",
        StopLossOrderRmoMutualExclusivityGsloExcludesOthersViolation
    );
    fixture!(
        TransactionRejectReason,
        "STOP_LOSS_ON_FILL_REQUIRED_FOR_PENDING_ORDER",
        StopLossOnFillRequiredForPendingOrder
    );
    fixture!(
        TransactionRejectReason,
        "STOP_LOSS_ON_FILL_GUARANTEED_NOT_ALLOWED",
        StopLossOnFillGuaranteedNotAllowed
    );
    fixture!(
        TransactionRejectReason,
        "STOP_LOSS_ON_FILL_GUARANTEED_REQUIRED",
        StopLossOnFillGuaranteedRequired
    );
    fixture!(
        TransactionRejectReason,
        "STOP_LOSS_ON_FILL_PRICE_MISSING",
        StopLossOnFillPriceMissing
    );
    fixture!(
        TransactionRejectReason,
        "STOP_LOSS_ON_FILL_PRICE_INVALID",
        StopLossOnFillPriceInvalid
    );
    fixture!(
        TransactionRejectReason,
        "STOP_LOSS_ON_FILL_PRICE_PRECISION_EXCEEDED",
        StopLossOnFillPricePrecisionExceeded
    );
    fixture!(
        TransactionRejectReason,
        "STOP_LOSS_ON_FILL_GUARANTEED_MINIMUM_DISTANCE_NOT_MET",
        StopLossOnFillGuaranteedMinimumDistanceNotMet
    );
    fixture!(
        TransactionRejectReason,
        "STOP_LOSS_ON_FILL_GUARANTEED_LEVEL_RESTRICTION_EXCEEDED",
        StopLossOnFillGuaranteedLevelRestrictionExceeded
    );
    fixture!(
        TransactionRejectReason,
        "STOP_LOSS_ON_FILL_DISTANCE_INVALID",
        StopLossOnFillDistanceInvalid
    );
    fixture!(
        TransactionRejectReason,
        "STOP_LOSS_ON_FILL_PRICE_DISTANCE_MAXIMUM_EXCEEDED",
        StopLossOnFillPriceDistanceMaximumExceeded
    );
    fixture!(
        TransactionRejectReason,
        "STOP_LOSS_ON_FILL_DISTANCE_PRECISION_EXCEEDED",
        StopLossOnFillDistancePrecisionExceeded
    );
    fixture!(
        TransactionRejectReason,
        "STOP_LOSS_ON_FILL_PRICE_AND_DISTANCE_BOTH_SPECIFIED",
        StopLossOnFillPriceAndDistanceBothSpecified
    );
    fixture!(
        TransactionRejectReason,
        "STOP_LOSS_ON_FILL_PRICE_AND_DISTANCE_BOTH_MISSING",
        StopLossOnFillPriceAndDistanceBothMissing
    );
    fixture!(
        TransactionRejectReason,
        "STOP_LOSS_ON_FILL_TIME_IN_FORCE_MISSING",
        StopLossOnFillTimeInForceMissing
    );
    fixture!(
        TransactionRejectReason,
        "STOP_LOSS_ON_FILL_TIME_IN_FORCE_INVALID",
        StopLossOnFillTimeInForceInvalid
    );
    fixture!(
        TransactionRejectReason,
        "STOP_LOSS_ON_FILL_GTD_TIMESTAMP_MISSING",
        StopLossOnFillGtdTimestampMissing
    );
    fixture!(
        TransactionRejectReason,
        "STOP_LOSS_ON_FILL_GTD_TIMESTAMP_IN_PAST",
        StopLossOnFillGtdTimestampInPast
    );
    fixture!(
        TransactionRejectReason,
        "STOP_LOSS_ON_FILL_CLIENT_ORDER_ID_INVALID",
        StopLossOnFillClientOrderIdInvalid
    );
    fixture!(
        TransactionRejectReason,
        "STOP_LOSS_ON_FILL_CLIENT_ORDER_TAG_INVALID",
        StopLossOnFillClientOrderTagInvalid
    );
    fixture!(
        TransactionRejectReason,
        "STOP_LOSS_ON_FILL_CLIENT_ORDER_COMMENT_INVALID",
        StopLossOnFillClientOrderCommentInvalid
    );
    fixture!(
        TransactionRejectReason,
        "STOP_LOSS_ON_FILL_TRIGGER_CONDITION_MISSING",
        StopLossOnFillTriggerConditionMissing
    );
    fixture!(
        TransactionRejectReason,
        "STOP_LOSS_ON_FILL_TRIGGER_CONDITION_INVALID",
        StopLossOnFillTriggerConditionInvalid
    );
    fixture!(
        TransactionRejectReason,
        "GUARANTEED_STOP_LOSS_ORDER_ALREADY_EXISTS",
        GuaranteedStopLossOrderAlreadyExists
    );
    fixture!(
        TransactionRejectReason,
        "GUARANTEED_STOP_LOSS_ORDER_REQUIRED",
        GuaranteedStopLossOrderRequired
    );
    fixture!(
        TransactionRejectReason,
        "GUARANTEED_STOP_LOSS_ORDER_PRICE_WITHIN_SPREAD",
        GuaranteedStopLossOrderPriceWithinSpread
    );
    fixture!(
        TransactionRejectReason,
        "GUARANTEED_STOP_LOSS_ORDER_NOT_ALLOWED",
        GuaranteedStopLossOrderNotAllowed
    );
    fixture!(
        TransactionRejectReason,
        "GUARANTEED_STOP_LOSS_ORDER_HALTED_CREATE_VIOLATION",
        GuaranteedStopLossOrderHaltedCreateViolation
    );
    fixture!(
        TransactionRejectReason,
        "GUARANTEED_STOP_LOSS_ORDER_CREATE_VIOLATION",
        GuaranteedStopLossOrderCreateViolation
    );
    fixture!(
        TransactionRejectReason,
        "GUARANTEED_STOP_LOSS_ORDER_HALTED_TIGHTEN_VIOLATION",
        GuaranteedStopLossOrderHaltedTightenViolation
    );
    fixture!(
        TransactionRejectReason,
        "GUARANTEED_STOP_LOSS_ORDER_TIGHTEN_VIOLATION",
        GuaranteedStopLossOrderTightenViolation
    );
    fixture!(
        TransactionRejectReason,
        "GUARANTEED_STOP_LOSS_ORDER_HEDGING_NOT_ALLOWED",
        GuaranteedStopLossOrderHedgingNotAllowed
    );
    fixture!(
        TransactionRejectReason,
        "GUARANTEED_STOP_LOSS_ORDER_MINIMUM_DISTANCE_NOT_MET",
        GuaranteedStopLossOrderMinimumDistanceNotMet
    );
    fixture!(
        TransactionRejectReason,
        "GUARANTEED_STOP_LOSS_ORDER_NOT_CANCELABLE",
        GuaranteedStopLossOrderNotCancelable
    );
    fixture!(
        TransactionRejectReason,
        "GUARANTEED_STOP_LOSS_ORDER_HALTED_NOT_CANCELABLE",
        GuaranteedStopLossOrderHaltedNotCancelable
    );
    fixture!(
        TransactionRejectReason,
        "GUARANTEED_STOP_LOSS_ORDER_NOT_REPLACEABLE",
        GuaranteedStopLossOrderNotReplaceable
    );
    fixture!(
        TransactionRejectReason,
        "GUARANTEED_STOP_LOSS_ORDER_HALTED_NOT_REPLACEABLE",
        GuaranteedStopLossOrderHaltedNotReplaceable
    );
    fixture!(
        TransactionRejectReason,
        "GUARANTEED_STOP_LOSS_ORDER_LEVEL_RESTRICTION_VOLUME_EXCEEDED",
        GuaranteedStopLossOrderLevelRestrictionVolumeExceeded
    );
    fixture!(
        TransactionRejectReason,
        "GUARANTEED_STOP_LOSS_ORDER_LEVEL_RESTRICTION_PRICE_RANGE_EXCEEDED",
        GuaranteedStopLossOrderLevelRestrictionPriceRangeExceeded
    );
    fixture!(
        TransactionRejectReason,
        "GUARANTEED_STOP_LOSS_ORDER_PRICE_AND_DISTANCE_BOTH_SPECIFIED",
        GuaranteedStopLossOrderPriceAndDistanceBothSpecified
    );
    fixture!(
        TransactionRejectReason,
        "GUARANTEED_STOP_LOSS_ORDER_PRICE_AND_DISTANCE_BOTH_MISSING",
        GuaranteedStopLossOrderPriceAndDistanceBothMissing
    );
    fixture!(
        TransactionRejectReason,
        "GUARANTEED_STOP_LOSS_ORDER_WOULD_VIOLATE_FIFO_VIOLATION_SAFEGUARD",
        GuaranteedStopLossOrderWouldViolateFifoViolationSafeguard
    );
    fixture!(
        TransactionRejectReason,
        "GUARANTEED_STOP_LOSS_ORDER_RMO_MUTUAL_EXCLUSIVITY_MUTUALLY_EXCLUSIVE_VIOLATION",
        GuaranteedStopLossOrderRmoMutualExclusivityMutuallyExclusiveViolation
    );
    fixture!(
        TransactionRejectReason,
        "GUARANTEED_STOP_LOSS_ORDER_RMO_MUTUAL_EXCLUSIVITY_GSLO_EXCLUDES_OTHERS_VIOLATION",
        GuaranteedStopLossOrderRmoMutualExclusivityGsloExcludesOthersViolation
    );
    fixture!(
        TransactionRejectReason,
        "GUARANTEED_STOP_LOSS_ON_FILL_REQUIRED_FOR_PENDING_ORDER",
        GuaranteedStopLossOnFillRequiredForPendingOrder
    );
    fixture!(
        TransactionRejectReason,
        "GUARANTEED_STOP_LOSS_ON_FILL_NOT_ALLOWED",
        GuaranteedStopLossOnFillNotAllowed
    );
    fixture!(
        TransactionRejectReason,
        "GUARANTEED_STOP_LOSS_ON_FILL_REQUIRED",
        GuaranteedStopLossOnFillRequired
    );
    fixture!(
        TransactionRejectReason,
        "GUARANTEED_STOP_LOSS_ON_FILL_PRICE_MISSING",
        GuaranteedStopLossOnFillPriceMissing
    );
    fixture!(
        TransactionRejectReason,
        "GUARANTEED_STOP_LOSS_ON_FILL_PRICE_INVALID",
        GuaranteedStopLossOnFillPriceInvalid
    );
    fixture!(
        TransactionRejectReason,
        "GUARANTEED_STOP_LOSS_ON_FILL_PRICE_PRECISION_EXCEEDED",
        GuaranteedStopLossOnFillPricePrecisionExceeded
    );
    fixture!(
        TransactionRejectReason,
        "GUARANTEED_STOP_LOSS_ON_FILL_MINIMUM_DISTANCE_NOT_MET",
        GuaranteedStopLossOnFillMinimumDistanceNotMet
    );
    fixture!(
        TransactionRejectReason,
        "GUARANTEED_STOP_LOSS_ON_FILL_LEVEL_RESTRICTION_VOLUME_EXCEEDED",
        GuaranteedStopLossOnFillLevelRestrictionVolumeExceeded
    );
    fixture!(
        TransactionRejectReason,
        "GUARANTEED_STOP_LOSS_ON_FILL_LEVEL_RESTRICTION_PRICE_RANGE_EXCEEDED",
        GuaranteedStopLossOnFillLevelRestrictionPriceRangeExceeded
    );
    fixture!(
        TransactionRejectReason,
        "GUARANTEED_STOP_LOSS_ON_FILL_DISTANCE_INVALID",
        GuaranteedStopLossOnFillDistanceInvalid
    );
    fixture!(
        TransactionRejectReason,
        "GUARANTEED_STOP_LOSS_ON_FILL_PRICE_DISTANCE_MAXIMUM_EXCEEDED",
        GuaranteedStopLossOnFillPriceDistanceMaximumExceeded
    );
    fixture!(
        TransactionRejectReason,
        "GUARANTEED_STOP_LOSS_ON_FILL_DISTANCE_PRECISION_EXCEEDED",
        GuaranteedStopLossOnFillDistancePrecisionExceeded
    );
    fixture!(
        TransactionRejectReason,
        "GUARANTEED_STOP_LOSS_ON_FILL_PRICE_AND_DISTANCE_BOTH_SPECIFIED",
        GuaranteedStopLossOnFillPriceAndDistanceBothSpecified
    );
    fixture!(
        TransactionRejectReason,
        "GUARANTEED_STOP_LOSS_ON_FILL_PRICE_AND_DISTANCE_BOTH_MISSING",
        GuaranteedStopLossOnFillPriceAndDistanceBothMissing
    );
    fixture!(
        TransactionRejectReason,
        "GUARANTEED_STOP_LOSS_ON_FILL_TIME_IN_FORCE_MISSING",
        GuaranteedStopLossOnFillTimeInForceMissing
    );
    fixture!(
        TransactionRejectReason,
        "GUARANTEED_STOP_LOSS_ON_FILL_TIME_IN_FORCE_INVALID",
        GuaranteedStopLossOnFillTimeInForceInvalid
    );
    fixture!(
        TransactionRejectReason,
        "GUARANTEED_STOP_LOSS_ON_FILL_GTD_TIMESTAMP_MISSING",
        GuaranteedStopLossOnFillGtdTimestampMissing
    );
    fixture!(
        TransactionRejectReason,
        "GUARANTEED_STOP_LOSS_ON_FILL_GTD_TIMESTAMP_IN_PAST",
        GuaranteedStopLossOnFillGtdTimestampInPast
    );
    fixture!(
        TransactionRejectReason,
        "GUARANTEED_STOP_LOSS_ON_FILL_CLIENT_ORDER_ID_INVALID",
        GuaranteedStopLossOnFillClientOrderIdInvalid
    );
    fixture!(
        TransactionRejectReason,
        "GUARANTEED_STOP_LOSS_ON_FILL_CLIENT_ORDER_TAG_INVALID",
        GuaranteedStopLossOnFillClientOrderTagInvalid
    );
    fixture!(
        TransactionRejectReason,
        "GUARANTEED_STOP_LOSS_ON_FILL_CLIENT_ORDER_COMMENT_INVALID",
        GuaranteedStopLossOnFillClientOrderCommentInvalid
    );
    fixture!(
        TransactionRejectReason,
        "GUARANTEED_STOP_LOSS_ON_FILL_TRIGGER_CONDITION_MISSING",
        GuaranteedStopLossOnFillTriggerConditionMissing
    );
    fixture!(
        TransactionRejectReason,
        "GUARANTEED_STOP_LOSS_ON_FILL_TRIGGER_CONDITION_INVALID",
        GuaranteedStopLossOnFillTriggerConditionInvalid
    );
    fixture!(
        TransactionRejectReason,
        "TRAILING_STOP_LOSS_ORDER_ALREADY_EXISTS",
        TrailingStopLossOrderAlreadyExists
    );
    fixture!(
        TransactionRejectReason,
        "TRAILING_STOP_LOSS_ORDER_WOULD_VIOLATE_FIFO_VIOLATION_SAFEGUARD",
        TrailingStopLossOrderWouldViolateFifoViolationSafeguard
    );
    fixture!(
        TransactionRejectReason,
        "TRAILING_STOP_LOSS_ORDER_RMO_MUTUAL_EXCLUSIVITY_MUTUALLY_EXCLUSIVE_VIOLATION",
        TrailingStopLossOrderRmoMutualExclusivityMutuallyExclusiveViolation
    );
    fixture!(
        TransactionRejectReason,
        "TRAILING_STOP_LOSS_ORDER_RMO_MUTUAL_EXCLUSIVITY_GSLO_EXCLUDES_OTHERS_VIOLATION",
        TrailingStopLossOrderRmoMutualExclusivityGsloExcludesOthersViolation
    );
    fixture!(
        TransactionRejectReason,
        "TRAILING_STOP_LOSS_ON_FILL_PRICE_DISTANCE_MISSING",
        TrailingStopLossOnFillPriceDistanceMissing
    );
    fixture!(
        TransactionRejectReason,
        "TRAILING_STOP_LOSS_ON_FILL_PRICE_DISTANCE_INVALID",
        TrailingStopLossOnFillPriceDistanceInvalid
    );
    fixture!(
        TransactionRejectReason,
        "TRAILING_STOP_LOSS_ON_FILL_PRICE_DISTANCE_PRECISION_EXCEEDED",
        TrailingStopLossOnFillPriceDistancePrecisionExceeded
    );
    fixture!(
        TransactionRejectReason,
        "TRAILING_STOP_LOSS_ON_FILL_PRICE_DISTANCE_MAXIMUM_EXCEEDED",
        TrailingStopLossOnFillPriceDistanceMaximumExceeded
    );
    fixture!(
        TransactionRejectReason,
        "TRAILING_STOP_LOSS_ON_FILL_PRICE_DISTANCE_MINIMUM_NOT_MET",
        TrailingStopLossOnFillPriceDistanceMinimumNotMet
    );
    fixture!(
        TransactionRejectReason,
        "TRAILING_STOP_LOSS_ON_FILL_TIME_IN_FORCE_MISSING",
        TrailingStopLossOnFillTimeInForceMissing
    );
    fixture!(
        TransactionRejectReason,
        "TRAILING_STOP_LOSS_ON_FILL_TIME_IN_FORCE_INVALID",
        TrailingStopLossOnFillTimeInForceInvalid
    );
    fixture!(
        TransactionRejectReason,
        "TRAILING_STOP_LOSS_ON_FILL_GTD_TIMESTAMP_MISSING",
        TrailingStopLossOnFillGtdTimestampMissing
    );
    fixture!(
        TransactionRejectReason,
        "TRAILING_STOP_LOSS_ON_FILL_GTD_TIMESTAMP_IN_PAST",
        TrailingStopLossOnFillGtdTimestampInPast
    );
    fixture!(
        TransactionRejectReason,
        "TRAILING_STOP_LOSS_ON_FILL_CLIENT_ORDER_ID_INVALID",
        TrailingStopLossOnFillClientOrderIdInvalid
    );
    fixture!(
        TransactionRejectReason,
        "TRAILING_STOP_LOSS_ON_FILL_CLIENT_ORDER_TAG_INVALID",
        TrailingStopLossOnFillClientOrderTagInvalid
    );
    fixture!(
        TransactionRejectReason,
        "TRAILING_STOP_LOSS_ON_FILL_CLIENT_ORDER_COMMENT_INVALID",
        TrailingStopLossOnFillClientOrderCommentInvalid
    );
    fixture!(
        TransactionRejectReason,
        "TRAILING_STOP_LOSS_ORDERS_NOT_SUPPORTED",
        TrailingStopLossOrdersNotSupported
    );
    fixture!(
        TransactionRejectReason,
        "TRAILING_STOP_LOSS_ON_FILL_TRIGGER_CONDITION_MISSING",
        TrailingStopLossOnFillTriggerConditionMissing
    );
    fixture!(
        TransactionRejectReason,
        "TRAILING_STOP_LOSS_ON_FILL_TRIGGER_CONDITION_INVALID",
        TrailingStopLossOnFillTriggerConditionInvalid
    );
    fixture!(
        TransactionRejectReason,
        "CLOSE_TRADE_TYPE_MISSING",
        CloseTradeTypeMissing
    );
    fixture!(
        TransactionRejectReason,
        "CLOSE_TRADE_PARTIAL_UNITS_MISSING",
        CloseTradePartialUnitsMissing
    );
    fixture!(
        TransactionRejectReason,
        "CLOSE_TRADE_UNITS_EXCEED_TRADE_SIZE",
        CloseTradeUnitsExceedTradeSize
    );
    fixture!(
        TransactionRejectReason,
        "CLOSEOUT_POSITION_DOESNT_EXIST",
        CloseoutPositionDoesntExist
    );
    fixture!(
        TransactionRejectReason,
        "CLOSEOUT_POSITION_INCOMPLETE_SPECIFICATION",
        CloseoutPositionIncompleteSpecification
    );
    fixture!(
        TransactionRejectReason,
        "CLOSEOUT_POSITION_UNITS_EXCEED_POSITION_SIZE",
        CloseoutPositionUnitsExceedPositionSize
    );
    fixture!(
        TransactionRejectReason,
        "CLOSEOUT_POSITION_REJECT",
        CloseoutPositionReject
    );
    fixture!(
        TransactionRejectReason,
        "CLOSEOUT_POSITION_PARTIAL_UNITS_MISSING",
        CloseoutPositionPartialUnitsMissing
    );
    fixture!(
        TransactionRejectReason,
        "MARKUP_GROUP_ID_INVALID",
        MarkupGroupIdInvalid
    );
    fixture!(
        TransactionRejectReason,
        "POSITION_AGGREGATION_MODE_INVALID",
        PositionAggregationModeInvalid
    );
    fixture!(
        TransactionRejectReason,
        "ADMIN_CONFIGURE_DATA_MISSING",
        AdminConfigureDataMissing
    );
    fixture!(
        TransactionRejectReason,
        "MARGIN_RATE_INVALID",
        MarginRateInvalid
    );
    fixture!(
        TransactionRejectReason,
        "MARGIN_RATE_WOULD_TRIGGER_CLOSEOUT",
        MarginRateWouldTriggerCloseout
    );
    fixture!(TransactionRejectReason, "ALIAS_INVALID", AliasInvalid);
    fixture!(
        TransactionRejectReason,
        "CLIENT_CONFIGURE_DATA_MISSING",
        ClientConfigureDataMissing
    );
    fixture!(
        TransactionRejectReason,
        "MARGIN_RATE_WOULD_TRIGGER_MARGIN_CALL",
        MarginRateWouldTriggerMarginCall
    );
    fixture!(TransactionRejectReason, "AMOUNT_INVALID", AmountInvalid);
    fixture!(
        TransactionRejectReason,
        "INSUFFICIENT_FUNDS",
        InsufficientFunds
    );
    fixture!(TransactionRejectReason, "AMOUNT_MISSING", AmountMissing);
    fixture!(
        TransactionRejectReason,
        "FUNDING_REASON_MISSING",
        FundingReasonMissing
    );
    fixture!(
        TransactionRejectReason,
        "OCA_ORDER_IDS_STOP_LOSS_NOT_ALLOWED",
        OcaOrderIdsStopLossNotAllowed
    );
    fixture!(
        TransactionRejectReason,
        "CLIENT_EXTENSIONS_DATA_MISSING",
        ClientExtensionsDataMissing
    );
    fixture!(
        TransactionRejectReason,
        "REPLACING_ORDER_INVALID",
        ReplacingOrderInvalid
    );
    fixture!(
        TransactionRejectReason,
        "REPLACING_TRADE_ID_INVALID",
        ReplacingTradeIdInvalid
    );
    fixture!(
        TransactionRejectReason,
        "ORDER_CANCEL_WOULD_TRIGGER_CLOSEOUT",
        OrderCancelWouldTriggerCloseout
    );
    fixture!(TransactionFilter, "ORDER", Order);
    fixture!(TransactionFilter, "FUNDING", Funding);
    fixture!(TransactionFilter, "ADMIN", Admin);
    fixture!(TransactionFilter, "CREATE", Create);
    fixture!(TransactionFilter, "CLOSE", Close);
    fixture!(TransactionFilter, "REOPEN", Reopen);
    fixture!(TransactionFilter, "CLIENT_CONFIGURE", ClientConfigure);
    fixture!(
        TransactionFilter,
        "CLIENT_CONFIGURE_REJECT",
        ClientConfigureReject
    );
    fixture!(TransactionFilter, "TRANSFER_FUNDS", TransferFunds);
    fixture!(
        TransactionFilter,
        "TRANSFER_FUNDS_REJECT",
        TransferFundsReject
    );
    fixture!(TransactionFilter, "MARKET_ORDER", MarketOrder);
    fixture!(TransactionFilter, "MARKET_ORDER_REJECT", MarketOrderReject);
    fixture!(TransactionFilter, "LIMIT_ORDER", LimitOrder);
    fixture!(TransactionFilter, "LIMIT_ORDER_REJECT", LimitOrderReject);
    fixture!(TransactionFilter, "STOP_ORDER", StopOrder);
    fixture!(TransactionFilter, "STOP_ORDER_REJECT", StopOrderReject);
    fixture!(
        TransactionFilter,
        "MARKET_IF_TOUCHED_ORDER",
        MarketIfTouchedOrder
    );
    fixture!(
        TransactionFilter,
        "MARKET_IF_TOUCHED_ORDER_REJECT",
        MarketIfTouchedOrderReject
    );
    fixture!(TransactionFilter, "TAKE_PROFIT_ORDER", TakeProfitOrder);
    fixture!(
        TransactionFilter,
        "TAKE_PROFIT_ORDER_REJECT",
        TakeProfitOrderReject
    );
    fixture!(TransactionFilter, "STOP_LOSS_ORDER", StopLossOrder);
    fixture!(
        TransactionFilter,
        "STOP_LOSS_ORDER_REJECT",
        StopLossOrderReject
    );
    fixture!(
        TransactionFilter,
        "GUARANTEED_STOP_LOSS_ORDER",
        GuaranteedStopLossOrder
    );
    fixture!(
        TransactionFilter,
        "GUARANTEED_STOP_LOSS_ORDER_REJECT",
        GuaranteedStopLossOrderReject
    );
    fixture!(
        TransactionFilter,
        "TRAILING_STOP_LOSS_ORDER",
        TrailingStopLossOrder
    );
    fixture!(
        TransactionFilter,
        "TRAILING_STOP_LOSS_ORDER_REJECT",
        TrailingStopLossOrderReject
    );
    fixture!(
        TransactionFilter,
        "ONE_CANCELS_ALL_ORDER",
        OneCancelsAllOrder
    );
    fixture!(
        TransactionFilter,
        "ONE_CANCELS_ALL_ORDER_REJECT",
        OneCancelsAllOrderReject
    );
    fixture!(
        TransactionFilter,
        "ONE_CANCELS_ALL_ORDER_TRIGGERED",
        OneCancelsAllOrderTriggered
    );
    fixture!(TransactionFilter, "ORDER_FILL", OrderFill);
    fixture!(TransactionFilter, "ORDER_CANCEL", OrderCancel);
    fixture!(TransactionFilter, "ORDER_CANCEL_REJECT", OrderCancelReject);
    fixture!(
        TransactionFilter,
        "ORDER_CLIENT_EXTENSIONS_MODIFY",
        OrderClientExtensionsModify
    );
    fixture!(
        TransactionFilter,
        "ORDER_CLIENT_EXTENSIONS_MODIFY_REJECT",
        OrderClientExtensionsModifyReject
    );
    fixture!(
        TransactionFilter,
        "TRADE_CLIENT_EXTENSIONS_MODIFY",
        TradeClientExtensionsModify
    );
    fixture!(
        TransactionFilter,
        "TRADE_CLIENT_EXTENSIONS_MODIFY_REJECT",
        TradeClientExtensionsModifyReject
    );
    fixture!(TransactionFilter, "MARGIN_CALL_ENTER", MarginCallEnter);
    fixture!(TransactionFilter, "MARGIN_CALL_EXTEND", MarginCallExtend);
    fixture!(TransactionFilter, "MARGIN_CALL_EXIT", MarginCallExit);
    fixture!(
        TransactionFilter,
        "DELAYED_TRADE_CLOSURE",
        DelayedTradeClosure
    );
    fixture!(TransactionFilter, "DAILY_FINANCING", DailyFinancing);
    fixture!(TransactionFilter, "RESET_RESETTABLE_PL", ResetResettablePl);
    fixture!(PriceStatus, "tradeable", Tradeable);
    fixture!(PriceStatus, "non-tradeable", NonTradeable);
    fixture!(PriceStatus, "invalid", Invalid);
    fixture!(InstrumentType, "CURRENCY", Currency);
    fixture!(InstrumentType, "CFD", Cfd);
    fixture!(InstrumentType, "METAL", Metal);
    fixture!(DayOfWeek, "SUNDAY", Sunday);
    fixture!(DayOfWeek, "MONDAY", Monday);
    fixture!(DayOfWeek, "TUESDAY", Tuesday);
    fixture!(DayOfWeek, "WEDNESDAY", Wednesday);
    fixture!(DayOfWeek, "THURSDAY", Thursday);
    fixture!(DayOfWeek, "FRIDAY", Friday);
    fixture!(DayOfWeek, "SATURDAY", Saturday);
    fixture!(AcceptDatetimeFormat, "UNIX", Unix);
    fixture!(AcceptDatetimeFormat, "RFC3339", Rfc3339);
    fixture!(
        GuaranteedStopLossOrderModeForInstrument,
        "DISABLED",
        Disabled
    );
    fixture!(GuaranteedStopLossOrderModeForInstrument, "ALLOWED", Allowed);
    fixture!(
        GuaranteedStopLossOrderModeForInstrument,
        "REQUIRED",
        Required
    );
    fixture!(Direction, "LONG", Long);
    fixture!(Direction, "SHORT", Short);
    assert_scalar_inventory(&tested);
}

#[should_panic(expected = "must decode to its documented variant")]
#[test]
fn an_undocumented_enum_value_fails_the_round_trip() {
    round_trip::<models::OrderType>("GARBAGE", &models::OrderType::Market);
}

fn assert_scalar_inventory(
    tested: &std::collections::BTreeMap<&str, std::collections::BTreeSet<&str>>,
) {
    let spec: serde_json::Value =
        serde_json::from_str(include_str!("../spec/official/v20-openapi.json"))
            .expect("pinned spec");
    let drift: serde_json::Value =
        serde_json::from_str(include_str!("fixtures/scalar-enum-website-drift.json"))
            .expect("reviewed website additions and removals");
    let expected_types: std::collections::BTreeSet<_> = [
        "AcceptDatetimeFormat",
        "AccountFinancingMode",
        "CancellableOrderType",
        "CandlestickGranularity",
        "DayOfWeek",
        "Direction",
        "FixedPriceOrderReason",
        "FundingReason",
        "GuaranteedStopLossOrderMode",
        "GuaranteedStopLossOrderModeForInstrument",
        "GuaranteedStopLossOrderMutability",
        "GuaranteedStopLossOrderReason",
        "InstrumentType",
        "LimitOrderReason",
        "MarketIfTouchedOrderReason",
        "MarketOrderMarginCloseoutReason",
        "MarketOrderReason",
        "OrderCancelReason",
        "OrderFillReason",
        "OrderPositionFill",
        "OrderState",
        "OrderStateFilter",
        "OrderTriggerCondition",
        "OrderType",
        "PositionAggregationMode",
        "PriceStatus",
        "StopLossOrderReason",
        "StopOrderReason",
        "TakeProfitOrderReason",
        "TimeInForce",
        "TradePL",
        "TradeState",
        "TradeStateFilter",
        "TrailingStopLossOrderReason",
        "TransactionFilter",
        "TransactionRejectReason",
        "TransactionType",
        "WeeklyAlignment",
    ]
    .into_iter()
    .collect();
    assert_eq!(
        tested
            .keys()
            .copied()
            .collect::<std::collections::BTreeSet<_>>(),
        expected_types,
        "a whole enum fixture group must not disappear"
    );
    for (name, actual) in tested {
        let mut expected: std::collections::BTreeSet<&str> = spec["definitions"][name]["enum"]
            .as_array()
            .into_iter()
            .flatten()
            .map(|value| value.as_str().expect("pinned enum string"))
            .collect();
        if let Some(delta) = drift.get(name) {
            for value in delta["remove"].as_array().expect("removals") {
                assert!(expected.remove(value.as_str().expect("removed enum string")));
            }
            for value in delta["add"].as_array().expect("additions") {
                assert!(expected.insert(value.as_str().expect("added enum string")));
            }
        }
        assert!(
            !expected.is_empty(),
            "{name} requires independent expected values"
        );
        assert_eq!(
            *actual, expected,
            "{name} fixture completeness against pinned spec and website drift"
        );
    }
}
