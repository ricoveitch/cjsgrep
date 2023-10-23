function baz(obj) {
  obj.baz = 1;
  return obj;
}

function double(obj) {
  obj.double = 1;
}

function takeUpSpaceWithFunctionName() {
  return;
}

function takeUpSpaceWithFunctionName2() {
  return;
}

module.exports = {
  baz,
  double,
  takeUpSpaceWithFunctionName,
  takeUpSpaceWithFunctionName2,
};
