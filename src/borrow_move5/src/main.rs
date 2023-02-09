#[derive(Default)]
struct MyBuilder {
    a: bool,
}

impl MyBuilder {
    fn with(mut self, a: bool) -> Self {
        self.a = a;
        self
    }

    fn build() -> Self {
        Self::default().with(true)
    }
}

fn main() {
    let _x = MyBuilder::build();
}
