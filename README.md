# into-a-byte

Build ORed byte from members of tuple of Into<u8> that acts as a bit switch.

# Example

Built byte is used to send to a I2C devise for example.

```rs
use into_a_bit::*;

enum FunctionA {
    Enabled = 1 << 3,
    Disabled = 0,
}

enum FunctionB {
    Enabled = 1 << 2,
    Disabled = 0,
}

enum FunctionC {
    Enabled = 1 << 1,
    Disabled = 0,
}

enum FunctionD {
    Enabled = 1,
    Disabled = 0,
}

enums_into_u8!(FunctionA, FunctionB, FunctionC, FunctionD);

fn send_to_device(value: (FunctionA, FunctionB, FunctionC, FunctionD)) {
    // A byte for send to a register for example.
    let byte = value.into_a_byte();
    // TODO
}

fn main() {
    send_to_device((
        FunctionA::Enabled,
        FunctionB::Disabled,
        FunctionC::Enabled,
        FunctionD::Enabled,
    ));
}
```
