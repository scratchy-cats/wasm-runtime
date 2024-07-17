#![allow(non_snake_case)]

use logging::setupLogging;
use module::Module;
use utils::dumpModuleAsHexString;

mod logging;
mod module;
mod utils;

fn main() {
  setupLogging();

  let modulePath = "./test/test.wasm";
  dumpModuleAsHexString(modulePath);

  // let _ = Module::new(&modulePath).unwrap();
}
