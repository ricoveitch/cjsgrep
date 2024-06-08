function bar() {
  pin = bar;
}

function baz() {
  pin = baz;
}

function foo() {
  // baz()
  /** baz() */
  bar();
  /*
  baz()
   */

  /**
   *
   *
   * baz()
   */
}

foo();
