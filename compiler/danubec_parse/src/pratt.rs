/// Binding Power
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Bp(pub u8);

impl Bp {
    pub const P0: (Self, Self) = (Bp(0), Bp(0));
    pub const P1: (Self, Self) = (Bp(1), Bp(2));
    pub const P2: (Self, Self) = (Bp(3), Bp(4));
    pub const P3: (Self, Self) = (Bp(5), Bp(6));
    pub const P4: (Self, Self) = (Bp(7), Bp(8));
    pub const P5: (Self, Self) = (Bp(9), Bp(10));
    pub const P6: (Self, Self) = (Bp(11), Bp(12));
    pub const P7: (Self, Self) = (Bp(13), Bp(14));
    pub const P8: (Self, Self) = (Bp(15), Bp(16));
    pub const P9: (Self, Self) = (Bp(17), Bp(18));
}
