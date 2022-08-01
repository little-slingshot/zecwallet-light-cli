import { log } from './logging'
import { expectStringEqual as expect } from './expect'
import * as lightwalletd from './lightwalletd'

window.onload = ()=>{
    main().catch(e=>{
      log(e);
      throw e;
    })
};

async function main(){
  console.log(`Starting wwwlibtest harness...`);
  console.log(`lightwalletd:`, lightwalletd);
  Object.assign(window, lightwalletd);  


  const wasmImport = await import('../pkg/zecwalletlitelib');

  console.log(`wasmImport:`, wasmImport);
  console.log(`Object.keys(wasmImport):`, Object.keys(wasmImport));
  const litelibFunctions = Object.keys(wasmImport).filter(fname=>String(fname).startsWith('litelib_'));
  litelibFunctions.forEach(fname=>{
    log(`${fname}()`);
  })

  const litelib_execute = wasmImport.litelib_execute;
  const litelib_initialize_existing = wasmImport.litelib_initialize_existing;
  const litelib_initialize_new = wasmImport.litelib_initialize_new;
  const litelib_initialize_new_from_phrase = wasmImport.litelib_initialize_new_from_phrase;

  // alert('Loaded');

  await expect(
    // generate random hex using `cat /dev/urandom | head -c32 | xxd -p -c 10000`
    async ()=>await litelib_initialize_new('556809f91dadf7ca8aac1fa20eab54f87a0fe9a3e76ca711b3d5d61cdf2ade61'), // (entropy: String)
    '{"seed":"fever doctor layer deputy term torch click loop pear inspire steel valley path visit moon unique poem mirror voice promote social nice kangaroo crazy","birthday":1758140}'
  );
  
  // We actually do not expect this to return corectly, as such a command does not exist...
  // await expect(
  //   async ()=>await litelib_execute('dir', '-w -s -something'), // (cmd: String, args_list: String)
  //   `Running litelib_execute('dir') command`
  // );


  // We actually do not expect this to return corectly, as such a command does not exist...
  await expect(
    async ()=>await litelib_execute('info', 'no-params'), // (cmd: String, args_list: String)
    `Running litelib_execute('info') command`
  );



  // We actually do not expect this to return corectly, as such a command does not exist...
  await expect(
    async ()=>await litelib_execute('update_historical_prices', 'no-params'), // (cmd: String, args_list: String)
    `Running litelib_execute('update_historical_prices') command`
  );


  // those two are dummies anyways...
  // await expect(
  //   async ()=>await litelib_initialize_existing('324124bacc'), // (wallet_hex: String)
  //   'Initialize a new lightclient and store its value'
  // );



  // await expect(
  //   async ()=>await litelib_initialize_new_from_phrase('seed-string', 100000n), // (seed: String, birthday: u64)
  //   'Restore a wallet from the seed phrase'
  // );
}



