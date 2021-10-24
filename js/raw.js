const dec = new TextDecoder();
const enc = new TextEncoder();

class WASMWrapper {
  constructor({ instance }) {
    this.instance = instance;
  }

  invoke(funName, value) {
    const fun = this.instance.exports[funName];
    window.performance.mark("start_ser");
    const valuePtr = this.copyStringToWasm(JSON.stringify(value));
    window.performance.mark("finish_ser");
    const retPtr = fun(valuePtr);
    window.performance.mark("finish_fun");
    const s = this.copyStringFromWasm(retPtr);
    let ret = JSON.parse(s);
    window.performance.mark("finish_deser");
    window.performance.measure("ser_time", "start_ser", "finish_ser");
    window.performance.measure("fun_time", "finish_ser", "finish_fun");
    window.performance.measure("deser_time", "finish_fun", "finish_deser");
    return ret;
  }

  copyStringFromWasm(jsInteropStr) {
    const { memory, stringData, stringLen } = this.instance.exports;
    const buf = new Uint8Array(
      memory.buffer,
      stringData(jsInteropStr),
      stringLen(jsInteropStr)
    );
    return dec.decode(buf);
  }

  copyStringToWasm(jsString) {
    const { memory, stringPrepare, stringData } = this.instance.exports;

    const encodedString = enc.encode(jsString);

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
    modules[moduleName] = new WASMWrapper(mod);
  }
  return modules[moduleName];
};

export async function invoke(moduleName, funName, value) {
  window.performance.mark("start_loading_mod");
  const mod = await loadModule(moduleName);
  window.performance.mark("loaded_mod");
  let ret = mod.invoke(funName, value);
  window.performance.mark("finished_funcall");
  window.performance.measure("load_time", "start_loading_mod", "loaded_mod");
  window.performance.measure("fn_time", "loaded_mod", "finished_funcall");
  return ret;
}

export async function difference(value) {
  let ret = await invoke("fib_bg", "difference", value);
  return ret;
}
