use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Request {
    Order {
        recipe_name: String,
    },
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Response {
    ProductionError {
        order_id: Option<String>,
        error: String,
    },
}