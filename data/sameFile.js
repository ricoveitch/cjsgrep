function foo() {
  let obj = {};
  let x = bar(obj);
  obj.x = x;
}

function bar(obj) {
  obj.y = 2;
  return 1;
}
