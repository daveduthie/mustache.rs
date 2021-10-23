import { Mustache, set_context, init_panic_hook } from "../mustache/pkg/index.js";
import { doMaths } from "./raw.js"

doMaths();

// init_panic_hook();

const templates = [
    Mustache.new("this is the thing: {{ calc.b.c }}, more stuff here: {{ other.stuff }}"),
    Mustache.new("this is x.y.z: {{ x.y.z }}, but I also want {{ calc.b.c }}"),
    Mustache.new("this doesn't exist: {{ calc.dontexist }}"),
];

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
