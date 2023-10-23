const {
  baz,
  double,
  takeUpSpaceWithFunctionName,
  takeUpSpaceWithFunctionName2,
} = require("./baz");
const { fixed } = require("./tar");
const { qux, xyz } = require("./qux");
const lar = require("./lar");

function foo() {
  let x = {};
  bar(x);
  x.x = 1;
  fixed(x);
  double(x);
  lar.foo(x);
  qux(x);
}

function bar(obj) {
  obj.bar = 2;
  baz(obj);
}
