//! Provides functions related to formatting using std::fmt
//!
//! ## The answer to the question:
//!
//! How do I convert a vector or iterator of numbers into a String of:
//!
//! * space padded binary.
//! * leading zero padded binary.
//! * leading zero padded upper case hexadecimal.
//! * space padded upper case hexadecimal.
//! * leading zero padded lower case hexadecimal.
//! * space padded lower case hexadecimal.
//!
//! ## Use
//!
//! * iter.map(binary)
//! * iter.map(octal)
//!
#[cfg(feature = "itertools")]
use itertools::Itertools;
use std::fmt::{Binary, Debug, LowerHex, Octal, UpperHex};

/// Convert anything that implements the [`std::fmt::Display`] trait into a String
///
/// use .map(to_string)
///
/// short for **format!("{}",d)** or ** |t| t.to_string() ** but self documenting
///
/// Other alternatives to using this function:
/// * String::from  for types that implement the into() String
/// * ToString::to_string if your type implements the Display trait
///     * depending on the iterator, e.g.: chars() or range iterator is not supported
///
/// # Example
/// ```
/// use i2u::prelude::*;
/// let str = "abcdef";
/// let map_to_strings: Vec<_> = str.chars().map(to_string).collect();
///
/// // let does_not_compile:Vec<_>="abcdef".to_string().chars().map(ToString::to_string).collect();
/// // error[E0631]: type mismatch in function arguments
/// // = note: expected function signature `fn(char) -> _`
/// //                found function signature `fn(&_) -> _`
/// // let does_not_compile2:Vec<_>=(0..10).map(ToString::to_string).collect();
///
/// let stdlib_map_to_strings: Vec<_> = "abcdef"
///             .chars()
///             .map(String::from)
///             .collect();
///
///
/// let expected:Vec<_>=vec!["a", "b", "c", "d", "e", "f"].into_iter().map(ToOwned::to_owned).collect();
///# assert_eq!(map_to_strings,expected);
///# assert_eq!(map_to_strings,stdlib_map_to_strings);
/// ```
pub fn to_string<T: ToString>(t: T) -> String {
    t.to_string()
}

/// Convert anything that implements the [`std::fmt::Debug`] trait into a String
///
/// use .map(debug)
///
/// Useful for mapping [`std::option::Option`] into a String
///
/// debug is short for **format!("{:?}",d)** but self documenting
///
/// # Examples
/// ```
/// use i2u::prelude::*;
/// #[derive(Debug)]
/// struct AStruct {
///     value: u32,
/// }
///
/// let s=AStruct{value:12};
/// let result=debug(s);
/// let expected="AStruct { value: 12 }";
/// assert_eq!(&result,expected);
/// ```
///
/// ```
/// use i2u::prelude::*;
/// let vec = vec![Some(1),None];
/// let map_to_strings: Vec<_> = vec.iter().map(debug).collect();
/// let stdlib_map_to_strings: Vec<_> = vec.iter().map(|option| format!("{:?}", option))
///             .collect();
///
/// //let does_not_compile:Vec<_>=vec.iter().map(ToString::to_string).collect();
/// //error[E0277]: `Option<{integer}>` doesn't implement `std::fmt::Display`
///
/// let expected=vec![String::from("Some(1)"), String::from("None")];
///# assert_eq!(map_to_strings,expected);
///# assert_eq!(map_to_strings,stdlib_map_to_strings);
/// ```
pub fn debug<D: Debug>(d: D) -> String {
    format!("{:?}", d)
}

/// Convert anything that implements the [`std::fmt::Debug`] trait into a pretty printed String
///
/// /// use .map(debug_pretty)
///
/// self documenting equivalent to **format!("{:#?}",d)**
///
/// # Example
/// ```
/// use i2u::prelude::*;
/// #[derive(Debug)]
/// struct AStruct {
///     value: u32,
/// }
///
/// let s=AStruct{value:12};
///
/// let result=debug_pretty(s);
/// let expected="AStruct {\n    value: 12,\n}";
/// assert_eq!(&result,expected);
/// ```
pub fn debug_pretty<D: Debug>(d: D) -> String {
    format!("{:#?}", d)
}

/// Convert numbers that implement the [`std::fmt::Octal`] trait into an octal String
///
/// use .map(octal)
///
/// self documenting **format!("{:0o}",num)**
///
/// # Example
/// ```
/// use i2u::prelude::*;
/// let vec = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
/// let vec_to_strings: Vec<_> = vec.iter().map(octal).collect();
/// let stdlib_vec_to_strings: Vec<_> = vec.iter().map(|o| format!("{:o}", o)).collect();
/// let vec_to_strings_consume: Vec<_> = vec.into_iter().map(octal).collect();
///# assert_eq!(vec_to_strings,stdlib_vec_to_strings);
///# assert_eq!(vec_to_strings_consume,stdlib_vec_to_strings);
/// ```
pub fn octal<O: Octal>(o: O) -> String {
    format!("{:o}", o)
}

/// Convert a number that implements the [`std::fmt::Binary`] trait into a binary String
///
/// Use .map(binary)
///
/// self documenting **format!("{:0b}",num)**
///
/// # Example
/// ```
/// use i2u::prelude::*;
/// let vec = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
/// let vec_to_strings: Vec<_> = vec.iter().map(binary).collect();
/// let stdlib_vec_to_strings: Vec<_> = vec.iter().map(|b| format!("{:b}", b)).collect();
/// let vec_to_strings_consume: Vec<_> = vec.into_iter().map(binary).collect();
/// // assert_eq!(vec_to_string,expected);
///# assert_eq!(vec_to_strings,stdlib_vec_to_strings);
///# assert_eq!(vec_to_strings_consume,stdlib_vec_to_strings);
/// ```
pub fn binary<B: Binary>(b: B) -> String {
    format!("{:b}", b)
}

/// Convert a number that implements the [`std::fmt::Binary`] trait into a binary String with padding of leading zeros
///
/// Use .map(binary_zero_pad::<8, _>) // 8 in this case is the width
///
/// short for **format!("{:08b}",num)**
///
/// # Examples
/// ```
/// use i2u::prelude::*;
/// let vec = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
/// let vec_to_strings: Vec<_> = vec.iter().map(binary_zero_pad::<8, _>).collect();
/// let stdlib_vec_to_strings: Vec<_> =
///     vec.iter().map(|b| format!("{:08b}", b)).collect();
/// let vec_to_strings_consume: Vec<_> =
///     vec.into_iter().map(binary_zero_pad::<8, _>).collect();
///# assert_eq!(vec_to_strings,stdlib_vec_to_strings);
///# assert_eq!(vec_to_strings_consume,stdlib_vec_to_strings);
/// ```
pub fn binary_zero_pad<const N: usize, B: Binary>(b: B) -> String {
    format!("{:0width$b}", b, width = N)
}

/// Convert a number that implements the [`std::fmt::Binary`] trait into a binary String with padding of leading spaces
///
/// /// Use .map(binary_pad::<8, _>) // 8 in this case is the width
///
/// short for **format!("{:3b}",num)**
///
/// # Examples
/// ```
/// use i2u::prelude::*;
/// let vec = vec![0, 1, 2, 3, 4, 5, 6, 7];
/// let vec_to_strings: Vec<_> = vec.iter().map(binary_pad::<3, _>).collect();
/// let stdlib_vec_to_strings: Vec<_> = vec.iter().map(|b| format!("{:3b}", b)).collect();
/// let vec_to_strings_consume: Vec<_> =
///     vec.into_iter().map(binary_pad::<3, _>).collect();
/// let expected:Vec<_>=vec!["  0", "  1", " 10", " 11", "100", "101", "110", "111"].iter().map(ToOwned::to_owned).collect();
/// assert_eq!(vec_to_strings,expected);
/// assert_eq!(vec_to_strings,stdlib_vec_to_strings);
/// assert_eq!(vec_to_strings_consume,stdlib_vec_to_strings);
///
/// ```
pub fn binary_pad<const N: usize, B: Binary>(b: B) -> String {
    format!("{:width$b}", b, width = N)
}

/// Convert any number that implements the [`std::fmt::LowerHex`] trait into a String
///
/// Use .map(lower_hex_pad::<2, _>)
///
/// short for **format!("{:2x}",num)**
///
/// # Example
/// ```
/// use i2u::prelude::*;
/// let vec = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
/// let vec_to_strings: Vec<_> = vec.iter().map(lower_hex_pad::<2, _>).collect();
/// let stdlib_vec_to_strings: Vec<_> = vec.iter().map(|h| format!("{:2x}", h)).collect();
/// let vec_to_strings_consume: Vec<_> =
///     vec.into_iter().map(lower_hex_pad::<2, _>).collect();
///# assert_eq!(vec_to_strings,stdlib_vec_to_strings);
///# assert_eq!(vec_to_strings_consume,stdlib_vec_to_strings);
/// ```
pub fn lower_hex_pad<const N: usize, H: LowerHex>(h: H) -> String {
    format!("{:width$x}", h, width = N)
}

/// Convert any number that implements the [`std::fmt::UpperHex`] trait into a String
///
/// Use .map(upper_hex_pad::<2, _>)
///
/// short for **format!("{:2X}",d)** except it handles width of const N at compile time
///
/// # Examples
/// ```
/// use i2u::prelude::*;
/// let vec = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
/// let vec_to_strings: Vec<_> = vec.iter().map(upper_hex_pad::<2, _>).collect();
/// let stdlib_vec_to_strings: Vec<_> = vec.iter().map(|h| format!("{:2X}", h)).collect();
/// let vec_to_strings_consume: Vec<_> =
///     vec.into_iter().map(upper_hex_pad::<2, _>).collect();
///# assert_eq!(vec_to_strings,stdlib_vec_to_strings);
///# assert_eq!(vec_to_strings_consume,stdlib_vec_to_strings);
/// ```
pub fn upper_hex_pad<const N: usize, H: UpperHex>(h: H) -> String {
    format!("{:width$X}", h, width = N)
}

/// Convert any number that implements the [`std::fmt::LowerHex`] trait into a String padded with leading zeros
///
/// Use .map(lower_hex_zeropad::<2, _>)
///
/// short for **format!("{:02x}",num)**
///
/// # Example
/// ```
/// use i2u::prelude::*;
/// let vec = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
/// let vec_to_strings: Vec<_> = vec.iter().map(lower_hex_zeropad::<2, _>).collect();
/// let stdlib_vec_to_strings: Vec<_> = vec.iter().map(|h| format!("{:02x}", h)).collect();
/// let vec_to_strings_consume: Vec<_> =
///     vec.into_iter().map(lower_hex_zeropad::<2, _>).collect();
/// assert_eq!(vec_to_strings,stdlib_vec_to_strings);
/// assert_eq!(vec_to_strings_consume,stdlib_vec_to_strings);
/// ```
pub fn lower_hex_zeropad<const N: usize, H: LowerHex>(h: H) -> String {
    format!("{:0width$x}", h, width = N)
}

/// Convert any number that implements the [`std::fmt::UpperHex`] trait into a String padded with leading zeros
///
/// Use .map(upper_hex_zeropad::<2, _>)
///
/// short for **format!("{:02X}",num)**
///
/// # Examples
/// ```
/// use i2u::prelude::*;
/// let vec = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];///
/// let vec_to_strings: Vec<_> = vec.iter().map(upper_hex_zeropad::<2, _>).collect();
/// let stdlib_vec_to_strings: Vec<_> = vec.iter().map(|h| format!("{:02X}", h)).collect();
/// let vec_to_strings_consume: Vec<_> =
///     vec.into_iter().map(upper_hex_zeropad::<2, _>).collect();
///# assert_eq!(vec_to_strings,stdlib_vec_to_strings);
///# assert_eq!(vec_to_strings_consume,stdlib_vec_to_strings);
/// ```
pub fn upper_hex_zeropad<const N: usize, H: UpperHex>(h: H) -> String {
    format!("{:0width$X}", h, width = N)
}

#[cfg(feature = "itertools")]
/// Takes a String or &str chunks it into groups of chunk_size characters and joins them with separator returns a String
///
/// # Example
///
/// ```
/// use i2u::prelude::chunk_join;
/// let string="FEEDC0FFEE";//this could be output from the hex crate for example
/// let result=chunk_join(string.to_string(),2," ");
/// let expected="FE ED C0 FF EE";
/// assert_eq!(result,expected);
/// ```
pub fn chunk_join<S: AsRef<str>, R: AsRef<str>>(
    string: S,
    chunk_size: usize,
    separator: R,
) -> String {
    assert!(!separator.as_ref().is_empty());
    string
        .as_ref()
        .chars()
        .chunks(chunk_size)
        .into_iter()
        .map(|mut chunk| chunk.join(""))
        .join(separator.as_ref())
}
