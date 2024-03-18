use bindings::exports::example::coffeeshop::product_api::{Guest, ItemTypeModel};

use crate::bindings::exports::example::coffeeshop::product_api::ItemType;

#[allow(warnings)]
mod bindings;

impl From<i8> for ItemType {
    fn from(item: i8) -> Self {
        match item {
            0 => ItemType::Cappuccino,
            1 => ItemType::CoffeeBlack,
            2 => ItemType::CoffeeWithRoom,
            3 => ItemType::Espresso,
            4 => ItemType::EspressoDouble,
            5 => ItemType::Latte,
            6 => ItemType::Cakepop,
            7 => ItemType::Croissant,
            8 => ItemType::Muffin,
            9 => ItemType::CroissantChocolate,
            _ => ItemType::CroissantChocolate,
        }
    }
}

struct ProductCore;

impl Guest for ProductCore {
    fn get_item_types() -> Vec<bindings::exports::example::coffeeshop::product_api::ItemTypeModel> {
        get_item_types()
    }

    fn get_item_by_types(
        types: Vec<String>,
    ) -> Vec<bindings::exports::example::coffeeshop::product_api::ItemTypeModel> {
        let mut temp: Vec<ItemTypeModel> = Vec::new();

        for item in get_item_types() {
            let item_cloned = item.clone();
            for item_type in types.clone().into_iter() {
                let Ok(item_type) = item_type.parse::<i8>() else {
                    return temp;
                };

                if item_cloned.item_type == item_type.into() {
                    temp.push(item_cloned.clone())
                }
            }
        }

        temp
    }
}

fn get_item_types() -> Vec<ItemTypeModel> {
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

bindings::export!(ProductCore with_types_in bindings);
