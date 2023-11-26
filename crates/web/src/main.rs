// region:    --- Modules

mod config;
mod crypt;
mod ctx;
mod error;
mod log;
mod model;
mod routes;
mod utils;

// #[cfg(test)] // Commented during early development.
pub mod _dev_utils;

pub use self::error::{Error, Result};
pub use config::config; // can use crate::config

use std::net::SocketAddr;

use crate::{
    model::ModelManager,
    routes::{
        mw_auth::{self, mw_ctx_resolve},
        mw_res_map::mw_response_map,
        routes_static, rpc,
    },
};
use axum::{middleware, Router};
use tower_cookies::CookieManagerLayer;
use tracing::info;
use tracing_subscriber::EnvFilter;

// endregion: --- Modules

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .without_time() // For early local development
        .with_target(false) // can set true if you want
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    // -- FOR DEV ONLY
    _dev_utils::init_dev().await;

    // Initialize ModelController.
    let mm = ModelManager::new().await?;

    // -- Define Routes
    let routes_rpc =
        rpc::routes(mm.clone()).route_layer(middleware::from_fn(mw_auth::mw_ctx_require));

    let routes_all = Router::new()
        .merge(routes::routes_login::routes(mm.clone()))
        .nest("/api", routes_rpc)
        .layer(middleware::map_response(mw_response_map))
        .layer(middleware::from_fn_with_state(mm.clone(), mw_ctx_resolve))
        .layer(CookieManagerLayer::new())
        .fallback_service(routes_static::serve_dir());

    // region:    --- Start Server
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    info!("{:<12} - {addr}\n", "LISTENING");
    axum::Server::bind(&addr)
        .serve(routes_all.into_make_service())
        .await
        .unwrap();
    // endregion: --- Start Server

    Ok(())
}
