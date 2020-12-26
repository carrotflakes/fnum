use fnum::Fnum;

#[derive(fnum_derive::Fnum)]
enum MyEnum {
    A(u64),
    B(String),
    C(u64, u32, u32, u32),
    D {
        hello: u32,
        world: String,
    },
    E
}

fn main() {
    unsafe {
        dbg!(std::mem::transmute::<MyEnum, [u32;8]>(MyEnum::A(123)))
    };
    unsafe {
        dbg!(std::mem::transmute::<MyEnum, [u32;8]>(MyEnum::B("hello".to_string())))
    };
    unsafe {
        dbg!(std::mem::transmute::<MyEnum, [u32;8]>(MyEnum::C(123, 5, 6, 7)))
    };
    unsafe {
        dbg!(std::mem::transmute::<MyEnum, [u32;8]>(MyEnum::E))
    };

    // dbg!(Enum::A(1).variant_idx());
    // dbg!(Enum::B("hello".to_string()).variant_idx());
    dbg!(MyEnum::variant_count()); // => 4
    dbg!(MyEnum::A(123).variant_idx()); // => 0 (index of variant)
    dbg!(MyEnum::size_of_variant(0));
    dbg!(MyEnum::size_of_variant(1));
    dbg!(MyEnum::size_of_variant(2));
    dbg!(MyEnum::size_of_variant(3));
    dbg!(MyEnum::size_of_variant(4));
    dbg!("done:)");
}
