trait Product {
    fn consume(&self);
}

struct IDCard {
    owner: String
}

impl Product for IDCard {
    fn consume(&self) {
        println!("{}のカードを使います。", self.owner);
    }
}

impl IDCard {
    fn new(owner: String) {
        println!("{}のカードを作ります。", owner);
    }
}

impl Factory {
    fn create(owner: &str, create_product: fn(&str) -> dyn Product, register_product: fn(dyn Product)) {


    }
}

fn main() {
