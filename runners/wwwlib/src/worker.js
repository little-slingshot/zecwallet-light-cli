// Polyfill
if (!WebAssembly.instantiateStreaming){
    /**
     * 
     * @param {Promise<Response>} pResp 
     * @param {*} importObject 
     * @returns {Promise<WebAssembly.WebAssemblyInstantiatedSource>}
     */
    WebAssembly.instantiateStreaming = async (pResp, importObject) => {
        const resp = await pResp;
        const source = await resp.arrayBuffer();
        return await WebAssembly.instantiate(source, importObject);
    }
}


let wasmResolve;
let wasmReady = new Promise(resolve=>{
    wasmResolve = resolve;
})


self.addEventListener('message',(event)=>{
    const { eventType, eventData, eventId } = event;
    // post("INITIALIZE", { data: "http://www.example.org/mywasm.wasm"});
    if ( "INITIALISE" === eventType ){
        WebAssembly.instantiateStreaming(fetch(eventData), { /* why empty */})
        .then(instantiatedModule=>{
            const wasmExports = instantiatedModule.instance.exports;

            wasmResolve(wasmExports);
        })

        self.postMessage({
            eventType: "INITIALISED",
            eventData: Object.keys(wasmExports),
        });
    }
    else if ("CALL" === eventType){
        wasmReady.then(wasmExports=>{
            const method = wasmExports[eventData.method];
            const result = /* that's a sync call */ method.apply(null, eventData.arguments);
            self.postMessage({
                eventType: 'RESULT',
                eventData: result,
                eventId,
            });
        })
        .catch(error=>{
            self.postMessage({
                eventType: 'ERROR',
                // TODO: maybe use serializeError() to JSON package?
                eventData: 'An error occured executing WASM instance function ' + error.toString(),
                
                eventId,
            })
        })
    }


}, false)