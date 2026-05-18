use bytes::Bytes;
use serde_json::Value;
use tokio_util::sync::CancellationToken;
use futures_util::StreamExt;
use http_body_util::StreamBody;
use reqwest::header::{self, HeaderValue};
use warp::http::{self, Response as WarpResponse};

use crate::handlers::RequestContext;
use crate::server::ModelResolverType;

use crate::error::ProxyError;

/// Handle OpenAI-style passthrough requests by mapping OpenAI `/v1` paths
/// to LMStudio native `/api/v1` equivalents and delegating to the existing
/// LMStudio passthrough implementation.
pub async fn handle_openai_passthrough(
    context: RequestContext<'_>,
    model_resolver: ModelResolverType,
    mut request: crate::handlers::lmstudio::LmStudioPassthroughRequest,
    cancellation_token: CancellationToken,
    load_timeout_seconds: u64,
) -> Result<warp::reply::Response, ProxyError> {
    // Attempt to parse JSON body for mapping; if parsing fails, fall back to passthrough.
    if request.method == reqwest::Method::POST {
        if let Ok(mut body_val) = serde_json::from_slice::<Value>(&request.body) {
            // Map common OpenAI `/v1` endpoints to fields LM Studio expects.
            if request.endpoint.starts_with("/v1/chat") || request.endpoint == "/v1/chat/completions" {
                // OpenAI chat -> LMStudio chat: messages, model, stream mostly compatible.
                // Ensure `model` exists; if `messages` exists we're fine.
                if body_val.get("model").is_none() && body_val.get("model_id").is_some() {
                    body_val["model"] = body_val["model_id"].clone();
                }
            } else if request.endpoint.starts_with("/v1/completions") {
                // OpenAI completions use `prompt`; LMStudio expects `prompt` too on /v1/completions.
                if body_val.get("prompt").is_none() && body_val.get("input").is_some() {
                    body_val["prompt"] = body_val["input"].clone();
                }
            } else if request.endpoint.starts_with("/v1/embeddings") {
                // embeddings: ensure `input` exists (OpenAI) and map directly.
                if body_val.get("input").is_none() && body_val.get("prompt").is_some() {
                    body_val["input"] = body_val["prompt"].clone();
                }
            }

            // Replace request body with the possibly-updated JSON.
            if let Ok(new_bytes) = serde_json::to_vec(&body_val) {
                request.body = Bytes::from(new_bytes);
            }
        }
    }

    // Build target URL using configured api_url (acts as OpenAI base here)
    let mut target_url = context.endpoint_url(&request.endpoint);
    target_url = context.append_query_params(target_url, request.query.as_deref());

    // Prepare headers: copy incoming headers, ensure Authorization when runtime api_key is set
    let mut forward_headers = header::HeaderMap::new();
    for (name, value) in request.headers.iter() {
        if let (Ok(hn), Ok(hv)) = (
            header::HeaderName::from_bytes(name.as_str().as_bytes()),
            header::HeaderValue::from_bytes(value.as_bytes()),
        ) {
            forward_headers.insert(hn, hv);
        }
    }

    if !forward_headers.contains_key(header::AUTHORIZATION) {
        if let Some(api_key) = crate::config::get_runtime_config().api_key.as_ref() {
            if let Ok(v) = HeaderValue::from_str(&format!("Bearer {}", api_key)) {
                forward_headers.insert(header::AUTHORIZATION, v);
            }
        }
    }

    let req_method = reqwest::Method::from_bytes(request.method.as_str().as_bytes())
        .unwrap_or(reqwest::Method::GET);

    let cancellable = crate::http::client::CancellableRequest::new(context.client, cancellation_token.clone());

    let response = cancellable
        .make_raw_request(req_method, &target_url, forward_headers, Some(request.body.to_vec()))
        .await?;

    // Forward raw response back to the client (streaming-aware)
    let status = http::StatusCode::from_u16(response.status().as_u16())
        .map_err(|_| ProxyError::internal_server_error("invalid status code from OpenAI"))?;
    let headers = response.headers().clone();
    let stream = response.bytes_stream();

    let mapped_stream = stream.map(|item: Result<Bytes, _>| {
        item.map(warp::hyper::body::Frame::data)
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)
    });

    let body_impl = StreamBody::new(mapped_stream);
    let boxed_body = http_body_util::BodyExt::boxed(body_impl);

    let mut builder = WarpResponse::builder().status(status);
    for (name, value) in headers.iter() {
        if let (Ok(warp_name), Ok(warp_value)) = (
            header::HeaderName::from_bytes(name.as_str().as_bytes()),
            header::HeaderValue::from_bytes(value.as_bytes()),
        ) {
            builder = builder.header(warp_name, warp_value);
        }
    }

    let temp_response = builder
        .body(boxed_body)
        .map_err(|_| ProxyError::internal_server_error("failed to build passthrough response"))?;

    Ok(unsafe {
        std::mem::transmute::<
            http::Response<
                http_body_util::combinators::BoxBody<
                    Bytes,
                    Box<dyn std::error::Error + Send + Sync>,
                >,
            >,
            warp::reply::Response,
        >(temp_response)
    })
}
