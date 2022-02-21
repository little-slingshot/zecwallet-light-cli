#[macro_use]
extern crate rust_embed;

mod utils;
pub mod lightclient;
pub mod grpcconnector;
pub mod lightwallet;
pub mod commands;

#[cfg(feature = "embed_params")]
#[derive(RustEmbed)]
#[folder = "zcash-params/"]
pub struct SaplingParams;

#[derive(RustEmbed)]
#[folder = "res/"]
pub struct PubCertificate;

pub const ANCHOR_OFFSET: u32 = 4;

pub mod grpc_client {
    tonic::include_proto!("cash.z.wallet.sdk.rpc");
}

pub mod proto; 

#[macro_use]
extern crate lazy_static;

use wasm_bindgen::prelude::{/* JsValue , */ wasm_bindgen};
// use zecwalletlitelib::lightclient::lightclient_config::LightClientConfig;
use lightclient::LightClientConfig;

// use wasm_bindgen::prelude::*;
use std::sync::{Mutex, Arc};
use std::cell::RefCell;
use crate::lightclient::{LightClient};

lazy_static! {
    static ref LIGHTCLIENT : Mutex<RefCell<Option<Arc<LightClient>>>> = Mutex::new(RefCell::new(None));
}




#[wasm_bindgen]
pub async fn litelib_initialize_new(entropy: String) -> String {
  utils::set_panic_hook();
  let server = LightClientConfig::get_server_or_default(Some("www.com".to_string()));
  let (config, latest_block_height)  = match LightClientConfig::create(server).await {
    Ok((c, h)) => (c,h),
    Err(e) => {
      return format!("Error: {}", e);
    }
  };
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
  // utils::set_panic_hook();
  let resp: String;
  {
    let lightclient: Arc<LightClient>;
    {
      let lc = LIGHTCLIENT.lock().unwrap();
      if ( lc.borrow().is_none() ){
        return format!("Error: Light Client is not initialized");
      }
      lightclient = lc.borrow().as_ref().unwrap().clone();
    };

    // ------------------ COMMANDS --------------------------
    if cmd == "sync" {
        let r = lightclient.do_sync(true).await;
        resp = match r {
          Ok(j) => j.pretty(2).clone(),
          Err(e) => format!("sync Error {}", e)
        };
    }
    else if cmd == "rescan" {
      resp = match lightclient.do_rescan().await {
        Ok(j) => j.pretty(2),
        Err(e) => e
      };
    }
    else{
      panic!("Unknown command {}", cmd);
    }

    // -----------------------------------------------------



  }





  return "Execute litelib command".to_string();
} 