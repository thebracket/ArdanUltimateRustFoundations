# Lifetimes

We're looking at lifetimes and the borrow checker together, because they are inextricably linked. The borrow checker prevents use-after-move, enforces ownership, and ensures that you don't have even the faintest possibility of changing the same piece of data from two places at a time.

Lifetimes attack another very common bug: using a pointer after the item being pointed to is no longer available (it moved, was freed, etc.).

