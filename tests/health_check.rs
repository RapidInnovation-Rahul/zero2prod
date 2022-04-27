// use zero2prod::run;

#[tokio::test]
async fn health_check_works() {
    spawn_app();
}

fn spawn_app() {
    let server = zero2prod::run("127.0.0.1:0").expect("failed to bind address");
    let _ = tokio::spawn(server);    
}
