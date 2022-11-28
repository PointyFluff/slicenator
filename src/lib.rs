use std::iter::Sum;
use std::ops::Mul;

/// # mul_slice()
/// Takes a reference to a couple slices of
/// type \<T\>.
///
/// Returns a new shiny Vec\<T\> of both
/// slices multiplied together.
///
/// Slices don't have to match in size but
/// will only return products upto the size of
/// smallest slice.
///
/// ## Example:
/// ```rust
/// fn mul_slice_check() {
///		let t: Vec<_> = (0..10).step_by(2).collect();
///		let u: Vec<_> = (0..20).step_by(4).collect();
///		let q: Vec<_> = mul_slice(&t, &u);
///		let e: Vec<_> = vec![0, 8, 32, 72, 128];
///		assert_eq!(e, q)
/// }
/// ```
pub fn mul_slice<T: Mul + Mul<Output = T> + Copy + Clone>(a: &[T], b: &[T]) -> Vec<T> {
    let len = if a.len() <= b.len() { a.len() } else { b.len() };
    let mut v: Vec<T> = Vec::new();

    for i in 0..len {
        v.push(a[i] * b[i]);
    }
    v
}

/// # dot_slice()
/// Takes a reference to a couple of slices.
///
/// Returns dot product of type \<T\>.
///
/// Slice sizes don't have to match,
/// but you will only calculate for the shortest slice.
///
/// ## Example:
/// ```rust
/// fn dot_slice_check() {
///     let t: Vec<_> = (0..10).step_by(2).collect();
///     let u: Vec<_> = (0..20).step_by(4).collect();
///     let q: i32 = dot_slice(&t, &u) - 198;
///     let e: i32 = 42;
///     assert_eq!(e, q)
/// }
/// ```
pub fn dot_slice<T: Mul + Mul<Output = T> + Copy + Clone + for<'a> Sum<&'a T>>(
    a: &[T],
    b: &[T],
) -> T {
    let len = if a.len() <= b.len() { a.len() } else { b.len() };
    let mut v: Vec<T> = Vec::new();

    for i in 0..len {
        v.push(a[i] * b[i]);
    }
    v.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    // #[test]
    // fn it_works() {
    //     assert!(true)
    // }

    // #[test]
    // #[should_panic]
    // fn it_fails() {
    //     assert!(false)
    // }

    #[test]
    fn mul_slice_check() {
        let t: Vec<_> = (0..10).step_by(2).collect();
        let u: Vec<_> = (0..20).step_by(4).collect();
        let q: Vec<_> = mul_slice(&t, &u);
        let e: Vec<_> = vec![0, 8, 32, 72, 128];
        assert_eq!(e, q)
    }

    #[test]
    fn mul_slice_with_vec_diff_check() {
        let t: Vec<_> = (0..10).step_by(2).collect();
        let u: Vec<_> = (0..10).step_by(4).collect();
        let q: Vec<_> = mul_slice(&t, &u);
        let e: Vec<_> = vec![0, 8, 32];
        assert_eq!(e, q)
    }

    #[test]
    fn dot_slice_check() {
        let t: Vec<_> = (0..10).step_by(2).collect();
        let u: Vec<_> = (0..20).step_by(4).collect();
        let q: i32 = dot_slice(&t, &u) - 198;
        let e: i32 = 42;
        assert_eq!(e, q)
    }

    #[test]
    fn dot_slice_with_vec_diff_check() {
        let t: Vec<_> = (0..10).step_by(2).collect();
        let u: Vec<_> = (0..10).step_by(4).collect();
        let q: i32 = dot_slice(&t, &u) + 2;
        let e: i32 = 42;
        assert_eq!(e, q)
    }
}
