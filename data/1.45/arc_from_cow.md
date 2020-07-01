+++
title = "`impl From<Cow<'_, _>>` for `Arc<_>`"
flag = "shared_from_cow"
impl_pr_id = 71447
items = ["""
impl<'a, B> From<Cow<'a, B>> for Arc<B>
where
    B: ToOwned + ?Sized,
    Arc<B>: From<&'a B> + From<B::Owned>,
"""]
+++
