use anyhow::Result;
use bindings::coffeeshop::counter::api::{get_fulfillment_order, place_order, CommandType, Location, OrderSource, PlaceOrderCommand};
use chrono::Utc;
use serde_json::json;
use spin_sdk::http::{IntoResponse, Params, Request, Response};
use spin_sdk::http_component;
use uuid::Uuid;

mod bindings;

#[http_component]
fn handle_counter_api(req: Request) -> Response {
    let mut router = spin_sdk::http::Router::default();
    router.get("/", health_handler);
    router.post("/v1-place-order", place_order_handler);
    router.get("/v1-get-fulfillment-orders", get_fulfillment_orders_handler);
    router.handle(req)
}

fn health_handler(_req: Request, _params: Params) -> Result<impl IntoResponse> {
    Ok(Response::builder().status(200).body("").build())
}

fn place_order_handler(_req: Request, _params: Params) -> Result<impl IntoResponse> {
    let command = PlaceOrderCommand {
        id: Uuid::new_v4().to_string(),
        command_type: CommandType::PlaceOrder,
        location: Location::Hochiminh,
        order_source: OrderSource::Web,
        loyalty_member_id: Uuid::new_v4().to_string(),
        timestamp: Utc::now().timestamp() as u64,
        barista_items: Some(vec![]),
        kitchen_items: Some(vec![]),
    };
    
    let result = json!(place_order(&command)).to_string();

    Ok(Response::builder()
        .status(200)
        .header("content-type", "application/json")
        .body(result)
        .build())
}

fn get_fulfillment_orders_handler(_req: Request, _params: Params) -> Result<impl IntoResponse> {
    let result = json!(get_fulfillment_order()).to_string();

    Ok(Response::builder()
        .status(200)
        .header("content-type", "application/json")
        .body(result)
        .build())
}
