// ---- cell 4 items ----
/// Double `x`.
///
/// TODO: replace `todo!()` with the doubled value.
fn double(x: i32) -> i32 {
    return 2 * x
}
// ---- cell 6 items ----
/// Return exactly `Hello, <name>!`.
///
/// TODO: build and return the string.
fn greeting(name: &str) -> String {
    return format!("Hello, {name}!")
}
// ---- cell 8 items ----
/// Return the square of `n`, using a tail expression.
///
/// TODO: no `return`, and no semicolon on the last expression.
fn square(n: i32) -> i32 {
    n.pow(2)
}
/// Return the square of `n`, using an explicit `return`.
///
/// TODO: same value, different spelling.
fn square_via_return(n: i32) -> i32 {
    return n.pow(2);
}
// ---- cell 10 items ----
/// Add 1 to `n` three times, then return it.
///
/// TODO: bind `n` as `mut`, then increment it three times.
use std::ops::Range;
fn thrice_incremented(mut n: i32) -> i32 {
    let range = Range{start: 0, end: 3};
    // let range = 0..3;
    for i in range {
        n = n + 1;
    }
    return n;
}
// ---- cell 12 items ----
// ---- cell 14 items ----
/// Double `n` by shadowing it, then report whether the result exceeds 10.
///
/// TODO: `let n = n * 2;` first, then answer with an `if` expression.
fn doubled_is_big(n: i32) -> &'static str {
    // if (n > 10) { return "big"; }
    // else { return "small"; }
    return (if (n * 2 > 10) {"big"} else {"small"});
}
// ---- cell 16 items ----
/// Parse `text` as an integer, then return it with 100 added.
///
/// TODO: shadow `text` into an `i32`, then return the sum.
fn offset(text: &str) -> i32 {
    let mut text: i32 = text.parse().unwrap();
    return text + 100;
}
// ---- cell 18 items ----
/// Return the largest possible value of a `u8`.
///
/// TODO: there is an associated constant for exactly this.
fn largest_u8() -> u8 {
    return u8::MAX;
}
/// Return how many bytes one `i64` occupies.
///
/// TODO: `std::mem::size_of` returns a `usize`.
fn bytes_in_i64() -> usize {
    return size_of::<i64>();
}
// ---- cell 20 items ----
/// Return `true` when `c` is an uppercase ASCII letter.
///
/// TODO: `char` methods will do this in one call.
fn is_upper_letter(c: char) -> bool {
    return c.is_ascii_uppercase();
}
/// Return the number of bytes `c` occupies in UTF-8.
///
/// TODO: one `char` method gives this directly.
fn utf8_width(c: char) -> usize {
    return c.len_utf8();
}
// ---- cell 22 items ----
/// Return 7 as a `u16`.
///
/// TODO: the literal alone would be an `i32`.
fn seven_as_u16() -> u16 {
    // return 7_u16;
    let n: u16 = 7;
    return n;
}
/// Return a value that is exactly one half, as an `f32`.
///
/// TODO: the literal alone would be an `f64`.
fn half_f32() -> f32 {
    // return 0.5_f32;
    let n: f32 = 0.5;
    return n;
}
// ---- cell 24 items ----
/// Parse `text` and return it as an `i64`.
///
/// TODO: the declared return type is the hint `parse` needs.
fn parse_i64(text: &str) -> i64 {
    return text.parse::<i64>().unwrap();
}
/// Parse `text` into whatever the caller asks for.
///
/// TODO: `text.parse::<T>().unwrap()` is the shape; the bounds are given.
fn parse_generic<T>(text: &str) -> T
where
    T: std::str::FromStr,
    T::Err: std::fmt::Debug,
{
    return text.parse::<T>().unwrap();
}
// ---- cell 26 items ----
/// Return one million as a `u32`, written readably.
///
/// TODO: use underscores.
fn one_million() -> u32 {
    let n: u32 = 1000000;
    return n;

}
/// Return the decimal value of the binary literal `0b1010_1010`.
///
/// TODO: write the literal; no arithmetic required.
fn binary_literal() -> u8 {
    let n: u8 = 0b1010_1010;
    return n;
}
/// Return 255 as a `u8`, written in hexadecimal.
///
/// TODO: use the `0x` prefix.
fn hex_literal() -> u8 {
    let n: u8 = 0xFF;
    return n;
}
// ---- cell 28 items ----
// ---- cell 31 items ----
/// Return `value` reduced to a `u8` the way `as` does it.
///
/// TODO: a single `as` cast.
fn as_u8(value: i32) -> u8 {
    return value as u8;
}
// ---- cell 33 items ----
/// Convert `x` to an `i32` by truncating toward zero.
///
/// TODO: `as` already truncates.
fn truncated(x: f64) -> i32 {
    return x as i32;
}
/// Convert `x` to an `i32` by rounding to the nearest integer.
///
/// TODO: round first, then cast.
fn rounded(x: f64) -> i32 {
    return x.round() as i32;
}
// ---- cell 35 items ----
/// Add `delta` to `start`, wrapping modulo 256.
///
/// TODO: one method call.
fn wrap_add(start: u8, delta: u8) -> u8 {
    return start.wrapping_add(delta);
}
/// Add `delta` to `start`, clamping at `u8::MAX`.
///
/// TODO: one method call.
fn sat_add(start: u8, delta: u8) -> u8 {
    return start.saturating_add(delta);
}
/// Add `delta` to `start`, returning `None` on overflow.
///
/// TODO: one method call.
fn checked_add(start: u8, delta: u8) -> Option<u8> {
    return start.checked_add(delta);
}
// ---- cell 37 items ----
/// Return `a % b`, which carries the sign of `a`.
///
/// TODO: the plain remainder operator.
fn trunc_rem(a: i32, b: i32) -> i32 {
    return a % b;
}
/// Return the non-negative remainder of `a` divided by `b`.
///
/// TODO: the Euclidean remainder, not the truncating one.
fn euclid_rem(a: i32, b: i32) -> i32 {
    return a.rem_euclid(b);
}
/// Return the low four bits of `byte`.
///
/// TODO: one mask.
fn low_nibble(byte: u8) -> u8 {
    return byte % 16;
}
/// Return the high four bits of `byte`.
///
/// TODO: one shift.
fn high_nibble(byte: u8) -> u8 {
    return byte / 16;
}
// ---- cell 39 items ----
/// Return `true` when `a` and `b` differ by no more than `tolerance`.
///
/// TODO: compare the absolute difference; do not use `==`.
fn approx_eq(a: f64, b: f64, tolerance: f64) -> bool {
    return (a - b).abs() <= tolerance
}
// ---- cell 41 items ----
/// Return `true` when `x` is not a number.
///
/// TODO: one method call.
fn not_a_number(x: f64) -> bool {
    return x.is_nan();
}
/// Sort `values` ascending, placing NaN at the end.
///
/// TODO: `f64` has no `Ord`, so `.sort()` will not compile.
fn sort_ascending(values: &mut [f64]) {
    return values.sort_by(f64::total_cmp);
}
// ---- cell 43 items ----
/// Return `true` when `c` is a decimal digit character.
///
/// TODO: one method call.
fn is_digit(c: char) -> bool {
    return c.is_digit(10);
}
/// Return the numeric value of a decimal digit character, or `None`.
///
/// TODO: one method call returning `Option<u32>`.
fn digit_value(c: char) -> Option<u32> {
    return c.to_digit(10);
}
// ---- cell 45 items ----
/// Return an owned copy of `text`.
///
/// TODO: convert the `&str` into a `String`.
fn owned(text: &str) -> String {
    return text.to_string();
}
/// Return a borrowed view of `text`.
///
/// TODO: no allocation. The lifetime is elided for you.
fn borrowed(text: &str) -> &str {
    return text;
}
// ---- cell 47 items ----
/// Return the number of UTF-8 bytes in `text`.
///
/// TODO: one method call.
fn byte_len(text: &str) -> usize {
    return text.to_string().len();
}
/// Return the number of Unicode scalar values in `text`.
///
/// TODO: one method call on the character iterator.
fn char_len(text: &str) -> usize {
    return text.chars().count();
}
/// Return `true` when every byte of `text` is ASCII.
///
/// TODO: one method call.
fn all_ascii(text: &str) -> bool {
    return text.to_string().is_ascii();
}
// ---- cell 49 items ----
/// Return a table row for `label` and `price`.
///
/// TODO: `{:<10}` then `|` then `{:>8.2}`, all inside one `format!`.
fn row(label: &str, price: f64) -> String {
    return format!("{:<10}|{:>8.2}", label, price);
}
// ---- cell 51 items ----
/// Return `n` in uppercase hex, `0x`-prefixed, zero-padded to four digits.
///
/// TODO: one spec does all of it.
fn hex_word(n: u16) -> String {
    todo!()
}
/// Return `x` with exactly three digits after the point.
///
/// TODO: precision only.
fn three_decimals(x: f64) -> String {
    todo!()
}
// ---- cell 53 items ----
/// Return `<name>: <value> <unit>`.
///
/// TODO: build a `String`; print nothing.
fn measurement(name: &str, value: i32, unit: &str) -> String {
    todo!()
}
// ---- cell 55 items ----
/// Return the compact `Debug` rendering of `value`.
///
/// TODO: one `format!` call.
fn debug_of<T: std::fmt::Debug>(value: T) -> String {
    todo!()
}
/// Return the pretty `Debug` rendering of `value`.
///
/// TODO: one `format!` call, one extra character in the spec.
fn pretty_of<T: std::fmt::Debug>(value: T) -> String {
    todo!()
}
// ---- cell 58 items ----
/// Return the sum of a fixed-size array of four integers.
///
/// TODO: iterate and add.
fn sum_of_four(values: [i32; 4]) -> i32 {
    todo!()
}
/// Return the middle two elements of a 4-element array.
///
/// TODO: index, or slice and convert.
fn middle_two(values: [i32; 4]) -> [i32; 2] {
    todo!()
}
/// Return the number of elements in an array of any size.
///
/// TODO: const generics are already in the signature.
fn array_len<T, const N: usize>(values: &[T; N]) -> usize {
    todo!()
}
// ---- cell 60 items ----
const MAX_RETRIES: u32 = 5;
static APP_NAME: &str = "training-rust";
type Celsius = f64;
/// Return twice `MAX_RETRIES`.
///
/// TODO: refer to the constant by name.
fn retry_budget() -> u32 {
    todo!()
}
/// Return `APP_NAME` as an owned `String`.
///
/// TODO: a `&str` has to be converted.
fn app_name() -> String {
    todo!()
}
/// Return the boiling point of water, typed as `Celsius`.
///
/// TODO: 100.0 as an `f64` literal is enough.
fn boiling_point() -> Celsius {
    todo!()
}
// ---- cell 62 items ----
/// Return the number of seconds in a day.
///
/// TODO: declare a local `const SECONDS_PER_DAY: u32` and return it.
fn seconds_per_day() -> u32 {
    todo!()
}
/// Return the greeting held by a local static.
///
/// TODO: declare a local `static GREETING: &str = "hi";` and return it.
fn static_greeting() -> &'static str {
    todo!()
}
// ---- cell 64 items ----
/// Return the string `[<tag>]`.
///
/// TODO: one `format!`, no trailing semicolon on the block's value.
fn label(tag: &str) -> String {
    todo!()
}
/// Call `label("k")`, throw the result away, and return `"discarded"`.
///
/// TODO: `let _ = label("k");` then return the string.
fn discard_demo() -> &'static str {
    todo!()
}
// ---- cell 66 items ----
/// Return `"big"` when `n > 100`, otherwise `"small"`.
///
/// TODO: `if` as an expression.
fn size(n: i32) -> &'static str {
    todo!()
}
/// Return the larger of `a` and `b`.
///
/// TODO: `if` as an expression, or `a.max(b)`.
fn larger(a: i32, b: i32) -> i32 {
    todo!()
}
/// Return `"zero"`, `"positive"` or `"negative"`.
///
/// TODO: an `else if` chain; mind the order.
fn sign(n: i32) -> &'static str {
    todo!()
}
// ---- cell 68 items ----
/// Return the `rustc` error code for `let x = 5; x = 6;`.
///
/// TODO: an immutable binding was assigned to again.
fn case_a() -> &'static str {
    todo!()
}
/// Return the `rustc` error code for `let n: i32 = "7";`.
///
/// TODO: the value and the annotation disagree.
fn case_b() -> &'static str {
    todo!()
}
/// Return the `rustc` error code for `fn f() -> i32 { 1; }`.
///
/// TODO: the declared type and the block's value disagree.
fn case_c() -> &'static str {
    todo!()
}
// ---- cell 70 items ----
/// Return `<label>: <value> (±<tolerance>)`, both numbers with 2 decimals.
///
/// TODO: one `format!` with two precision specs.
fn report(label: &str, value: f64, tolerance: f64) -> String {
    todo!()
}
/// Return `true` when `a` and `b` differ by no more than `tolerance`.
///
/// TODO: absolute difference, `<=`.
fn agrees(a: f64, b: f64, tolerance: f64) -> bool {
    todo!()
}
/// Return `u8::MAX` doubled without overflowing.
///
/// TODO: the result must be `None`.
fn doubled_max() -> Option<u8> {
    todo!()
}
// ---- cell 72 items ----
/// Parse `text` as an `i64` and add `delta`.
///
/// TODO: shadow the `&str` into an `i64`, then return the sum.
fn parse_and_add(text: &str, delta: i64) -> i64 {
    todo!()
}
/// Convert `celsius` to Fahrenheit.
///
/// TODO: `c * 9 / 5 + 32`, with float literals.
fn to_fahrenheit(celsius: f64) -> f64 {
    todo!()
}
/// Return `n` as a `u8`, clamping at both ends.
///
/// TODO: below 0 -> 0, above 255 -> 255.
fn clamp_to_u8(n: i64) -> u8 {
    todo!()
}
fn main() {
// ---- cell 4 ----

// --- check (do not modify) ---
assert_eq!(double(21), 42);
assert_eq!(double(-3), -6);
assert_eq!(double(0), 0);
println!("Exercise 0.1 passed");
// ---- cell 6 ----

// --- check (do not modify) ---
assert_eq!(greeting("Ada"), "Hello, Ada!");
assert_eq!(greeting("Rust"), "Hello, Rust!");
println!("Exercise 1.1 passed");
// ---- cell 8 ----


// --- check (do not modify) ---
assert_eq!(square(6), 36);
assert_eq!(square(-4), 16);
assert_eq!(square_via_return(6), 36);
assert_eq!(square(0), square_via_return(0));
println!("Exercise 1.2 passed");
// ---- cell 10 ----

// --- check (do not modify) ---
assert_eq!(thrice_incremented(0), 3);
assert_eq!(thrice_incremented(-3), 0);
assert_eq!(thrice_incremented(10), 13);
println!("Exercise 2.1 passed");
// ---- cell 12 ----
let values = [4, 9, 1, 16];

// TODO: total up `values` using a mutable accumulator.
let mut total = 0;
for value in values { total += value; }
// let total: i32 = values.iter().sum();

// --- check (do not modify) ---
assert_eq!(total, 30);
println!("Exercise 2.2 passed");
// ---- cell 14 ----

// --- check (do not modify) ---
assert_eq!(doubled_is_big(6), "big");
assert_eq!(doubled_is_big(5), "small");
assert_eq!(doubled_is_big(-1), "small");
println!("Exercise 3.1 passed");
// ---- cell 16 ----

// --- check (do not modify) ---
assert_eq!(offset("42"), 142);
assert_eq!(offset("-2"), 98);
assert_eq!(offset("0"), 100);
println!("Exercise 3.2 passed");
// ---- cell 18 ----


// --- check (do not modify) ---
assert_eq!(largest_u8(), 255);
assert_eq!(bytes_in_i64(), 8);
assert_eq!(largest_u8().wrapping_add(1), 0);
println!("Exercise 4.1 passed");
// ---- cell 20 ----


// --- check (do not modify) ---
assert!(is_upper_letter('A'));
assert!(!is_upper_letter('a'));
assert!(!is_upper_letter('5'));
assert!(!is_upper_letter('Ä'));
assert_eq!(utf8_width('a'), 1);
assert_eq!(utf8_width('ß'), 2);
assert_eq!(utf8_width('€'), 3);
assert_eq!(utf8_width('😀'), 4);
println!("Exercise 4.2 passed");
// ---- cell 22 ----


// --- check (do not modify) ---
assert_eq!(seven_as_u16(), 7u16);
assert_eq!(half_f32(), 0.5f32);
println!("Exercise 5.1 passed");
// ---- cell 24 ----


// --- check (do not modify) ---
assert_eq!(parse_i64("1234"), 1234i64);
assert_eq!(parse_i64("-8"), -8i64);
assert_eq!(parse_generic::<i32>("-9"), -9i32);
assert_eq!(parse_generic::<u8>("200"), 200u8);
assert_eq!(parse_generic::<f64>("1.5"), 1.5f64);
println!("Exercise 5.2 passed");
// ---- cell 26 ----



// --- check (do not modify) ---
assert_eq!(one_million(), 1_000_000);
assert_eq!(binary_literal(), 0b1010_1010);
assert_eq!(binary_literal(), 170);
assert_eq!(hex_literal(), 0xFF);
println!("Exercise 6.1 passed");
// ---- cell 28 ----
// TODO: bind `mask` to the `u8` whose bits are 1111_0000, written as a binary
// literal, and `shifted` to that same value shifted right by 4.
let mask: u8 = 0b1111_0000;
let shifted: u8 = mask >> 4;

// --- check (do not modify) ---
assert_eq!(mask, 0b1111_0000);
assert_eq!(shifted, 0b0000_1111);
assert_eq!(mask / 16, shifted);
println!("Exercise 6.2 passed");
// ---- cell 31 ----

// --- check (do not modify) ---
assert_eq!(as_u8(255), 255);
assert_eq!(as_u8(256), 0);
assert_eq!(as_u8(300), 44);
assert_eq!(as_u8(-1), 255);
println!("Exercise 7.1 passed");
// ---- cell 33 ----


// --- check (do not modify) ---
assert_eq!(truncated(2.9), 2);
assert_eq!(truncated(-2.9), -2);
assert_eq!(rounded(2.5), 3);
assert_eq!(rounded(-2.5), -3);
println!("Exercise 7.2 passed");
// ---- cell 35 ----



// --- check (do not modify) ---
assert_eq!(wrap_add(250, 10), 4);
assert_eq!(sat_add(250, 10), 255);
assert_eq!(checked_add(250, 10), None);
assert_eq!(checked_add(1, 2), Some(3));
assert_eq!(wrap_add(1, 2), 3);
println!("Exercise 7.3 passed");
// ---- cell 37 ----




// --- check (do not modify) ---
assert_eq!(trunc_rem(-7, 3), -1);
assert_eq!(trunc_rem(7, -3), 1);
assert_eq!(euclid_rem(-7, 3), 2);
assert_eq!(euclid_rem(7, 3), 1);
assert_eq!(low_nibble(0xAB), 0x0B);
assert_eq!(high_nibble(0xAB), 0x0A);
println!("Exercise 7.4 passed");
// ---- cell 39 ----

// --- check (do not modify) ---
assert!(approx_eq(0.1 + 0.2, 0.3, 1e-12));
assert!(!approx_eq(0.1 + 0.2, 0.3, 0.0));
assert!(approx_eq(1.0, 1.0, 0.0));
assert!(approx_eq(-2.0, -2.000_000_1, 1e-5));
println!("Exercise 8.1 passed");
// ---- cell 41 ----


// --- check (do not modify) ---
assert!(not_a_number(f64::NAN));
assert!(!not_a_number(1.0));
let mut xs = [3.0, f64::NAN, -1.0, 2.0];
sort_ascending(&mut xs);
// Note: `assert_eq!(xs, [...])` cannot be used here -- NaN != NaN.
assert_eq!(xs[0], -1.0);
assert_eq!(xs[1], 2.0);
assert_eq!(xs[2], 3.0);
assert!(xs[3].is_nan());
println!("Exercise 8.2 passed");
// ---- cell 43 ----


// --- check (do not modify) ---
assert!(is_digit('7'));
assert!(!is_digit('x'));
assert!(!is_digit(' '));
assert_eq!(digit_value('7'), Some(7));
assert_eq!(digit_value('0'), Some(0));
assert_eq!(digit_value('x'), None);
println!("Exercise 9.1 passed");
// ---- cell 45 ----


// --- check (do not modify) ---
assert_eq!(owned("hi"), String::from("hi"));
assert_eq!(borrowed("hi"), "hi");
assert_eq!(owned("").len(), 0);
println!("Exercise 9.2 passed");
// ---- cell 47 ----



// --- check (do not modify) ---
let sample = "héllo 😀";
assert_eq!(byte_len(sample), 11);
assert_eq!(char_len(sample), 7);
assert!(!all_ascii(sample));
assert!(all_ascii("plain"));
println!("Exercise 9.3 passed");
// ---- cell 49 ----

// --- check (do not modify) ---
assert_eq!(row("tea", 2.5), "tea       |    2.50");
assert_eq!(row("sandwich", 12.0), "sandwich  |   12.00");
println!("Exercise 10.1 passed");
// ---- cell 51 ----


// --- check (do not modify) ---
assert_eq!(hex_word(255), "0x00FF");
assert_eq!(hex_word(4095), "0x0FFF");
assert_eq!(hex_word(0), "0x0000");
assert_eq!(three_decimals(3.14159), "3.142");
assert_eq!(three_decimals(2.0), "2.000");
println!("Exercise 10.2 passed");
// ---- cell 53 ----

// --- check (do not modify) ---
assert_eq!(measurement("width", 12, "mm"), "width: 12 mm");
assert_eq!(measurement("mass", 3, "kg"), "mass: 3 kg");
println!("Exercise 11.1 passed");
// ---- cell 55 ----


// --- check (do not modify) ---
assert_eq!(debug_of(42), "42");
assert_eq!(debug_of("hi"), "\"hi\"");
assert_eq!(debug_of((1, 'a')), "(1, 'a')");
assert_eq!(debug_of(Some(3)), "Some(3)");
assert!(pretty_of((1, 2)).contains('\n'));
println!("Exercise 11.2 passed");
// ---- cell 58 ----



// --- check (do not modify) ---
assert_eq!(sum_of_four([1, 2, 3, 4]), 10);
assert_eq!(sum_of_four([0, 0, 0, 0]), 0);
assert_eq!(middle_two([1, 2, 3, 4]), [2, 3]);
assert_eq!(array_len(&[0u8; 9]), 9);
assert_eq!(array_len(&[true, false]), 2);
println!("Exercise 12.2 passed");
// ---- cell 60 ----
// Given: a constant, a static and a type alias.




// --- check (do not modify) ---
assert_eq!(retry_budget(), 10);
assert_eq!(app_name(), "training-rust");
assert_eq!(boiling_point(), 100.0);
println!("Exercise 13.1 passed");
// ---- cell 62 ----


// --- check (do not modify) ---
assert_eq!(seconds_per_day(), 86_400);
assert_eq!(static_greeting(), "hi");
println!("Exercise 13.2 passed");
// ---- cell 64 ----


// --- check (do not modify) ---
assert_eq!(label("k"), "[k]");
assert_eq!(label(""), "[]");
assert_eq!(discard_demo(), "discarded");
println!("Exercise 14.1 passed");
// ---- cell 66 ----



// --- check (do not modify) ---
assert_eq!(size(101), "big");
assert_eq!(size(100), "small");
assert_eq!(larger(3, 9), 9);
assert_eq!(larger(9, 3), 9);
assert_eq!(sign(0), "zero");
assert_eq!(sign(-1), "negative");
assert_eq!(sign(5), "positive");
println!("Exercise 14.2 passed");
// ---- cell 68 ----



// --- check (do not modify) ---
assert_eq!(case_a(), "E0384");
assert_eq!(case_b(), "E0308");
assert_eq!(case_c(), "E0308");
println!("Exercise 15.1 passed");
// ---- cell 70 ----



// --- check (do not modify) ---
assert_eq!(report("mass", 12.3456, 0.25), "mass: 12.35 (±0.25)");
assert_eq!(report("len", 2.0, 0.5), "len: 2.00 (±0.50)");
assert!(agrees(0.1 + 0.2, 0.3, 1e-9));
assert!(!agrees(0.1 + 0.2, 0.3, 1e-18));
assert_eq!(doubled_max(), None);
println!("Exercise 16.1 passed");
// ---- cell 72 ----



// --- check (do not modify) ---
assert_eq!(parse_and_add("40", 2), 42);
assert_eq!(parse_and_add("-10", 10), 0);
assert_eq!(to_fahrenheit(100.0), 212.0);
assert_eq!(to_fahrenheit(0.0), 32.0);
assert_eq!(clamp_to_u8(300), 255);
assert_eq!(clamp_to_u8(-5), 0);
assert_eq!(clamp_to_u8(7), 7);
println!("Exercise 16.2 passed");
}
