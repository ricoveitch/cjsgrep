function baz(obj) {
  obj.baz = 1;
  return obj;
}

function baz2(obj) {
  obj.baz = 2;
}

function takeUpSpaceWithFunctionName() {
  return;
}

function takeUpSpaceWithFunctionName2() {
  return;
}

module.exports = {
  baz,
  double: baz2,
  takeUpSpaceWithFunctionName,
  takeUpSpaceWithFunctionName2,
};
