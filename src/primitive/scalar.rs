

/*  Scalar Types (4 types = integer, float, character, boolean)
INTEGER:

Unsigned
u8, u16, u32, u64, u128
usize

Signed
i8, i16, i32 (fastest and default), i64, i128
isize

Note:- Not all bytes supported in all architectures

Decimal     100000          100_000 (underscore ignored)
Hex         0xdefb78ea
Octal       0o0775435
Binary      0b1111101101
Byte(u8 only)        b'A'


FLOAT:
f32
f64 (default but can be slow on <64 bit architectures)

FLOATING POINT LITERAL
let y = 3.142;

// can annotate type as a suffix if needed
let x 5u16;
let y = 3.14f32;   3.14_f32

BOOLEAN:
x: bool = true
y: bool = false

can't do arithmetic on boolean
unless casting like
 true as u8
 false as u8


 CHARACTER:
 A single character 4 bytes(32 bits)


*/

pub(super) fn integer_types() {

    let i = 100_000; // use _ for ease of readability
    let j = 0o0775435;
    let k = b'A';

    println!("Integers are i={} j={} k={}", i, j, k);
    
}


pub(super) fn character_types() {
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';

    println!("Characters are c={} z={} cat={}", c, z, heart_eyed_cat);
    
}
