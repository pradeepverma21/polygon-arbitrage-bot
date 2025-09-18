use anyhow::Result;
use dotenv::dotenv;

mod config;
mod modules;
mod database;  

use config::Config;
use modules::price_fetcher::PriceFetcher;
use modules::arbitrage_detector::ArbitrageDetector;
use modules::profit_calculator::ProfitCalculator;
use database::Database; 

#[tokio::main]
async fn main() -> Result<()> {
    // Load environment variables
    dotenv().ok();
    
    println!("🚀 Polygon Arbitrage Opportunity Detector Bot");
    println!("============================================");
    
    // Initialize configuration
    let mut config = Config::new()?;
    config.min_profit_threshold = 0.001; // 0.1%
    
    println!("✅ Configuration loaded:");
    println!("  - RPC URL: {}", config.polygon_rpc_url);
    println!("  - Min Profit Threshold: {:.2}%", config.min_profit_threshold * 100.0);
    println!("  - Token Pairs: {}", config.token_pairs.len());
    println!("  - DEX Contracts: {}", config.dex_contracts.len());
    
    // Initialize database connection  // <-- ADD THIS SECTION
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let database = Database::new(&database_url).await?;
    println!("✅ Database connected");
    
    // Initialize components
    let price_fetcher = PriceFetcher::new(&config.polygon_rpc_url, config.dex_contracts.clone())?;
    let arbitrage_detector = ArbitrageDetector::new(config.min_profit_threshold);
    let profit_calculator = ProfitCalculator::new();
    println!("✅ All components initialized");
    
    // Rest of the code stays the same for now...
    println!("\n📈 Starting price monitoring...");
    
    for cycle in 1..=3 {
        println!("\n🔄 Monitoring Cycle #{}", cycle);
        println!("{}", "-".repeat(50));
        
        let prices = price_fetcher.fetch_all_prices(&config.token_pairs).await?;
        let opportunities = arbitrage_detector.detect_opportunities(&prices)?;
        
        if !opportunities.is_empty() {
            arbitrage_detector.print_opportunities(&opportunities);
            
            // Store opportunities in database  // <-- ADD THIS SECTION
            for opportunity in &opportunities {
                let analysis = profit_calculator.calculate_detailed_profit(opportunity)?;
                let id = database.store_opportunity(opportunity, Some(&analysis)).await?;
                println!("💾 Stored opportunity #{} in database", id);
            }
            
            if let Some(best_opportunity) = opportunities.first() {
                println!("\n🎯 Detailed Analysis of Best Opportunity:");
                profit_calculator.print_detailed_analysis(best_opportunity)?;
            }
        } else {
            println!("❌ No opportunities found this cycle");
        }
        
        if cycle < 3 {
            println!("\n⏳ Waiting 5 seconds before next check...");
            tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
        }
    }
    
    // Show database stats  // <-- ADD THIS SECTION
    println!("\n📊 Database Statistics:");
    let stats = database.get_stats().await?;
    println!("  - Total opportunities stored: {}", stats.total_opportunities);
    println!("  - Average daily profit: {:.3}%", stats.avg_daily_profit * 100.0);
    if let Some((pair, profit)) = stats.best_daily_pair {
        println!("  - Best daily pair: {} ({:.3}%)", pair, profit * 100.0);
    }
    
    println!("\n🏁 Monitoring complete!");
    println!("📝 All opportunities stored in database!");
    
    Ok(())
}