use auth_service::{routes::SignupResponse, ErrorResponse};

use crate::helpers::{TestApp, get_random_email};

#[tokio::test]
async fn should_return_400_if_invalid_input() {
    let app = TestApp::new().await;

    // The input is considered invalid if:
    // - The email is empty or does not contain '@'
    // - The password is less than 8 characters
    let test_cases = [
        
        // empty email
        serde_json::json!({
            "email": "",
            "password": "",
            "requires2FA": true
        }),

        // missing @
        serde_json::json!({
            "email": "somemail.com".to_string(),
            "password": "legitimatepassword".to_string(),
            "requires2FA": true,
        }),

        // password less than 8
        serde_json::json!({
            "email": "avalidemail@me.com",
            "password": "short",
            "requires2FA": true,
        })
    ];

    for test in test_cases.iter() {
        let response = app.post_signup(test).await;
        assert_eq!(response.status().as_u16(), 
                    400,
                    "Failed on input, {:?}",
                    test);
        assert_eq!(
            response
                .json::<ErrorResponse>()
                .await
                .expect("Could not deserialize response body to ErrorResponse")
                .error,
                "Invalid credentials".to_owned()
        );

    }

    // Create an array of invalid inputs. Then, iterate through the array and 
    // make HTTP calls to the signup route. Assert a 400 HTTP status code is returned.
} 

#[tokio::test]
async fn should_return_409_if_email_exits() {
    let app = TestApp::new().await;
    let user = serde_json::json!({
            "email": "avaliduser@mail.com",
            "password": "avalidpassword",
            "requires2FA": true
        });

    // signup a user
    app.post_signup(&user).await;

    // try to signup user again (should return 409)
    let respone = app.post_signup(&user).await;
    assert_eq!(
        respone.status().as_u16(), 
        409,
        "Faild for input: {:?}",
        user
    );

    assert_eq!(
        respone
            .json::<ErrorResponse>()
            .await
            .expect("Could not deserialize response body to ErrorResponse")
            .error,
            "User already exists".to_owned()
    );
} 

#[tokio::test]
async fn should_return_201_if_valid_input() {
    let app = TestApp::new().await;
    let random_email = get_random_email();

    let test_case = serde_json::json!({
        "email": random_email,
        "password": "password123",
        "requires2FA": true,
        });

    let response = app.post_signup(&test_case).await;

    assert_eq!(response.status().as_u16(), 201);

    let expected_response = SignupResponse {
        message: "User created successfully!".to_owned(),
    };

    assert_eq!(
        response
            .json::<SignupResponse>()
            .await
            .expect("Could not deserialize response body to UserBody"),
        expected_response
    );
    
}

#[tokio::test]
async fn should_return_422_if_malformed_input() {
    let app = TestApp::new().await;
    let random_email = get_random_email();

    let test_cases = [

        // missing email
        serde_json::json!({
        "password": "password123",
        "requires2FA": false,
        }),

        // missing password
        serde_json::json!({
        "email": random_email,
        "requires2FA": true,
        }),

        // missing 2FA
        serde_json::json!({
        "email": random_email,
        "password": ""
        }),

        // integer email (String required by SignupRequest Struct)
        serde_json::json!({
        "email": 8,
        "password": ""
        }),
        serde_json::json!({}),
    ];

    for test in test_cases.iter() {
        let response = app.post_signup(test).await;
        assert_eq!(response.status().as_u16(), 
                    422,
                    "Failed for input, {:?}",
                    test
                );
    }


}