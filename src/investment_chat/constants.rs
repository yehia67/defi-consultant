use std::collections::HashSet;
use std::sync::OnceLock;

/// Get the set of common crypto project names
pub fn crypto_projects() -> &'static HashSet<&'static str> {
    static PROJECTS: OnceLock<HashSet<&'static str>> = OnceLock::new();
    
    PROJECTS.get_or_init(|| {
        let mut set = HashSet::new();
        set.insert("bitcoin");
        set.insert("ethereum");
        set.insert("solana");
        set.insert("cardano");
        set.insert("polkadot");
        set.insert("avalanche");
        set.insert("chainlink");
        set.insert("polygon");
        set.insert("uniswap");
        set.insert("aave");
        set.insert("compound");
        set.insert("maker");
        set.insert("sushi");
        set.insert("curve");
        set.insert("yearn");
        set.insert("arbitrum");
        set.insert("optimism");
        set.insert("base");
        set.insert("bnb");
        set.insert("xrp");
        set.insert("dogecoin");
        set.insert("shiba inu");
        set.insert("litecoin");
        set.insert("cosmos");
        set.insert("near");
        set.insert("fantom");
        set.insert("tron");
        set.insert("filecoin");
        set.insert("the graph");
        set.insert("1inch");
        set.insert("pancakeswap");
        set.insert("gmx");
        set.insert("gains");
        set.insert("pendle");
        set.insert("aerodrome");
        set.insert("velodrome");
        set.insert("balancer");
        set
    })
}

/// Get the set of investment-related keywords
pub fn investment_keywords() -> &'static HashSet<&'static str> {
    static KEYWORDS: OnceLock<HashSet<&'static str>> = OnceLock::new();
    
    KEYWORDS.get_or_init(|| {
        let mut set = HashSet::new();
        set.insert("invest");
        set.insert("risk");
        set.insert("return");
        set.insert("strategy");
        set.insert("portfolio");
        set.insert("diversify");
        set.insert("allocation");
        set.insert("market");
        set.insert("bull");
        set.insert("bear");
        set.insert("trend");
        set.insert("analysis");
        set.insert("technical");
        set.insert("fundamental");
        set.insert("defi");
        set.insert("yield");
        set.insert("farming");
        set.insert("staking");
        set.insert("liquidity");
        set.insert("pool");
        set.insert("swap");
        set.insert("trade");
        set.insert("long");
        set.insert("short");
        set.insert("leverage");
        set.insert("margin");
        set.insert("volatility");
        set.insert("market cap");
        set.insert("volume");
        set.insert("tokenomics");
        set.insert("supply");
        set.insert("inflation");
        set.insert("team");
        set.insert("roadmap");
        set.insert("whitepaper");
        set
    })
}
