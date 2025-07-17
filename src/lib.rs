#![warn(
    clippy::correctness,

    clippy::shadow_unrelated,
    clippy::shadow_same,
    clippy::shadow_unrelated,

    clippy::indexing_slicing,      // 禁止直接索引访问（可能导致越界 panic）
    clippy::unwrap_used,           // 禁止使用 `unwrap`
    clippy::expect_used,           // 禁止使用 `expect`
    clippy::panic_in_result_fn,    // 禁止在返回 `Result` 的函数中调用 `panic!`
    clippy::todo,                  // 禁止使用 `todo!`
    clippy::unreachable,           // 禁止使用 `unreachable!`
    clippy::unimplemented,         // 禁止使用 `unimplemented!`
    )]
#![warn(
    clippy::complexity,
    clippy::perf,

    clippy::suspicious,
    clippy::style,
    clippy::pedantic,

    clippy::nursery,

    // clippy::restriction,
    // clippy::cargo,
)]
#![feature(array_chunks)]

pub mod blueprint;
pub mod dspbptk_building;
pub mod error;
pub mod io;
pub mod item;
pub mod toolkit;

pub mod tesselation_structure;

// TODO 给已经基本稳定下来的函数写文档
// TODO cargo clippy --fix
// TODO cargo tarpaulin --ignore-tests

// TODO 跨纬度的坐标计算
