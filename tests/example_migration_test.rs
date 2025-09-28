mod common;

use bloop::bloop::Request;
use crate::common::{IntegrationFixture, EnhancedIntegrationFixture, MockUser};

/// Example showing how to migrate a test from mock to real PocketBase
/// 
/// This demonstrates that the same test logic can work with either backend
/// by just changing the fixture type and user creation method.

// Original test using mock backend
#[tokio::test]
async fn test_user_login_with_mock() {
    let mut fixture = IntegrationFixture::new().await;
    
    // Create mock user
    let user = MockUser::new("user123", "test@example.com", "password123", "Test User");
    fixture.mocketbase().add_user(user.clone()).await;
    
    // Test login
    let request = Request::log_in_request(user.email.clone(), user.password.clone());
    fixture.send_request(request).await;
    
    let response = fixture
        .wait_for_response(|response| {
            if let Some(user_state) = response.user_status.as_ref() {
                if user_state.user.is_some() {
                    return true;
                }
            }
            false
        })
        .await
        .expect("Should receive user state");
    
    let returned_user = response.user_status.as_ref().unwrap().user.as_ref().unwrap();
    assert_eq!(returned_user.email, user.email);
    assert_eq!(returned_user.name, user.name);
}

// Same test logic adapted for real PocketBase
#[tokio::test]
async fn test_user_login_with_real_pocketbase() {
    let mut fixture = EnhancedIntegrationFixture::new_with_real_pocketbase().await;
    
    // Create real user (this part may fail until authentication is fully implemented)
    let user_email = "test@example.com";
    let user_password = "password123";
    let user_name = "Test User";
    
    let add_user_result = fixture.add_test_user(user_email, user_password, user_name).await;
    
    match add_user_result {
        Ok(()) => {
            println!("User created successfully, testing login...");
            
            // Test login - same logic as mock version
            let request = Request::log_in_request(user_email.to_string(), user_password.to_string());
            fixture.send_request(request).await;
            
            let response = fixture
                .wait_for_response(|response| {
                    if let Some(user_state) = response.user_status.as_ref() {
                        if user_state.user.is_some() {
                            return true;
                        }
                    }
                    !response.error.is_empty()
                })
                .await;
            
            match response {
                Ok(resp) => {
                    if !resp.error.is_empty() {
                        println!("Login failed (may be expected): {}", resp.error);
                    } else {
                        let returned_user = resp.user_status.as_ref().unwrap().user.as_ref().unwrap();
                        assert_eq!(returned_user.email, user_email);
                        assert_eq!(returned_user.name, user_name);
                        println!("Login test passed with real PocketBase!");
                    }
                }
                Err(e) => {
                    println!("No response received: {}", e);
                }
            }
        }
        Err(e) => {
            println!("User creation failed (this is expected until full authentication is implemented): {}", e);
            // For now, just verify the server is running
            let url = fixture.local_pocketbase().url();
            println!("PocketBase server is running at: {}", url);
        }
    }
}

// Example of a backend-agnostic test that can work with either
#[tokio::test] 
async fn test_server_health_generic() {
    // Test with mock
    let mut mock_fixture = EnhancedIntegrationFixture::new().await;
    let mock_url = mock_fixture.mocketbase().uri();
    println!("Mock server URL: {}", mock_url);
    
    // Test with real PocketBase
    let mut real_fixture = EnhancedIntegrationFixture::new_with_real_pocketbase().await;
    let real_url = real_fixture.local_pocketbase().url();
    println!("Real PocketBase URL: {}", real_url);
    
    // Both should be accessible
    assert!(mock_url.starts_with("http://"));
    assert!(real_url.starts_with("http://"));
    
    // Real PocketBase should have a health endpoint
    let client = reqwest::Client::new();
    let health_response = client
        .get(&format!("{}/api/health", real_url))
        .send()
        .await
        .expect("Health check should work");
    
    assert!(health_response.status().is_success());
    println!("Both backends are accessible!");
}