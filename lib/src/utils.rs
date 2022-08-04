use log::Level;
use log::info;

pub fn set_panic_hook() {

  console_log::init_with_level(Level::Debug);
  info!("console_log::init_with_level(Level::Debug);");
  
  // When the `console_error_panic_hook` feature is enabled, we can call the
  // `set_panic_hook` function at least once during initialization, and then
  // we will get better error messages if our code ever panics.
  //
  // For more details see
  // https://github.com/rustwasm/console_error_panic_hook#readme
  info!("console_error_panic_hook::set_once();");
  //#[cfg(feature = "console_error_panic_hook")]
  console_error_panic_hook::set_once();
  

}
