const { baz: bazz } = require("./mixed/baz");
const { double } = require("./mixed/baz");
const bazi = require("./mixed/baz");

function moo() {
  //
}

function foo() {
  moo();
  bazz();
  if (something) {
    double();
  }
}
