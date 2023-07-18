#![cfg_attr(docsrs, feature(doc_auto_cfg))]
//! A `Result` helper for Rust
//! 
//! ## How to use
//! ```rust
//! erreport::gen_report_code!();
//! use report::{Report, ToReport};
//! ```

/// This will create a mod called `report` in current scope.
#[macro_export]
macro_rules! gen_report_code {
    () => {
        pub(crate) mod report {
            use std::error::Error;

            const PKG_DIR_LEN: usize = env!("CARGO_MANIFEST_DIR").len();
            const PKG_NAME: &str = env!("CARGO_PKG_NAME");
            const PKG_VERSION: &str = env!("CARGO_PKG_VERSION");

            /// You can use `.source()` to get the first real source in Report
            pub struct Report {
                file: &'static str,
                line: u32,
                err: Box<dyn Error>,
            }
            pub(crate) trait ToReport<T> {
                fn to_report(self) -> Result<T, Report>;
            }

            impl<T, E: std::error::Error + 'static> ToReport<T> for Result<T, E> {
                #[track_caller]
                fn to_report(self) -> Result<T, Report> {
                    match self {
                        Ok(t) => Ok(t),
                        Err(err) => {
                            let loc = core::panic::Location::caller();
                            Err(Report {
                                file: loc.file().get(PKG_DIR_LEN + 1..).unwrap_or(loc.file()),
                                line: loc.line(),
                                err: err.into(),
                            })
                        }
                    }
                }
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
                    let err_str = match self.err.downcast_ref::<Report>() {
                        Some(report) => report.to_string(is_debug, index + 1),
                        None => {
                            if is_debug {
                                format!("{:?}", self.err)
                            } else {
                                self.err.to_string()
                            }
                        }
                    };

                    match index {
                        0 => {
                            format!(
                                "{{{PKG_NAME}@{PKG_VERSION}}} {}:{} -> {}",
                                self.file, self.line, err_str
                            )
                        }
                        _ => {
                            format!("{}:{} -> {}", self.file, self.line, err_str)
                        }
                    }
                }
            }
        }
    };
}
