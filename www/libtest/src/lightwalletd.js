import {
  Empty,
  LightdInfo,
  ChainSpec,
  RawTransaction,
  TransparentAddressBlockFilter,
  TxFilter,
  BlockRange,
  BlockID,
  PriceRequest,
  // TreeState,
  CompactTxStreamerPromiseClient,
} from './proto/service_grpc_web_pb';

import { TreeState } from './proto/service_pb'
import hex from 'hex-string';

// const ENVOY_PROXY="http://localhost:8080";
// const ENVOY_PROXY = 'http://23.146.144.13:8080';
// const ENVOY_PROXY = 'https://lightwalletd.zecwallet.co:443';
const ENVOY_PROXY = 'http://yamamoto:8080';

const assert = ()=>{};

function isBigIntWhichIsSafe(b){
  if ('bigint' !== typeof b){
    throw new Error(`Parameter unixTimestamp must be of type BigInt, actual type ${typeof b}`);
  }
  const isAboveMin =  Number.MIN_SAFE_INTEGER <= b;
  const isBelowMax =  b <= Number.MAX_SAFE_INTEGER;
  return isAboveMin && isBelowMax;
}

function  fromBigIntToSafeNumber(b){
  if ( !isBigIntWhichIsSafe(b) ){
    throw new Error(`Cannot convert BigInt to Number. The BigInt you supplied ${b} must be within [Number.MIN_SAFE_INTEGER, Number.MAX_SAFE_INTEGER] range.`);
  }
  return Number(b);
}

window.unix = function(){
  return Math.floor(Date.now()/1000);
}

window.hex2a = function hex2a(hex) {
  var str = '';
  for (var i = 0; i < hex.length; i += 2) str += String.fromCharCode(parseInt(hex.substr(i, 2), 16));
  return str;
};

/**
 * 
 * @returns {CompactTxStreamerPromiseClient}
 */
function client(){
  // TODO: should I wrap it around in some 'logging' wrapper or smth? 
  const client = new CompactTxStreamerPromiseClient(ENVOY_PROXY);
  
  return client;
}

/**
 * @param {*} resp 
 * @returns {String} hex string
 */
function asHex(resp){
  const binSerialized = resp.serializeBinary();
  const hexSerialized = hex.encode(binSerialized);
  return hexSerialized;
}

export async function fake_getInfo() {
  // return some hex string which should be 
  // serialized into LightdInfo struct...

  const info = new LightdInfo();
  
  
  info.setVersion('v42');
  info.setVendor('vnd001');
  info.setTaddrsupport(true);
  info.setChainname('CHN001');
  info.setSaplingactivationheight(100000);
  info.setConsensusbranchid("cnsns-branch");
  info.setBlockheight(200000);
  info.setGitcommit('010203040a0b0c');
  info.setBranch('some-branch');
  info.setBuilddate('20220101'); // maybe in different format
  info.setBuilduser('JoeBloggs');
  info.setEstimatedheight(300000);
  info.setZcashdbuild('v3.0Z'); // maybe in different format
  info.setZcashdsubversion('vv11.00'); // maybe in different format


  // info.setHello('Jupiter');
  // info.setAge(42);
  const binSerialized = info.serializeBinary();
  const hexSerialized = hex.encode(binSerialized);
  
  return hexSerialized;
  // return "DEADBEEF";
}


export async function getInfo() {
  try {
    const client = new CompactTxStreamerPromiseClient(ENVOY_PROXY);
    const request = new Empty();
    // generated files are not very ts-friendly /** @type {LightdInfo} */
    const resp = await client.getLightdInfo(request, {});

    assert(null !== resp, 'Response cannot be null');
    assert(resp instanceof LightdInfo,'Response must be instance of LightdInfo');
    console.log(`RPC response: `, resp);


    const hexBin = hex.encode(resp.serializeBinary());

    return hexBin;
  } catch (err) {
    console.log('grpc error(getInfo)', err);
    throw err;
  }
}



/**
 *
 * @param {String|{ blockHeight: Number, txIndex : Number}} txhex 
 *                       hex string representing transaction hash 
 *                       (64 bytes hex-string) representing
 *                        32 byte transaction hash,
 *                        or "block coordinates" as tuple-like object
 *                        of { blockHeight, txIndex }
 * @returns {String} hexadecimal representation of binary payload
 */
export async function getFullTx(txhex) {
  try {
    const request = new TxFilter();

    if ( 'string' === typeof txhex ){
      console.warn(`getFullTx(${txhex})`);
      request.setHash(hex.decode(txhex).reverse());
    }
    else{
      // looks like currenlty lightwalletd (at least master) 
      // does NOT support querying  by blockHeight & txIndex
      const { blockHeight, txIndex } = txhex;
      console.warn(
        `Querying transaction by 'block coordinates' tx(${blockHeight}[${txIndex}]) is unlikely ` 
      + `supported by your lightwalletd server. Check documentation or implementation for ` 
      + `the specific build of your lightwalletd server.`
      );
      let _block = new BlockID();
      _block.setHeight(blockHeight);
      request.setBlock(_block);
      request.setIndex(txIndex);
    }

    const resp = await client().getTransaction(request, {});
    console.log(`getFullTx(${hex.decode(txhex)}): `, resp.toObject());
    const hexBin = hex.encode(resp.serializeBinary());

    return hexBin;
  } catch (err) {
    console.log('grpc error(getInfo)', err);
    throw err;
  }
}

// Returns a single string, delimited by "\n", each row contains a hex-encoded block
export function getTransparentTxids(address, startHeight, endHeight) {
  return new Promise((resolve, reject) => {
    try {
      debugger;
      const startBlock = new BlockID();
      startBlock.setHeight(fromBigIntToSafeNumber(startHeight));
      const endBlock = new BlockID();
      endBlock.setHeight(fromBigIntToSafeNumber(endHeight));

      const blockRange = new BlockRange();
      blockRange.setStart(startBlock);
      blockRange.setEnd(endBlock);

      const request = new TransparentAddressBlockFilter();
      request.setRange(blockRange);
      request.setAddress(address);

      const retValue = [];

      const resp = client().getTaddressTxids(request);
      resp.on('data', (resp) => {
        console.log('transaction: ', resp.toObject());
        const hexBin = hex.encode(resp.serializeBinary());
        retValue.push(hexBin);
      });

      resp.on('error', (e) => {
        console.log('get Transparent Txids error:', e);
        reject(e);
      });

      resp.on('end', () => {
        resolve(retValue.join('\n'));
      });
    } catch (err) {
      console.log('grpc error (getTransparentTxids)', err);
      reject('grpc error (getTransparentTxids)' + err);
    }
  });
}

// // returns??? what  (if it returns txids?)
// export async function getAddressTxids(address){
//   // how to handle address?
//   // return asHex(await client().getAddressTxids(req));
//   const req = new Empty();
//   // I would need a different request here...
//   const resp = await client().getAddressTxids(req);

//   // and how do we handle the response? 
//   // as single value? 
//   // or as stream of data? 
//   // and what is the return value? 
//   // isn't this the same as ??? 
// }  

// Returns a single string, delimited by "\n", each row contains a hex-encoded block
export function getBlockRange(startHeight, endHeight) {
  return new Promise((resolve, reject) => {
    console.log('JS getBlockRange', startHeight, ' to ', endHeight);
    try {
      const startBlock = new BlockID();
      startBlock.setHeight(Number(startHeight));
      const endBlock = new BlockID();
      endBlock.setHeight(Number(endHeight));

      const blockRange = new BlockRange();
      blockRange.setStart(startBlock);
      blockRange.setEnd(endBlock);

      const retValue = [];

      const resp = client().getBlockRange(blockRange);
      resp.on('data', (resp) => {
        const hexBin = hex.encode(resp.serializeBinary());
        retValue.push(hexBin);
      });

      resp.on('error', (e) => {
        console.log('Block Range error', e);
        reject(e);
      });

      resp.on('end', () => {
        resolve(retValue.join('\n'));
      });
    } catch (err) {
      console.log('grpc error (getLatestBlock)', err);
      reject('grpc error (getLatestBlock)' + err);
    }
  });
}

export async function getLatestBlock() {
  try {
    const request = new ChainSpec();
    const resp = await client().getLatestBlock(request, {});

    console.log('getLatestBlock = ', resp.getHeight());

    const hexBin = hex.encode(resp.serializeBinary());

    return hexBin;
  } catch (err) {
    console.log('grpc error (getLatestBlock)', err);
    throw err;
  }
}

export async function sendTransaction(txHex) {
  try {
    const client = new CompactTxStreamerPromiseClient(ENVOY_PROXY);

    const request = new RawTransaction();
    request.setData(hex.decode(txHex));

    const resp = await client.sendTransaction(request, {});

    console.log('send Transaction = ', resp.getErrorcode(), ':', resp.getErrormessage());

    const hexBin = hex.encode(resp.serializeBinary());

    return hexBin;
  } catch (err) {
    console.log('grpc error (sendTransaction)', err);
    throw err;
  }
}

/**
 * 
 * @param {BigInt} unixTimestamp 
 * @param {String} currency fiat currency to receive ZEC price in, eg 'USD'
 * @returns {String} hex-representation of binary struct data
 */
export async function getZecPrice(unixTimestamp, currency){
  // const unixTimestamp = Math.floor(Date.now() / 1000);
  // if ('bigint' !== typeof unixTimestamp){
  //   throw new Error(`Parameter unixTimestamp must be of type BigInt, actual type ${typeof unixTimestamp}`);
  // }
  debugger;
  const request = new PriceRequest();
  request.setCurrency(currency);
  request.setTimestamp(fromBigIntToSafeNumber(unixTimestamp));
  const res = await client().getZECPrice(request);
  return asHex(res);
}

export async function getCurrentZecPrice(){
  try{

    const client = new CompactTxStreamerPromiseClient(ENVOY_PROXY);
    const request = new Empty();
    
    const resp = await client.getCurrentZECPrice(request);
    
    console.log(`RPC response: `, resp);
    const binSerialized = resp.serializeBinary();
    const hexSerialized = hex.encode(binSerialized);
    
    return hexSerialized;
  }
  catch(err){
    console.error(`grpc error::getCurrentZecPrice()`, err);
    throw err;
  }

}

/**
 * 
 * @param {Number} blockHeight 
 * @param {String} blockHashAsHexString 
 * @returns 
 */
export async function getTreeState(blockHeight, blockHashAsHexString){

  debugger;
  const req = new BlockID();
  // TODO: as those parameters will be called by wasm, they will ALWAYS be set
  // the question is .... what values can they be? (should they be Options?)
  if ( !blockHeight && !blockHashAsHexString ){
    throw new Error(`At least one of the argumnets ({ blockHeight, blockHashAsHexString }) must be set`);
  }

  blockHeight && req.setHeight(blockHeight);
  blockHashAsHexString && req.setHash(blockHashAsHexString);

  // req.setHash(); // or maybe we don't set anything?
  // req.setHash('01020a..hexstring');
  // const uint8Array = Uint8Array(/* how to instantiate one */); /
  // req.setHash(uint8Array);

  // protobuf types do NOT use constructor parameters, but fluent setters instead
  // req.setBlockId(new BlockID({ 
  //   /*?*/height: 123, 
  //   /*?*/hash: [/* bytes */],  
  // }))

  /** @type {TreeState} res */
  const res = await client().getTreeState(req);

  return asHex(res);

  // res.getNetwork(); // string
  // res.getHeight();
  // res.getHash();
  // res.getTime();
  // res.getTree(); // merkle tree root hash hex string

}

export function doLog(value) {
  console.log('doLog', value);
}
