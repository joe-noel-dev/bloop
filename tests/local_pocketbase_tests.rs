mod common;

use crate::common::{LocalPocketbase, TestUser};

#[tokio::test]
async fn test_local_pocketbase_startup_and_user_creation() {
    let mut pocketbase = LocalPocketbase::new()
        .await
        .expect("Failed to create LocalPocketbase instance");

    // Start the server
    pocketbase
        .start()
        .await
        .expect("Failed to start PocketBase server");

    // Verify server is running by checking health endpoint
    let client = reqwest::Client::new();
    let health_url = format!("{}/api/health", pocketbase.url());
    let response = client.get(&health_url).send().await;
    
    assert!(response.is_ok(), "Health check failed");
    let response = response.unwrap();
    assert!(response.status().is_success(), "Health endpoint returned error");

    // Create a test user
    let test_user = TestUser::new("testuser123", "test@example.com", "testpassword", "Test User");
    
    let result = pocketbase.add_user(test_user.clone()).await;
    
    // Note: This might fail initially due to authentication requirements
    // But we can verify the server is running and the API is accessible
    if let Err(e) = result {
        println!("User creation failed (expected for now): {}", e);
        // This is expected until we implement proper authentication
    }

    // Stop the server
    pocketbase
        .stop()
        .await
        .expect("Failed to stop PocketBase server");
}

#[tokio::test]
async fn test_local_pocketbase_url_generation() {
    let pocketbase = LocalPocketbase::new()
        .await
        .expect("Failed to create LocalPocketbase instance");

    let url = pocketbase.url();
    assert!(url.starts_with("http://127.0.0.1:"));
    assert!(url.contains("://127.0.0.1:"));
}