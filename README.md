# fnum
Fnum is an utility trait for enum.

## Usage (require fnum_derive crate)
``` rust
use fnum::Fnum;

#[derive(fnum_derive::Fnum)]
enum MyEnum {
    A(u64),
    B(String),
    C(u64, u32, u32, u32),
    D {
        hello: u32,
        world: String,
    }
}

fn main() {
    dbg!(MyEnum::variant_count()); // => 4

    dbg!(MyEnum::A(0).variant_idx()); // => 0 (index of variant)

    dbg!(MyEnum::size_of_variant(0)); // => 16 (minimum bytes required by `MyEnum::A(..)`)
    dbg!(MyEnum::size_of_variant(1)); // => 32 (minimum bytes required by `MyEnum::B(..)`)
    dbg!(MyEnum::size_of_variant(2)); // => 24 (minimum bytes required by `MyEnum::C(..)`)
    dbg!(MyEnum::size_of_variant(3)); // => 32 (minimum bytes required by `MyEnum::D{..}`)
}
```

## Author

* carrotflakes (carrotflakes@gmail.com)

## Copyright

Copyright (c) 2020 carrotflakes (carrotflakes@gmail.com)

## License

Licensed under the MIT License.
