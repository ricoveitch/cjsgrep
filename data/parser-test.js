function abc(a) {
  //xxxx
  pin.abc_outer = 1;
}

const arrow = () => {
  moo(pin);
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
  const abc = (pin) => {
    let pin = c;
  };
  arrow();
}

foo();
