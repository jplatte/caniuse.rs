+++
title = "`FromIterator` implementations for `Box<str>`"
flag = "boxed_str_from_iter"
impl_pr_id = 70094
items = [
    "impl<'a> FromIterator<&'a char> for Box<str>",
    "impl<'a> FromIterator<&'a str> for Box<str>",
    "impl<'a> FromIterator<Cow<'a, str>> for Box<str>",
    "impl FromIterator<String> for Box<str>",
    "impl FromIterator<char> for Box<str>",
]
+++
