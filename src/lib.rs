use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::io::{self, BufRead, Write};

#[derive(Debug, Deserialize, Serialize)]
pub struct MarketOrder {
    pub duration: i32,
    pub is_buy_order: bool,
    pub issued: String,
    pub location_id: i64,
    pub min_volume: i32,
    pub order_id: i64,
    pub price: f64,
    pub range: String,
    pub system_id: i32,
    pub type_id: i32,
    pub volume_remain: i32,
    pub volume_total: i32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MarketType {
    pub type_id: i32,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug)]
pub struct TraderGraderApplication {
    name: String,
    version: String,
    http_client: Client,
}

impl TraderGraderApplication {
    pub fn new() -> Self {
        Self {
            name: "TraderGrader".to_string(),
            version: "0.1.0".to_string(),
            http_client: Client::builder()
                .user_agent("TraderGrader/0.1.0 (https://github.com/your-username/tradergrader)")
                .build()
                .expect("Failed to create HTTP client"),
        }
    }

    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("🚀 TraderGrader MCP Server starting...");
        println!("Ready to receive MCP JSON-RPC messages on stdin");

        // Simple MCP server loop - reads JSON-RPC from stdin, responds on stdout
        let stdin = io::stdin();
        let mut stdout = io::stdout();

        for line in stdin.lock().lines() {
            let line = line?;
            if line.trim().is_empty() {
                continue;
            }

            // Parse incoming JSON-RPC message
            match serde_json::from_str::<Value>(&line) {
                Ok(message) => {
                    let response = self.handle_message(message).await;
                    let response_str = serde_json::to_string(&response)?;
                    writeln!(stdout, "{response_str}")?;
                    stdout.flush()?;
                }
                Err(e) => {
                    eprintln!("Failed to parse message: {e}");
                }
            }
        }

        Ok(())
    }

    pub async fn fetch_market_orders(
        &self,
        region_id: i32,
        type_id: Option<i32>,
    ) -> Result<Vec<MarketOrder>, Box<dyn std::error::Error>> {
        let mut url = format!(
            "https://esi.evetech.net/latest/markets/{region_id}/orders/"
        );

        if let Some(tid) = type_id {
            url = format!("{url}?type_id={tid}");
        }

        let response = self.http_client.get(&url).send().await?;

        if !response.status().is_success() {
            return Err(
                format!("ESI API request failed with status: {}", response.status()).into(),
            );
        }

        let orders: Vec<MarketOrder> = response.json().await?;
        Ok(orders)
    }

    pub async fn get_market_summary(
        &self,
        region_id: i32,
        type_id: i32,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let orders = self.fetch_market_orders(region_id, Some(type_id)).await?;

        let buy_orders: Vec<&MarketOrder> = orders.iter().filter(|o| o.is_buy_order).collect();
        let sell_orders: Vec<&MarketOrder> = orders.iter().filter(|o| !o.is_buy_order).collect();

        let highest_buy = buy_orders
            .iter()
            .max_by(|a, b| a.price.partial_cmp(&b.price).unwrap());
        let lowest_sell = sell_orders
            .iter()
            .min_by(|a, b| a.price.partial_cmp(&b.price).unwrap());

        let summary = format!(
            "Market Summary for Type {} in Region {}:\n\
            Total Orders: {}\n\
            Buy Orders: {}\n\
            Sell Orders: {}\n\
            Highest Buy: {:.2} ISK\n\
            Lowest Sell: {:.2} ISK\n\
            Spread: {:.2} ISK",
            type_id,
            region_id,
            orders.len(),
            buy_orders.len(),
            sell_orders.len(),
            highest_buy.map(|o| o.price).unwrap_or(0.0),
            lowest_sell.map(|o| o.price).unwrap_or(0.0),
            if let (Some(sell), Some(buy)) = (lowest_sell, highest_buy) {
                sell.price - buy.price
            } else {
                0.0
            }
        );

        Ok(summary)
    }

    async fn handle_message(&self, message: Value) -> Value {
        // Basic MCP message handling
        if let Some(method) = message.get("method").and_then(|m| m.as_str()) {
            match method {
                "initialize" => self.handle_initialize(),
                "tools/list" => self.handle_tools_list(),
                "tools/call" => self.handle_tool_call(&message).await,
                _ => json!({
                    "jsonrpc": "2.0",
                    "id": message.get("id"),
                    "error": {
                        "code": -32601,
                        "message": "Method not found"
                    }
                }),
            }
        } else {
            json!({
                "jsonrpc": "2.0",
                "id": message.get("id"),
                "error": {
                    "code": -32600,
                    "message": "Invalid Request"
                }
            })
        }
    }

    fn handle_initialize(&self) -> Value {
        json!({
            "jsonrpc": "2.0",
            "id": 1,
            "result": {
                "protocolVersion": "2024-11-05",
                "capabilities": {
                    "tools": {}
                },
                "serverInfo": {
                    "name": self.name,
                    "version": self.version
                }
            }
        })
    }

    fn handle_tools_list(&self) -> Value {
        json!({
            "jsonrpc": "2.0",
            "id": 2,
            "result": {
                "tools": [
                    {
                        "name": "health_check",
                        "description": "Check if the TraderGrader MCP server is running",
                        "inputSchema": {
                            "type": "object",
                            "properties": {},
                            "required": []
                        }
                    },
                    {
                        "name": "get_market_orders",
                        "description": "Fetch current market orders for a specific region and optionally filter by item type",
                        "inputSchema": {
                            "type": "object",
                            "properties": {
                                "region_id": {
                                    "type": "integer",
                                    "description": "EVE Online region ID (e.g., 10000002 for The Forge)"
                                },
                                "type_id": {
                                    "type": "integer",
                                    "description": "Optional item type ID to filter orders"
                                }
                            },
                            "required": ["region_id"]
                        }
                    },
                    {
                        "name": "get_market_summary",
                        "description": "Get a summary of market data including buy/sell orders and price spread for a specific item type in a region",
                        "inputSchema": {
                            "type": "object",
                            "properties": {
                                "region_id": {
                                    "type": "integer",
                                    "description": "EVE Online region ID (e.g., 10000002 for The Forge)"
                                },
                                "type_id": {
                                    "type": "integer",
                                    "description": "Item type ID to analyze"
                                }
                            },
                            "required": ["region_id", "type_id"]
                        }
                    }
                ]
            }
        })
    }

    async fn handle_tool_call(&self, message: &Value) -> Value {
        if let Some(params) = message.get("params") {
            if let Some(name) = params.get("name").and_then(|n| n.as_str()) {
                match name {
                    "health_check" => json!({
                        "jsonrpc": "2.0",
                        "id": message.get("id"),
                        "result": {
                            "content": [{
                                "type": "text",
                                "text": format!("✅ {} v{} is healthy and running!\nTimestamp: {}",
                                    self.name,
                                    self.version,
                                    chrono::Utc::now().to_rfc3339()
                                )
                            }]
                        }
                    }),
                    "get_market_orders" => {
                        if let Some(arguments) = params.get("arguments") {
                            let region_id = arguments
                                .get("region_id")
                                .and_then(|v| v.as_i64())
                                .unwrap_or(0) as i32;
                            let type_id = arguments
                                .get("type_id")
                                .and_then(|v| v.as_i64())
                                .map(|v| v as i32);

                            match self.fetch_market_orders(region_id, type_id).await {
                                Ok(orders) => json!({
                                    "jsonrpc": "2.0",
                                    "id": message.get("id"),
                                    "result": {
                                        "content": [{
                                            "type": "text",
                                            "text": format!("Found {} market orders for region {}", orders.len(), region_id)
                                        }]
                                    }
                                }),
                                Err(e) => json!({
                                    "jsonrpc": "2.0",
                                    "id": message.get("id"),
                                    "error": {
                                        "code": -32603,
                                        "message": format!("Failed to fetch market orders: {}", e)
                                    }
                                }),
                            }
                        } else {
                            json!({
                                "jsonrpc": "2.0",
                                "id": message.get("id"),
                                "error": {
                                    "code": -32602,
                                    "message": "Missing arguments for get_market_orders"
                                }
                            })
                        }
                    }
                    "get_market_summary" => {
                        if let Some(arguments) = params.get("arguments") {
                            let region_id = arguments
                                .get("region_id")
                                .and_then(|v| v.as_i64())
                                .unwrap_or(0) as i32;
                            let type_id = arguments
                                .get("type_id")
                                .and_then(|v| v.as_i64())
                                .unwrap_or(0) as i32;

                            match self.get_market_summary(region_id, type_id).await {
                                Ok(summary) => json!({
                                    "jsonrpc": "2.0",
                                    "id": message.get("id"),
                                    "result": {
                                        "content": [{
                                            "type": "text",
                                            "text": summary
                                        }]
                                    }
                                }),
                                Err(e) => json!({
                                    "jsonrpc": "2.0",
                                    "id": message.get("id"),
                                    "error": {
                                        "code": -32603,
                                        "message": format!("Failed to get market summary: {}", e)
                                    }
                                }),
                            }
                        } else {
                            json!({
                                "jsonrpc": "2.0",
                                "id": message.get("id"),
                                "error": {
                                    "code": -32602,
                                    "message": "Missing arguments for get_market_summary"
                                }
                            })
                        }
                    }
                    _ => json!({
                        "jsonrpc": "2.0",
                        "id": message.get("id"),
                        "error": {
                            "code": -32601,
                            "message": format!("Unknown tool: {}", name)
                        }
                    }),
                }
            } else {
                json!({
                    "jsonrpc": "2.0",
                    "id": message.get("id"),
                    "error": {
                        "code": -32602,
                        "message": "Invalid tool call parameters"
                    }
                })
            }
        } else {
            json!({
                "jsonrpc": "2.0",
                "id": message.get("id"),
                "error": {
                    "code": -32602,
                    "message": "Missing parameters"
                }
            })
        }
    }
}

impl Default for TraderGraderApplication {
    fn default() -> Self {
        Self::new()
    }
}
