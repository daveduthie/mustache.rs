import { Mustache, set_context, init_panic_hook } from "../mustache/pkg/index.js";

// init_panic_hook();

const templates = [
    Mustache.new("this is the thing: {{ calc.b.c }}, more stuff here: {{ other.stuff }}"),
    Mustache.new("this is x.y.z: {{ x.y.z }}, but I also want {{ calc.b.c }}"),
    Mustache.new("this doesn't exist: {{ calc.dontexist }}"),
];

const importObject = {
    imports: { imported_func: arg => console.log(arg) }
};

const userCalcs = {
    "b": function(_ctx) {
        console.log("Fun b evaluated")
        return { c: "bananas" }
    },
    "ifail": function(_ctx) {
        throw new Error("I fail")
    }
}

const initial_ctx = {
    other: { stuff: true },
    x: { y: { z: 123 } }
}

const calcPrefix = ["calc"];
const calcResults = {};

templates.map((m) => {
    m.deps(calcPrefix).map((dep) => {
        if (!calcResults[dep]) {
            const fun = userCalcs[dep];
            if (fun) {
                const res = fun(initial_ctx);
                calcResults[dep] = res;
            }

        }
    })
})

initial_ctx["calc"] = calcResults;
set_context(initial_ctx);

const rendered = templates.reduce((acc, m) => { return acc + m.render() + "\n" }, "")
const app = document.getElementById("app");
app.innerText = rendered;

// let i = 0;

// while (i++ < 10) {
//     performance.mark("start");
//     let m1 = Mustache.new("this is the thing: {{ a.b.c }}");
//     performance.mark("parsed");
//     let rendered1 = m1.render(ctx);
//     performance.mark("rendered");
//     performance.measure("parseTime", "start", "parsed");
//     performance.measure("renderTime", "parsed", "rendered");
// }

// const funcName = "exported_func";
// const moduleName = "simple"
// const modules = {};

// const loadModule = async (moduleName) => {
//     if (!modules[moduleName]) {
//         const mod = await WebAssembly
//             .instantiateStreaming(fetch(`${moduleName}.wasm`), importObject);
//         modules[moduleName] = mod;
//     }
// }

// const invokeFunc = async (moduleName, funcName) => {
//     await loadModule(moduleName);
//     const func = modules[moduleName].instance.exports[funcName];
//     console.log({modules});
//     return func();
// }

// invokeFunc("simple", "exported_func");
// invokeFunc("fib_bg", "fib");