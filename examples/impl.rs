use fnum::Fnum;

enum Enum {
    A(u64),
    B(u32),
    C(u32, u64),
    D(u64, u32),
}

impl Fnum for Enum {
    fn variant_count() -> usize {
        4
    }

    fn variant_index(&self) -> usize {
        match self {
            Enum::A(_) => 0,
            Enum::B(_) => 1,
            Enum::C(_, _) => 2,
            Enum::D(_, _) => 4,
        }
    }

    unsafe fn uninit_variant(idx: usize) -> Self {
        assert!(idx < Self::variant_count());
        match idx {
            0 => Enum::A(std::mem::MaybeUninit::uninit().assume_init()),
            1 => Enum::B(std::mem::MaybeUninit::uninit().assume_init()),
            2 => Enum::C(std::mem::MaybeUninit::uninit().assume_init(), std::mem::MaybeUninit::uninit().assume_init()),
            3 => Enum::D(std::mem::MaybeUninit::uninit().assume_init(), std::mem::MaybeUninit::uninit().assume_init()),
            _ => unreachable!(),
        }
    }

    fn size_of_variant(idx: usize) -> usize {
        use once_cell::sync::Lazy;
        static TABLE: Lazy<[usize; 4]> = Lazy::new(|| {
            pub fn pointer<T>(t: &T) -> usize {
                t as *const _ as usize
            }
            pub fn right_pointer<T>(t: &T) -> usize {
                unsafe {(t as *const T).offset(1) as usize}
            }
            [
                {
                    let e = unsafe { Enum::uninit_variant(0) };
                    let size = if let Enum::A(x) = &e {
                        [right_pointer(x)].iter().max().unwrap() - pointer(&e)
                    } else {unreachable!()};
                    std::mem::forget(e);
                    size
                },
                {
                    let e = unsafe { Enum::uninit_variant(1) };
                    let size = if let Enum::B(x) = &e {
                        [right_pointer(x)].iter().max().unwrap() - pointer(&e)
                    } else {unreachable!()};
                    std::mem::forget(e);
                    size
                },
                {
                    let e = unsafe { Enum::uninit_variant(2) };
                    let size = if let Enum::C(x, y) = &e {
                        [right_pointer(x), right_pointer(y)].iter().max().unwrap() - pointer(&e)
                    } else {unreachable!()};
                    std::mem::forget(e);
                    size
                },
                {
                    let e = unsafe { Enum::uninit_variant(3) };
                    let size = if let Enum::D(x, y) = &e {
                        [right_pointer(x), right_pointer(y)].iter().max().unwrap() - pointer(&e)
                    } else {unreachable!()};
                    std::mem::forget(e);
                    size
                },
            ]
        });
        (*TABLE)[idx]
    }
}

fn main() {
    dbg!(Enum::size_of_variant(0), Enum::size_of_variant(1), Enum::size_of_variant(2), Enum::size_of_variant(3));
    dbg!(std::mem::size_of::<Enum>());
    unsafe {
        dbg!(std::mem::transmute::<Enum, [u32;4]>(Enum::B(123)))
    };
    let aa = Enum::A(1);
    if let Enum::A(ref x) = aa {
        dbg!(pointer_distance(x, &aa));
    }
    let bb = Enum::B(1);
    if let Enum::B(ref x) = bb {
        dbg!(pointer_distance(x, &bb));
    }
}

fn pointer_distance<T, S>(t: &T, s: &S) -> usize {
    (t as * const _ as usize) - (s as * const _ as usize)
}
