#[macro_use]
extern crate rust_embed;

mod utils;
pub mod blaze;
pub mod commands;
pub mod compact_formats;
pub mod grpc_connector;
pub mod lightclient;
pub mod lightwallet;

#[cfg(feature = "embed_params")]
#[derive(RustEmbed)]
#[folder = "zcash-params/"]
pub struct SaplingParams;

pub const ANCHOR_OFFSET : u32 = 4;
pub mod proto; 

#[macro_use]
extern crate lazy_static;

use wasm_bindgen::prelude::{JsValue, wasm_bindgen};
use json::{object};
// use zecwalletlitelib::lightclient::lightclient_config::LightClientConfig;
use lightclient::lightclient_config::LightClientConfig;

// use wasm_bindgen::prelude::*;
use std::sync::{Mutex, Arc};
use std::cell::RefCell;

use crate::lightclient::{LightClient};
// use crate::lightclient::{LightClient, LightClientConfig};

lazy_static! {
  static ref LIGHTCLIENT : Mutex<RefCell<Option<Arc<LightClient>>>> = Mutex::new(RefCell::new(None));
}
// pub mod blaze;
// pub mod compact_formats;
// pub mod grpc_connector;
// pub mod lightclient;
// pub mod lightwallet;

// use lightclient::LightClient;

// fn main() {
//     let seed = std::fs::read_to_string("./testdata/seed.txt").unwrap();
//     let lc = LightClient::new(Some(seed)).unwrap();
//     lc.start_sync();
// }

/// Create a new wallet and return the seed for the newly created wallet.
#[wasm_bindgen]
pub async fn litelib_initialize_new(entropy: String) -> String {
  utils::set_panic_hook();
  let server = LightClientConfig::get_server_or_default(Some("www.com".to_string().to_owned()));
  let (config, latest_block_height)  = match LightClientConfig::create(server).await {
    Ok((c, h)) => (c,h),
    Err(e) => {
      return format!("Error: {}", e);
    }
  };

  let entropy : String = "entropy123".to_string();
  let lightclient = match LightClient::new(&config, latest_block_height, entropy) {
    Ok(l) => l,
    Err(e) => {
      return format!("Error: {}", e);
    }
  };

  let _ = lightclient.init_logging();
  let seed = match lightclient.do_seed_phrase().await {
    Ok(s) => s.dump(),
    Err(e) => {
      return format!("Error: {}", e);
    }
  };

  LIGHTCLIENT.lock().unwrap().replace(Some(Arc::new(lightclient)));

  return seed;
  // return "Create a new wallet and return the seed for the newly created wallet.".to_string();
}


/// Restore a wallet from the seed phrase
#[wasm_bindgen]
pub async fn litelib_initialize_new_from_phrase(seed: String, birthday: u64) -> String {
  utils::set_panic_hook();

  let server = LightClientConfig::get_server_or_default(Some("www.com".to_string().to_owned()));
  let (config, _) = match LightClientConfig::create(server).await {
    Ok((c,h)) => (c,h),
    Err(e) => {
      return format!("Error: {}", e);
    }
  };

  let lightclient = match LightClient::new_from_phrase(seed, &config, birthday, false) {
    Ok(l) => l, 
    Err(e) => {
      return format!("Error: {}", e);
    }
  };

  let _ = lightclient.init_logging();

  LIGHTCLIENT.lock().unwrap().replace(Some(Arc::new(lightclient)));

  format!("OK")
  // return "Restore a wallet from the seed phrase".to_string();
}


// Initialize a new lightclient and store its value
#[wasm_bindgen]
pub async fn litelib_initialize_existing(wallet_hex: String) -> String {
  utils::set_panic_hook();

  let server = LightClientConfig::get_server_or_default(None);
  let (config,_) = match LightClientConfig::create(server).await {
    Ok((c,h)) => (c,h),
    Err(e) => {
      return format!("Error: {}", e);
    }
  };

  let wallet_bytes = match hex::decode(wallet_hex) {
    Ok(bytes) => bytes,
    Err(e)=> {
      return format!("Error: {}", e);
    }
  };

  let lightclient = match LightClient::read_from_buffer(&config, &wallet_bytes[..]) {
    Ok(l) => l, 
    Err(e) => {
      return format!("Error: {}", e);
    }
  };

  // Initialize logging
  let _ = lightclient.init_logging();

  LIGHTCLIENT.lock().unwrap().replace(Some(Arc::new(lightclient)));

  format!("OK")

  // return "Initialize a new lightclient and store its value".to_string();
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

    // ------------------------------------------------------
    // ------------------ COMMANDS --------------------------
    // ------------------------------------------------------

    // =================    sync    =========================
    if cmd == "sync" {
      let r = lightclient.do_sync(true).await;
      resp = match r {
        Ok(j) => j.pretty(2).clone(),
        Err(e) => format!("sync Error {}", e)
      };
    }
    // =================    rescan   =========================
    else if cmd == "rescan" {
      resp = match lightclient.do_rescan().await {
        Ok(j) => j.pretty(2),
        Err(e) => e
      };
    }
    // =================     send    =========================
    else if cmd == "send" {
      let json_args = match json::parse(&args_list){
        Ok(j) => j,
        Err(e) => {
          let es = format!("Couldn't understand JSON: {}", e);
          return format!("{}", es);
        }
      };

      if !json_args.is_array() {
        return format!("Couldn't parse argument as array");
      }

      let maybe_send_args = json_args.members().map( |j| {
        if !j.has_key("address") || !j.has_key("amount") {
          Err(format!("Need 'address' and 'amount'\n"))
        }
        else{
          Ok((
            j["address"].as_str().unwrap().to_string().clone(),
            j["amount"].as_u64().unwrap(),
            j["memo"].as_str().map(|s| s.to_string().clone())
          ))
        }
      }).collect::<Result<Vec<(String, u64, Option<String>)>, String>>();

      let send_args = match maybe_send_args {
        Ok(a) => a.clone(),
        Err(s) => { return format!("Error: {}", s); }
      };

      // Do a sync
      let r = lightclient.do_sync(true).await;
      resp = match r {
        Ok(_) => {
          // Convert to the right format. String -> &str
          let tos = send_args.iter().map(|(a,v,m)| (a.as_str(), *v, m.clone()) ).collect::<Vec<_>>();
          match lightclient.do_send(tos).await {
            Ok(txid) => { object!{ "txid" => txid } },
            Err(e) => { object!{ "error" => e }}
          }.pretty(2)
        },
        Err(e) => format!("sync Error {}", e)
      };
    }
    // =================     save    =========================
    else if cmd == "save" {
      let wallet_bytes = (lightclient.do_save_to_buffer().await).unwrap();
      //                                                    ^^^^^^^^^^^^^^ is this the right way to handle await?
      resp = hex::encode(&wallet_bytes);
    }
    // =================     info    =========================
    else if cmd == "info" {
      resp = lightclient.do_info().await;
    }
    // =================    balance  =========================
    else if cmd == "balance" {
      resp = format!("{}", (lightclient.do_balance().await).pretty(2));
    }
    // =================     notes    =========================
    else if cmd == "notes" {
      resp = format!("{}", lightclient.do_list_notes(false).await.pretty(2));
    }
    // =================   export    =========================
    else if cmd == "export" {
      resp = match lightclient.do_export(Some(args_list)).await {
        Ok(j) => j,
        Err(e) => object!{ "error" => e }
      }.pretty(2);
    }
    // =================      new    =========================
    else if cmd == "new" {
      resp = match lightclient.do_new_address(&args_list).await  {
        Ok(j) => j,
        Err(e) => object!{ "error" => e }
      }.pretty(2);
    }
    // =================     seed    =========================
    else if cmd == "seed" {
      resp = match lightclient.do_seed_phrase().await {
        Ok(j) => j, 
        Err(e) => object!{ "error" => e }
      }.pretty(2);
    }
    // =================     list    =========================
    else if cmd == "list" {
      resp = lightclient.do_list_transactions(false).await.pretty(2);
    }
    // ================  syncstatus  =========================
    // !!! This cannot be ported to `target` as lightclient::sync_status 
    // !!! was phased out and do_scan_status() depends on it. 
    // } else if cmd == "syncstatus" {
    //   let status = lightclient.do_scan_status();
    //   resp = match status.is_syncing {
    //       false => object!{ "syncing" => "false" },
    //       true  => object!{ "syncing" => "true",
    //                         "synced_blocks" => status.synced_blocks,
    //                         "total_blocks" => status.total_blocks }
    //   }.pretty(2);    
    // =================     else    =========================
    else{
      panic!("Unknown command {}", cmd);
    }

    // -----------------------------------------------------



  }





  return "Execute litelib command".to_string();
}