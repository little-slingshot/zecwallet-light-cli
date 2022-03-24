// export all functions, but we will implement them via JS 
// first we can do it with stubs 

// ------------------------------------
// import from js world 
// ------------------------------------

//-------------------------------------------
fn getInfo() -> js_sys::Promise {
  return js_sys::Promise::resolve(&42.into());
}

fn getFullTx(txhash: String) -> js_sys::Promise {
  return js_sys::Promise::resolve(&42.into());
}

fn getLatestBlock() -> js_sys::Promise {
  return js_sys::Promise::resolve(&42.into());
}

fn getBlockRange(start: u64, end: u64) -> js_sys::Promise {
  return js_sys::Promise::resolve(&42.into());
}

fn getTransparentTxidx(address: String, start: u64, end: u64) -> js_sys::Promise {
  return js_sys::Promise::resolve(&42.into());
}

fn sendTransaction(txhex: String) -> js_sys::Promise {
  return js_sys::Promise::resolve(&42.into());
}

fn doLog(s: &str){
  println!("{}", s);
}


//-------------------------------------------


pub fn get_info(uri: &http::Uri) -> Result<LightdInfo, String> {
  // how can I implement this .
  // now the question is: 
  // how can I call an async function, right? 

}