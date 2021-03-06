import { Mustache, set_context } from "../mustache/pkg/index.js";
import { difference } from "./raw.js";

const templates = [];

for (let i = 0; i < 1000; i++) {
  templates.push(
    Mustache.new(
      "this is the thing: {{ calc.somefn.theanswer }}, more stuff here: {{ other.stuff }}"
    )
  );
}

const userCalcs = {
  somefn: function (_ctx) {
    console.log("Fun b evaluated");
    return { theanswer: "bananas" };
  },
  ifail: function (_ctx) {
    throw new Error("I fail");
  },
};

const initial_ctx = {
  other: { stuff: true },
  x: { y: { z: 123 } },
};

const calcPrefix = ["calc"];
const calcResults = {};

const render = async () => {
  templates.map((m) => {
    m.deps(calcPrefix).map((dep) => {
      console.log({ dep });
      if (!calcResults[dep]) {
        const fun = userCalcs[dep];
        if (fun) {
          const res = fun(initial_ctx);
          calcResults[dep] = res;
        }
      }
    });
  });

  initial_ctx["calc"] = calcResults;
  set_context(initial_ctx);

  let rendered = templates.reduce((acc, m) => {
    return acc + m.render() + "\n";
  }, "");

  const big_json = await fetch("circlecistatus.json").then((req) => req.json());
  let d = await difference({ a: 1123, b: 2, ...big_json });
  let d2 = await difference({ a: 2, b: 1000, ...big_json });
  rendered += JSON.stringify(d, null, 2);
  rendered += "\n";
  rendered += JSON.stringify(d2, null, 2);
  rendered += "\n";
  const app = document.getElementById("app");
  app.innerText = rendered;
};

render();
