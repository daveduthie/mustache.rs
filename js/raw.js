class WASMWrapper {
  constructor(instance) {
    this.instance = instance;
    this.decoder = new TextDecoder();
    this.encoder = new TextEncoder();
  }

  invoke(funName, value) {
    const fun = this.instance.exports[funName];
    const valuePtr = this.copyStringToWasm(JSON.stringify(value));
    const s = this.copyStringFromWasm(fun(valuePtr));
    return JSON.parse(s);
  }

  copyStringFromWasm(jsInteropStr) {
    const { memory, stringData, stringLen } = this.instance.exports;
    const buf = new Uint8Array(
      memory.buffer,
      stringData(jsInteropStr),
      stringLen(jsInteropStr)
    );
    return this.decoder.decode(buf);
  }

  copyStringToWasm(jsString) {
    const { memory, stringPrepare, stringData } = this.instance.exports;

    const encodedString = this.encoder.encode(jsString);

    // Ask WASM code to allocate a string inside of the module's memory
    const wasmStr = stringPrepare(encodedString.length);

    // Get a JS view of the string data
    const rustStringData = stringData(wasmStr);
    const asBytes = new Uint8Array(
      memory.buffer,
      rustStringData,
      encodedString.length
    );

    // Copy the UTF-8 into the WASM memory.
    asBytes.set(encodedString);

    return wasmStr;
  }
}

const modules = {};

const loadModule = async (moduleName) => {
  if (!modules[moduleName]) {
    const mod = await WebAssembly.instantiateStreaming(
      fetch(`${moduleName}.wasm`)
    );
    modules[moduleName] = mod;
  }

  return new WASMWrapper(modules[moduleName].instance);
};

export async function doMaths() {
  const fib = await loadModule("fib_bg");
  const result = fib.invoke("difference", { a: 1000, b: 200 });
  console.log(JSON.stringify({ result }, null, 2));
}
