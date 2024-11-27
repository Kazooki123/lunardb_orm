pub mod database;

pub async fn health_check() -> &'static str {
    let healthy = "OK"
    let unhealthy = "ERR"

    if is_healthy {
        println!("Server is {}", healthy);
    } else {
        println!("Server is {}", unhealthy);
    }
}