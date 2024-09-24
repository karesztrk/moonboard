#[rustfmt::skip]
pub const MOONLANDER_MODEL_LAYOUT: [[usize; 14]; 6] = [
    [  0,  1,  2,  3,  4,  5,  6,      36, 37, 38, 39, 40, 41, 42 ],
    [  7,  8,  9, 10, 11, 12, 13,      43, 44, 45, 46, 47, 48, 49 ],
    [ 14, 15, 16, 17, 18, 19, 20,      50, 51, 52, 53, 54, 55, 56 ],
    [ 21, 22, 23, 24, 25, 26, XX,      XX, 57, 58, 59, 60, 61, 62 ],
    [ 27, 28, 29, 30, 31, XX, XX,      XX, XX, 63, 64, 65, 66, 67 ],
    [ XX, XX, XX, 32, 33, 34, 35,      68, 69, 70, 71, XX, XX, XX ],
];
pub const XX: usize = 99;
pub const CR: char = ' ';
#[rustfmt::skip]
pub const MOONLANDER_KEY_LAYOUT: [[char; 14]; 6] = [
    [ CR,  CR,  CR,  CR,  CR,  CR, CR,     CR,  CR,  CR,  CR,  CR,  CR, CR, ],
    [ CR, 'q', 'w', 'd', 'f', 'k', CR,     CR, 'j', 'u', 'r', 'l', 'é', CR, ],
    [ CR, 'a', 's', 'e', 't', 'g', CR,     CR, 'y', 'n', 'i', 'o', 'h', CR, ],
    [ CR, 'z', 'x', 'c', 'v', 'b', CR,     CR, 'p', 'm', ',', '.', '-', CR, ],
    [ CR,  CR,  CR,  CR,  CR,  CR, CR,     CR,  CR,  CR,  CR,  CR,  CR, CR, ],
    [ CR,  CR,  CR,  CR,  CR,  CR, CR,     CR,  CR,  CR,  CR,  CR,  CR, CR, ],
];
