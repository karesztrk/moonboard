#[rustfmt::skip]
pub const MODEL_LAYOUT: [[usize; 14]; 6] = [
    [  0,  1,  2,  3,  4,  5,  6,      36, 37, 38, 39, 40, 41, 42 ],
    [  7,  8,  9, 10, 11, 12, 13,      43, 44, 45, 46, 47, 48, 49 ],
    [ 14, 15, 16, 17, 18, 19, 20,      50, 51, 52, 53, 54, 55, 56 ],
    [ 21, 22, 23, 24, 25, 26, XX,      XX, 57, 58, 59, 60, 61, 62 ],
    [ 27, 28, 29, 30, 31, XX, XX,      XX, XX, 63, 64, 65, 66, 67 ],
    [ XX, XX, XX, 32, 33, 34, 35,      68, 69, 70, 71, XX, XX, XX ],
];
pub const XX: usize = 99;
pub const NA: char = '$';
#[rustfmt::skip]
pub const QWERTY_LAYOUT: [[char; 14]; 6] = [
    [ NA,  NA,  NA,  NA,  NA,  NA, NA,     NA,  NA,  NA,  NA,  NA,  NA, NA, ],
    [ NA, 'q', 'w', 'e', 'r', 't', NA,     NA, 'z', 'u', 'i', 'o', 'p', NA, ],
    [ NA, 'a', 's', 'd', 'f', 'g', NA,     NA, 'h', 'j', 'k', 'l', 'é', NA, ],
    [ NA, 'x', 'y', 'c', 'v', 'b', NA,     NA, 'm', 'n', ',', '.', '-', NA, ],
    [ NA,  NA,  NA,  NA,  NA,  NA, NA,     NA,  NA,  NA,  NA,  NA,  NA, NA, ],
    [ NA,  NA,  NA,  NA,  ' ',  NA, NA,     NA,  NA,  NA,  NA,  NA,  NA, NA, ],
];

#[rustfmt::skip]
pub const NORMAN_LAYOUT: [[char; 14]; 6] = [
    [ NA,  NA,  NA,  NA,  NA,  NA, NA,     NA,  NA,  NA,  NA,  NA,  NA, NA, ],
    [ NA, 'q', 'w', 'd', 'f', 'k', NA,     NA, 'j', 'u', 'r', 'l', 'é', NA, ],
    [ NA, 'a', 's', 'e', 't', 'g', NA,     NA, 'y', 'n', 'i', 'o', 'h', NA, ],
    [ NA, 'z', 'x', 'c', 'v', 'b', NA,     NA, 'p', 'm', ',', '.', '-', NA, ],
    [ NA,  NA,  NA,  NA,  NA,  NA, NA,     NA,  NA,  NA,  NA,  NA,  NA, NA, ],
    [ NA,  NA,  NA,  NA,  ' ',  NA, NA,     NA,  NA,  NA,  NA,  NA,  NA, NA, ],
];
