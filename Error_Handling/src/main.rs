pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        // 这是“契约”：值必须在 1-100 之间
        if value < 1 || value > 100 {
            // 如果违反契约，直接 panic!，因为这是调用者的错
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }
        Guess { value }
    }

    // 提供一个 getter 来获取值，但不允许外部直接修改
    pub fn value(&self) -> i32 {
        self.value
    }
}

fn main() {
    let guess = Guess::new(50);
    // let bad_guess = Guess::new(150); // 这会触发 panic
    println!("Guess value: {}", guess.value());
}
