macro_rules! really_push {
    ($target: expr, $val: expr) => {
        $target.push($val);
    };
}

#[macro_export]
macro_rules! push {
    ($target: expr, $($val: expr),+) => {
        $(
            really_push!($target, $val);
        )+
    };
}

fn main() {
    let mut vec = Vec::new();
    push!(vec, 1, 2, 3, 4);
    println!("{:?}", vec);
}
