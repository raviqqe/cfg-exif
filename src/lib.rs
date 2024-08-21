#![doc = include_str!("../README.md")]
#![no_std]

/// Compiles expressions conditionally on features.
///
/// # Examples
///
/// ```rust
/// assert_eq!(
///     cfg_exif::feature!(if "foo" {
///         13
///     } else if "bar" {
///         13
///     } else {
///         42
///     }),
///     42
/// );
/// ```
#[macro_export]
macro_rules! feature {
    (if $name1:literal { $then1:expr } $(else if $name2:literal { $then2:expr })* else { $else:expr }) => {
        {
            #[cfg(feature = $name1)]
            { $then1 }
            $(
                #[cfg(feature = $name2)]
                { $then2 }
            )*
            #[cfg(not(feature = $name1))]
            $(#[cfg(not(feature = $name2))])*
            { $else }
        }
    };
}

/// Compiles expressions conditionally on compile configurations.
///
/// # Examples
///
/// ```rust
/// # use cfg_exif::cfg;
/// # fn main() {
/// assert_eq!(
///     cfg!(if (feature == "foo") {
///         0
///     } else if (not(target_os == "linux")) {
///         1
///     } else {
///         42
///     }),
///     42
/// );
/// # }
/// ```
#[macro_export]
macro_rules! cfg {
    (if ($key:ident == $value:literal) { $then1:expr } else $(if $condition:tt { $then2:expr } else)* { $else:expr }) => {{
        #[cfg($key = $value)]
        {
            $then1
        }
        #[cfg(not($key = $value))]
        {
            cfg!($(if $condition { $then2 } else)* { $else })
        }
    }};
    (if (not($key:ident == $value:literal)) { $then1:expr } else $(if $condition:tt { $then2:expr } else)* { $else:expr }) => {{
        #[cfg(not($key = $value))]
        {
            $then1
        }
        #[cfg($key = $value)]
        {
            cfg!($(if $condition { $then2 } else)* { $else })
        }
    }};
    ({ $else:expr }) => {{
        {
            $else
        }
    }};
}
