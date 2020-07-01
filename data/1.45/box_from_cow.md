+++
title = "`From<Cow<'_, _>>` implementations for `Box`ed types"
flag = "box_from_cow"
impl_pr_id = 71447
items = [
    "impl From<Cow<str>> for Box<str>",
    "impl From<Cow<CStr>> for Box<CStr>",
    "impl From<Cow<OsStr>> for Box<OsStr>",
    "impl From<Cow<Path>> for Box<Path>",
    "impl<T: Copy> From<Cow<[T]>> for Box<[T]>",
]
+++
