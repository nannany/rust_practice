trait Template {
    fn open();
    fn print();
    fn close();

    fn display() {
        Self::open();
        for _ in 0..5 {
            Self::print();
        }
        Self::close();
    }
}

struct ConcreteJa;

impl Template for ConcreteJa {
    fn open() {
        println!("開始");
    }

    fn print() {
        println!("こんにちは");
    }

    fn close() {
        println!("終了");
    }
}
struct ConcreteEn;

impl Template for ConcreteEn {
    fn open() {
        println!("start");
    }

    fn print() {
        println!("hello");
    }

    fn close() {
        println!("end");
    }
}


fn main() {
    ConcreteJa::display();
    ConcreteEn::display();
}
