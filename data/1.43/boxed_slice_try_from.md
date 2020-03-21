+++
title = "`TryFrom` implementations from boxed slices to boxed arrays"
flag = "boxed_slice_try_from"
tracking_issue_id = 69202
stabilization_pr_id = 69538
items = [
    "impl<T, const N: usize> TryFrom<Box<[T]>> for Box<[T; N]>",
    "impl<T, const N: usize> TryFrom<Rc<[T]>> for Rc<[T; N]>",
    "impl<T, const N: usize> TryFrom<Arc<[T]>> for Arc<[T; N]>",
]
+++
