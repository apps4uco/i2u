## Common useful functions that perhaps could be part of Rust's standard library one day.

If you do not want to import the i2u crate, you can always copy the implementation,
or use the documented std::fmt equivalent, see each module for details.

* designed for use in map(), e.g.: To convert an iterator of Item=Option to Strings use: iter_of_option.map(debug);
* can be used standalone e.g.: let string:String=debug(my_struct);
* All functions have doctests
* 90% of the way to version 1.0.0

## This crate provides an answer to the questions:

### How do I convert o vector or iterator of a type that implements Display or Debug into Strings

### How do I convert a vector or iterator of numbers into a String of:

* binary
* lower case hexadecimal
* upper case hexadecimal
* octal

### with

* no padding
* leading zero padding
* leading space padding

## Zero Cost Abstraction

Although the methods are not marked #\[inline\], they are generic methods and so are codegened into each compilation unit separately and therefore candidates for inlining.
Therefore the compiler should generate exactly the same code as if you had used the standard library directly.

*To be confirmed*

## Why use this crate

- Faster Development
    - Less to type
    - Save time looking in search engines
- More readable code
- Should not increase final binary size (to be confirmed)
- In Production at [`i2u.co`]

## Why not use this crate

- Extra dependency for quite simple tasks
- You like typing
- No one is ever going to read your code, ever!!!

## Contributions Welcome

If you notice a mistake in the Documentation, can simplify the code or have another function that you would like to include please create a Pull Request or an Issue

[`i2u.co`]: http://i2u.co

