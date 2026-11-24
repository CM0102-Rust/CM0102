// ---------- batch %%LEVA%%: copying a linked list (main.cpp) ----------

/// Empties the list (as `LLIST::clear`) and appends a copy of every item of the source list, in order
/// (`LLIST::add_tail` with the source node's size). The source is walked with its own cursor, which starts at its head
/// and moves on before each append; the copy stops at the end or at a node with a null item, where the source cursor
/// stays.
/// Original: `LLIST::copy_from` (0x68d2f0, main.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn llist_assign(this: usize, source: usize) {
    manager_manager_clear(this);
    let src = Llist(source);
    src.set_cursor(src.head()); // sx13_3: from the source's head
    loop {
        let node = LlistNode(src.cursor());
        if node.is_null() || node.data() == 0 {
            break;
        }
        covered!("sx13_3 item copied");
        src.set_cursor(node.next());
        // the original pushes the size as a word (the high half is stale); add_tail reads the word only
        exe::llist_add_tail(this, node.data() as u32 as usize, node.data_size() as u16 as u32);
    }
}

/// One item of a list as `llist_assign` copies it: the bytes of the node's item.
unsafe fn llist_item_bytes(node: LlistNode) -> Vec<u8> {
    let n = node.data_size() as i32;
    if n <= 0 {
        return Vec::new();
    }
    core::slice::from_raw_parts(node.data() as u32 as *const u8, n as usize).to_vec()
}

/// Comparer for `llist_assign`: only the original (it frees and allocates nodes), checked against the plan read from the
/// source before: the same items in the same order (bytes and sizes), the tail on the last node, the cursor of the copy
/// null and the source cursor where the walk stopped.
pub(crate) unsafe extern "thiscall" fn compare_llist_assign(this: usize, source: usize) {
    let src = Llist(source);
    let mut items = Vec::new();
    let mut stop = src.head();
    while stop != 0 && LlistNode(stop).data() != 0 {
        items.push((LlistNode(stop).data_size(), llist_item_bytes(LlistNode(stop))));
        stop = LlistNode(stop).next();
    }
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I0%%)(this, source);
    let list = Llist(this);
    let mut copied = Vec::new();
    let (mut node, mut last) = (list.head(), 0);
    while node != 0 && copied.len() <= items.len() {
        copied.push((LlistNode(node).data_size(), llist_item_bytes(LlistNode(node))));
        last = node;
        node = LlistNode(node).next();
    }
    let ok = this == source
        || (copied == items && list.count() as usize == items.len() && list.tail() == last && list.cursor() == 0 && src.cursor() == stop);
    SCOREBOARD.record(ok, || format!("llist_assign({:#x} <- {:#x}): {} items expected, {} copied (count {}), source cursor {:#x} (expected {:#x})",
        this, source, items.len(), copied.len(), list.count(), src.cursor(), stop));
}
