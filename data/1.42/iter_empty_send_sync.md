+++
title = "unconditional `Send` and `Sync` implementations for `iter::Empty<T>`"
flag = "iter_empty_send_sync"
impl_pr_id = 68348
items = [
    "unsafe impl<T> Send for Empty<T> {}",
    "unsafe impl<T> Sync for Empty<T> {}",
]
+++
