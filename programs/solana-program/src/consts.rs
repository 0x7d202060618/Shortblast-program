pub const INITIAL_PRICE_DIVIDER: u64 = 800_000;       // lamports per one token (without decimal)
pub const INITIAL_LAMPORTS_FOR_POOL: u64 = 60_000_000_000;   // 60SOL
pub const TOKEN_SELL_LIMIT_PERCENT: u64 = 8000;     //  80%
//Formula: Price = INITIAL_PROPORTION * (TOKEN_SUPPLY ^ INITIAL_EXPONENT)
pub const INITIAL_EXPONENT: f64 = 0.000000003606;      //  
pub const INITIAL_PROPORTION: f64 = 0.6015;      //  

