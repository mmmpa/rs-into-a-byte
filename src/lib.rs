#![no_std]

/// Build ORed byte from members of tuple of Into<u8> that acts as a bit switch.
///
/// # Example
///
/// Built byte is used to send to a I2C devise for example.
///
/// ```
/// use into_a_byte::*;
///
/// enum FunctionA {
///     Enabled = 1 << 3,
///     Disabled = 0,
/// }
///
/// enum FunctionB {
///     Enabled = 1 << 2,
///     Disabled = 0,
/// }
///
/// enum FunctionC {
///     Enabled = 1 << 1,
///     Disabled = 0,
/// }
///
/// enum FunctionD {
///     Enabled = 1,
///     Disabled = 0,
/// }
///
/// enums_into_u8!(FunctionA, FunctionB, FunctionC, FunctionD);
///
/// fn send_to_device(value: (FunctionA, FunctionB, FunctionC, FunctionD)) {
///     // A byte for send to a register for example.
///     let byte = value.into_a_byte();
///
///     // TODO
/// }
///
/// fn main() {
///     send_to_device((
///         FunctionA::Enabled,
///         FunctionB::Disabled,
///         FunctionC::Enabled,
///         FunctionD::Enabled,
///     ));
/// }
/// ```
pub trait IntoAByte {
    fn into_a_byte(self) -> u8;
}

impl<A: Into<u8>> IntoAByte for (A,) {
    fn into_a_byte(self) -> u8 {
        let (a,) = self;
        a.into()
    }
}

impl<A: Into<u8>, B: Into<u8>> IntoAByte for (A, B) {
    fn into_a_byte(self) -> u8 {
        let (a, b) = self;
        a.into() | b.into()
    }
}

impl<A: Into<u8>, B: Into<u8>, C: Into<u8>> IntoAByte for (A, B, C) {
    fn into_a_byte(self) -> u8 {
        let (a, b, c) = self;
        a.into() | b.into() | c.into()
    }
}

impl<A: Into<u8>, B: Into<u8>, C: Into<u8>, D: Into<u8>> IntoAByte for (A, B, C, D) {
    fn into_a_byte(self) -> u8 {
        let (a, b, c, d) = self;
        a.into() | b.into() | c.into() | d.into()
    }
}

impl<A: Into<u8>, B: Into<u8>, C: Into<u8>, D: Into<u8>, E: Into<u8>> IntoAByte
    for (A, B, C, D, E)
{
    fn into_a_byte(self) -> u8 {
        let (a, b, c, d, e) = self;
        a.into() | b.into() | c.into() | d.into() | e.into()
    }
}

impl<A: Into<u8>, B: Into<u8>, C: Into<u8>, D: Into<u8>, E: Into<u8>, F: Into<u8>> IntoAByte
    for (A, B, C, D, E, F)
{
    fn into_a_byte(self) -> u8 {
        let (a, b, c, d, e, f) = self;
        a.into() | b.into() | c.into() | d.into() | e.into() | f.into()
    }
}

impl<A: Into<u8>, B: Into<u8>, C: Into<u8>, D: Into<u8>, E: Into<u8>, F: Into<u8>, G: Into<u8>>
    IntoAByte for (A, B, C, D, E, F, G)
{
    fn into_a_byte(self) -> u8 {
        let (a, b, c, d, e, f, g) = self;
        a.into() | b.into() | c.into() | d.into() | e.into() | f.into() | g.into()
    }
}

impl<
        A: Into<u8>,
        B: Into<u8>,
        C: Into<u8>,
        D: Into<u8>,
        E: Into<u8>,
        F: Into<u8>,
        G: Into<u8>,
        H: Into<u8>,
    > IntoAByte for (A, B, C, D, E, F, G, H)
{
    fn into_a_byte(self) -> u8 {
        let (a, b, c, d, e, f, g, h) = self;
        a.into() | b.into() | c.into() | d.into() | e.into() | f.into() | g.into() | h.into()
    }
}

/// Helper to impl Into<u8> for enums.
///
/// ```
/// use into_a_byte::enums_into_u8;
///
/// enum FunctionA {
///     Enabled = 1 << 3,
///     Disabled = 0,
/// }
///
/// enum FunctionB {
///     Enabled = 1 << 2,
///     Disabled = 0,
/// }
///
/// enum FunctionC {
///     Enabled = 1 << 1,
///     Disabled = 0,
/// }
///
/// enum FunctionD {
///     Enabled = 1,
///     Disabled = 0,
/// }
///
/// enums_into_u8!(FunctionA, FunctionB, FunctionC, FunctionD);
/// ```
#[macro_export]
macro_rules! enums_into_u8 {
    ( $( $x:ident ),* ) => {
        $(
            impl Into<u8> for $x {
                fn into(self) -> u8 {
                    self as u8
                }
            }
        )*

    };
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn it_works() {
        #[allow(unused)]
        enum FunctionA {
            Enabled = 1 << 3,
            Disabled = 0,
        }

        #[allow(unused)]
        enum FunctionB {
            Enabled = 1 << 2,
            Disabled = 0,
        }

        #[allow(unused)]
        enum FunctionC {
            Enabled = 1 << 1,
            Disabled = 0,
        }

        #[allow(unused)]
        enum FunctionD {
            Enabled = 1,
            Disabled = 0,
        }

        enums_into_u8!(FunctionA, FunctionB, FunctionC, FunctionD);

        fn send_to_device(value: (FunctionA, FunctionB, FunctionC, FunctionD)) {
            assert_eq!(value.into_a_byte(), 0b0000_1011)
        }

        send_to_device((
            FunctionA::Enabled,
            FunctionB::Disabled,
            FunctionC::Enabled,
            FunctionD::Enabled,
        ))
    }
}
