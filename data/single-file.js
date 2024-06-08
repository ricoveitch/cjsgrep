function abc(a) {
  //xxxx
  pin.abc_outer = 1;
}

const arrow = () => {
  moo(pin);
};

const bar = () => {
  // pin bar
};

function foo() {
  if (true) {
    const abc = () => {
      let pin = a;
    };
    abc();
  }
  let pin = b;
  abc(pin);
  //pin
  // bar()
  const abc = (pin) => {
    let pin = c;
  };
  arrow();
}

foo();
