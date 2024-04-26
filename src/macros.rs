macro_rules! cfg_compat {
    ($($item:item)*) => {
        $(
        #[cfg(feature = "compat")]
        #[cfg_attr(docsrs, doc(cfg(feature = "compat")))]
        $item
        )*
    };
}

pub(crate) use cfg_compat;
