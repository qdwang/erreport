#![cfg_attr(docsrs, feature(doc_auto_cfg))]
//! erreport
//! ---
//! A `Result` helper to catch all the `Err` propagation path
//! 
//! ### Why?
//! When an `Err` propagated, we need a method catch all the propagation path.
//! 
//! So we can get this report in `release` mode with `debug=0`:
//! ```
//! {package0@version0} src/lib.rs:56 -> src/xx.rs:23 -> {package1@version1} src/lib.rs:42 -> src/yy.rs:632 -> src/zz.rs:56  
//!   -> {package2@version2} src/lib.rs:251 -> InvalidXXX
//! ```
//! 
//! #### Pros over `Backtrace`
//! * No `RUST_BACKTRACE=1` needed.
//! * No `debug=1` needed.
//! * Packages and their versions are recorded.
//!
//! ### How to use 
//! ```rust
//! // This will generate a trait called `pub(crate) trait ToReport<T>` to help to convert any `Result<T, E: std::error::Error>` to `Report`.
//! // You just need to call once for each crate.
//! erreport::gen_trait_to_report!(); 
//! 
//! fn test() -> Result<(), erreport::Report> {
//!     any_result_impl_std_error_Error.to_report()?;
//!     any_result_impl_std_error_Error.to_report()?;
//!     Ok(())
//! }
//! ```
//! 
//! ### How to access the actual Error?
//! ```rust
//! fn main() {
//!     if let Err(err) = test() {
//!         // This method will bypass all the `Report` wrappers and get the first actual `Error` value.
//!         err.source() 
//!     }
//! }
//! ```


use std::error::Error;

/// You can use `.source()` to get the first real source in Report
pub struct Report {
    pub pkg_name: &'static str,
    pub pkg_version: &'static str,
    pub file: &'static str,
    pub line: u32,
    pub err: Box<dyn Error>,
}

impl std::fmt::Debug for Report {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string(true, 0))
    }
}
impl std::fmt::Display for Report {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string(false, 0))
    }
}
impl Error for Report {
    /// This method will ignore the report stack and get the first real source
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self.err.downcast_ref::<Report>() {
            Some(e) => e.source(),
            None => Some(self.err.as_ref()),
        }
    }
}

impl Report {
    fn to_string(&self, is_debug: bool, index: u16) -> String {
        let err_str = self
            .err
            .downcast_ref::<Self>()
            .and_then(|report| {
                if report.pkg_name == self.pkg_name && report.pkg_version == self.pkg_version {
                    Some(report.to_string(is_debug, index + 1))
                } else {
                    None
                }
            })
            .unwrap_or(if is_debug {
                format!("{:?}", self.err)
            } else {
                self.err.to_string()
            });

        match index {
            0 => {
                format!(
                    "{{{}@{}}} {}:{} -> {}",
                    self.pkg_name, self.pkg_version, self.file, self.line, err_str
                )
            }
            _ => {
                format!("{}:{} -> {}", self.file, self.line, err_str)
            }
        }
    }
}

/// This will generate a trait called `pub(crate) trait ToReport<T>` to help to convert any `Result<T, E: std::error::Error>` to `Report`.
#[macro_export]
macro_rules! gen_trait_to_report {
    () => {
        pub(crate) trait ToReport<T> {
            fn to_report(self) -> Result<T, erreport::Report>;
        }

        impl<T, E: std::error::Error + 'static> ToReport<T> for Result<T, E> {
            #[track_caller]
            fn to_report(self) -> Result<T, erreport::Report> {
                match self {
                    Ok(t) => Ok(t),
                    Err(err) => {
                        let loc = core::panic::Location::caller();
                        Err(erreport::Report {
                            pkg_name: env!("CARGO_PKG_NAME"),
                            pkg_version: env!("CARGO_PKG_VERSION"),
                            file: loc
                                .file()
                                .get(env!("CARGO_MANIFEST_DIR").len() + 1..)
                                .unwrap_or(loc.file()),
                            line: loc.line(),
                            err: err.into(),
                        })
                    }
                }
            }
        }
    };
}
