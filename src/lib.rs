#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![deny(clippy::unwrap_used)]
#![deny(unused_must_use)]

#![deny(clippy::await_holding_lock)]
#![deny(clippy::needless_pass_by_value)]
#![deny(clippy::todo)]
#![deny(clippy::trivially_copy_pass_by_ref)]
#![deny(clippy::unimplemented)]
#![deny(clippy::unreachable)]
#![deny(warnings)]
#![warn(unused_extern_crates)]

pub mod client;
pub mod flags;
pub mod output;
pub mod protocols;
pub mod seat;