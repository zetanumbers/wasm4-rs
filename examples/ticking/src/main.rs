#![no_main]

struct MyRuntime {
    count: i32,
}

// prints "tick..." every second
impl wasm4::Runtime for MyRuntime {
    fn start(_: wasm4::Resources) -> Self {
        MyRuntime { count: 0 }
    }

    fn update(&mut self) {
        if self.count % 60 == 0 {
            wasm4::trace("tick");
            self.count = 0;
        }
        self.count += 1;
    }
}

wasm4::main! { MyRuntime }
