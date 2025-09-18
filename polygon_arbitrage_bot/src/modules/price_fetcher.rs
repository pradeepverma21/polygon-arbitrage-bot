use anyhow::{Result, anyhow};
   use ethers::prelude::*;
   use serde::{Deserialize, Serialize};
   use std::collections::HashMap;
   use std::sync::Arc;

   #[derive(Debug, Clone, Serialize, Deserialize)]
   pub struct PriceData {
       pub dex_name: String,
       pub token_pair: String,
       pub price: f64,
       pub timestamp: u64,
       pub liquidity: f64,
   }

   pub struct PriceFetcher {
       client: Arc<Provider<Http>>,
       dex_contracts: HashMap<String, String>,
   }

   impl PriceFetcher {
       pub fn new(rpc_url: &str, dex_contracts: HashMap<String, String>) -> Result<Self> {
           let provider = Provider::<Http>::try_from(rpc_url)
               .map_err(|e| anyhow!("Failed to connect to RPC: {}", e))?;
           
           Ok(Self {
               client: Arc::new(provider),
               dex_contracts,
           })
       }

       pub async fn fetch_all_prices(&self, token_pairs: &[crate::config::TokenPair]) -> Result<Vec<PriceData>> {
           let mut all_prices = Vec::new();
           
           for pair in token_pairs {
               println!("  Fetching prices for {}/{}", pair.base, pair.quote);
               
               // Fetch from each DEX
               for (dex_name, _contract_address) in &self.dex_contracts {
                   match self.fetch_dex_price(dex_name, pair).await {
                       Ok(price_data) => {
                           println!("    {}: ${:.4}", dex_name, price_data.price);
                           all_prices.push(price_data);
                       }
                       Err(e) => {
                           println!("    {}: Error - {}", dex_name, e);
                       }
                   }
               }
           }
           
           Ok(all_prices)
       }

       async fn fetch_dex_price(&self, dex_name: &str, pair: &crate::config::TokenPair) -> Result<PriceData> {
           // For now, we'll simulate price fetching
           // In a real implementation, you'd query the actual DEX contracts
           let simulated_price = self.simulate_price(dex_name, pair);
           
           Ok(PriceData {
               dex_name: dex_name.to_string(),
               token_pair: format!("{}/{}", pair.base, pair.quote),
               price: simulated_price,
               timestamp: std::time::SystemTime::now()
                   .duration_since(std::time::UNIX_EPOCH)?
                   .as_secs(),
               liquidity: 100000.0, // Simulated liquidity
           })
       }

       fn simulate_price(&self, dex_name: &str, pair: &crate::config::TokenPair) -> f64 {
           // Simulate different prices on different DEXes
           let base_price = match pair.base.as_str() {
               "WETH" => 2500.0,
               "WBTC" => 45000.0,
               _ => 1.0,
           };
           
           // Add small variations per DEX to simulate arbitrage opportunities
           let variation = match dex_name {
               "uniswap_v3" => 0.0,
               "sushiswap" => 0.002, // 0.2% difference
               _ => 0.001,
           };
           
           base_price * (1.0 + variation)
       }
   }