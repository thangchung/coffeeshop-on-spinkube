use bindings::exports::coffeeshop::counter::api::{Guest, OrderDto, PlaceOrderCommand};

#[allow(warnings)]
mod bindings;

struct CounterCoreComponent;

impl Guest for CounterCoreComponent {
    fn get_fulfillment_order() -> Vec<OrderDto> {
        vec![]
    }
    
    fn place_order(_command: PlaceOrderCommand) -> bool {
        true
    }
}

bindings::export!(CounterCoreComponent with_types_in bindings);
