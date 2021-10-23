class QuickMaths {
  constructor(instance) {
    this.instance = instance;
  }

  difference(n1, n2) {
    const { compute } = this.instance.exports;
    const op = this.copyJsStringToRust("DIFF");
    return compute(op, n1, n2);
  }

  copyJsStringToRust(jsString) {
    const { memory, stringPrepare, stringData, stringLen } = this.instance.exports;

    const encoder = new TextEncoder();
    const encodedString = encoder.encode(jsString);

    // Ask Rust code to allocate a string inside of the module's memory
    const rustString = stringPrepare(encodedString.length);

    // Get a JS view of the string data
    const rustStringData = stringData(rustString);
    const asBytes = new Uint8Array(memory.buffer, rustStringData, encodedString.length);

    // Copy the UTF-8 into the WASM memory.
    asBytes.set(encodedString);

    return rustString;
  }
}

const modules = {};

const loadModule = async (moduleName) => {
    if (!modules[moduleName]) {
        const mod = await WebAssembly
            .instantiateStreaming(fetch(`${moduleName}.wasm`));
        modules[moduleName] = mod;
    }

    return modules[moduleName];
}

export async function doMaths() {

  const { instance } = await loadModule("fib_bg")
  const maffs = new QuickMaths(instance);

  console.log(maffs.difference(100, 201));
}
