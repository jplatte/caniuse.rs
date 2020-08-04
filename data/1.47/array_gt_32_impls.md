+++
title = "`[T; N]` trait implementations with N > 32"
impl_pr_id = 74060
items = [
    "impl<T, const N: usize> AsRef<[T]> for [T; N]",
    "impl<T, const N: usize> AsMut<[T]> for [T; N]",
    "impl<T, const N: usize> Borrow<[T]> for [T; N]",
    "impl<T, const N: usize> BorrowMut<[T]> for [T; N]",
    "impl<'a, T, const N: usize> IntoIterator for &'a [T; N]",
    "impl<'a, T, const N: usize> IntoIterator for &'a mut [T; N]",

    "impl<T, const N: usize> From<[T; N]> for Vec<T>",
    "impl<T, const N: usize> From<[T; N]> for Box<[T]>",
    "impl<T, const N: usize> TryFrom<&[T]> for [T; N]",
    "impl<'a, T, const N: usize> TryFrom<&'a [T]> for &'a [T; N]",
    "impl<'a, T, const N: usize> TryFrom<&'a mut [T]> for &'a mut [T; N]",
    "impl<T, const N: usize> TryFrom<Box<[T]>> for Box<[T; N]>",
    "impl<T, const N: usize> TryFrom<Rc<[T]>> for Rc<[T; N]>",
    "impl<T, const N: usize> TryFrom<Arc<[T]>> for Arc<[T; N]>",

    "impl<T: Debug, const N: usize> Debug for [T; N]",
    "impl<T: Eq, const N: usize> Eq for [T; N]",
    "impl<T: Hash, const N: usize> Hash for [T; N]",
    "impl<T: Ord, const N: usize> Ord for [T; N]",
    "impl<T: PartialOrd, const N: usize> PartialOrd for [T; N]",

    "impl<A, B, const N: usize> PartialEq<[B; N]> for [A; N]",
    "impl<A, B, const N: usize> PartialEq<[B]> for [A; N]",
    "impl<A, B, const N: usize> PartialEq<[A; N]> for [B]",
    "impl<'b, A, B, const N: usize> PartialEq<&'b [B]> for [A; N]",
    "impl<'b, A, B, const N: usize> PartialEq<[A; N]> for &'b [B]",
    "impl<'b, A, B, const N: usize> PartialEq<&'b mut [B]> for [A; N]",
    "impl<'b, A, B, const N: usize> PartialEq<[A; N]> for &'b mut [B]",

    "impl<A, B, const N: usize> PartialEq<[B; N]> for Vec<A>",
    "impl<A, B, const N: usize> PartialEq<&[B; N]> for Vec<A>",
    "impl<A, B, const N: usize> PartialEq<[B; N]> for VecDeque<A>",
    "impl<A, B, const N: usize> PartialEq<&[B; N]> for VecDeque<A>",
    "impl<A, B, const N: usize> PartialEq<&mut [B; N]> for VecDeque<A>",
]
+++
