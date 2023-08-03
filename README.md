erreport -> A `Result` helper to catch all the `Err` propagation path for Rust
---

## Why?
When an `Err` propagated, we need a method catch all the propagation path.

So we can get this report:
```
{package0@version0} src/lib.rs:56 -> src/xx.rs:23 -> {package1@version1} src/lib.rs:42 -> src/yy.rs:632 -> src/zz.rs:56  
  -> {package2@version2} src/lib.rs:251 -> InvalidXXX
```

## How to use 
```rust
// This will generate a trait called `pub(crate) trait ToReport<T>` to help to convert any `Result<T, E: std::error::Error>` to `Report`.
// You just need to call once for each crate.
erreport::gen_trait_to_report!(); 

fn test() -> Result<(), erreport::Report> {
    any_result_impl_std_error_Error.to_report()?;
    any_result_impl_std_error_Error.to_report()?;
    Ok(())
}
```

## How to access the actual Error?
```rust
fn main() {
    if let Err(err) = test() {
        // This method will bypass all the `Report` wrappers and get the first actual `Error` value.
        err.source() 
    }
}
```
