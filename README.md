# fnum
Fnum is an utility trait for enum, that provides `size_of_variant` method.
`size_of_variant` returns the minimum size that specified variant requiring.

## Usage
``` rust
use fnum::Fnum;

#[derive(Fnum)]
enum MyEnum {
    A(u64),
    B(String),
    C(u64, u32, u32, u32),
    D {
        foo: u32,
        bar: String,
    }
}

fn main() {
    dbg!(MyEnum::size_of_variant(0)); // => 16 (bytes required by `MyEnum::A(..)`)
    dbg!(MyEnum::size_of_variant(1)); // => 32 (bytes required by `MyEnum::B(..)`)
    dbg!(MyEnum::size_of_variant(2)); // => 24 (bytes required by `MyEnum::C(..)`)
    dbg!(MyEnum::size_of_variant(3)); // => 32 (bytes required by `MyEnum::D{..}`)
}
```

## Author

* carrotflakes (carrotflakes@gmail.com)

## Copyright

Copyright (c) 2020 carrotflakes (carrotflakes@gmail.com)

## License

Licensed under the MIT License.
