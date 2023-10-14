const { baz } = require("./baz");
const { fixed } = require("./tar");
const lar = require("./lar");

function foo() {
  let obj = {};
  let x = bar(obj);
  obj.x = x;
  baz(x);
  fixed(x);
  lar.lar(x);
}

function bar(obj) {
  obj.y = 2;
  return 1;
}
