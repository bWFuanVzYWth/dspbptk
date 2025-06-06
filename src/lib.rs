#![deny(
    clippy::unwrap_used,           // 禁止使用 `unwrap`
    clippy::panic_in_result_fn,    // 禁止在返回 `Result` 的函数中调用 `panic!`
    clippy::todo,                  // 禁止使用 `todo!`
    clippy::unreachable,           // 禁止使用 `unreachable!`
    clippy::unimplemented,         // 禁止使用 `unimplemented!`

    clippy::shadow_unrelated,
    clippy::shadow_same,
    clippy::shadow_unrelated,
)]
#![warn(
    clippy::pedantic,              // 启用所有严格的 Lint
    clippy::cargo,                 // 启用与 Cargo 相关的 Lint
    clippy::nursery,               // 启用实验性的 Lint

    clippy::indexing_slicing,      // 警告直接索引访问（可能导致越界 panic）
    clippy::expect_used,           // 警告使用 `expect`
)]
#![allow(clippy::missing_errors_doc)]

pub mod blueprint;
pub mod dspbptk_building;
pub mod error;
pub mod io;
pub mod item;
pub mod toolkit;

pub mod tesselation_structure;

// TODO 给已经基本稳定下来的函数写文档
// TODO cargo clippy --fix -- -D clippy::pedantic -D clippy::cargo -D clippy::nursery
// TODO cargo tarpaulin --ignore-tests

// TODO 消除concat
