// 可能崩溃
#![deny(
    clippy::indexing_slicing,
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic_in_result_fn,
    clippy::todo,
    clippy::unreachable,
    clippy::unimplemented
)]
// 疑似劣质代码
#![warn(
    clippy::cargo,
    clippy::complexity,
    clippy::correctness,
    clippy::nursery,
    clippy::pedantic,
    clippy::perf,
    clippy::style,
    clippy::suspicious
)]

pub mod blueprint;
pub mod dspbptk_blueprint;
pub mod error;
pub mod item;
pub mod planet;
pub mod workflow;

// TODO 给已经基本稳定下来的函数写文档
// TODO cargo clippy --fix
// TODO cargo tarpaulin --ignore-tests
