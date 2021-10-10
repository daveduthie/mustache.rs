import { Mustache, set_context, init_panic_hook } from "../pkg/index.js";

// init_panic_hook();

const templates = [
    Mustache.new("this is the thing: {{ calc.b.c }}, more stuff here: {{ other.stuff }}"),
    Mustache.new("this is x.y.z: {{ x.y.z }}, but I also want {{ calc.b.c }}"),
    Mustache.new("this doesn't exist: {{ calc.dontexist }}"),
];

const userCalcs = {
    "b": function (_ctx) {
        console.log("Fun b evaluated")
        return { c: "bananas" }
    },
    "ifail": function (_ctx) {
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

initial_ctx.calc = calcResults;
set_context(initial_ctx);

templates.map((m) => { console.log(m.render()) })

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