use crate::market::MarketClient;
use serde_json::{Value, json};

/// MCP protocol handler for TraderGrader
/// 
/// Handles all Model Context Protocol (MCP) message processing, including
/// initialization, tool listing, tool execution, and protocol compliance.
/// This is the core component that bridges MCP requests to EVE Online market data.
#[derive(Debug)]
pub struct McpHandler {
    pub market_client: MarketClient,
    server_name: String,
    server_version: String,
}

impl McpHandler {
    /// Creates a new MCP protocol handler
    /// 
    /// # Arguments
    /// 
    /// * `name` - The server name to report in MCP initialize responses
    /// * `version` - The server version to report in MCP initialize responses
    /// 
    /// # Examples
    /// 
    /// ```
    /// use tradergrader::McpHandler;
    /// let handler = McpHandler::new("TraderGrader".to_string(), "0.1.0".to_string());
    /// ```
    pub fn new(name: String, version: String) -> Self {
        Self {
            market_client: MarketClient::new(),
            server_name: name,
            server_version: version,
        }
    }

    /// Handles incoming MCP protocol messages
    /// 
    /// This is the main entry point for processing MCP JSON-RPC messages.
    /// It routes messages to appropriate handlers based on the method field
    /// and returns properly formatted JSON-RPC responses.
    /// 
    /// # Arguments
    /// 
    /// * `message` - A JSON-RPC message as a serde_json::Value
    /// 
    /// # Returns
    /// 
    /// Returns a JSON-RPC response as a serde_json::Value, or null for notifications
    /// 
    /// # Examples
    /// 
    /// ```no_run
    /// # use tradergrader::McpHandler;
    /// # use serde_json::{json, Value};
    /// # async fn example() {
    /// let handler = McpHandler::new("Test".to_string(), "1.0".to_string());
    /// let message = json!({
    ///     "jsonrpc": "2.0",
    ///     "id": 1,
    ///     "method": "tools/list"
    /// });
    /// let response = handler.handle_message(message).await;
    /// # }
    /// ```
    pub async fn handle_message(&self, message: Value) -> Value {
        // Basic MCP message handling
        if let Some(method) = message.get("method").and_then(|m| m.as_str()) {
            match method {
                "initialize" => self.handle_initialize(&message),
                "initialized" => self.handle_initialized(),
                "notifications/initialized" => self.handle_initialized(),
                "notifications/cancelled" => self.handle_cancelled(&message),
                "tools/list" => self.handle_tools_list(&message),
                "tools/call" => self.handle_tool_call(&message).await,
                "ping" => self.handle_ping(&message),
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

    /// Handle MCP initialize request
    fn handle_initialize(&self, message: &Value) -> Value {
        json!({
            "jsonrpc": "2.0",
            "id": message.get("id"),
            "result": {
                "protocolVersion": "2025-03-26",
                "capabilities": {
                    "tools": {
                        "listChanged": false
                    }
                },
                "serverInfo": {
                    "name": self.server_name,
                    "version": self.server_version,
                    "instructions": "EVE Online market data MCP server providing real-time trading analysis and historical price data through ESI API integration."
                }
            }
        })
    }

    /// Handle initialized notification (no response required)
    fn handle_initialized(&self) -> Value {
        // Notifications don't require responses, but we return empty for consistency
        json!(null)
    }

    /// Handle ping request for connection testing
    fn handle_ping(&self, message: &Value) -> Value {
        json!({
            "jsonrpc": "2.0",
            "id": message.get("id"),
            "result": {}
        })
    }

    /// Handle tools/list request - return available tools
    fn handle_tools_list(&self, message: &Value) -> Value {
        json!({
            "jsonrpc": "2.0",
            "id": message.get("id"),
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
                    },
                    {
                        "name": "get_market_history",
                        "description": "Fetch historical market data (price, volume, order count) for a specific item in a region",
                        "inputSchema": {
                            "type": "object",
                            "properties": {
                                "region_id": {
                                    "type": "integer",
                                    "description": "EVE Online region ID (e.g., 10000002 for The Forge)"
                                },
                                "type_id": {
                                    "type": "integer",
                                    "description": "Item type ID to get history for"
                                }
                            },
                            "required": ["region_id", "type_id"]
                        }
                    },
                    {
                        "name": "get_price_analysis",
                        "description": "Analyze price trends including daily/weekly/monthly changes, volatility, and trend direction",
                        "inputSchema": {
                            "type": "object",
                            "properties": {
                                "region_id": {
                                    "type": "integer",
                                    "description": "EVE Online region ID (e.g., 10000002 for The Forge)"
                                },
                                "type_id": {
                                    "type": "integer",
                                    "description": "Item type ID to analyze trends for"
                                }
                            },
                            "required": ["region_id", "type_id"]
                        }
                    }
                ]
            }
        })
    }

    /// Handle tools/call request - execute specific tool
    async fn handle_tool_call(&self, message: &Value) -> Value {
        if let Some(params) = message.get("params") {
            if let Some(name) = params.get("name").and_then(|n| n.as_str()) {
                match name {
                    "health_check" => self.handle_health_check(message),
                    "get_market_orders" => self.handle_get_market_orders(message, params).await,
                    "get_market_summary" => self.handle_get_market_summary(message, params).await,
                    "get_market_history" => self.handle_get_market_history(message, params).await,
                    "get_price_analysis" => self.handle_get_price_analysis(message, params).await,
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

    /// Handle health check tool
    fn handle_health_check(&self, message: &Value) -> Value {
        json!({
            "jsonrpc": "2.0",
            "id": message.get("id"),
            "result": {
                "content": [{
                    "type": "text",
                    "text": format!("✅ {} v{} is healthy and running!\nTimestamp: {}",
                        self.server_name,
                        self.server_version,
                        chrono::Utc::now().to_rfc3339()
                    )
                }]
            }
        })
    }

    /// Handle get_market_orders tool
    async fn handle_get_market_orders(&self, message: &Value, params: &Value) -> Value {
        if let Some(arguments) = params.get("arguments") {
            let region_id = arguments
                .get("region_id")
                .and_then(|v| v.as_i64())
                .unwrap_or(0) as i32;
            let type_id = arguments
                .get("type_id")
                .and_then(|v| v.as_i64())
                .map(|v| v as i32);

            match self.market_client.fetch_market_orders(region_id, type_id).await {
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

    /// Handle get_market_summary tool
    async fn handle_get_market_summary(&self, message: &Value, params: &Value) -> Value {
        if let Some(arguments) = params.get("arguments") {
            let region_id = arguments
                .get("region_id")
                .and_then(|v| v.as_i64())
                .unwrap_or(0) as i32;
            let type_id = arguments
                .get("type_id")
                .and_then(|v| v.as_i64())
                .unwrap_or(0) as i32;

            match self.market_client.get_market_summary(region_id, type_id).await {
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

    /// Handle get_market_history tool
    async fn handle_get_market_history(&self, message: &Value, params: &Value) -> Value {
        if let Some(arguments) = params.get("arguments") {
            let region_id = arguments
                .get("region_id")
                .and_then(|v| v.as_i64())
                .unwrap_or(0) as i32;
            let type_id = arguments
                .get("type_id")
                .and_then(|v| v.as_i64())
                .unwrap_or(0) as i32;
            
            match self.market_client.fetch_market_history(region_id, type_id).await {
                Ok(history) => {
                    let history_text = if history.is_empty() {
                        "No historical data available".to_string()
                    } else {
                        let recent_days = history.iter().take(10);
                        let mut text = format!("Recent {} days of market history:\n", std::cmp::min(history.len(), 10));
                        for day in recent_days {
                            text.push_str(&format!(
                                "{}: Avg: {:.2} ISK, High: {:.2} ISK, Low: {:.2} ISK, Volume: {}\n",
                                day.date, day.average, day.highest, day.lowest, day.volume
                            ));
                        }
                        text
                    };
                    
                    json!({
                        "jsonrpc": "2.0",
                        "id": message.get("id"),
                        "result": {
                            "content": [{
                                "type": "text",
                                "text": history_text
                            }]
                        }
                    })
                },
                Err(e) => json!({
                    "jsonrpc": "2.0",
                    "id": message.get("id"),
                    "error": {
                        "code": -32603,
                        "message": format!("Failed to fetch market history: {}", e)
                    }
                })
            }
        } else {
            json!({
                "jsonrpc": "2.0",
                "id": message.get("id"),
                "error": {
                    "code": -32602,
                    "message": "Missing arguments for get_market_history"
                }
            })
        }
    }

    /// Handle get_price_analysis tool
    async fn handle_get_price_analysis(&self, message: &Value, params: &Value) -> Value {
        if let Some(arguments) = params.get("arguments") {
            let region_id = arguments
                .get("region_id")
                .and_then(|v| v.as_i64())
                .unwrap_or(0) as i32;
            let type_id = arguments
                .get("type_id")
                .and_then(|v| v.as_i64())
                .unwrap_or(0) as i32;
            
            match self.market_client.get_price_history_summary(region_id, type_id).await {
                Ok(analysis) => json!({
                    "jsonrpc": "2.0",
                    "id": message.get("id"),
                    "result": {
                        "content": [{
                            "type": "text",
                            "text": analysis
                        }]
                    }
                }),
                Err(e) => json!({
                    "jsonrpc": "2.0",
                    "id": message.get("id"),
                    "error": {
                        "code": -32603,
                        "message": format!("Failed to get price analysis: {}", e)
                    }
                })
            }
        } else {
            json!({
                "jsonrpc": "2.0",
                "id": message.get("id"),
                "error": {
                    "code": -32602,
                    "message": "Missing arguments for get_price_analysis"
                }
            })
        }
    }

    /// Handle cancellation notifications
    fn handle_cancelled(&self, _message: &Value) -> Value {
        // Notifications don't require responses
        json!(null)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_mcp_handler_creation() {
        let handler = McpHandler::new("TestServer".to_string(), "1.0.0".to_string());
        assert_eq!(handler.server_name, "TestServer");
        assert_eq!(handler.server_version, "1.0.0");
    }

    #[test]
    fn test_initialize_message() {
        let handler = McpHandler::new("TestServer".to_string(), "1.0.0".to_string());
        let message = json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "initialize",
            "params": {}
        });
        
        let response = handler.handle_initialize(&message);
        
        assert_eq!(response["jsonrpc"], "2.0");
        assert_eq!(response["id"], 1);
        assert_eq!(response["result"]["protocolVersion"], "2025-03-26");
        assert_eq!(response["result"]["serverInfo"]["name"], "TestServer");
    }

    #[test]
    fn test_tools_list_message() {
        let handler = McpHandler::new("TestServer".to_string(), "1.0.0".to_string());
        let message = json!({
            "jsonrpc": "2.0",
            "id": 2,
            "method": "tools/list"
        });
        
        let response = handler.handle_tools_list(&message);
        
        assert_eq!(response["jsonrpc"], "2.0");
        assert_eq!(response["id"], 2);
        assert!(response["result"]["tools"].is_array());
        
        let tools = response["result"]["tools"].as_array().unwrap();
        assert!(tools.len() >= 5); // Should have at least 5 tools
        
        // Check for expected tool names
        let tool_names: Vec<&str> = tools.iter()
            .map(|tool| tool["name"].as_str().unwrap())
            .collect();
        
        assert!(tool_names.contains(&"health_check"));
        assert!(tool_names.contains(&"get_market_orders"));
        assert!(tool_names.contains(&"get_market_summary"));
        assert!(tool_names.contains(&"get_market_history"));
        assert!(tool_names.contains(&"get_price_analysis"));
    }

    #[test]
    fn test_ping_message() {
        let handler = McpHandler::new("TestServer".to_string(), "1.0.0".to_string());
        let message = json!({
            "jsonrpc": "2.0",
            "id": 3,
            "method": "ping"
        });
        
        let response = handler.handle_ping(&message);
        
        assert_eq!(response["jsonrpc"], "2.0");
        assert_eq!(response["id"], 3);
        assert_eq!(response["result"], json!({}));
    }

    #[test]
    fn test_invalid_method() {
        let handler = McpHandler::new("TestServer".to_string(), "1.0.0".to_string());
        let message = json!({
            "jsonrpc": "2.0",
            "id": 4,
            "method": "nonexistent_method"
        });
        
        let response = tokio_test::block_on(handler.handle_message(message));
        
        assert_eq!(response["jsonrpc"], "2.0");
        assert_eq!(response["id"], 4);
        assert_eq!(response["error"]["code"], -32601);
        assert_eq!(response["error"]["message"], "Method not found");
    }

    #[test]
    fn test_initialized_notification() {
        let handler = McpHandler::new("TestServer".to_string(), "1.0.0".to_string());
        let response = handler.handle_initialized();
        assert_eq!(response, json!(null));
    }

    #[test]
    fn test_cancelled_notification() {
        let handler = McpHandler::new("TestServer".to_string(), "1.0.0".to_string());
        let message = json!({
            "jsonrpc": "2.0",
            "method": "notifications/cancelled"
        });
        let response = handler.handle_cancelled(&message);
        assert_eq!(response, json!(null));
    }
}