# Delay and Timeout using Monotonics

A convenient way to express miniminal timing requirements is by delaying progression.

This can be achieved by instantiating a monotonic timer (for implementations, see [`rtic-monotonics`]):

[`rtic-monotonics`]: https://github.com/rtic-rs/rtic/tree/master/rtic-monotonics
[`rtic-time`]: https://github.com/rtic-rs/rtic/tree/master/rtic-time
[`Monotonic`]: https://docs.rs/rtic-time/latest/rtic_time/trait.Monotonic.html
[Implementing a `Monotonic`]: ../monotonic_impl.md

```rust,noplayground
...
{{#include ../../../../examples/lm3s6965/examples/async-timeout.rs:init}}
        ...
```

A _software_ task can `await` the delay to expire:

```rust,noplayground
#[task]
async fn foo(_cx: foo::Context) {
    ...
    Mono::delay(100.millis()).await;
    ...
}

```

<details>
<summary>A complete example</summary>

```rust,noplayground
{{#include ../../../../examples/lm3s6965/examples/async-delay.rs}}
```

```console
$ cargo xtask qemu --verbose --example async-delay --features test-critical-section
```

```console
{{#include ../../../../ci/expected/lm3s6965/async-delay.run}}
```

</details>

> Interested in contributing new implementations of [`Monotonic`], or more information about the inner workings of monotonics?
> Check out the [Implementing a `Monotonic`] chapter!

## Timeout

Rust [`Future`]s (underlying Rust `async`/`await`) are composable. This makes it possible to `select` in between `Futures` that have completed.

[`Future`]: https://doc.rust-lang.org/std/future/trait.Future.html

A common use case is transactions with an associated timeout. In the examples shown below, we introduce a fake HAL device that performs some imagined transaction when you call `hal_get(n).await`. We have modelled the time it takes based on the input parameter (`n`) as `350ms + n * 100ms`.

Using the `select_biased` macro from the `futures` crate it may look like this:

```rust,noplayground,noplayground
{{#include ../../../../examples/lm3s6965/examples/async-timeout.rs:select_biased}}
```

Assuming the `hal_get` will take 450ms to finish, a short timeout of 200ms will expire before `hal_get` can complete.

Extending the timeout to 1000ms would cause `hal_get` will to complete first.

Using `select_biased` any number of futures can be combined, so its very powerful. However, as the timeout pattern is frequently used, more ergonomic support is baked into RTIC, provided by the [`rtic-monotonics`] and [`rtic-time`] crates. Here's another example, using `Mono::delay_until` and `Mono::timeout_after`:

```rust,noplayground
{{#include ../../../../examples/lm3s6965/examples/async-timeout.rs:timeout_at_basic}}
```

In cases where you want exact control over time without drift we can use exact points in time using `Instant`, and spans of time using `Duration`. Operations on the `Instant` and `Duration` types come from the [`fugit`] crate.

[`fugit`]: https://crates.io/crates/fugit

`let mut instant = Mono::now()` sets the starting time of execution.

We want to call `hal_get` every 1000ms relative to this starting time. We accomplish this by incrementing our `instant` by 1000 ms and then using `Mono::delay_until(instant).await`. Any additional delays incurred as we iterate around this loop are compensated for by delaying until 'previous + 1000' as opposed to 'now + 1000' (which would cause our loop timing to drift).

To show an alternative to the `select!` async timeout example above, we define a future point in time as `timeout`, and call `Mono::timeout_at(timeout, hal_get(n)).await`.

For the first iteration of the loop, with `n == 0`, the `hal_get` will take 350ms (as described above), and finishes before the timeout. For the second iteration, the delay is 450ms, which still finishes before the timeout. For the third iteration, with `n == 2`, `hal_get` will take 550ms to finish, in which case we will run into a timeout.

<details>
<summary>A complete example</summary>

```rust,noplayground
{{#include ../../../../examples/lm3s6965/examples/async-timeout.rs}}
```

```console
$ cargo xtask qemu --verbose --example async-timeout --features test-critical-section
```

```console
{{#include ../../../../ci/expected/lm3s6965/async-timeout.run}}
```

</details>
