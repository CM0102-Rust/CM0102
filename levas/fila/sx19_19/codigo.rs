// ---------- sx19_19: a ruling body's day: the pending bans whose date has come (ruling_body/ruling_body.cpp) ----------

/// Whether a pending ban's start (node +0x14: day i16, year u16) is after `date`: the list is in date order, so the
/// day's work stops at the first such node.
unsafe fn pending_ban_after(node: usize, date: usize) -> bool {
    let (yn, yd) = (read_u16(node + 0x16), read_u16(date + 2));
    yd < yn || (yd == yn && read_i16(date) < read_i16(node + 0x14))
}

/// Applies the body's pending bans whose start date has come.
///
/// From the start of the body's list, until the first node that starts after `date`: the node's ban record takes the
/// node's start date (+0x14 into +4) and the player's entry is looked up (not added). Without an entry the node is just
/// removed. Otherwise the player's bans move on (`Discipline::update_ban_progress` with the body's rule set and 0
/// matches); without a ban in the body, the node's ban is added as it is (flag bit 0 cleared, +0x6f = 1,
/// `Discipline::add_ban`); with one, the node's matches (or days, for a ban by days) go onto it (replacing a 0xff or −1)
/// with flag bit 2, and the node gets flag bit 0. The ban news is posted (the player, the node and the node's fixture at
/// +0x20) and the node removed; each removal restarts the walk from the head.
/// Original: `RulingBody::daily_update` (0x7e7d50, ruling_body.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn ruling_body_daily_update(this: usize, date: usize) {
    let list = Llist(read_usize(this + RULING_BODY_LIST));
    if list.is_null() {
        return;
    }
    let d = read_usize(G_DISCIPLINE);
    list.set_cursor(list.head());
    let mut node = exe::llist_get_next(list);
    while node != 0 {
        if pending_ban_after(node, date) {
            return;
        }
        covered!("sx19_19 pending ban due");
        copy_record_date(node + ban_record::START_DATE, node + 0x14); // the pending ban starts on its date
        let id = read_u32(node + pending_ban::PERSON_ID);
        let e = exe::rulingbody_find_or_add_discipline_entry(this, id, 0);
        if e != 0 {
            exe::discipline_update_ban_progress(d, id, read_u8(this + ruling_body::RULE_SET_INDEX) as u32, 0);
            let b = read_i16(e + ruling_body_entry::BAN_INDEX);
            if b == -1 {
                write_u8(node + ban_record::FLAGS, read_u8(node + ban_record::FLAGS) & 0xfe);
                write_u8(node + 0x6f, 1); // the pending ban's byte +0x6f
                add_ban(d, id, node);
            } else {
                covered!("sx19_19 pending ban merged");
                write_u8(node + ban_record::FLAGS, read_u8(node + ban_record::FLAGS) | 1);
                let ban = read_usize(d + discipline_fields::BANS).wrapping_add((b as isize * 20) as usize);
                let matches = read_u8(node + ban_record::MATCHES);
                if matches != 0xff {
                    let old = read_u8(ban + ban_record::MATCHES);
                    write_u8(ban + ban_record::MATCHES, if old == 0xff { matches } else { old.wrapping_add(matches) }); // sx19_19: the pending matches added
                } else {
                    let (old, days) = (read_i16(ban + ban_record::DAYS), read_i16(node + ban_record::DAYS));
                    write_i16(ban + ban_record::DAYS, if old == -1 { days } else { old.wrapping_add(days) });
                }
                write_u8(ban + ban_record::FLAGS, read_u8(ban + ban_record::FLAGS) | 4);
            }
            let post: unsafe extern "thiscall" fn(usize, usize, usize, usize) = core::mem::transmute(FN_POST_BAN_NEWS);
            post(d, staff_at(id as isize).addr(), node, node + 0x20);
        }
        exe::llist_remove(list, node);
        list.set_cursor(list.head());
        node = exe::llist_get_next(list);
    }
}

/// Comparer of the day: the bans and their news cannot be handed out twice, so only the original runs; checked
/// afterwards: the nodes due that day (the leading ones not after the date) left the list and the others stayed.
pub(crate) unsafe extern "thiscall" fn compare_ruling_body_daily_update(this: usize, date: usize) {
    let list = Llist(read_usize(this + RULING_BODY_LIST));
    let (mut due, mut total) = (0u16, 0u16);
    if !list.is_null() {
        total = list.count() as u16;
        list.set_cursor(list.head());
        let mut node = exe::llist_get_next(list);
        while node != 0 && !pending_ban_after(node, date) {
            due += 1;
            node = exe::llist_get_next(list);
        }
    }
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I0%%)(this, date);
    let left = if list.is_null() { 0 } else { list.count() as u16 };
    SCOREBOARD.record(left == total - due, || format!("ruling_body_daily_update({:#x}): {} nodes left of {} ({} due)", this, left, total, due));
}
