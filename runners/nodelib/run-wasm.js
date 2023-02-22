const log = console.log;
global.XMLHttpRequest = require("w3c-xmlhttprequest").XMLHttpRequest;
// global.XMLHttpRequest = require("xmlhttprequest").XMLHttpRequest; // this one can't seem to return even single request

global.window = global;
const lightwalletd = require('../wwwlib/src/lightwalletd'); // for web build, webpack needs it to be under src/

const wasmImport = require('./pkg.node/zecwalletlitelib');
const assert = require('node:assert');
const expect = require('./expect').expectStringEqual;

(async () => {
  console.log(`Starting lwd-runner for NodeJS`);
  console.log(`lightwalletd:`, lightwalletd);
  Object.assign(global, lightwalletd);

  const {
    add,
    add_magic,
    get_info,
  } = wasmImport;


  console.log(`wasmImport:`, wasmImport);
  console.log(`Object.keys(wasmImport):`, Object.keys(wasmImport));


  const litelibFunctions = Object.keys(wasmImport).filter(fname => String(fname).startsWith('litelib_'));
  litelibFunctions.forEach(fname => {
    log(`${fname}()`);
  })




  const litelib_execute = (...args) => {
    console.log(`calling litelib_execute(${args[0]}, ${args[1]})...`);
    return wasmImport.litelib_execute(...args);
  }
  const litelib_initialize_existing = wasmImport.litelib_initialize_existing;
  const litelib_initialize_new = wasmImport.litelib_initialize_new;
  const litelib_initialize_new_from_phrase = wasmImport.litelib_initialize_new_from_phrase;



  await expect(
    // generate random hex using `cat /dev/urandom | head -c32 | xxd -p -c 10000`
    async () => await litelib_initialize_new_from_phrase(
      // (seed: String, birthday: u64)
      // "fever doctor layer deputy term torch click loop pear inspire steel valley path visit moon unique poem mirror voice promote social nice kangaroo crazy",
      "slot deal enter foot glare autumn hammer large point toss skate machine skate tube assume float strategy grant pair start gorilla jazz bullet chicken",
      // BigInt(1624864-10000),
      BigInt(1462247 - 10000), // block 1462247 is where our test transaction is located
    ),
    // '{"seed":"fever doctor layer deputy term torch click loop pear inspire steel valley path visit moon unique poem mirror voice promote social nice kangaroo crazy","birthday":1758140}'
    '{"seed":"slot deal enter foot glare autumn hammer large point toss skate machine skate tube assume float strategy grant pair start gorilla jazz bullet chicken","birthday":1758140}'
  );

  // We actually do not expect this to return corectly, as such a command does not exist...
  await expect(
    async ()=>await litelib_execute('info', 'no-params'), // (cmd: String, args_list: String)
    `Running litelib_execute('info') command`
  );



  await expect(
    async ()=>await litelib_execute('sync_internal', 'no-params'), // (cmd: String, args_list: String)
    `Running litelib_execute('sync_internal') command`
  );  


})().catch(console.error);
