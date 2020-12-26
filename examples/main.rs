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

    fn variant_idx(&self) -> usize {
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
        pub fn pointer<T>(t: &T) -> usize {
            t as *const _ as usize
        }
        pub fn right_pointer<T>(t: &T) -> usize {
            unsafe {(t as *const T).offset(1) as usize}
        }
        let e = unsafe { Self::uninit_variant(idx) };
        let size = match &e {
            Enum::A(x) => [right_pointer(x)].iter().max().unwrap() - pointer(&e),
            Enum::B(x) => [right_pointer(x)].iter().max().unwrap() - pointer(&e),
            Enum::C(x, y) => [right_pointer(x), right_pointer(y)].iter().max().unwrap() - pointer(&e),
            Enum::D(x, y) => [right_pointer(x), right_pointer(y)].iter().max().unwrap() - pointer(&e),
        };
        std::mem::forget(e);
        size
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
