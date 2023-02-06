#[macro_use]
extern crate rust_embed;

mod utils;
pub mod lightclient;
#[cfg(feature = "zephyr_wasm")]
#[path = "grpc/grpcconnector_js.rs"] pub mod grpcconnector;

#[cfg(not(feature = "zephyr_wasm"))]
#[path = "grpc/grpcconnector_socket.rs"] pub mod grpcconnector;


pub mod lightwallet;

#[cfg(not(feature = "zephyr_wasm"))]
pub mod commands;

pub mod fakeoxide;

#[cfg(feature = "embed_params")]
#[derive(RustEmbed)]
#[folder = "zcash-params/"]
pub struct SaplingParams;

#[cfg(not(feature = "zephyr_wasm"))]
#[derive(RustEmbed)]
#[folder = "res/"]
pub struct PubCertificate;

pub const ANCHOR_OFFSET: u32 = 4;

#[cfg(not(feature = "zephyr_wasm"))]
pub mod grpc_client {
    tonic::include_proto!("cash.z.wallet.sdk.rpc");
}

//#[cfg(not(feature = "zephyr_wasm"))]
pub mod proto; 

#[macro_use]
extern crate lazy_static;

use log::warn;
use web_sys::console;
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




/// Initializes new client/wallet and returns string with seed or error message starting with "Error:"
#[wasm_bindgen]
pub async fn litelib_initialize_new(entropy: String) -> String {
  utils::set_panic_hook();

  println!("println: hello");
  eprintln!("eprintln: hello");

  console::log_1(&"Printing from Rust!! Again".into());
  console::log_2(&"Numbers: ".into(), &1234.into());
  // error!("error! macro"); // how to use it? this is actually log::error!() 

  let server = LightClientConfig::get_server_or_default(Some("www.com".to_string()));
  let server2 = server.clone();
  let (config, latest_block_height)  = match LightClientConfig::create(server).await {
    Ok((c, h)) => (c,h),
    Err(e) => {
      let edump = format!("LightClientConfig::create() error: {:#?}", e);
      console::log_1(&edump.into());
      return format!("Error calling LightClientConfig::create({}) {}", server2, e);
    }
  };

  // let entropy: String = "entropy123".to_string();
  let lightclient = match LightClient::new(&config, latest_block_height, entropy).await { 
      Ok(l) => l,
      Err(e) => {
          return format!("Error calling LightClient::new(): {}", e);
      }
  };

  // Initialize logging 
  #[cfg(not(feature = "zephyr_wasm"))]
  {
    let _ = lightclient.init_logging();
  }

  
  let seed_or_errormsg = match lightclient.do_seed_phrase() {
    Ok(s)=>s.dump(),
    Err(e) => {
      return format!("Error: {}", e);
    }
  };
  
  // TODO: is this the right way to use Arc mutable?
  {
    let lc = LIGHTCLIENT.lock().unwrap();
    // lc.borrow_mut().set(lightclient); // borow_mut() is part of RefCell API
    lc.replace(Some(Arc::new(lightclient)));

  }

  return seed_or_errormsg;
  // return "Create a new wallet and return the seed for the newly created wallet.".to_string();
}


/// Restore a wallet from the seed phrase
#[wasm_bindgen]
pub async fn litelib_initialize_new_from_phrase(seed: String, birthday: u64) -> String {
  utils::set_panic_hook();

  let server = LightClientConfig::get_server_or_default(Some("www.com".to_string()));
  let (config, _ ) = match LightClientConfig::create(server.clone()).await {
    Ok((config, height))=> (config, height),
    Err(e)=>{
      let edump = format!("LightClientConfig::create() error: {:#?}",e);
      warn!("{}", edump);
      return format!("Error calling LightClientConfig::create({}) {}", server, e);
    }
  };

  let lightclient = match LightClient::new_from_phrase(seed, &config, birthday, false).await {
    Ok(l) => l,
    Err(e) => {
      return format!("Error calling Lightclient::new_from_phrase(config, <seed>, birthday={}): {}", birthday, e);
    }
  };

  {
    let lc = LIGHTCLIENT.lock().unwrap();
    lc.replace(Some(Arc::new(lightclient)));
  }

  format!("OK")
}


// Initialize a new lightclient and store its value
#[wasm_bindgen]
pub async fn litelib_initialize_existing(_wallet_hex: String) -> String {
  utils::set_panic_hook();
  return "Initialize a new lightclient and store its value".to_string();
}


#[wasm_bindgen]
pub async fn litelib_execute(cmd: String, _args_list: String) -> String {
  utils::set_panic_hook();
  let resp: String;
  {
    let lightclient: Arc<LightClient>;
    {
      let lc = LIGHTCLIENT.lock().unwrap();
      if  lc.borrow().is_none() {
        return format!("Error: Light Client is not initialized");
      }
      lightclient = lc.borrow().as_ref().unwrap().clone();
    };

    // I think only nightly can compile this not, 1.49
    // let lightclient: Arc<LightClient> = {
    //   let lc = LIGHTCLIENT.lock().unwrap();
    //   if  lc.borrow().is_none() {
    //     return format!("Error: Light Client is not initialized");
    //   }
    //   lc.borrow().as_ref().unwrap().clone()
    // };

    // ------------------ COMMANDS --------------------------
    if cmd == "sync" {
        let r = lightclient.do_sync(true).await;
        resp = match r {
          Ok(j) => j.pretty(2).clone(),
          Err(e) => format!("sync Error {}", e)
        };
    }
    else if cmd == "sync_internal" {
      let r = lightclient.do_sync_internal(true, 0).await;
      resp = match r { 
        Ok(j)=>j.pretty(2).clone(),
        Err(e)=>format!("sync_internal() Error {}",e)
      }
    }
    else if cmd == "list_transactions" {
      let j = lightclient.do_list_transactions(false);
      resp = j.pretty(2).clone()
    }
    // else if cmd == "rescan" {
    //   resp = match lightclient.do_rescan().await {
    //     Ok(j) => j.pretty(2),
    //     Err(e) => e
    //   };
    // }
    else if cmd == "info" {
      resp = lightclient.do_info().await;
      // resp = match lightclient.do_info().await {
      //   Ok(s) => s,
      //   Err(e) => e
      // };
    }
    else if cmd == "update_historical_prices"  {
      console::log_1(&"Running lightclient.update_historical_prices()".into());
      
      lightclient.update_historical_prices().await;

      resp = String::from("called client.update_historical_prices()");

    }
    else if cmd == "update_current_price"  {
      console::log_1(&"Running lightclient.update_current_price()".into());
      
      lightclient.update_current_price().await;

      resp = String::from("called client.update_current_price()");

    }
    else if cmd == "do_verify_from_last_checkpoint"  {
      console::log_1(&"Running lightclient.do_verify_from_last_checkpoint()".into());
      
      let result = lightclient.do_verify_from_last_checkpoint().await;

      // resp = String::from("called client.do_verify_from_last_checkpoint()");
      resp = format!("result of client.do_verify_from_last_checkpoint() = {:?}", result);

    }
    else{
      panic!("Unknown command {}", cmd);
    }

    // -----------------------------------------------------

    return resp;

  }
  // return "Execute litelib command".to_string();
} 