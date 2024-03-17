wit_bindgen::generate!({ world: "hostapp" });

use http::Response;
use anyhow::Result;
use bytes::Bytes;
use serde::{Serialize, Deserialize};
use spin_sdk::http::{IntoResponse, Params, Request};
use spin_sdk::http_component;

use crate::example::coffeeshop::api::{ItemType, ItemTypeModel};

#[derive(Debug, Deserialize)]
struct GetItemByTypeModel{
    types: String,
}

// impl TryFrom<&Option<Bytes>> for GetItemByTypeModel {
//     type Error = anyhow::Error;

//     fn try_from(value: &Option<Bytes>) -> std::result::Result<Self, Self::Error> {
//         match value {
//             Some(b) => Ok(serde_json::from_slice::<GetItemByTypeModel>(b)?),
//             None => Err(anyhow::anyhow!("No body")),
//         }
//     }
// }

/// A simple Spin HTTP component.
#[http_component]
fn handle_product_api(req: Request) -> Result<impl IntoResponse> {
    // println!("{:?}", req.headers());
    let mut router = spin_sdk::http::Router::default();
    router.get("/", health_handler);
    router.get("/v1-get-item-types", get_item_types_handler);
    router.get("/v1-get-items-by-types/:types", get_item_by_types_handler);
    Ok(router.handle(req))
}

fn health_handler(_req: Request, _params: Params) -> Result<impl IntoResponse> {
    Ok(Response::builder().status(200).body("".into())?)
}

fn get_item_types_handler(_req: Request, _params: Params) -> Result<impl IntoResponse> {
    Ok(Response::new(200, Some("")))
}

fn get_item_by_types_handler(req: Request, params: Params) -> Result<impl IntoResponse> {
    // let a = req.body().clone();
    // let Ok(model) = GetItemByTypeModel::try_from(&req.body().clone()) else {
    //     // return Ok(http::Response::builder()
    //     // .status(http::StatusCode::BAD_REQUEST)
    //     // .body(None)?);

    //     return Ok(Response::new(http::StatusCode::BAD_REQUEST, None))
    // };

    let Some(types) = params.get("types") else {
        return Ok(Response::builder().status(404).body("Missing types")?);
    };

    Ok(Response::new(200, Some("")))
}

fn get_item_types() -> Vec<example::coffeeshop::api::ItemTypeModel> {
    vec![
        ItemTypeModel {
            name: "CAPPUCCINO".to_string(),
            item_type: ItemType::Cappuccino,
            price: 4.5,
            image: "img/CAPPUCCINO.png".to_string(),
        },
        ItemTypeModel {
            name: "COFFEE_BLACK".to_string(),
            item_type: ItemType::CoffeeBlack,
            price: 3.0,
            image: "img/COFFEE_BLACK.png".to_string(),
        },
        ItemTypeModel {
            name: "COFFEE_WITH_ROOM".to_string(),
            item_type: ItemType::CoffeeWithRoom,
            price: 3.0,
            image: "img/COFFEE_WITH_ROOM.png".to_string(),
        },
        ItemTypeModel {
            name: "ESPRESSO".to_string(),
            item_type: ItemType::Espresso,
            price: 3.5,
            image: "img/ESPRESSO.png".to_string(),
        },
        ItemTypeModel {
            name: "ESPRESSO_DOUBLE".to_string(),
            item_type: ItemType::EspressoDouble,
            price: 4.5,
            image: "img/ESPRESSO_DOUBLE.png".to_string(),
        },
        ItemTypeModel {
            name: "LATTE".to_string(),
            item_type: ItemType::Latte,
            price: 4.5,
            image: "img/LATTE.png".to_string(),
        },
        ItemTypeModel {
            name: "CAKEPOP".to_string(),
            item_type: ItemType::Cakepop,
            price: 2.5,
            image: "img/CAKEPOP.png".to_string(),
        },
        ItemTypeModel {
            name: "CROISSANT".to_string(),
            item_type: ItemType::Croissant,
            price: 3.25,
            image: "img/CROISSANT.png".to_string(),
        },
        ItemTypeModel {
            name: "MUFFIN".to_string(),
            item_type: ItemType::Muffin,
            price: 3.0,
            image: "img/MUFFIN.png".to_string(),
        },
        ItemTypeModel {
            name: "CROISSANT_CHOCOLATE".to_string(),
            item_type: ItemType::CroissantChocolate,
            price: 3.5,
            image: "img/CROISSANT_CHOCOLATE.png".to_string(),
        },
    ]
}
