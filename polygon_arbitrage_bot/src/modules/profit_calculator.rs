use anyhow::Result;
   use crate::modules::arbitrage_detector::ArbitrageOpportunity;
   use serde::{Deserialize, Serialize};

   #[derive(Debug, Clone, Serialize, Deserialize)]
   pub struct ProfitAnalysis {
       pub gross_profit: f64,
       pub gas_costs: f64,
       pub slippage_cost: f64,
       pub net_profit: f64,
       pub roi_percentage: f64,
       pub execution_time_estimate: u64, // seconds
   }

   pub struct ProfitCalculator {
       gas_price_gwei: f64,
       slippage_percentage: f64,
       swap_gas_limit: u64,
   }

   impl ProfitCalculator {
       pub fn new() -> Self {
           Self {
               gas_price_gwei: 30.0, // Average Polygon gas price
               slippage_percentage: 0.005, // 0.5% slippage
               swap_gas_limit: 200_000, // Estimated gas for DEX swaps
           }
       }

       pub fn calculate_detailed_profit(&self, opportunity: &ArbitrageOpportunity) -> Result<ProfitAnalysis> {
           // Calculate gross profit
           let gross_profit = opportunity.profit_usd;

           // Calculate gas costs (in USD)
           // Polygon gas costs are very low compared to Ethereum
           let gas_cost_matic = (self.gas_price_gwei * self.swap_gas_limit as f64 * 2.0) / 1_000_000_000.0; // 2 swaps
           let matic_price = 0.8; // Approximate MATIC price in USD
           let gas_costs = gas_cost_matic * matic_price;

           // Calculate slippage costs
           let slippage_cost = opportunity.trade_size * self.slippage_percentage;

           // Calculate net profit
           let net_profit = gross_profit - gas_costs - slippage_cost;

           // Calculate ROI
           let roi_percentage = (net_profit / opportunity.trade_size) * 100.0;

           Ok(ProfitAnalysis {
               gross_profit,
               gas_costs,
               slippage_cost,
               net_profit,
               roi_percentage,
               execution_time_estimate: 30, // Estimated 30 seconds for execution
           })
       }

       pub fn print_detailed_analysis(&self, opportunity: &ArbitrageOpportunity) -> Result<()> {
           let analysis = self.calculate_detailed_profit(opportunity)?;

           println!("💰 Detailed Profit Analysis for {}", opportunity.token_pair);
           println!("{}", "=".repeat(60));
           println!("📊 Trade Details:");
           println!("   Buy from:     {} at ${:.4}", opportunity.buy_dex, opportunity.buy_price);
           println!("   Sell on:      {} at ${:.4}", opportunity.sell_dex, opportunity.sell_price);
           println!("   Trade size:   ${:.0}", opportunity.trade_size);
           println!();
           println!("💵 Profit Breakdown:");
           println!("   Gross profit:   ${:.2}", analysis.gross_profit);
           println!("   Gas costs:      ${:.2}", analysis.gas_costs);
           println!("   Slippage:       ${:.2}", analysis.slippage_cost);
           println!("   NET PROFIT:     ${:.2}", analysis.net_profit);
           println!("   ROI:            {:.3}%", analysis.roi_percentage);
           println!();
           println!("⏱️  Estimated execution time: {} seconds", analysis.execution_time_estimate);
           
           if analysis.net_profit > 0.0 {
               println!("✅ PROFITABLE OPPORTUNITY!");
           } else {
               println!("❌ Not profitable after costs");
           }

           Ok(())
       }
   }

   impl Default for ProfitCalculator {
       fn default() -> Self {
           Self::new()
       }
   }