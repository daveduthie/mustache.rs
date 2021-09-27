import { Mustache } from "../pkg/index.js";

const m1 = Mustache.new("this is the thing: {{ x.y.z }}");
const rendered = m1.render({ x: { y: { z: "banana" } } });

console.log({ m1, rendered });