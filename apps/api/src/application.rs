use std::net::SocketAddr;

use acme::BoxedError;
use async_trait::async_trait;
use axum;
use http::header;
use tower_http::{
    compression::CompressionLayer,
    propagate_header::PropagateHeaderLayer,
    sensitive_headers::SetSensitiveHeadersLayer,
    trace,
};

use crate::{endpoints, settings::Settings};
use crate::actors::{Context, Logger};

#[async_trait]
pub trait Api {
    type Configuration;

    async fn configure(&self) -> Result<Self::Configuration, BoxedError>;
    async fn run() -> Result<(), BoxedError>;
}

#[derive(Clone, Debug)]
pub struct Application {
    pub context: Context
}

impl Application {
    pub fn setup() -> Self {
        let settings = match Settings::new() {
            Ok(value) => value,
            Err(err) => panic!("ConfigurationError: {:#?}", err)
        };

        println!("{}", settings);

        Logger::setup(&settings);

        let host = [0, 0, 0, 0];
        let port = settings.server.port;

        let address: SocketAddr = SocketAddr::from((host, port));
        let context = Context::new(settings.clone());
        Self {
            context: Context::new(settings.clone())
        }
    }
    pub async fn client(&self) -> axum::Router {
        let client = axum::Router::new()
            .merge(endpoints::base_router())
            .layer(
                trace::TraceLayer::new_for_http()
                    .make_span_with(trace::DefaultMakeSpan::new().include_headers(true))
                    .on_request(trace::DefaultOnRequest::new().level(tracing::Level::INFO))
                    .on_response(trace::DefaultOnResponse::new().level(tracing::Level::INFO)),
            )
            .layer(
                SetSensitiveHeadersLayer::new(std::iter::once(header::AUTHORIZATION))
            )
            .layer(
                CompressionLayer::new()
            )
            .layer(
                PropagateHeaderLayer::new(header::HeaderName::from_static("x-request-id")
                )
            )
            .layer(
                axum::Extension(self.context.clone())
            );
        return client
    }

    pub async fn server(&self) {
        let host = [0, 0, 0, 0];
        let port = self.context.settings.server.port.clone();
        let address = SocketAddr::from((host, port));

        println!("View the application locally at http://localhost:{}", &port);

        let server = axum::Server::bind(&address)
            .serve(self.client().await.into_make_service())
            .await
            .expect("Config Error: Failed to start the server...");
    }

    pub async fn run(&self) -> Result<Self, BoxedError> {
        self.server().await;
        Ok(
            Self {
                context: self.context.clone()
            }
        )
    }
}