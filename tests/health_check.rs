// use zero2prod::run;
// use actix_web::{web, App, HttpResponse, HttpServer};
use std::net::TcpListener;
// use std::fmt::Display;


fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("failed to bind random port");
    
    let port = listener.local_addr().unwrap().port();
    let server = zero2prod::run(listener).expect("failed to bind address");
    let _ = tokio::spawn(server);
    format!("http://127.0.0.1:{}", port)
}


#[tokio::test]
async fn health_check_works() {
    let address = spawn_app();
    let client = reqwest::Client::new();
    let response = client
                    .get(&format!("{}/health_check", &address))
                    .send()
                    .await
                    .expect("failed to execute request");
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}




//if subscriber is found return 200
#[tokio::test]
async fn subscriber_found(){
    let app_address = spawn_app();
    let client = reqwest::Client::new();
    let  body = "name=rahul%20mondal&email=rahulmondal%40gmail.com";
    let response = client
                    .post(&format!("{}/subscription",&app_address))
                    .header("Content-type","application/x-www-form-urlencoded")
                    .body(body)
                    .send()
                    .await
                    .expect("failed to execute request");
    assert_eq!(200, response.status().as_u16());
}

// if subscriber is not found return 400 error
#[tokio::test]
async fn subscriber_not_found(){
    let app_address = spawn_app();
    let client = reqwest::Client::new();

    // test_cases for testing purpose
    let test_cases = vec![
        ("name=rahul%20mondal", "missing the email"),
        ("email=rahulmondal%40gmail.com", "missing the name"),
        ("", "missing both name and email")
        // ("name=rahul%20mondal","email=rahulmondal%40gmail.com" )
        ];

    for(invalid_body, error_massege) in test_cases {
        let response = client 
                        .post(&format!("{}/subscription",&app_address))
                        .header("Content-Type", "application/x-www-form-urlencoded")
                        .body(invalid_body)
                        .send()
                        .await.expect("failed to execute request");
                        
                        
                        
        assert_eq!(
            400,
            response.status().as_u16(),
            // customize error massege for test failure
            "The API did not fail with 400 when the payload was {}", error_massege
        );
    } 
}