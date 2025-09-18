use anyhow::Result;
   use sqlx::{MySqlPool, Row};
   use chrono::{DateTime, Utc};
   use crate::modules::arbitrage_detector::ArbitrageOpportunity;
   use crate::modules::profit_calculator::ProfitAnalysis;

   pub struct Database {
       pool: MySqlPool,
   }

   #[derive(Debug)]
   pub struct StoredOpportunity {
       pub id: i32,
       pub token_pair: String,
       pub buy_dex: String,
       pub sell_dex: String,
       pub buy_price: f64,
       pub sell_price: f64,
       pub profit_percentage: f64,
       pub profit_usd: f64,
       pub trade_size: f64,
       pub net_profit: Option<f64>,
       pub gas_costs: Option<f64>,
       pub created_at: DateTime<Utc>,
   }

   impl Database {
       pub async fn new(database_url: &str) -> Result<Self> {
           let pool = MySqlPool::connect(database_url).await?;
           
           let db = Database { pool };
           db.create_tables().await?;
           
           Ok(db)
       }

       async fn create_tables(&self) -> Result<()> {
           sqlx::query(
               r#"
               CREATE TABLE IF NOT EXISTS arbitrage_opportunities (
                   id INT AUTO_INCREMENT PRIMARY KEY,
                   token_pair VARCHAR(50) NOT NULL,
                   buy_dex VARCHAR(50) NOT NULL,
                   sell_dex VARCHAR(50) NOT NULL,
                   buy_price DECIMAL(20, 8) NOT NULL,
                   sell_price DECIMAL(20, 8) NOT NULL,
                   profit_percentage DECIMAL(10, 6) NOT NULL,
                   profit_usd DECIMAL(20, 2) NOT NULL,
                   trade_size DECIMAL(20, 2) NOT NULL,
                   net_profit DECIMAL(20, 2) NULL,
                   gas_costs DECIMAL(20, 8) NULL,
                   created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                   INDEX idx_token_pair (token_pair),
                   INDEX idx_created_at (created_at),
                   INDEX idx_profit_percentage (profit_percentage)
               )
               "#
           )
           .execute(&self.pool)
           .await?;

           println!("✅ Database tables created/verified");
           Ok(())
       }

       pub async fn store_opportunity(
           &self,
           opportunity: &ArbitrageOpportunity,
           analysis: Option<&ProfitAnalysis>,
       ) -> Result<i32> {
           let result = sqlx::query(
               r#"
               INSERT INTO arbitrage_opportunities 
               (token_pair, buy_dex, sell_dex, buy_price, sell_price, profit_percentage, profit_usd, trade_size, net_profit, gas_costs)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
               "#
           )
           .bind(&opportunity.token_pair)
           .bind(&opportunity.buy_dex)
           .bind(&opportunity.sell_dex)
           .bind(opportunity.buy_price)
           .bind(opportunity.sell_price)
           .bind(opportunity.profit_percentage)
           .bind(opportunity.profit_usd)
           .bind(opportunity.trade_size)
           .bind(analysis.map(|a| a.net_profit))
           .bind(analysis.map(|a| a.gas_costs))
           .execute(&self.pool)
           .await?;

           Ok(result.last_insert_id() as i32)
       }

       pub async fn get_recent_opportunities(&self, limit: i32) -> Result<Vec<StoredOpportunity>> {
           let rows = sqlx::query(
               r#"
               SELECT id, token_pair, buy_dex, sell_dex, buy_price, sell_price, 
                      profit_percentage, profit_usd, trade_size, net_profit, gas_costs, created_at
               FROM arbitrage_opportunities 
               ORDER BY created_at DESC 
               LIMIT ?
               "#
           )
           .bind(limit)
           .fetch_all(&self.pool)
           .await?;

           let mut opportunities = Vec::new();
           for row in rows {
               opportunities.push(StoredOpportunity {
                   id: row.get("id"),
                   token_pair: row.get("token_pair"),
                   buy_dex: row.get("buy_dex"),
                   sell_dex: row.get("sell_dex"),
                   buy_price: row.get("buy_price"),
                   sell_price: row.get("sell_price"),
                   profit_percentage: row.get("profit_percentage"),
                   profit_usd: row.get("profit_usd"),
                   trade_size: row.get("trade_size"),
                   net_profit: row.get("net_profit"),
                   gas_costs: row.get("gas_costs"),
                   created_at: row.get("created_at"),
               });
           }

           Ok(opportunities)
       }

       pub async fn get_stats(&self) -> Result<DatabaseStats> {
           let total_opportunities: (i64,) = sqlx::query_as(
               "SELECT COUNT(*) FROM arbitrage_opportunities"
           )
           .fetch_one(&self.pool)
           .await?;

           let avg_profit: (Option<f64>,) = sqlx::query_as(
               "SELECT AVG(profit_percentage) FROM arbitrage_opportunities WHERE created_at >= DATE_SUB(NOW(), INTERVAL 1 DAY)"
           )
           .fetch_one(&self.pool)
           .await?;

           let best_opportunity: Option<(String, f64)> = sqlx::query_as(
               "SELECT token_pair, MAX(profit_percentage) FROM arbitrage_opportunities WHERE created_at >= DATE_SUB(NOW(), INTERVAL 1 DAY) GROUP BY token_pair ORDER BY MAX(profit_percentage) DESC LIMIT 1"
           )
           .fetch_optional(&self.pool)
           .await?;

           Ok(DatabaseStats {
               total_opportunities: total_opportunities.0,
               avg_daily_profit: avg_profit.0.unwrap_or(0.0),
               best_daily_pair: best_opportunity.map(|(pair, profit)| (pair, profit)),
           })
       }
   }

   #[derive(Debug)]
   pub struct DatabaseStats {
       pub total_opportunities: i64,
       pub avg_daily_profit: f64,
       pub best_daily_pair: Option<(String, f64)>,
   }