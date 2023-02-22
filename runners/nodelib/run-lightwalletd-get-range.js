global.window = global;
global.XMLHttpRequest = require("w3c-xmlhttprequest").XMLHttpRequest;

const lightwalletd = require('../wwwlib/src/lightwalletd'); // for web build, webpack needs it to be under src/


(async ()=>{


    const range = await lightwalletd.getBlockRange(1240001, 1452147);

})().catch(console.error);