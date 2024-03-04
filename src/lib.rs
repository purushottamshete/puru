//! # Puru
//! `puru` is collection of utility functions
//!

/// Add one to the number given
/// 
/// # Example
///
/// ```
/// let num = 10;
/// let answer = puru::add_one(num);
/// assert_eq!(11, answer);
/// ```

pub fn add_one(num: i32) -> i32 {
    return num + 1
}