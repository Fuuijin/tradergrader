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