#![allow(unused)]

#[macro_use]
extern crate rust_embed;


use log::{info, warn, error, LevelFilter};

#[cfg(feature = "embed_params")]
#[derive(RustEmbed)]
#[folder = "zcash-params/"]
pub struct SaplingParams;


// Note: `cargo build` or `wasm-pack build` will first pull in and compile ALL the dependencies
//       from Cargo.toml regardless whether they are _actually_ imported or not in the codebase.

// Without those two modules imported via `pub mod xyz` this is just
// a skeleton of the library (only function stubs will be compiled).

// To bring in the _actual logic_ of the zwl-lib library
// please uncomment both modules below 
// (and also uncommenting grpcconnector may help rid of grpcconnector not in the root errors)
// (at the same time as they circularly depend on each other)
pub mod lightclient;
pub mod lightwallet;
pub mod grpcconnector;

pub mod fakeoxide;


// This is to test imports
// use crate::lightwallet::{LightWallet, message::Message};
// use crate::lightwallet::{self, LightWallet, message::Message};
// use crate::lightwallet::*;

// use crate::lightclient::{LightClient};


mod utils;
// pub mod commands; // this module was removed from wasm-version as per Aditya's wasm-shopping list.

pub const ANCHOR_OFFSET: u32 = 4;

#[macro_use]
extern crate lazy_static;

use wasm_bindgen::prelude::{/* JsValue , */ wasm_bindgen};

// use wasm_bindgen::prelude::*;
use std::sync::{Mutex, Arc};
use std::cell::RefCell;




#[wasm_bindgen]
pub async fn litelib_initialize_new(entropy: String) -> String {
  utils::set_panic_hook();
 
  return "Create a new wallet and return the seed for the newly created wallet.".to_string();
}


/// Restore a wallet from the seed phrase
#[wasm_bindgen]
pub async fn litelib_initialize_new_from_phrase(seed: String, birthday: u64) -> String {
  utils::set_panic_hook();
  return "Restore a wallet from the seed phrase".to_string();
}


// Initialize a new lightclient and store its value
#[wasm_bindgen]
pub async fn litelib_initialize_existing(wallet_hex: String) -> String {
  utils::set_panic_hook();
  return "Initialize a new lightclient and store its value".to_string();
}


#[wasm_bindgen]
pub async fn litelib_execute(cmd: String, args_list: String) -> String {
  utils::set_panic_hook();
  return "Execute litelib command".to_string();
} 