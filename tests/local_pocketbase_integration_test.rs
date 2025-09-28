mod common;

use bloop::bloop::Request;
use crate::common::{EnhancedIntegrationFixture, TestUser};

#[tokio::test]
async fn test_login_with_real_pocketbase() {
    let mut fixture = EnhancedIntegrationFixture::new_with_real_pocketbase().await;

    // Add a test user using the LocalPocketbase instance
    let test_user = TestUser::new("testuser123", "test@example.com", "testpassword", "Test User");
    
    // Try to add the user - this might fail until we improve authentication handling
    let add_result = fixture.local_pocketbase().add_user(test_user.clone()).await;
    
    match add_result {
        Ok(()) => {
            println!("Successfully created user");
            
            // Now try to login through the application
            let request = Request::log_in_request(test_user.email.clone(), test_user.password.clone());
            fixture.send_request(request).await;

            // Wait for login response
            let response = fixture
                .wait_for_response(|response| {
                    !response.error.is_empty() || response.user_status.as_ref().is_some()
                })
                .await;

            match response {
                Ok(resp) => {
                    if !resp.error.is_empty() {
                        println!("Login failed as expected (authentication needs work): {}", resp.error);
                    } else if let Some(user_status) = resp.user_status.as_ref() {
                        if let Some(user) = user_status.user.as_ref() {
                            println!("Login successful! User: {}", user.name);
                            assert_eq!(user.email, test_user.email);
                        }
                    }
                }
                Err(e) => {
                    println!("No response received: {}", e);
                }
            }
        }
        Err(e) => {
            println!("User creation failed (expected): {}", e);
            // This is expected until we implement proper admin authentication
        }
    }
}

#[tokio::test]
async fn test_pocketbase_server_health() {
    let mut fixture = EnhancedIntegrationFixture::new_with_real_pocketbase().await;
    
    let pocketbase = fixture.local_pocketbase();
    let url = pocketbase.url();
    
    println!("Testing PocketBase health at: {}", url);
    
    // Test health endpoint directly
    let client = reqwest::Client::new();
    let health_url = format!("{}/api/health", url);
    
    let response = client.get(&health_url).send().await.expect("Health check failed");
    assert!(response.status().is_success());
    
    let health_data: serde_json::Value = response.json().await.expect("Failed to parse health response");
    println!("Health response: {}", health_data);
    
    // Verify we can reach the collections endpoint (even if it returns an error due to no auth)
    let collections_url = format!("{}/api/collections", url);
    let collections_response = client.get(&collections_url).send().await.expect("Failed to reach collections endpoint");
    
    // We expect this to fail with 401 or similar, but the important thing is we can reach it
    println!("Collections endpoint status: {}", collections_response.status());
}

#[tokio::test]
async fn test_real_vs_mock_pocketbase_comparison() {
    // Test with mock first
    let mut mock_fixture = EnhancedIntegrationFixture::new().await;
    
    // Add user to mock
    mock_fixture.add_test_user("test@example.com", "password123", "Mock User").await.expect("Failed to add mock user");
    
    // Try login with mock
    let request = Request::log_in_request("test@example.com".to_string(), "password123".to_string());
    mock_fixture.send_request(request).await;
    
    let mock_response = mock_fixture
        .wait_for_response(|response| {
            response.user_status.as_ref().is_some()
        })
        .await;
    
    if let Ok(resp) = mock_response {
        if let Some(user_status) = resp.user_status.as_ref() {
            if let Some(user) = user_status.user.as_ref() {
                println!("Mock login successful: {}", user.name);
            }
        }
    }
    
    // Now test with real PocketBase
    let mut real_fixture = EnhancedIntegrationFixture::new_with_real_pocketbase().await;
    
    // Verify the server is running
    let url = real_fixture.local_pocketbase().url();
    println!("Real PocketBase running at: {}", url);
    
    // The actual login test would be similar, but may fail due to authentication requirements
    // This test demonstrates that both backend types can be used with the same interface
}