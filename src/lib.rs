#![feature(array_chunks)]
// 潜在的panic风险
#![warn(
    clippy::indexing_slicing,
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic_in_result_fn,
    clippy::todo,
    clippy::unreachable,
    clippy::unimplemented
)]
// 潜在的劣质代码
#![warn(
    clippy::correctness,
    clippy::shadow_unrelated,
    clippy::shadow_same,
    clippy::shadow_unrelated,
    clippy::complexity,
    clippy::perf,
    clippy::suspicious,
    clippy::style,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo,
    // clippy::restriction,
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
