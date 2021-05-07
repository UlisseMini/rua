# rua

Rust(subset) to lua transpiler


Differences are dealt with by adding libraries to the language with the missing features,
eg I might add iterators to lua so I can say
```lua
for i in range(1, 5) do end
```
Instead of
```lua
for i=1,5 do end
```

Ditto with coroutines, I might write something like
```rust
mod coroutine {
  fn create() { /* cursed threading stuff */ }
  // ...
}

fn main() {
  let coro = coroutine::create();
  // ...
}
```
Since I want to be able to write tests in rust.
the whole point of this project is to make writing large lua programs easier with static types,
and leveraging rust error messages, and testing/package infastructure.


