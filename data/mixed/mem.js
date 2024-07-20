function bar() {
  obj = 1;
  baz();
}

function baz() {
  obj = 2;
}

function foo() {
  obj = 3;
}

module.exports = {
  bar,
  foo: foo,
};
