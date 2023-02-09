#[derive(Default, Clone)]
struct MyBuilder {
    a: bool,
}

impl MyBuilder {
    fn with(&mut self, a: bool) -> &mut Self {
        self.a = a;
        self
    }

    fn build() -> &'static mut Self {
        Self::default().with(true)
    }
}

fn main() {
    let x = MyBuilder::build();
}
