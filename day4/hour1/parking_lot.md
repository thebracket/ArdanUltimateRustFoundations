# Parking Lot

I'm mostly bringing this one up to help with repetitive strain injury.

Whenever you've used `Mutex` or `RwLock`, you've had to handle errors from the lock. It's unlikely that these errors are recoverable, since the most common errors are a dead-lock!

If you include the `parking_lot` crate, it includes versions of `Mutex` and `RwLock` that don't require the `unwrap`. They otherwise function very similarly. On some platforms (Windows), `parking_lot` is a touch faster than the standard library.

