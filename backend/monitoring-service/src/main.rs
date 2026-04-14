use monitoring_service::bootstrap::startup;

#[tokio::main]
async fn main() -> Result<(), monitoring_service::shared::error::AppError> {
    startup::run().await
}
