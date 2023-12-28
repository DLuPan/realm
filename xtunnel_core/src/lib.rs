//! xtunnel's core facilities.

pub mod dns;
pub mod tcp;
pub mod udp;
pub mod time;
pub mod trick;
pub mod endpoint;

pub use xtunnel_io;
pub use xtunnel_syscall;

#[cfg(feature = "hook")]
pub use xtunnel_hook as hook;

#[cfg(feature = "balance")]
pub use xtunnel_lb as balance;

#[cfg(feature = "transport")]
pub use kaminari;
