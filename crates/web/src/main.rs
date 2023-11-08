mod ctx;
mod error;
mod log;
mod model;
mod routes;

// region:    --- Modules
pub use self::error::{Error, Result};

use std::net::SocketAddr;

use crate::{
    model::ModelManager,
    routes::{mw_auth::mw_ctx_resolve, mw_res_map::mw_response_map, routes_static},
};
use axum::{middleware, Router};
use tower_cookies::CookieManagerLayer;

// endregion: --- Modules

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize ModelController.
    let mm = ModelManager::new().await?;

    // -- Define Routes
    // let routes_rpc = rpc::routes(mm.clone())
    //     .route_layer(middleware::from_fn(mw_auth::mw_require_auth));
    let routes_all = Router::new()
        .merge(routes::routes_login::routes())
        // .nest("/api", routes_apis)
        .layer(middleware::map_response(mw_response_map))
        .layer(middleware::from_fn_with_state(mm.clone(), mw_ctx_resolve))
        .layer(CookieManagerLayer::new())
        .fallback_service(routes_static::serve_dir());

    // region:    --- Start Server
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("->> LISTENING on {addr}\n");
    axum::Server::bind(&addr)
        .serve(routes_all.into_make_service())
        .await
        .unwrap();
    // endregion: --- Start Server

    Ok(())
}
