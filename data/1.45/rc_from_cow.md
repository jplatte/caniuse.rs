+++
title = "`impl From<Cow<'_, _>` for `Rc<_>`"
flag = "shared_from_cow"
impl_pr_id = 71447
items = ["""
impl<'a, B> From<Cow<'a, B>> for Rc<B>
where
    B: ToOwned + ?Sized,
    Rc<B>: From<&'a B> + From<B::Owned>,
"""]
+++
