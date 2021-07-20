#[macro_export]
macro_rules! pub_struct {
    ($name:ident {$($field:ident: $t:ty,)*}) => {
        #[derive(Debug, Clone, PartialEq)] // ewww
        pub struct $name {
            $(pub $field: $t),*
        }
    }
}

#[macro_export]
macro_rules! _index_offset {
    ( $arr:expr, $offset:expr, $type:ty, $index:expr ) => {
        {
            (($arr[$offset + $index] as $type) << (8*($index)))
        }
    };
}

#[macro_export]
macro_rules! bytes_u16 {
    ( $arr:expr, $offset:expr ) => {
        {
            _index_offset!($arr, $offset, u16, 0) +
            _index_offset!($arr, $offset, u16, 1)
        }
    };
}

#[macro_export]
macro_rules! bytes_u32 {
    ( $arr:expr, $offset:expr ) => {
        {
            _index_offset!($arr, $offset, u32, 0) +
            _index_offset!($arr, $offset, u32, 1) +
            _index_offset!($arr, $offset, u32, 2) +
            _index_offset!($arr, $offset, u32, 3)
        }
    };
}

#[macro_export]
macro_rules! bytes_u64 {
    ( $arr:expr, $offset:expr ) => {
        {
            _index_offset!($arr, $offset, u64, 0) +
            _index_offset!($arr, $offset, u64, 1) +
            _index_offset!($arr, $offset, u64, 2) +
            _index_offset!($arr, $offset, u64, 3) +
            _index_offset!($arr, $offset, u64, 4) +
            _index_offset!($arr, $offset, u64, 5) +
            _index_offset!($arr, $offset, u64, 6) +
            _index_offset!($arr, $offset, u64, 7)
        }
    };
}
