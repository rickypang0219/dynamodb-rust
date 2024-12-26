use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "e", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum UserDataUpdate {
    ListenKeyExpired(ListenKeyExpiredEvent),
    AccountUpdate(BalancePositionUpdateEvent),
    MarginCallUpdate(MarginCallUpdateEvent),
    OrderTradeUpdate(OrderTradeUpdateEvent),
    TradeLite(TradeLiteUpdateEvent),
    AccountConfigUpdate(AccountConfigUpdateEvent),
    StrategyUpdate(StrategyUpdateEvent),
    GridUpdate(GridUpdateEvent),
    ConditionalOrderTriggerReject(ConditionalOrderTriggerRejectEvent),
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct ListenKeyExpiredEvent {
    pub E: u64,
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct BalancePositionUpdateEvent {
    #[allow(non_snake_case)]
    pub E: u64,
    #[allow(non_snake_case)]
    pub T: u64,
    pub a: BalancePositionData,
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct MarginCallUpdateEvent {
    pub E: u64,
    pub cw: String,
    pub p: MarginPositionData,
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct OrderTradeUpdateEvent {
    pub E: u64,
    pub T: u64,
    pub o: OrderUpdateData,
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct TradeLiteUpdateEvent {
    pub E: u64,
    #[allow(non_snake_case)]
    pub T: u64,
    pub s: String, // Symbol
    pub q: String, //Original Quantity
    pub p: String, // Original Price
    pub m: bool,   // Is Maker?
    pub c: String, // Client Order ID
    #[allow(non_snake_case)]
    pub S: String, // Side
    #[allow(non_snake_case)]
    pub L: String, // Last Filled Price
    pub l: String, // Order Last Filled Quantity
    pub t: u64,    // Trade ID
    pub i: u64,    // Order ID
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct AccountConfigUpdateEvent {
    pub E: u64, // Event Time
    pub T: u64, // Transaction Time
    #[serde(flatten)] // Use flattening for different configurations
    pub account_config: AccountConfig, // User's Account Configuration
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct StrategyUpdateEvent {
    pub T: u64,              // Transaction Time
    pub E: u64,              // Event Time
    pub su: StrategyDetails, // Strategy Update Details
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct GridUpdateEvent {
    pub T: u64,                // Transaction Time
    pub E: u64,                // Event Time
    pub gu: GridUpdateDetails, // Grid Update Details
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct ConditionalOrderTriggerRejectEvent {
    pub E: u64,                 // Event Time
    pub T: u64,                 // Message Send Time
    pub or: OrderRejectDetails, // Order Details
}

// Balance Position Updates
#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct BalancePositionData {
    pub m: String,        // Event reason type
    pub B: Vec<Balance>,  // Balances
    pub P: Vec<Position>, // Positions
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct Balance {
    pub a: String,  // Asset
    pub wb: String, // Wallet Balance
    pub cw: String, // Cross Wallet Balance
    pub bc: String, // Balance Change except PnL and Commission
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Position {
    pub s: String,   // Symbol
    pub pa: String,  // Position Amount
    pub ep: String,  // Entry Price
    pub bep: String, // Breakeven Price
    pub cr: String,  // (Pre-fee) Accumulated Realized
    pub up: String,  // Unrealized PnL
    pub mt: String,  // Margin Type
    pub iw: String,  // Isolated Wallet (if isolated position)
    pub ps: String,  // Position Side
}

// Margin Position Data
#[derive(Serialize, Deserialize, Debug)]
pub struct MarginPositionData {
    pub s: String, // Symbol
    #[allow(non_snake_case)]
    pub ps: String, // Position Side
    pub pa: String, // Position Amount
    pub mt: String, // Margin Type
    pub iw: String, // Isolated Wallet (if isolated position)
    pub mp: String, // Mark Price
    pub up: String, // Unrealized PnL
    pub mm: String, // Maintenance Margin Required
}

// Order Status Update
#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct OrderUpdateData {
    pub s: String,          // Symbol
    pub c: String,          // Client Order Id
    pub S: String,          // Side
    pub o: String,          // Order Type
    pub f: String,          // Time in Force
    pub q: String,          // Original Quantity
    pub p: String,          // Original Price
    pub ap: String,         // Average Price
    pub sp: String,         // Stop Price
    pub x: String,          // Execution Type
    pub X: String,          // Order Status
    pub i: u64,             // Order Id
    pub l: String,          // Order Last Filled Quantity
    pub z: String,          // Order Filled Accumulated Quantity
    pub L: String,          // Last Filled Price
    pub N: String,          // Commission Asset
    pub n: String,          // Commission
    pub T: u64,             // Order Trade Time
    pub t: u64,             // Trade Id
    pub b: String,          // Bids Notional
    pub a: String,          // Ask Notional
    pub m: bool,            // Is this trade the maker side?
    pub R: bool,            // Is this reduce only
    pub wt: String,         // Stop Price Working Type
    pub ot: String,         // Original Order Type
    pub ps: String,         // Position Side
    pub cp: bool,           // If Close-All
    pub AP: Option<String>, // Activation Price
    pub cr: Option<String>, // Callback Rate
    pub pP: bool,           // If price protection is turned on
    pub si: u64,            // ignore
    pub ss: u64,            // ignore
    pub rp: String,         // Realized Profit of the trade
    pub V: String,          // STP mode
    pub pm: String,         // Price match mode
    pub gtd: u64,           // TIF GTD order auto cancel time
}

// Account Config Update
#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
#[serde(untagged)] // Allows for multiple formats in the same struct
pub enum AccountConfig {
    LeverageConfig(AccountLeverage),       // For leverage configuration
    MultiAssetsConfig(AccountMultiAssets), // For multi-assets configuration
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AccountLeverage {
    pub ac: AccountAssetConfig, // Account Asset Configuration
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AccountAssetConfig {
    pub s: String, // Symbol
    pub l: u32,    // Leverage
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AccountMultiAssets {
    pub ai: MultiAssetsInfo, // Multi-Assets Information
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MultiAssetsInfo {
    pub j: bool, // Multi-Assets Mode
}

// Strategy Detail
#[derive(Serialize, Deserialize, Debug)]
pub struct StrategyDetails {
    pub si: u64,    // Strategy ID
    pub st: String, // Strategy Type
    pub ss: String, // Strategy Status
    pub s: String,  // Symbol
    pub ut: u64,    // Update Time
    pub c: u32,     // opCode
}

// Grid Update Detail
#[derive(Serialize, Deserialize, Debug)]
pub struct GridUpdateDetails {
    pub si: u64,    // Strategy ID
    pub st: String, // Strategy Type
    pub ss: String, // Strategy Status
    pub s: String,  // Symbol
    pub r: String,  // Realized PNL
    pub up: String, // Unmatched Average Price
    pub uq: String, // Unmatched Qty
    pub uf: String, // Unmatched Fee
    pub mp: String, // Matched PNL
    pub ut: u64,    // Update Time
}

// Order Trigger Rejected
#[derive(Serialize, Deserialize, Debug)]
pub struct OrderRejectDetails {
    pub s: String, // Symbol
    pub i: u64,    // Order ID
    pub r: String, // Reject Reason
}
