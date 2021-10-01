import { Mustache } from "../pkg/index.js";

const ctx = {
    a: { b: { c: "banana" } },
    x: { y: { z: { moar: ["stuff", 123] } } }
};

const m1 = Mustache.new("this is the thing: {{ a.b.c }}");
const rendered1 = m1.render(ctx);

const m2 = Mustache.new("this is the other: {{ x.y.z }}");
const rendered2 = m2.render(ctx);

console.log({ m1, rendered1 });
console.log({ m2, rendered2 });