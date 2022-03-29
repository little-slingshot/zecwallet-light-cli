import { log } from './logging'
import { expectStringEqual as expect } from './expect'
window.onload = ()=>{
    main().catch(e=>{
      log(e);
      throw e;
    })
};

async function main(){
  for (let i = 0; i < 2; i++) {
    log(`hello ${i}`);
  }

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
    async ()=>await litelib_execute('dir', '-w -s -something'), // (cmd: String, args_list: String)
    'Execute litelib command'
  );

  await expect(
    async ()=>await litelib_initialize_existing('324124bacc'), // (wallet_hex: String)
    'Initialize a new lightclient and store its value'
  );

  await expect(
    async ()=>await litelib_initialize_new('123123098098dsfjladsfljasdfasdf'), // (entropy: String)
    'Create a new wallet and return the seed for the newly created wallet.'
  );

  await expect(
    async ()=>await litelib_initialize_new_from_phrase('seed-string', 100000n), // (seed: String, birthday: u64)
    'Restore a wallet from the seed phrase'
  );
}



