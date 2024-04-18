use anyhow::Result;
use bindings::coffeeshop::counter::api::{CommandType, Location, OrderSource, PlaceOrderCommand};
use bindings::coffeeshop::counter::shared::OrderItemLineDto;
use chrono::Utc;
use serde::Serialize;
use serde_json::json;
use spin_sdk::http::{self, send, IntoResponse, Params, Request, Response};
use spin_sdk::http_component;
use uuid::Uuid;

use crate::bindings::coffeeshop::counter::api::{get_fulfillment_order, place_order};

const DAPR_URL_ENV: &str = "DAPR_URL";

mod bindings;

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
struct SubscribeModel {
    pubsubname: String,
    topic: String,
    route: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct OrderPlaced {
    pub order_id: Uuid,
    pub item_lines: Vec<OrderItemLineDto>,
}

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
    println!("place_order_handler");

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

    let evt = OrderPlaced {
        order_id: Uuid::new_v4(),
        item_lines: Vec::<OrderItemLineDto>::new(),
    };

    dapr_publish_order_placed(evt);

    Ok(Response::builder()
        .status(200)
        .header("content-type", "application/json")
        .body(result)
        .build())
}

fn get_fulfillment_orders_handler(_req: Request, _params: Params) -> Result<impl IntoResponse> {
    println!("get_fulfillment_orders_handler");

    let result = json!(get_fulfillment_order()).to_string();

    Ok(Response::builder()
        .status(200)
        .header("content-type", "application/json")
        .body(result)
        .build())
}

fn dapr_publish_order_placed(evt: OrderPlaced) -> bool {
    let dapr_uri =
        std::env::var(DAPR_URL_ENV).unwrap_or_else(|_| "http://localhost:3500".to_string());

    println!("OrderPlaced event: {:?}", evt);

    let uri = format!(
        "{}/v1.0/publish/{}/{}",
        dapr_uri, "orderpubsub", "orderplaced"
    );
    println!("url: {}", uri);

    let body = json!(evt).to_string();

    _ = send::<_, http::Response>(
        http::Request::builder()
            .method(http::Method::Post)
            .uri(uri)
            .body(body)
            .build(),
    );

    true
}
