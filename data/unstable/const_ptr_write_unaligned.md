+++
title = "pointer `write_unaligned` as `const fn`"
flag = "const_ptr_write"
impl_pr_id = 81167
tracking_issue_id = 86302
items = [
    "core::ptr::write_unaligned",
    "<*const T>::write_unaligned",
    "<*mut T>::write_unaligned",
]
+++
