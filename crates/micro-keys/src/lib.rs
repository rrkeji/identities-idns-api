pub mod account;
mod command;
mod error;
pub mod kvstore;
pub mod utils;

pub use command::*;
pub use error::*;
pub use utils::idns_utils::*;
//
pub mod idns {
    //
    pub mod account {
        include!(concat!(env!("OUT_DIR"), "/idns.account.rs"));
    }
    //
    pub mod system {
        include!(concat!(env!("OUT_DIR"), "/idns.system.rs"));
    }
}
