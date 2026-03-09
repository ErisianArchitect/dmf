use paste::paste;

macro_rules! align_structs {
    ($($num:literal),*$(,)?) => {
        $(
            paste! {
                #[repr(C, align($num))]
                #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
                pub struct [<Align $num>]<T>(T);
            }
        )*
    };
}

align_structs!(1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024);