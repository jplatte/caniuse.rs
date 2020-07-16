+++
title = "the `len` method on raw slices"
flag = "slice_ptr_len"
impl_pr_id = 71082 # and 71940
tracking_issue_id = 71146
items = [
    "<*const [T]>::len",
    "<*mut [T]>::len",
    "NonNull<[T]>::len",
]
+++
