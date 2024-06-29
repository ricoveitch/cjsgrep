const { baz } = require("./mixed/baz");
const { double: d } = require("./mixed/baz");

function moo() {
  //
}

function foo() {
  moo();
  baz();
  if (something) {
    d();
  }
}
