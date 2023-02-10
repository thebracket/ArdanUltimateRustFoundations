# Async and Tokio

For CPU-bound problems, Rayon is great. Not every problem is CPU bound---and Rust can *also* do async/await, or "green threading". This model is best for when you are waiting on something---it could be waiting for a file to load, external services to give result, network traffic. You can even mix-and-match the two, but some caution is needed when you do.

"Green" threads aren't the same as threads:

| **Thread**                                | **Green-Thread**                       |
|-------------------------------------------|----------------------------------------|
| Managed by the OS scheduler               | Managed by your `async` runtime        |
| Expensive to create                       | Cheap to crate                         |
| Keep running while other threads are busy | Depend on the runtime to remain active |

> We're live-coding. See [here]() for the Github version.

