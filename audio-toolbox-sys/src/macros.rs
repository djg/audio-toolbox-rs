macro_rules! cs {
    ($($name:ident = $val:expr);+;) => {
        $(
            pub const $name: &'static str = concat!($val, "\0");
        )+
    };
    ($(#define $name:ident $val:expr)+) => {
        $(
            pub const $name: &'static str = concat!($val, "\0");
        )+
    }
}

macro_rules! e {
    (CF_ENUM($type:ident) { $($variants:tt)* };) => {
        e!(__gen__ $type, 0, $($variants)*);
    };
    (typedef CF_OPTIONS($type:ident, $name:ident) { $($variants:tt)* };) => {
        pub type $name = $type;
        e!(__gen__ $type, 0, $($variants)*);
    };
    (typedef CF_ENUM($type:ident, $name:ident) { $($variants:tt)* };) => {
        pub type $name = $type;
        e!(__gen__ $type, 0, $($variants)*);
    };
    (enum { $($variants:tt)* };) => {
        e!(__gen__ u32, 0, $($variants)*);
    };
    (enum $name:ident: $type:ident { $($variants:tt)* }) => {
        pub type $name = $type;
        e!(__gen__ $type, 0, $($variants)*);
    };
    (__gen__ $type:ident, $val:expr, $variant:ident, $($rest:tt)*) => {
        pub const $variant: $type = $val;
        e!(__gen__ $type, $val+1, $($rest)*);
    };
    (__gen__ $type:ident, $val:expr, $variant:ident = $e:expr, $($rest:tt)*) => {
        pub const $variant: $type = $e;
        e!(__gen__ $type, $e+1, $($rest)*);
    };
    (__gen__ $type:ident, $val:expr, $variant:ident = $e:expr) => {
        pub const $variant: $type = $e;
    };
    (__gen__ $type:ident, $val:expr, ) => {};
    (__gen__ $type:ident, $val:expr) => {}
}

macro_rules! s {
    ($($(#[$attr:meta])* struct $i:ident { $($field:tt)* })*) => ($(
        __item! {
            #[repr(C)]
            #[derive(Debug)]
            $(#[$attr])*
            pub struct $i { $($field)* }
        }
        impl Default for $i {
            fn default() -> Self {
                unsafe { mem::zeroed() }
            }
        }
    )*)
}

macro_rules! cfg_if {
    ($(
        if #[cfg($($meta:meta),*)] { $($it:item)* }
    ) else * else {
        $($it2:item)*
    }) => {
        __cfg_if_items! {
            () ;
            $( ( ($($meta),*) ($($it)*) ), )*
            ( () ($($it2)*) ),
        }
    }
}

// Implementation

macro_rules! __cfg_if_items {
    (($($not:meta,)*) ; ) => {};
    (($($not:meta,)*) ; ( ($($m:meta),*) ($($it:item)*) ), $($rest:tt)*) => {
        __cfg_if_apply! { cfg(all(not(any($($not),*)), $($m,)*)), $($it)* }
        __cfg_if_items! { ($($not,)* $($m,)*) ; $($rest)* }
    }
}

macro_rules! __cfg_if_apply {
    ($m:meta, $($it:item)*) => {
        $(#[$m] $it)*
    }
}

macro_rules! __item {
    ($i:item) => ($i)
}
