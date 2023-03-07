// A version of coerce-expect-unsized that uses type ascription.
// Doesn't work so far, but supposed to work eventually

#![feature(box_syntax, type_ascription)]

use std::fmt::Debug;

pub fn main() {
    let _ = type_ascribe!(box { [1, 2, 3] }, Box<[i32]>); //~ ERROR mismatched types
    let _ = type_ascribe!(box if true { [1, 2, 3] } else { [1, 3, 4] }, Box<[i32]>); //~ ERROR mismatched types
    let _ = type_ascribe!(box match true { true => [1, 2, 3], false => [1, 3, 4] }, Box<[i32]>);
    //~^ ERROR mismatched types
    let _ = type_ascribe!(box { |x| (x as u8) }, Box<dyn Fn(i32) -> _>); //~ ERROR mismatched types
    let _ = type_ascribe!(box if true { false } else { true }, Box<dyn Debug>); //~ ERROR mismatched types
    let _ = type_ascribe!(box match true { true => 'a', false => 'b' }, Box<dyn Debug>); //~ ERROR mismatched types

    let _ = type_ascribe!(&{ [1, 2, 3] }, &[i32]); //~ ERROR mismatched types
    let _ = type_ascribe!(&if true { [1, 2, 3] } else { [1, 3, 4] }, &[i32]); //~ ERROR mismatched types
    let _ = type_ascribe!(&match true { true => [1, 2, 3], false => [1, 3, 4] }, &[i32]);
    //~^ ERROR mismatched types
    let _ = type_ascribe!(&{ |x| (x as u8) }, &dyn Fn(i32) -> _); //~ ERROR mismatched types
    let _ = type_ascribe!(&if true { false } else { true }, &dyn Debug); //~ ERROR mismatched types
    let _ = type_ascribe!(&match true { true => 'a', false => 'b' }, &dyn Debug); //~ ERROR mismatched types

    let _ = type_ascribe!(Box::new([1, 2, 3]), Box<[i32]>); //~ ERROR mismatched types
    let _ = type_ascribe!(Box::new(|x| (x as u8)), Box<dyn Fn(i32) -> _>); //~ ERROR mismatched types

    let _ = type_ascribe!(vec![
        Box::new(|x| (x as u8)),
        box |x| (x as i16 as u8),
    ], Vec<Box<dyn Fn(i32) -> _>>);
}
