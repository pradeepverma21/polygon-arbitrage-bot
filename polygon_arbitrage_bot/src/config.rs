use serde::{Deserialize, Serialize};
   use std::collections::HashMap;
   use anyhow::Result;

   #[derive(Debug, Clone, Serialize, Deserialize)]
   pub struct Config {
       pub polygon_rpc_url: String,
       pub min_profit_threshold: f64,
       pub token_pairs: Vec<TokenPair>,
       pub dex_contracts: HashMap<String, String>,
   }

   #[derive(Debug, Clone, Serialize, Deserialize)]
   pub struct TokenPair {
       pub base: String,
       pub quote: String,
       pub base_address: String,
       pub quote_address: String,
   }

   impl Config {
       pub fn new() -> Result<Self> {
           let config = Config {
               polygon_rpc_url: std::env::var("POLYGON_RPC_URL")
                   .unwrap_or_else(|_| "https://polygon-rpc.com".to_string()),
               min_profit_threshold: std::env::var("MIN_PROFIT_THRESHOLD")
                   .unwrap_or_else(|_| "0.01".to_string())
                   .parse()
                   .unwrap_or(0.01),
               token_pairs: Self::default_token_pairs(),
               dex_contracts: Self::default_dex_contracts(),
           };
           
           Ok(config)
       }

       fn default_token_pairs() -> Vec<TokenPair> {
           vec![
               TokenPair {
                   base: "WETH".to_string(),
                   quote: "USDC".to_string(),
                   base_address: "0x7ceB23fD6bC0adD59E62ac25578270cFf1b9f619".to_string(),
                   quote_address: "0x2791Bca1f2de4661ED88A30C99A7a9449Aa84174".to_string(),
               },
               TokenPair {
                   base: "WBTC".to_string(),
                   quote: "USDC".to_string(),
                   base_address: "0x1BFD67037B42Cf73acF2047067bd4F2C47D9BfD6".to_string(),
                   quote_address: "0x2791Bca1f2de4661ED88A30C99A7a9449Aa84174".to_string(),
               },
           ]
       }

       fn default_dex_contracts() -> HashMap<String, String> {
           let mut dex_map = HashMap::new();
           
           // Uniswap V3 Router on Polygon
           dex_map.insert(
               "uniswap_v3".to_string(),
               "0xE592427A0AEce92De3Edee1F18E0157C05861564".to_string()
           );
           
           // SushiSwap Router on Polygon  
           dex_map.insert(
               "sushiswap".to_string(),
               "0x1b02dA8Cb0d097eB8D57A175b88c7D8b47997506".to_string()
           );
           
           dex_map
       }
   }