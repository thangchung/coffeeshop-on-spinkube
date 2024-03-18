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
    let temp = get_item_types()
        .into_iter()
        .map(|s| json!(s).to_string())
        .reduce(|acc, e| format!("{} {}", acc, e))
        .unwrap_or(String::from(""));

    // println!("{0}", temp);
    let result = json!(temp).to_string();

    Ok(Response::builder()
        .status(200)
        .header("Content-Type", "application/json")
        .body(result)
        .build())
}

fn get_item_by_types_handler(_req: Request, params: Params) -> Result<impl IntoResponse> {
    let Some(types) = params.get("types") else {
        return Ok(Response::builder()
            .status(404)
            .body("Missing types")
            .build());
    };

    let type_arr: Vec<String> = types.split(",").map(|s| s.parse().unwrap()).collect();

    let temp = get_item_by_types(type_arr.as_slice())
        .into_iter()
        .map(|s| json!(s).to_string())
        .reduce(|acc, e| format!("{} {}", acc, e))
        .unwrap_or(String::from(""));

    // println!("{0}", temp);

    let result = json!(temp).to_string();

    Ok(Response::builder()
        .status(200)
        .header("Content-Type", "application/json")
        .body(result)
        .build())
}
