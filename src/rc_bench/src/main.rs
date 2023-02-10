// Store cats in a simple vector and index by vector position.
// VERY fast, has the downside that you can't insert or delete cats
// without ruining every index
mod cat_vec;

// Store cats in a HashMap, giving each cat a stable ID number.
// Slower.
mod cat_store;

// Store each cat as a reference counted pointer, and hand a
// reference to the cat to each owner. Simpler, there's no
// cat store at all.
mod rc_cat;

// Reference counted cats, but using thread-safe reference
// counting.
mod atomic_rc_cat;

const NUMBER_OF_CATS: usize = 10_000_000;

fn print_result(method: &str, time: std::time::Duration) {
    let usecs = format!("{} μsecs", time.as_micros());
    let nanos_per_cat = format!("{} nanos per cat", time.as_nanos() as usize / NUMBER_OF_CATS);
    println!("{method:<30}{usecs:<20}{nanos_per_cat:<20}");
}

fn main() {
    cat_vec::feed_cats_by_id(NUMBER_OF_CATS);
    rc_cat::feed_cats(NUMBER_OF_CATS);
    atomic_rc_cat::feed_cats(NUMBER_OF_CATS);
    cat_store::feed_cats_by_id(NUMBER_OF_CATS);
}
