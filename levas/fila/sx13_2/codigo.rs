// ---------- batch %%LEVA%%: freeing a list, walking it backwards and unlinking a node (main.cpp) ----------

/// Frees every node of a linked list, from the head: each node's item copy (`LLIST_NODE::~LLIST_NODE`) and the node
/// (`operator delete`), the head moving to the next one each time. The count, the tail and the cursor are left as they
/// were (the list is going away); the head ends null.
/// Original: `LLIST::~LLIST` (0x68d040, main.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn llist_free_nodes(this: usize) {
    let list = Llist(this);
    while list.head() != 0 {
        let node = list.head();
        let next = LlistNode(node).next();
        exe::drop_llist_node(node);
        exe::crt_operator_delete(node);
        list.set_head(next);
    }
}

/// Returns the item of the cursor node and moves the cursor to the previous node; with no cursor, 0 (the cursor stays
/// null).
/// Original: `LLIST::get_prev` (0x68d1d0, main.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn llist_prev_item(this: usize) -> u32 {
    let list = Llist(this);
    let node = LlistNode(list.cursor());
    if node.is_null() {
        return 0;
    }
    covered!("sx13_2 step back");
    list.set_cursor(node.prev()); // sx13_2: the cursor goes back
    node.data() as u32
}

/// Unlinks a given node from the list and frees it: the head (when it is the head) or the previous node's next link
/// skips it, the tail (when it is the tail) or the next node's previous link skips it, the count goes down by one, a
/// cursor on it moves to its next node, and the item copy and the node are freed. Returns 1.
/// Original: `LLIST::remove_node` (0x68d280, main.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn llist_unlink(this: usize, node: usize) -> u32 {
    let list = Llist(this);
    let n = LlistNode(node);
    if node == list.head() {
        list.set_head(n.next());
    } else {
        LlistNode(n.prev()).set_next(n.next());
    }
    if node == list.tail() {
        covered!("sx13_2 tail unlinked");
        list.set_tail(n.prev()); // sx13_2: the tail goes back
    } else {
        LlistNode(n.next()).set_prev(n.prev());
    }
    let cursor = list.cursor();
    list.set_count(list.count().wrapping_sub(1));
    if node == cursor {
        list.set_cursor(LlistNode(cursor).next());
    }
    if node != 0 {
        exe::drop_llist_node(node);
        exe::crt_operator_delete(node);
    }
    1
}

/// Comparer for `llist_free_nodes`: only the original (it frees the nodes); the head ends null and the count, the tail
/// and the cursor are untouched.
pub(crate) unsafe extern "thiscall" fn compare_llist_free_nodes(this: usize) {
    let list = Llist(this);
    let (count, tail, cursor) = (list.count(), list.tail(), list.cursor());
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I0%%)(this);
    let ok = list.head() == 0 && list.count() == count && list.tail() == tail && list.cursor() == cursor;
    SCOREBOARD.record(ok, || format!("llist_free_nodes({:#x}): header {:x?}", this, core::ptr::read_unaligned(this as *const [u8; 14])));
}

/// Comparer for `llist_prev_item`: the cursor is saved and restored around each side (the game calls it very often).
pub(crate) unsafe extern "thiscall" fn compare_llist_prev_item(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I1%%);
    let list = Llist(this);
    let before = list.cursor();
    let r = o(this);
    let after = list.cursor();
    list.set_cursor(before);
    let v = llist_prev_item(this);
    let after_r = list.cursor();
    list.set_cursor(after);
    SCOREBOARD.record(after == after_r && r == v, || format!("llist_prev_item({:#x}) original={:#x} cursor {:#x} rust={:#x} cursor {:#x}", this, r, after, v, after_r));
    r
}

/// Comparer for `llist_unlink`: only the original (it frees the node), checked against the plan read before: the count
/// down by one, the head, the tail and the cursor, and the neighbours' links.
pub(crate) unsafe extern "thiscall" fn compare_llist_unlink(this: usize, node: usize) -> u32 {
    let list = Llist(this);
    let n = LlistNode(node);
    let before: [u8; 14] = core::ptr::read_unaligned(this as *const [u8; 14]);
    let (next, prev) = (n.next(), n.prev());
    let (count, head, tail, cursor) = (list.count(), list.head(), list.tail(), list.cursor());
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(%%I2%%)(this, node);
    let ok = r == 1
        && list.count() == count.wrapping_sub(1)
        && list.head() == if node == head { next } else { head }
        && list.tail() == if node == tail { prev } else { tail }
        && list.cursor() == if node == cursor { next } else { cursor }
        && (node == head || LlistNode(prev).next() == next)
        && (node == tail || LlistNode(next).prev() == prev);
    SCOREBOARD.record(ok, || format!("llist_unlink({:#x}, node {:#x}) returned {}: header {:x?} -> {:x?}", this, node, r, before,
        core::ptr::read_unaligned(this as *const [u8; 14])));
    r
}
