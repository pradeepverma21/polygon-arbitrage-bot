use anyhow::Result;
   use dotenv::dotenv;

   mod config;
   mod modules;

   use config::Config;  

   #[tokio::main]
   async fn main() -> Result<()> {
       // Load environment variables
       dotenv().ok();
       
       println!("Polygon Arbitrage Opportunity Detector Bot");
       println!("============================================");
       
       // TODO: Initialize configuration
       let config = Config::new()?;
       println!("    Configuration loaded:");
       println!("  - RPC URL: {}", config.polygon_rpc_url);
       println!("  - Min Profit Threshold: {:.2}%", config.min_profit_threshold * 100.0);
       println!("  - Token Pairs: {}", config.token_pairs.len());
       println!("  - DEX Contracts: {}", config.dex_contracts.len());
       // TODO: Start price fetching
       // TODO: Monitor for arbitrage opportunities

       // Initialize price fetcher
       let price_fetcher = PriceFetcher::new(&config.polygon_rpc_url, config.dex_contracts.clone())?;
       println!(" Price fetcher initialized");
       
       // Fetch prices
       println!("\n Fetching current prices...");
       let prices = price_fetcher.fetch_all_prices(&config.token_pairs).await?;
       
       println!("\n Price Summary:");
       for price in &prices {
           println!("  {} on {}: ${:.4}", price.token_pair, price.dex_name, price.price);
       }
       
       println!("\n Ready for arbitrage detection!");
       
     //  println!("Bot initialized successfully!");
       
       Ok(())
   }