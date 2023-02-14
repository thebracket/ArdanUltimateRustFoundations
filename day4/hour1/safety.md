# Rust Safety Guarantees

Just to finish off, I'd like to remind you what is and isn't made safe by Rust.

## Memory Safety

Rust's memory safety guarantees specifically ensure that you won't have use-after free, buffer overrun, dangling pointer, or iterator invalidation. These account for a LOT of security vulnerabilities.

Rust specifically does NOT guarantee that you won't experience a memory leak. It even includes a call (`std::mem::forget`) to *explicitly* leak memory! With that said, Rust does make it tough to leak memory---the defaults are very safe, and drop-handling/RAII does a great job of largely removing the problem.

## Thread Safety

Rust prevents data-races, a very common form of error in multi-threaded systems. 

Rust does *not* prevent you from creating a deadlock by misusing locks (such as Mutex). The RAII scope-bound locks make it easier to avoid this pitfall, but you still have to be careful.

## Type Safety

Rust does a great job of enforcing type-safety, but you have to elect to use it (with `NewTypes` and similar).

## The "Unsafe" Tag

Marking a block of code `unsafe` does *not* mean that it will explode. It means that it cannot be proved to Rust's static analyzer as safe. You'll always need `unsafe` when you talk directly to hardware, call programs through FFI (since *they* can't offer Rust guarantees).

## Audits

If you use Rust in production, do yourself a favor: run `cargo install cargo-audit` and periodically run `cargo audit`. (Github can do this for you). It will find any CVE reports for crates that you are using.
