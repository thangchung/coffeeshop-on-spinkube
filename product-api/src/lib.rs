use anyhow::Result;
use bindings::example::coffeeshop::product_api::{get_item_by_types, get_item_types};
use serde_json::json;
use spin_sdk::http::{IntoResponse, Params, Request, Response};
use spin_sdk::http_component;

mod bindings;

#[http_component]
fn handle_product_api(req: Request) -> Response {
    let mut router = spin_sdk::http::Router::default();
    router.get("/", health_handler);
    router.get("/v1-get-item-types", get_item_types_handler);
    router.get("/v1-get-items-by-types/:types", get_item_by_types_handler);
    router.handle(req)
}

fn health_handler(_req: Request, _params: Params) -> Result<impl IntoResponse> {
    Ok(Response::builder().status(200).body("").build())
}

fn get_item_types_handler(_req: Request, _params: Params) -> Result<impl IntoResponse> {
    println!("start get_item_types_handler");

    let result = json!(get_item_types()).to_string();

    Ok(Response::builder()
        .status(200)
        .header("content-type", "application/json")
        .body(result)
        .build())
}

fn get_item_by_types_handler(_req: Request, params: Params) -> Result<impl IntoResponse> {
    println!("start get_item_by_types_handler");

    let Some(types) = params.get("types") else {
        return Ok(Response::builder()
            .status(404)
            .body("Missing types")
            .build());
    };

    let type_arr: Vec<String> = types.split(",").map(|s| s.parse().unwrap()).collect();
    println!("filter params: {0}", json!(type_arr).to_string());

    let result = json!(get_item_by_types(type_arr.as_slice())).to_string();

    Ok(Response::builder()
        .status(200)
        .header("Content-Type", "application/json")
        .body(result)
        .build())
}
