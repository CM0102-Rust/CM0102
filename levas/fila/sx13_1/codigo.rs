// ---------- batch %%LEVA%%: the linked list node and the empty list (main.cpp) ----------

/// Copies `n` bytes from `from` to `to` (the `rep movsd`/`rep movsb` of the list routines); nothing for 0.
pub(crate) unsafe fn llist_copy_bytes(from: usize, to: usize, n: usize) {
    if n > 0 {
        core::ptr::copy_nonoverlapping(from as *const u8, to as *mut u8, n);
    }
}

/// Builds a list node that holds its own copy of an item: the next and previous links are cleared, a buffer of `size`
/// bytes (the signed word widened) is allocated with `operator new`, the size is kept as a word and the item's bytes are
/// copied into the buffer. Returns the node. Only `LLIST::add_tail` (0x68d0d0) builds nodes.
/// Defect of the original: the allocation is not checked, so a failed `operator new` copies into address 0.
/// Original: `LLIST_NODE::LLIST_NODE` (0x68cfc0, main.cpp, thiscall, ret 8; the size is read as a word).
pub(crate) unsafe extern "thiscall" fn llist_node_new(this: usize, data: usize, size: u32) -> usize {
    let node = LlistNode(this);
    covered!("sx13_1 node built");
    let size = size as i16;
    node.set_prev(0);
    node.set_next(0);
    let copy = exe::crt_operator_new(size as i32 as u32);
    node.set_data_size(size);
    node.set_data(copy as i32);
    llist_copy_bytes(data, copy, size as i32 as usize); // sx13_1: the whole item
    this
}

/// Frees the copy of the item that a list node owns (`operator delete` of the node's data). The data pointer stays in the
/// node: the callers free the node right after.
/// Original: `LLIST_NODE::~LLIST_NODE` (0x68d010, main.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn llist_node_drop(this: usize) {
    exe::crt_operator_delete(LlistNode(this).data() as u32 as usize);
}

/// Initialises an empty linked list: the head, the tail and the cursor are cleared and the count is zero. Returns the
/// list.
/// Original: `LLIST::LLIST` (0x68d020, main.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn llist_new(this: usize) -> usize {
    let list = Llist(this);
    list.set_head(0);
    list.set_tail(0);
    list.set_cursor(0); // sx13_1: the cursor cleared
    list.set_count(0);
    this
}

/// Comparer for `llist_node_new`: only the original (it allocates the copy); the node must hold cleared links, the size
/// and a buffer with the item's bytes.
pub(crate) unsafe extern "thiscall" fn compare_llist_node_new(this: usize, data: usize, size: u32) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> usize>(%%I0%%)(this, data, size);
    let node = LlistNode(this);
    let n = size as i16 as i32 as usize;
    let copy = node.data() as u32 as usize;
    let same_bytes = n == 0 || core::slice::from_raw_parts(copy as *const u8, n) == core::slice::from_raw_parts(data as *const u8, n);
    let ok = r == this && node.next() == 0 && node.prev() == 0 && node.data_size() == size as i16 && copy != 0 && same_bytes;
    SCOREBOARD.record(ok, || format!("llist_node_new(node {:#x}, data {:#x}, size {}) returned {:#x}: links {:#x}/{:#x}, size {}, copy {:#x}, same bytes {}",
        this, data, size as i16, r, node.next(), node.prev(), node.data_size(), copy, same_bytes));
    r
}

/// Comparer for `llist_node_drop`: only the original (it frees the copy); the node itself is not touched.
pub(crate) unsafe extern "thiscall" fn compare_llist_node_drop(this: usize) {
    let before: [u8; 14] = core::ptr::read_unaligned(this as *const [u8; 14]);
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I1%%)(this);
    let after: [u8; 14] = core::ptr::read_unaligned(this as *const [u8; 14]);
    SCOREBOARD.record(after == before, || format!("llist_node_drop(node {:#x}): node {:x?} -> {:x?}", this, before, after));
}

/// Comparer for `llist_new`: side by side over the 14-byte list header and the returned list.
pub(crate) unsafe extern "thiscall" fn compare_llist_new(this: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> usize>(%%I2%%);
    let (r, n, equal, mo, mr) = crate::comparison::side_by_side_light(&[(this, 14)], || o(this), || llist_new(this));
    SCOREBOARD.record(equal && r == n, || format!("llist_new({:#x}) original={:#x} rust={:#x}: differences in {:?}", this, r, n, first_differences(&mo, &mr)));
    r
}
