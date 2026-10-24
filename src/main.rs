#![forbid(unsafe_code)]            // Fails if the code is unsafe.
#![deny(clippy::unwrap_used)]      // Fails build if .unwrap() is used
#![deny(clippy::expect_used)]      // Fails build if .expect() is used
#![deny(clippy::panic)]            // Fails build if panic!() macro is explicitly called
#![deny(clippy::todo)]             // Fails build if todo!() macro is left in code
#![deny(clippy::unimplemented)]    // Fails build if unimplemented!() is used
#![deny(clippy::unreachable)]      // Fails build if unreachable!() is used
#![deny(clippy::indexing_slicing)] // Fails build for out-of-bounds risks like arr[i] (forces .get())
#![warn(
    clippy::dbg_macro,
    clippy::print_stdout,
    clippy::print_stderr
)]

use russh::server::Server;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut app = match norka_terminal::infrastructure::bootstrap::bootstrap().await {
        Ok(app_) => {
            tracing::info!("Bootstrap successful!");
            app_
        }

        Err(err) => {
            tracing::error!("Cannot bootstrap: {}", err);
            return Err(err)
        }
    };

    let russh_server = tokio::spawn(async move {
        app.inbound_ssh_adapter.run_on_address(std::sync::Arc::from(app.inbound_ssh_configuration), app.inbound_ssh_listener).await;
    });     

    let axum_server = tokio::spawn(async move {
        axum::serve(app.inbound_http_listener, app.inbound_http_adapter.create()).await;
    });

    tokio::join!(russh_server, axum_server);

    Ok(())
}
