# The Borrow Checker and Lifetimes

Probably the most common thing that I hear from people migrating to Rust is:

*I tried really hard to like it, but the borrow checker made it impossible to do anything.*

So we're going to spend this hour going over some of the more common difficulties, and wrap up with some general advice on how to learn to love the borrow checker. It may be Stockholm Syndrome, but the borrow checker really does help.

Later on, we'll see examples of the borrow checker prevents you from making terrible mistakes. 

> Google and Microsoft both report a 70% reduction in memory-related errors in their code after moving parts of their codebase to Rust.