use crate::protocol::Request;

pub fn process_request(req: Request, recipes: &Vec<String>) -> String {
    match req {
        Request::Order { recipe_name } => {

            if recipes.contains(&recipe_name) {
                let mut pizza = String::new();
                pizza.push_str("Dough prepared\n");
                pizza.push_str("Base added\n");
                pizza.push_str("Cheese added\n");
                pizza.push_str("Baked\n");
                pizza
            } else {
                println!("🔄 Forwarding request...");
                "Forwarded to another agent".to_string()
            }
        }
    }
}