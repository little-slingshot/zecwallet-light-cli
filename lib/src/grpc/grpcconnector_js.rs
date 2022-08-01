#![allow(unused_imports, unused_variables, dead_code)]

use wasm_bindgen::prelude::*;
// use wasm_bindgen::prelude::IntoJsResult;
// use wasm_bindgen::IntoJsResult;
use wasm_bindgen::__rt::IntoJsResult;

// use crate::proto::lightdinfo::LightdInfo;

#[allow(deprecated)]
use protobuf::parse_from_bytes;

use std::collections::HashMap;
use zcash_primitives::transaction::{TxId, Transaction};


use std::fmt;
// use log::{error, warn};


use crate::proto::compact_formats::{
  CompactBlock
};

use crate::proto::service::{
  LightdInfo,
  PriceResponse,
  TreeState, 
  BlockID, 
  RawTransaction,
  SendResponse
};
use std::convert::TryInto;

// ------------------------------------
// import from js world 
// ------------------------------------
#[wasm_bindgen]
extern "C" {
  fn getInfo() -> js_sys::Promise /* LightdInfo */;

  fn getCurrentZecPrice() -> js_sys::Promise /* PriceResponse */;

  fn getZecPrice(timestamp: u64, currency: String) -> js_sys::Promise /* PriceResponse */;

  fn getTreeState(block_height: i32,  block_hash_hex: String) -> js_sys::Promise /* TreeState */;

  fn getLatestBlock() -> js_sys::Promise /* BlockID */;

  fn getTransparentTxids(address: String, start_height: u64, end_height: u64) -> js_sys::Promise /* RawTransaction[] 
                                                                            (newline separated string of RawTransaction 
                                                                            hex-serialized representations) */;

  fn getBlockRange(start_height: u64, end_height: u64) -> js_sys::Promise /* CompactBlock[] as NewLineSeparatedHexString */;

  fn getFullTx(txhash: String) -> js_sys::Promise /* RawTransaction as HexString */;

  fn sendTransaction(txhex: String) -> js_sys::Promise /* */;

  // not part of the lightwallet RPC interface, but a helper to log to browser console.
  fn doLog(s: &str);
}

// #[allow(non_snake_case)]
// fn doLog(s: &str){
//   println!("{}", s);
// }

//-------------------------------------------
// MyError
//-------------------------------------------
#[derive(Debug)]
struct MyError {}

impl std::error::Error for MyError{}

impl MyError{
  pub fn pop_into_existence() -> Self {
    MyError {}
  }
}

impl  fmt::Display for MyError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "MyError display.")
  }
}



// **********************************************************************
// **********************************************************************
// *** get_info
// **********************************************************************
// **********************************************************************
pub async fn get_info(_client_uri: &http::Uri) -> Result<LightdInfo, String> {
  // how can I implement this .
  // now the question is: 
  // how can I call an async function, right? 
  let promise: js_sys::Promise = getInfo();
  let lightdinfo = wasm_bindgen_futures::JsFuture::from(promise).await;

  // I am confused, am I supposed to return
  // just one value? how come I can 
  // _also_ return error at the same time? 
  // I can follow the ternary operator intuition:
  // let valOrNull = someTest ? 'Value is here' : null /* error marker */;
  // but then i still need to test for the error down the line... 

  // also it's not clear how to unwrap JsValue???

  let r = match lightdinfo { // that's a result...
    Ok(jsv_lightdinfo) => {
      // how 
      let hex_serialized : String = jsv_lightdinfo.as_string().unwrap(); // no don't we have to take it as string,
      //                                      ^^^^^^^^^^^^ this is 'runtime cast' to string
                                // but later inflate our structure from the string? 
      // and here we're actually should inflate our structure

      // TODO!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
      // TODO!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
      // TODO!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
      // TODO!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
      // TODO!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
      // TODO!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
      // we will deserialize the struct here... 
      // using protobuf::parse_bytes()
      let bin_serialized = hex::decode(hex_serialized).unwrap();
      #[allow(deprecated)]
      let lightd : LightdInfo = parse_from_bytes(&bin_serialized).unwrap();
      Ok(lightd)


      // Ok(LightdInfo{
      //   hello: serialized,
      //   // hello: String::from("world")
      //   // hello: "world"
      // })
      // jsvLightdinfo.unwrap() // how to parse jsvValues from info...
    }
    Err(jsv_error)=>{
      return Err(String::from(format!("jsvError: {:?}", jsv_error)));
    }
  };
  
  r
  // Err("Not implemented") // this doesn't work
  // Err(String::from("get_info() to be implemented and is just a stub"))

}


// **********************************************************************
// **********************************************************************
// *** get_current_zec_price
// **********************************************************************
// **********************************************************************
pub async fn get_current_zec_price(uri: &http::Uri) -> Result<f64, String> {
  let promise : js_sys::Promise = getCurrentZecPrice();
  let priceresult = wasm_bindgen_futures::JsFuture::from(promise).await; // This should be a Result? 

  let r  = match priceresult {
    Ok(jsv_price) => {
      let hex_serialized : String = jsv_price.as_string().unwrap(); // runtime 'cast' to String
      let bin_serialized = hex::decode(hex_serialized).unwrap();

      #[allow(deprecated)]
      let priceresponse : PriceResponse = parse_from_bytes(&bin_serialized).unwrap();
      Ok(priceresponse.price)
    }
    Err(jsv_error) => {
      return Err(String::from(format!("jsvError: {:?}", jsv_error)));
    }
  }; 
  r
}

// **********************************************************************
// **********************************************************************
// *** get_zec_price
// **********************************************************************
// **********************************************************************
// this is NOT part of zwl-lib grpcfacade, but it is a 
// stepping stone for implementing _get_historical_zec_prices_async()
// we're trying to figure out how to pass parameters to JS function 
// i still make it pub as from_grpcconnector still needs it
pub async fn get_zec_price(_client_uri: &http::Uri, timestamp: u64, currency: String) -> Result<f64, String> { 
  // TODO: what does it return???? a structure?
  //       > no, it returns what it returns...
  //       
  let promise : js_sys::Promise = getZecPrice(timestamp, currency);
  let priceresult = wasm_bindgen_futures::JsFuture::from(promise).await;

  let r = match priceresult {
    Ok(jsv_price) => {
      let hex_serialized : String = jsv_price.as_string().unwrap(); // runtime 'cast' to String
      let bin_serialized = hex::decode(hex_serialized).unwrap();

      #[allow(deprecated)]
      let priceresponse : PriceResponse = parse_from_bytes(&bin_serialized).unwrap();
      Ok(priceresponse.price)
    }
    Err(jsv_error) => {
      return Err(String::from(format!("jsvError: {:?}", jsv_error)));
    }
  };
  r
}



// **********************************************************************
// **********************************************************************
// *** get_historical_zec_prices
// **********************************************************************
// **********************************************************************
pub async fn get_historical_zec_prices(client_uri: &http::Uri, txids: Vec<(TxId, u64)> , currency: String)
  -> Result<HashMap<TxId, Option<f64>>,String>
{
  match _get_historical_zec_prices_async(client_uri, txids, currency).await // how to map error to string
  {
    Ok(pricemap)=>Ok(pricemap),
    Err(err)=>Err(String::from(format!("jsvError: (actually not jsvError) {:?}", err)))
  }
}


// @see https://github.com/little-slingshot/zecwallet-light-cli-forum/issues/58#issuecomment-1109508128
// we focus on asyn versions of all the functions now, and later
// we sort out the sync versions of them
async fn _get_historical_zec_prices_async(uri: &http::Uri, txids: Vec<(TxId, u64)>, currency: String )
  -> Result<HashMap<TxId, Option<f64>>, Box<dyn std::error::Error>>
{
  let mut prices = HashMap::new();

  for (txid, ts) in txids {
    let promise : js_sys::Promise = getZecPrice(ts, String::from(&currency));
    // let promise : js_sys::Promise = getZecPrice(ts, String::from("USD"));
    let priceresult = wasm_bindgen_futures::JsFuture::from(promise).await;
    match priceresult {
      Ok(jsv_price)=>{
        let hex_serialized : String = jsv_price.as_string().unwrap();
        let bin_serialized = hex::decode(hex_serialized).unwrap();

        #[allow(deprecated)]
        let priceresponse : PriceResponse = parse_from_bytes(&bin_serialized).unwrap();
        
        // prices.insert(txid, Some(11_f64));
        prices.insert(txid, Some(priceresponse.price));
      }
      Err(jsv_error)=> {
        dbg!(jsv_error);
        // return Err(String::from(format!("jsvError: {:?}", jsv_error)));
        // return Err(Box::new(jsv_error));
        // return Err(Box::new(std::error::Error));
        // return Err(Box::new(MyError::new())); // doesn't implement
        let me = MyError {}; // is this going to be a stack variable?
        return Err(Box::new(me));
      }
    }
  }// endfor
  Ok(prices)
}

// **********************************************************************
// **********************************************************************
// *** get_sapling_tree
// **********************************************************************
// **********************************************************************
pub async fn get_sapling_tree(client_uri: &http::Uri, height: i32) -> Result<TreeState, String> {
  // let mut rt = tokio::runtime::Runtime::new().map_err(|e| e.to_string())?;
  // rt.block_on(get_sapling_tree_async(client_uri, height)).map_err(|e| e.to_string())
  match _get_sapling_tree_async(client_uri, height).await {
    Ok(tree_state)=>Ok(tree_state),
    Err(err)=>Err(String::from(format!("{:?}", err)))
  }
}

async fn _get_sapling_tree_async(uri: &http::Uri, height: i32) -> Result<TreeState, Box<dyn std::error::Error>> {
  // let mut client = get_client(uri).await?;

  // let b = BlockId{ height: height as u64, hash: vec![]};
  // let response = client.get_tree_state(Request::new(b)).await?;

  // Ok(response.into_inner())
  // TODO: hash is not used here... :(  (but keep in mind that argument is missing too)
  //       I think we need to add an alternative function with two parameters for completeness..
  //       also keep in mind that 'falsy' string (in JS realm will be interpreted as empty one)
  //       maybe we need a getTreeStateFromHeight() and getTreeState(height, hash) ?
  let promise : js_sys::Promise = getTreeState(height, String::from(""));
  let treestateresult = wasm_bindgen_futures::JsFuture::from(promise).await;
  match treestateresult {
    Ok(jsv_treestate)=>{
      // now parse hex string back into the data structure... 
      let hex_serialized : String = jsv_treestate.as_string().unwrap();
      let bin_serialized : &[u8] = &hex::decode(hex_serialized).unwrap();

      #[allow(deprecated)]
      let treestateresponse : TreeState = parse_from_bytes(bin_serialized).unwrap();

      Ok(treestateresponse)

    }
    Err(jsv_error)=>{
      dbg!(jsv_error);
      return Err(Box::new(MyError{})); // TODO: I suspect that we don't really need this type of error returned
                                       //       as this error seems to be artifact of using client.get_tree_state() 
                                       //       and is NOT part of a public interface...
    }
  }
}


// **********************************************************************
// **********************************************************************
// *** fetch_latest_block
// **********************************************************************
// **********************************************************************
pub async fn fetch_latest_block(client_uri: &http::Uri) -> Result<BlockID, String> {
  // let mut rt = match tokio::runtime::Runtime::new() {
  //     Ok(r) => r,
  //     Err(e) => {
  //         let errstr = format!("Error creating runtime {}", e.to_string());
  //         eprintln!("{}", errstr);
  //         return Err(errstr);
  //     }
  // };

  // rt.block_on(_get_latest_block_async(client_uri)).map_err(|e| {
  //     let errstr = format!("Error getting latest block {}", e.to_string());
  //     eprintln!("{}", errstr);
  //     errstr
  // })
  match _get_latest_block_async(client_uri).await {
    Ok(latest_block)=>Ok(latest_block),
    Err(err)=>Err(String::from(format!("{:?}", err)))
  }
}


// get_latest_block GRPC call
async fn _get_latest_block_async(uri: &http::Uri) -> Result<BlockID, Box<dyn std::error::Error>> {

  let promise : js_sys::Promise = getLatestBlock();
  let blockresult = wasm_bindgen_futures::JsFuture::from(promise).await;

  match blockresult {
    Ok(jsv_block)=>{
      let hex_serialized : String = jsv_block.as_string().unwrap();
      let bin_serialized : &[u8] = &hex::decode(hex_serialized).unwrap();

      #[allow(deprecated)]
      let blockresponse : BlockID = parse_from_bytes(bin_serialized).unwrap();

      Ok(blockresponse)
    }
    Err(jsv_error)=>{
      dbg!(jsv_error);
      return Err(Box::new(MyError{})); // TODO: we probably don't need such a boxed error, as this is
                                       //       artifact or using client.get_latest_block()
    }
  }

  // let mut client = get_client(uri).await?;
  // let request = Request::new(ChainSpec {});
  // let response = client.get_latest_block(request).await?;
  // Ok(response.into_inner())
}





// **********************************************************************
// **********************************************************************
// *** fetch_transparent_txids
// **********************************************************************
// **********************************************************************
pub async fn fetch_transparent_txids<F: 'static + std::marker::Send>(uri: &http::Uri, address: String, 
  start_height: u64, end_height: u64, c: F) -> Result<(), String>
where F : Fn(&[u8], u64) {

    // let mut rt = match tokio::runtime::Runtime::new() {
    //   Ok(r) => r,
    //   Err(e) => {
    //       let e = format!("Error creating runtime {:?}", e);
    //       error!("{}", e);
    //       eprintln!("{}", e);
    //       return Err(e);
    //   }
    // };


    // match rt.block_on(_get_address_txids_async(uri, address.clone(), start_height, end_height, c)) {
    match _get_address_txids_async(uri, address.clone(), start_height, end_height, c).await {
      Ok(o) => Ok(o), // pass through ()
      Err(err)=>Err(String::from(format!("{:?}", err)))
      // Err(e) => {
      //     let e = format!("Error with _get_address_txids_async runtime {:?}", e);
      //     error!("{}", e);
      //     eprintln!("{}", e);
      //     Err(e)
      // }
    }
}

pub async fn fetch_transparent_txids2(uri: &http::Uri, address: String, 
  start_height: u64, end_height: u64) -> Result<Vec<(Transaction, u64)>, String>
{
    // match rt.block_on(_get_address_txids_async(uri, address.clone(), start_height, end_height, c)) {
    match _get_address_txids_async2(uri, address.clone(), start_height, end_height).await {
      Ok(o) => Ok(o), // pass through ()
      Err(err)=>Err(String::from(format!("{:?}", err)))
      // Err(e) => {
      //     let e = format!("Error with _get_address_txids_async runtime {:?}", e);
      //     error!("{}", e);
      //     eprintln!("{}", e);
      //     Err(e)
      // }
    }
}

/*
// _get_address_txids_async0 GRPC call
async fn _get_address_txids_async0<F : 'static + std::marker::Send>(uri: &http::Uri, address: String, 
  start_height: u64, end_height: u64, c: F) -> Result<(), Box<dyn std::error::Error>>
where F : Fn(&[u8], u64) {

    let mut client = get_client(uri).await?;
    let start = Some(BlockId{ height: start_height, hash: vec!()});
    let end   = Some(BlockId{ height: end_height,   hash: vec!()});

    let args = TransparentAddressBlockFilter{ address, range: Some(BlockRange{start, end}) };
    let request = Request::new(args.clone());

    let maybe_response = match client.get_taddress_txids(request).await {
      Ok(r) => r,
      Err(e) => {
          if e.code() == tonic::Code::Unimplemented {
              // Try the old, legacy API
              let request = Request::new(args);
              client.get_address_txids(request).await?
          } else {
              return Err(e)?;
          }
      }
    };

    let mut response = maybe_response.into_inner();

    while let Some(tx) = response.message().await? {
      c(&tx.data, tx.height);
    }

    Ok(())
}
*/

// _get_address_txids_async GRPC call
async fn _get_address_txids_async<F : 'static + std::marker::Send>(uri: &http::Uri, address: String, 
  start_height: u64, end_height: u64, c: F) -> Result<(), Box<dyn std::error::Error>>
where F : Fn(&[u8], u64) {
  // call the function 
  let promise : js_sys::Promise = getTransparentTxids(address, start_height, end_height);
  let transactions_result = wasm_bindgen_futures::JsFuture::from(promise).await;
  let mut all_txs : Vec<RawTransaction>= Vec::new();
  match transactions_result  {
    Ok(jsv_transactions)=>{
      // I need to rehspae them transactions
      // there will be lines of them.... 
      let hex_serialized_lines  : String = jsv_transactions.as_string().unwrap();
      let all_lines : Vec<&str> = hex_serialized_lines.lines().collect();

      for hex_serialized_line in all_lines {
        
        let bin_serialized : &[u8] = &hex::decode(hex_serialized_line).unwrap();

        // all_txs.push(TxId {
        //   0: tx_bytevec.try_into().expect("slice with incorrect length")
        // })
        // all_txs.push(RawTransaction {
        //   data: tx_bytevec.try_into().expect("slice with incorrect length"),
        //   height: 10
        // })

        #[allow(deprecated)]
        let raw_transaction : RawTransaction = parse_from_bytes(bin_serialized).unwrap();
        all_txs.push(raw_transaction);
      }
      // Ok(()); // we don't need, 'cos match has trailing semicolon ; and hence 
                 // and hence is a statement, not an expression
    }
    Err(jsv_error)=>{
      // TODO: actually this parameter is not used.. !!!
      dbg!(jsv_error);
      return Err(Box::new(MyError{}));
    }
  };

  // TODO: REVERT!!!!!!!!!!!!!!!!
  // TODO: REVERT!!!!!!!!!!!!!!!!
  // TODO: REVERT!!!!!!!!!!!!!!!!
  // TODO: REVERT!!!!!!!!!!!!!!!!
  // TODO: REVERT!!!!!!!!!!!!!!!!
  for tx in all_txs {
    c(&tx.data, tx.height);
  }

  Ok(())
}




// _get_address_txids_async GRPC call
async fn _get_address_txids_async2(uri: &http::Uri, address: String, 
  start_height: u64, end_height: u64) -> Result<Vec<(Transaction,  u64)>, Box<dyn std::error::Error>>
  {
  // call the function 
  let promise : js_sys::Promise = getTransparentTxids(address, start_height, end_height);
  let transactions_result = wasm_bindgen_futures::JsFuture::from(promise).await;
  let mut all_txs : Vec<RawTransaction>= Vec::new();
  match transactions_result  {
    Ok(jsv_transactions)=>{
      // I need to rehspae them transactions
      // there will be lines of them.... 
      let hex_serialized_lines  : String = jsv_transactions.as_string().unwrap();
      let all_lines : Vec<&str> = hex_serialized_lines.lines().collect();

      for hex_serialized_line in all_lines {
        
        let bin_serialized : &[u8] = &hex::decode(hex_serialized_line).unwrap();

        // all_txs.push(TxId {
        //   0: tx_bytevec.try_into().expect("slice with incorrect length")
        // })
        // all_txs.push(RawTransaction {
        //   data: tx_bytevec.try_into().expect("slice with incorrect length"),
        //   height: 10
        // })

        #[allow(deprecated)]
        let raw_transaction : RawTransaction = parse_from_bytes(bin_serialized).unwrap();
        all_txs.push(raw_transaction);
      }
      // Ok(()); // we don't need, 'cos match has trailing semicolon ; and hence 
                 // and hence is a statement, not an expression
    }
    Err(jsv_error)=>{
      // TODO: actually this parameter is not used.. !!!
      dbg!(jsv_error);
      return Err(Box::new(MyError{}));
    }
  };

  let mut result_transaction_tuples = Vec::new();
  for raw_tx in all_txs {
    let tx = Transaction::read(raw_tx.data.as_slice()).unwrap();
    result_transaction_tuples.push((tx, raw_tx.height));
  }

  Ok(result_transaction_tuples)
}


// **********************************************************************
// **********************************************************************
// *** fetch_blocks
// **********************************************************************
// **********************************************************************
// pub fn fetch_blocks2(uri: &http::Uri, start_height: u64, end_height: u64) -> Result</* whatever the reutrn value */, String> {
  // 
pub async fn fetch_blocks2(uri: &http::Uri, start_height: u64, end_height: u64) -> Result<
  Vec<(Vec<u8>, u64)>,  
   String> 
{
  // > What is the value of this function?
  //   this function is merely a thin delegate (whos only responsibility used to be to convert async->sync; 
  //   now this is pure delegate without value)
  match _get_block_range_async2(uri, start_height, end_height).await {
    Ok(resulting_blocks)=>Ok(resulting_blocks),
    // Err(err)=>Err(String::from("Some kind of error")) // TODO: fix error 
    Err(err)=>Err(String::from(format!("{:?}", err))) // same approach as in get_sapling_tree() delegate
  }
}


async fn _get_block_range_async2(url: &http::Uri, start_height: u64, end_height: u64) -> Result<
  Vec<(Vec<u8>, u64)>,  
  Box<dyn std::error::Error>
  > 
{

  // call the lightwalletd.js function
  let promise : js_sys::Promise = getBlockRange(start_height, end_height);
  let blockrange_result = wasm_bindgen_futures::JsFuture::from(promise).await;
  let mut all_blocks : Vec<CompactBlock> = Vec::new();

  match blockrange_result {
    Ok(jsv_blocks)=>{
      // unmarshall the results  (from hex-strings)
      let hex_serialized_lines : String = jsv_blocks.as_string().unwrap();
      let all_lines : Vec<&str> = hex_serialized_lines.lines().collect();
      for hex_serialized_line in all_lines {
        let bin_serialized : &[u8] = &hex::decode(hex_serialized_line).unwrap();
        #[allow(deprecated)]
        let compact_block : CompactBlock  = parse_from_bytes(bin_serialized).unwrap();
        all_blocks.push(compact_block);
      }
    }
    Err(jsv_error)=>{
      dbg!(jsv_error);
      return Err(Box::new(MyError{}))
    }
  }


  // prepare the resulting vector..
  let mut result_block_tuples : Vec<(Vec<u8>, u64)> = Vec::new();

  for compact_block in all_blocks {
    // let mut encoded_buf = Vec::new();
    // compact_block.encode(&mut encoded_buf).unwrap();
    use protobuf::Message;
    // let mut encoded_buf : Vec<u8> = compact_block.write_to_vec();
    let encoded_buf : Vec<u8> = compact_block.write_to_bytes().unwrap();
    result_block_tuples.push((encoded_buf, compact_block.height));
    // result_block_tuples.push((encoded_buf.as_slice(), compact_block.height));
    // result_block_tuples.push((&Box::new(encoded_buf.clone()), compact_block.height));
    // result_block_tuples.push((encoded_buf.into_boxed_slice(), compact_block.height));
  }
  Ok(result_block_tuples)


  // if I am already using getBlockRange() somewhere means that I already have this code logic ...
  // Ok(Vec::new())
  // how to throw this dyn error? 
  // Err(Box::new(std::error::Error));
} 

/*
pub fn fetch_blocks<F : 'static + std::marker::Send>(uri: &http::Uri, start_height: u64, end_height: u64, pool: ThreadPool, c: F) -> Result<(), String>
    where F : Fn(&[u8], u64)  {
    
    let mut rt = match tokio::runtime::Runtime::new() {
        Ok(r) => r,
        Err(e) => {
            let es = format!("Error creating runtime {:?}", e);
            error!("{}", es);
            eprintln!("{}", e);
            return Err(es);
        }
    };

    match rt.block_on(get_block_range(uri, start_height, end_height, pool, c)) {
        Ok(o) => Ok(o),
        Err(e) => {
            let e = format!("Error fetching blocks {:?}", e);
            error!("{}", e);
            eprintln!("{}", e);
            Err(e)
        }
    }
}


async fn get_block_range<F : 'static + std::marker::Send>(uri: &http::Uri, start_height: u64, end_height: u64, pool: ThreadPool, c: F) 
    -> Result<(), Box<dyn std::error::Error>> 
where F : Fn(&[u8], u64) {
    let mut client = get_client(uri).await?;

    let bs = BlockId{ height: start_height, hash: vec!()};
    let be = BlockId{ height: end_height,   hash: vec!()};

    let request = Request::new(BlockRange{ start: Some(bs), end: Some(be) });

    // Channel where the blocks are sent. A None signifies end of all blocks
    let (tx, rx) = channel::<Option<CompactBlock>>();

    // Channel that the processor signals it is done, so the method can return
    let (ftx, frx) = channel();

    // The processor runs on a different thread, so that the network calls don't
    // block on this
    pool.execute(move || {
        while let Some(block) = rx.recv().unwrap() {
            use prost::Message;
            let mut encoded_buf = vec![];

            block.encode(&mut encoded_buf).unwrap();
            c(&encoded_buf, block.height);
        }
        
        ftx.send(Ok(())).unwrap();
    });

    let mut response = client.get_block_range(request).await?.into_inner();
    while let Some(block) = response.message().await? {
        tx.send(Some(block)).unwrap();
    }
    tx.send(None).unwrap();

    // Wait for the processor to exit
    frx.iter().take(1).collect::<Result<Vec<()>, String>>()?;

    Ok(())
}

*/



// **********************************************************************
// **********************************************************************
// *** fetch_full_tx
// **********************************************************************
// **********************************************************************
pub async fn fetch_full_tx(uri: &http::Uri, txid: TxId) -> Result<Vec<u8>, String> {


  match _get_transaction_async(uri, txid).await {
      Ok(rawtx) => Ok(rawtx.data.to_vec()),
      Err(e) => {
          let errstr = format!("Error in _get_transaction_async call {}", e.to_string());
          // error!("{}", errstr);
          eprintln!("{}", errstr);
          Err(errstr)
      }
  }    
}


// get_transaction GRPC call
async fn _get_transaction_async(uri: &http::Uri, txid: TxId) 
    -> Result<RawTransaction, Box<dyn std::error::Error>> {
    // let mut client = get_client(uri).await?;

    // let txhash : String = String::from("A1B2C3D4E5F6");
    let txhash : String = hex::encode(txid.0);
    let promise : js_sys::Promise = getFullTx(txhash);
    let fulltx_result = wasm_bindgen_futures::JsFuture::from(promise).await;

    match fulltx_result  { 
      Ok(jsv_fulltx) => {
        // I kind of have to 
        let hex_serialized : String = jsv_fulltx.as_string().unwrap();
        let bin_serialized : &[u8] = &hex::decode(hex_serialized).unwrap();

        #[allow(deprecated)]
        let fulltx_response : RawTransaction = parse_from_bytes(bin_serialized).unwrap();

        Ok(fulltx_response)
      }
      Err(jsv_error) => {
        // I have to throw this confusing error
        dbg!(jsv_error);
        // Err(MyError {})
        // Err(MyError::pop_into_existence())
        Err(Box::new(MyError::pop_into_existence()))

      }
    }
    // let request = Request::new(TxFilter { block: None, index: 0, hash: txid.0.to_vec() });

    // let response = client.get_transaction(request).await?;



    // Ok(response.into_inner())
}

// **********************************************************************
// **********************************************************************
// *** broadcast_raw_tx
// **********************************************************************
// **********************************************************************
pub async fn broadcast_raw_tx(uri: &http::Uri, tx_bytes: Box<[u8]>) -> Result<String, String> {

  match _send_transaction_async(uri, tx_bytes).await {
    Ok(txid)=>Ok(txid),
    Err(e)=>{
      // stringify error
      let errstr = format!("Error in _send_transaction_async() call {}", e.to_string());
      eprintln!("{}", errstr);
      Err(errstr)
    }
  }
}

// send_transaction GRPC call
async fn _send_transaction_async(_uri: &http::Uri, tx_bytes: Box<[u8]>) -> Result<String, Box<dyn std::error::Error>> {

  // let request = Request::new(RawTransaction {data: tx_bytes.to_vec(), height: 0});
  
  // TODO: I wonder if there's again goingto be a problem with endianness..
  let tx_hex_serialized : String = hex::encode(tx_bytes);
  // let tx_hex_serialized : String = String::from("A1B2C3");
  let promise : js_sys::Promise = sendTransaction(tx_hex_serialized);

  let send_response_result = wasm_bindgen_futures::JsFuture::from(promise).await;

  match send_response_result {
    Ok(jsv_send_response)=>{
      let hex_serialized : String = jsv_send_response.as_string().unwrap();
      let bin_serialized : &[u8] = &hex::decode(hex_serialized).unwrap();

      #[allow(deprecated)]
      let send_response : SendResponse = parse_from_bytes(bin_serialized).unwrap();

      // Why do we return string? 
      if 0 == send_response.errorCode {
        let mut txid = send_response.errorMessage;
        // strip quotation marks
        if txid.starts_with("\"") && txid.ends_with("\"") {
          txid = txid[1..txid.len()-1].to_string();
        }
        Ok(txid)
      }
      else {
        Err(Box::from(format!("Error: {:?}", send_response)))
      }
    }
    Err(jsv_error)=>{
      // dbg!(jsv_error);
      // return Err(String::from(format!("jsvError: {:?}", jsv_error)));
      return Err(Box::from(format!("Error: {:?}", jsv_error)));
    }
  }
  
  // let sendresponse = response.into_inner();
  // if sendresponse.error_code == 0 {
  //     let mut txid = sendresponse.error_message;
  //     if txid.starts_with("\"") && txid.ends_with("\"") {
  //         txid = txid[1..txid.len()-1].to_string();
  //     }

  //     Ok(txid)
  // } else {
  //     Err(Box::from(format!("Error: {:?}", sendresponse)))
  // }
}



// -------------------------------------
// Other stubs
// -------------------------------------


// #[derive(std::fmt::Display)]
// #[derive(Debug)]
// pub struct LightdInfo {
//   hello: String
// }


// impl <IntoJsResult>  LightdInfo {
    
// }
// impl <IntoJsResult> for LightdInfo {
    
// }

// is this the syntax?
// impl IntoJsResult for LightdInfo {
    
// }
// is this the syntax?
impl IntoJsResult for LightdInfo {
 fn into_js_result(self) -> Result<JsValue, JsValue> {
   Ok(JsValue::undefined())
 }   
}