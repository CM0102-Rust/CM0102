// --- sx09c_6: the dispute object's construction and release, and filing a complaint to the Player Status Committee (dispute.cpp) ---

/// `DISPUTE::load` (0x569910, dispute.cpp; thiscall(dispute, save), ret 4, al): reads dispute.dat (sx09c_3).
const FN_DISPUTE_LOAD: usize = 0x569910;
/// `dispute_post_news` (0x56a3b0, dispute.cpp; thiscall(dispute, person, club, outcome, a, b), ret 0x14; sx09b_6).
const FN_DISPUTE_POST_COMPLAINT_NEWS: usize = 0x56a3b0;

/// Frees the dispute object's flag table (`free`) and destroys its list (`LLIST::~LLIST` and `operator delete`), each
/// present one, setting the pointers to null.
unsafe fn dispute_free_tables(this: usize) {
    if read_usize(this + 4) != 0 {
        exe::crt_free(read_usize(this + 4));
        write_usize(this + 4, 0);
    }
    let list = read_usize(this);
    if list != 0 {
        exe::drop_llist(list);
        exe::crt_operator_delete(list);
        write_usize(this, 0);
    }
}

/// Builds the dispute object: from the save's dispute.dat (`DISPUTE::load`) or new: an empty per-person flag table and
/// an empty dispute list.
///
/// `DISPUTE::DISPUTE(save)`: the flag table (+4) and the list (+0) null; with a save, `DISPUTE::load`
/// (`FN_DISPUTE_LOAD`); when it answers, nothing else; when it fails, what it left is freed (`dispute_free_tables`).
/// Then (also without a save) the flag table: one zeroed byte per person (`calloc(G_STAFF_COUNT, 1)`), and a new list
/// (`operator new(0xe)` and `LLIST::LLIST`, null when the allocation fails). Returns the object.
/// Original: `DISPUTE::DISPUTE` (0x5697e0, dispute.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn new_dispute(this: usize, save: usize) -> usize {
    write_usize(this + 4, 0);
    write_usize(this, 0);
    if save != 0 {
        let load: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(FN_DISPUTE_LOAD);
        if load(this, save) != 0 {
            return this;
        }
        covered!("sx09c_6 dispute.dat not loaded");
        dispute_free_tables(this);
    }
    write_usize(this + 4, exe::crt_calloc(read_u32(G_STAFF_COUNT), 1)); // sx09c_6: the per-person dispute flags
    write_usize(this, club_history_new_list());
    this
}

/// Comparer for `new_dispute`: it allocates (and, with a save, reads the save item), so it cannot run twice; only the
/// original runs. The copy is proven by the new career (Rust mode).
pub(crate) unsafe extern "thiscall" fn compare_new_dispute(this: usize, save: usize) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I0%%)(this, save)
}

/// Releases the dispute object: the flag table and the list (`dispute_free_tables`).
/// Original: `DISPUTE::~DISPUTE` (0x5698d0, dispute.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn drop_dispute(this: usize) {
    dispute_free_tables(this);
}

/// Comparer for `drop_dispute`: it frees, so it cannot run twice; only the original runs, and both pointers must end
/// null.
pub(crate) unsafe extern "thiscall" fn compare_drop_dispute(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I1%%)(this);
    SCOREBOARD.record(read_usize(this) == 0 && read_usize(this + 4) == 0, || format!("drop_dispute({:#x}): pointers left", this));
}

/// Whether `file_complaint` goes ahead for the person: a person with a player record and an id below `G_STAFF_COUNT`,
/// not already in a dispute (his flag byte), not absent without leave (`exe::awol_is_player_awol` on `G_AWOL` with 0)
/// and the FIFA transfer rules in force (`G_FIFA_TRANSFER_RULES_IN_FORCE`).
unsafe fn complaint_goes_ahead(this: usize, person: Staff) -> bool {
    if person.is_null() || person.player().is_null() || read_i32(person.addr()) >= read_i32(G_STAFF_COUNT) {
        return false;
    }
    let flag = read_usize(this + 4).wrapping_add(read_usize(person.addr()));
    read_u8(flag) == 0 && exe::awol_is_player_awol(read_usize(G_AWOL), person, 0) == 0 && read_u8(flag) == 0
        && read_u8(G_FIFA_TRANSFER_RULES_IN_FORCE) != 0
}

/// Starts a player's complaint to the Player Status Committee: his dispute flag is set, his reputations drop, he and
/// his club fall out, the dispute is queued for a hearing 2 to 21 days ahead and the complaint news is posted.
///
/// `DISPUTE::file_complaint(person, club, a, b)` (from the fine 0x5a692b and the media 0x741ebe): nothing unless
/// `complaint_goes_ahead`. The flag byte set to 1. The player record's current, home and world reputations (in that
/// order) each become `_ftol((1.0 − random(21) × 0.01) × reputation)`. With a club (`club_job()`): the player dislikes
/// its manager (`exe::staff_add_disliked_staff` with 1), the manager dislikes him unless the club is human
/// (`exe::human_manager_is_human_club` with 1, 1, 0, 0); when the low byte of `b` is 0, each squad member (50 slots)
/// other than him with random(loyalty) above 5 and random(professionalism) = 0 dislikes him; and the chairman dislikes
/// him. The 17-byte dispute (person id; the id of `club`, read even when it is null; the hearing date: today plus
/// random(20) + 2 days, `exe::date_add_days`; the byte +0x10: the low byte of `b`, or 0 when random(10) = 0 or `b` is 0,
/// random(10) drawn only when it is not) goes to the end of the list. The news (`dispute_post_news` with `club`, or the
/// player's club when it is null, outcome 0, `a` and `b`).
/// Original: `DISPUTE::file_complaint` (0x56a100, dispute.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn file_complaint(this: usize, person: usize, club: usize, a: u32, b: u32) {
    let person = Staff(person);
    if !complaint_goes_ahead(this, person) {
        return;
    }
    covered!("sx09c_6 complaint filed");
    write_u8(read_usize(this + 4).wrapping_add(read_usize(person.addr())), 1);
    let cut = |r: i16| ftol_x87((read_f64(rdata::F64_1) - exe::utils_random(0x15) as f64 * read_f64(rdata::F64_0_01)) * r as f64) as i16;
    let v = cut(person.player().current_reputation()); // sx09c_6: the reputations drop up to 20%
    person.player().set_current_reputation(v);
    let v = cut(person.player().home_reputation());
    person.player().set_home_reputation(v);
    let v = cut(person.player().world_reputation());
    person.player().set_world_reputation(v);
    let keep = b as u8;
    if !person.club_job().is_null() {
        exe::staff_add_disliked_staff(person, person.club_job().manager(), 1);
        if exe::human_manager_is_human_club(person.club_job(), 1, 1, 0, 0) == 0 {
            exe::staff_add_disliked_staff(person.club_job().manager(), person, 1);
        }
        if keep == 0 {
            for k in 0..50usize {
                let mate = Staff(read_usize(person.club_job().addr() + club::SQUAD + 4 * k));
                if !mate.is_null() && mate.addr() != person.addr() && exe::utils_random(mate.loyalty() as i32) > 5
                    && exe::utils_random(mate.professionalism() as i32) == 0 {
                    exe::staff_add_disliked_staff(mate, person, 1);
                }
            }
        }
        exe::staff_add_disliked_staff(person.club_job().chairman(), person, 1);
    }
    let mut record = [0u8; 0x11];
    let r = record.as_mut_ptr() as usize;
    let mut scratch = [0u8; 8];
    exe::date_date_54a110(r + 8);
    write_u32(r, read_u32(person.addr()));
    write_u32(r + 4, read_u32(club)); // the club's id
    exe::date_operator_3d(r + 8, scratch.as_mut_ptr() as usize, G_CURRENT_DATE);
    let days = exe::utils_random(0x14) + 2;
    let mut later = [0u8; 8];
    let hearing = exe::date_add_days(r + 8, later.as_mut_ptr() as usize, days as u32);
    exe::date_operator_3d(r + 8, scratch.as_mut_ptr() as usize, hearing);
    write_u8(r + 0x10, if keep != 0 && exe::utils_random(10) != 0 { keep } else { 0 });
    exe::llist_add_tail(read_usize(this), r, 0x11);
    let news_club = if club == 0 { person.club_job().addr() } else { club };
    let post: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32) = core::mem::transmute(FN_DISPUTE_POST_COMPLAINT_NEWS);
    post(this, person.addr(), news_club, 0, a, b);
}

/// Comparer for `file_complaint`: it draws at random, changes relations and posts news, so only the original runs;
/// when the complaint goes ahead (checked before), the person's flag must end at 1 and the list must have one more
/// dispute; otherwise neither changes.
pub(crate) unsafe extern "thiscall" fn compare_file_complaint(this: usize, person: usize, club: usize, a: u32, b: u32) {
    let goes = complaint_goes_ahead(this, Staff(person));
    let list = Llist(read_usize(this));
    let before = list.count();
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32)>(%%I2%%)(this, person, club, a, b);
    let ok = if goes {
        read_u8(read_usize(this + 4).wrapping_add(read_usize(person))) == 1 && list.count() == before + 1
    } else {
        list.count() == before
    };
    SCOREBOARD.record(ok, || format!("file_complaint(person {:#x}): goes ahead {}, disputes {} -> {}", person, goes, before, list.count()));
}
