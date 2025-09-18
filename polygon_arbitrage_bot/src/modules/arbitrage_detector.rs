use anyhow::Result;
   use crate::modules::price_fetcher::PriceData;
   use serde::{Deserialize, Serialize};

   #[derive(Debug, Clone, Serialize, Deserialize)]
   pub struct ArbitrageOpportunity {
       pub token_pair: String,
       pub buy_dex: String,
       pub sell_dex: String,
       pub buy_price: f64,
       pub sell_price: f64,
       pub profit_percentage: f64,
       pub profit_usd: f64,
       pub trade_size: f64,
       pub timestamp: u64,
   }

   pub struct ArbitrageDetector {
       min_profit_threshold: f64,
       default_trade_size: f64,
   }

   impl ArbitrageDetector {
       pub fn new(min_profit_threshold: f64) -> Self {
           Self {
               min_profit_threshold,
               default_trade_size: 1000.0, // $1000 default trade size
           }
       }

       pub fn detect_opportunities(&self, prices: &[PriceData]) -> Result<Vec<ArbitrageOpportunity>> {
           let mut opportunities = Vec::new();
           
           // Group prices by token pair
           let mut price_by_pair = std::collections::HashMap::new();
           for price in prices {
               price_by_pair
                   .entry(&price.token_pair)
                   .or_insert_with(Vec::new)
                   .push(price);
           }

           // Check each token pair for arbitrage opportunities
           for (token_pair, pair_prices) in price_by_pair {
               if let Some(opportunity) = self.find_best_arbitrage(token_pair, &pair_prices)? {
                   if opportunity.profit_percentage >= self.min_profit_threshold {
                       opportunities.push(opportunity);
                   }
               }
           }

           // Sort by profit percentage (highest first)
           opportunities.sort_by(|a, b| b.profit_percentage.partial_cmp(&a.profit_percentage).unwrap());

           Ok(opportunities)
       }

       fn find_best_arbitrage(&self, token_pair: &str, prices: &[&PriceData]) -> Result<Option<ArbitrageOpportunity>> {
           if prices.len() < 2 {
               return Ok(None);
           }

           let mut min_price = f64::MAX;
           let mut max_price = f64::MIN;
           let mut buy_dex = String::new();
           let mut sell_dex = String::new();

           // Find the lowest and highest prices
           for price_data in prices {
               if price_data.price < min_price {
                   min_price = price_data.price;
                   buy_dex = price_data.dex_name.clone();
               }
               if price_data.price > max_price {
                   max_price = price_data.price;
                   sell_dex = price_data.dex_name.clone();
               }
           }

           // Calculate profit
           let profit_percentage = (max_price - min_price) / min_price;
           let profit_usd = self.default_trade_size * profit_percentage;

           Ok(Some(ArbitrageOpportunity {
               token_pair: token_pair.to_string(),
               buy_dex,
               sell_dex,
               buy_price: min_price,
               sell_price: max_price,
               profit_percentage,
               profit_usd,
               trade_size: self.default_trade_size,
               timestamp: std::time::SystemTime::now()
                   .duration_since(std::time::UNIX_EPOCH)?
                   .as_secs(),
           }))
       }

       pub fn print_opportunities(&self, opportunities: &[ArbitrageOpportunity]) {
           if opportunities.is_empty() {
               println!("❌ No arbitrage opportunities found above {:.2}% threshold", self.min_profit_threshold * 100.0);
               return;
           }

           println!("💰 Found {} Arbitrage Opportunities:", opportunities.len());
           println!("{}", "=".repeat(80));

           for (i, opp) in opportunities.iter().enumerate() {
               println!("🎯 Opportunity #{}", i + 1);
               println!("   Token Pair: {}", opp.token_pair);
               println!("   Buy from:   {} at ${:.4}", opp.buy_dex, opp.buy_price);
               println!("   Sell on:    {} at ${:.4}", opp.sell_dex, opp.sell_price);
               println!("   Profit:     {:.2}% (${:.2} on ${:.0} trade)", 
                       opp.profit_percentage * 100.0, opp.profit_usd, opp.trade_size);
               println!("{}", "-".repeat(50));
           }
       }
   }