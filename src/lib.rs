pub mod cmd;
pub mod conf;
pub mod consts;
pub use xtunnel_core as core;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const ENV_CONFIG: &str = "xtunnel_CONF";
