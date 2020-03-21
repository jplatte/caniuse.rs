+++
title = "boxed closure `Fn*` trait implementations"
flag = "boxed_closure_impls"
impl_pr_id = 59500
items = [
    "impl<A, F> FnOnce for Box<F> where F: FnOnce<A> + ?Sized",
    "impl<A, F> FnMut for Box<F> where F: FnMut<A> + ?Sized",
    "impl<A, F> Fn for Box<F> where F: Fn<A> + ?Sized",
]
+++
