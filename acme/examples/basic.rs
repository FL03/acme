/*
    appellation: basic <module>
    authors: @FL03
*/
//! a basic example of using the `acme` engine
use acme::engine::Engine;

#[tokio::main]
async fn main() -> acme::Result<()> {
    // setup the tracing subscriber
    tracing_subscriber::fmt()
        .with_line_number(false)
        .with_max_level(tracing::Level::TRACE)
        .with_thread_ids(false)
        .with_target(true)
        .compact()
        .init();
    // verify the tracing layers are initialized
    tracing::info!("Welcome to {name}", name = acme::consts::VERSION);
    // initialize a new instance of the engine
    let acme = Engine::new()?;
    // start the engine's scheduler
    acme.run().await?;

    Ok(())
}
