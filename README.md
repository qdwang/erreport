# erreport
A `Result` helper for Rust

## How to use
```rust
erreport::gen_report_code!(); // This will create a mod called `report` in current scope.
use report::{Report, ToReport};

fn test() -> Result<(), Report> {
    any_result_impl_std_error_Error.to_report()?;
    Ok(())
}
```
## Visibility
```rust
pub(crate) mod report {
    pub Report;
    pub(crate) ToReport;
}
```