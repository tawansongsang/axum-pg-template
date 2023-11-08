use axum::{
    http::{Method, Uri},
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use uuid::Uuid;

use crate::{ctx::Ctx, log::log_request, routes};

pub async fn mw_response_map(
    ctx: Option<Ctx>,
    uri: Uri,
    req_method: Method,
    res: Response,
) -> Response {
    println!("--> {:<12} - mw_response_map", "RES_MAPPER");
    let uuid = Uuid::new_v4();

    // -- Get the eventual response error.
    let route_error = res.extensions().get::<routes::Error>();
    let client_status_error = route_error.map(|se| se.client_status_and_error());

    //  -- If client error, build the new response.
    let error_response = client_status_error
        .as_ref()
        .map(|(status_code, client_error)| {
            let client_error_body = json!({
                "error": {
                    "type": client_error.as_ref(),
                    "req_uuid": uuid.to_string(),
                }
            });

            println!("->> CLIENT ERROR BODY:\n{client_error_body}");

            // Build the new response from the client_error_body
            (*status_code, Json(client_error_body)).into_response()
        });

    // -- Build and log the server log line.
    let client_error = client_status_error.unzip().1;
    // TODO: Need to hander if log_request fail (but should not fail request)
    let _ = log_request(uuid, req_method, uri, ctx, route_error, client_error).await;

    println!("\n");

    error_response.unwrap_or(res)
}
