// Storage daemon (stored): microservice frontend for different storage backends
// used in LNP/BP nodes.
//
// Written in 2022 by
//     Dr. Maxim Orlovsky <orlovsky@lnp-bp.org>
//
// Copyright (C) 2022 by LNP/BP Standards Association, Switzerland.
//
// You should have received a copy of the MIT License along with this software.
// If not, see <https://opensource.org/licenses/MIT>.

#![recursion_limit = "256"]

#[macro_use]
extern crate amplify;
// #[macro_use]
// extern crate strict_encoding;
#[macro_use]
extern crate internet2;
#[macro_use]
extern crate log;

#[cfg(feature = "serde")]
extern crate serde_crate as serde;
//#[cfg(feature = "serde")]
//#[macro_use]
//extern crate serde_with;

pub mod client;
mod error;
mod reply;
mod request;

pub use error::FailureCode;
pub use reply::Reply;
pub use request::Request;

// TODO: Change port
pub const STORED_RPC_ENDPOINT: &'static str = "0.0.0.0:61399";