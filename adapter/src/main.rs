struct Old {
    strings: String
}

impl Old {
    fn show_str(&self) {
        println!("{}", self.strings);
    }
}

struct Adapter {
    old: Old
}

impl Adapter {
    fn new() -> Self {
        Adapter {
            old: Old { strings: "old method".to_string() }
        }
    }
    fn show(&self) {
        self.old.show_str();
    }
}

fn main() {
    let adapter = Adapter::new();
    adapter.show();
}


