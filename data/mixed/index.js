const { fixed } = require("./tar");
const { qux, xyz } = require("./qux");

function bar(obj) {
  obj.bar = 2;
  return obj;
}

function foo() {
  let x = bar();
  fixed(obj);
}
