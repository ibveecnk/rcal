#![allow(dead_code)] // Disable warnings for unused code for now

mod cache;
mod database;
mod model;

#[tokio::main]
async fn main() {
    console_subscriber::init();
    tracing::info!("Starting the application...");

    let shutdown_controller = tokio_utils::ShutdownController::new();

    for i in 0..5 {
        let mut monitor = shutdown_controller.subscribe();

        tokio::spawn(async move {
            while !monitor.is_shutdown() {
                tokio::select! {
                    _ = monitor.recv() => {
                        tracing::info!("Shutting down task {}", i);
                        break;
                    }
                    _ = tokio::time::sleep(tokio::time::Duration::from_secs(1)) => {
                        tracing::info!("Task {} is running", i);
                    }
                }
            }
        });
    }

    tokio::signal::ctrl_c().await.unwrap();
    shutdown_controller.shutdown().await;
}
