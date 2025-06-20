use tradergrader::TraderGraderApplication;

#[tokio::test]
async fn test_health_check() {
    let app = TraderGraderApplication::new();
    // This just tests that the application can be created successfully
    // Since fields are private, we just test that it can be created
    let _ = app;
}

#[tokio::test]
#[ignore] // Ignore by default since it makes real API calls
async fn test_fetch_market_orders() {
    let app = TraderGraderApplication::new();
    
    // Test fetching market orders for The Forge (region_id: 10000002)
    // This is a real API call, so we ignore it by default
    let result = app.fetch_market_orders(10000002, None).await;
    assert!(result.is_ok(), "Should be able to fetch market orders");
    
    let orders = result.unwrap();
    assert!(!orders.is_empty(), "Should have some market orders");
}

#[tokio::test]
#[ignore] // Ignore by default since it makes real API calls
async fn test_market_summary() {
    let app = TraderGraderApplication::new();
    
    // Test market summary for Tritanium (type_id: 34) in The Forge (region_id: 10000002)
    let result = app.get_market_summary(10000002, 34).await;
    assert!(result.is_ok(), "Should be able to get market summary");
    
    let summary = result.unwrap();
    assert!(summary.contains("Market Summary"), "Summary should contain expected text");
    assert!(summary.contains("Type 34"), "Summary should mention the type ID");
}

#[tokio::test]
#[ignore] // Ignore by default since it makes real API calls
async fn test_market_history() {
    let app = TraderGraderApplication::new();
    
    // Test market history for Tritanium in The Forge
    let result = app.fetch_market_history(10000002, 34).await;
    assert!(result.is_ok(), "Should be able to fetch market history");
    
    let history = result.unwrap();
    assert!(!history.is_empty(), "Should have historical data");
    
    // Check the structure of the first entry
    let first_entry = &history[0];
    assert!(first_entry.average > 0.0, "Average price should be positive");
    assert!(first_entry.highest >= first_entry.lowest, "Highest should be >= lowest");
    assert!(first_entry.volume >= 0, "Volume should be non-negative");
    assert!(!first_entry.date.is_empty(), "Date should not be empty");
}

#[tokio::test]
#[ignore] // Ignore by default since it makes real API calls
async fn test_price_analysis() {
    let app = TraderGraderApplication::new();
    
    // Test price analysis for Skill Injectors
    let result = app.analyze_price_trends(10000002, 44992).await;
    assert!(result.is_ok(), "Should be able to analyze price trends");
    
    let analysis = result.unwrap();
    assert!(analysis.current_price > 0.0, "Current price should be positive");
    assert!(!analysis.trend.is_empty(), "Trend should not be empty");
    
    // Test the summary format
    let summary_result = app.get_price_history_summary(10000002, 44992).await;
    assert!(summary_result.is_ok(), "Should be able to get price history summary");
    
    let summary = summary_result.unwrap();
    assert!(summary.contains("Price Analysis"), "Summary should contain expected text");
    assert!(summary.contains("Current Price"), "Summary should mention current price");
    assert!(summary.contains("Trend"), "Summary should mention trend");
}