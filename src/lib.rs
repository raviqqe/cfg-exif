#![no_std]

#[macro_export]
macro_rules! feature {
    (if $name1:literal { $then1:expr } $(else if $name2:literal { $then2:expr })* else { $else:expr }) => {
        (|| {
            #[cfg(feature = $name1)]
            return $then1;
            $(
                #[cfg(feature = $name2)]
                return $then2;
            )*
            #[cfg(not(feature = $name1))]
            $(#[cfg(not(feature = $name2))])*
            return $else;
        })()
    };
}
