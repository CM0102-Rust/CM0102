/// The team award's selection (`TEAM_AWARD`): the position mask word of each slot at +0x26 (2 bytes a slot), the
/// 20 slots of 0x19 bytes at +0x4e (+0 staff id, +4/+8/+0xc the three name ids, +0x10 the club id, +0x14 the squad
/// number, +0x15 the rating, f32), the filled count at +0x252, the substitutes count at +0x253 (the slots are 11 +
/// it) and the formation mode at +0x25c.
#[derive(Clone, Copy)]
pub(crate) struct TeamAward(pub(crate) usize);

pub(crate) const TEAM_AWARD_SLOT_SIZE: usize = 0x19;

impl TeamAward {
    pub(crate) unsafe fn slot_count(self) -> i8 {
        (read_u8(self.0 + 0x253) as i8).wrapping_add(11)
    }
    pub(crate) unsafe fn filled(self) -> i8 {
        read_i8(self.0 + 0x252)
    }
    pub(crate) fn slot(self, k: usize) -> usize {
        self.0 + 0x4e + k * TEAM_AWARD_SLOT_SIZE
    }
    pub(crate) unsafe fn slot_mask(self, k: usize) -> u16 {
        read_u16(self.0 + 0x26 + 2 * k)
    }
    pub(crate) unsafe fn slot_id(self, k: usize) -> i32 {
        read_i32(self.slot(k))
    }
    /// The slot accepts a player whose (widened) positions are `mask`: all its own bits are in it.
    pub(crate) unsafe fn fits(self, k: usize, mask: u16) -> bool {
        self.slot_mask(k) & mask == self.slot_mask(k)
    }
    /// The selection is complete: the filled count equals the slot count.
    pub(crate) unsafe fn complete(self) -> bool {
        self.filled() == self.slot_count()
    }
}

/// A player's position bits widened for the team award slots: 2 also gives 4, 8 or 0x20 give 0x10, and 0x40 with
/// 0x80 or with 0x800 gives 0x200.
pub(crate) fn widen_award_positions(mut m: u16) -> u16 {
    if m & 2 != 0 {
        m |= 4;
    }
    if m & 8 != 0 {
        m |= 0x10;
    }
    if m & 0x20 != 0 {
        m |= 0x10;
    }
    if m & 0x40 != 0 && (m & 0x80 != 0 || m & 0x800 != 0) {
        m |= 0x200;
    }
    m
}

/// The name id of a staff name record (+0x33), -1 without one.
unsafe fn award_name_id(record: usize) -> i32 {
    if record == 0 { -1 } else { read_i32(record + 0x33) }
}

/// Writes `person` into team award slot `k`: his id, rating, club id, squad number and three name ids.
unsafe fn team_award_place(t: TeamAward, k: usize, person: Staff, club: usize, rating: f32, number: u8) {
    let s = t.slot(k);
    write_i32(s, read_i32(person.addr()));
    write_f32(s + 0x15, rating);
    write_i32(s + 0x10, read_i32(club));
    write_u8(s + 0x14, number);
    write_i32(s + 4, award_name_id(person.first_name()));
    write_i32(s + 8, award_name_id(person.second_name()));
    write_i32(s + 0xc, award_name_id(person.common_name()));
}

/// Offers a player to the team award. Without a person, a player sheet or a club, nothing. Already chosen, his rating
/// rises to `rating` when higher. Otherwise his positions (`position_mask`, or his sheet's `exe::formation_get_position_mask`
/// when 0) are widened (`widen_award_positions`) and he takes the first empty slot that fits; with none, the first
/// filled slot that fits him whose occupant fits an empty slot: the occupant moves there and he takes the slot. His
/// squad number (absolute) is kept when his club uses squad numbers. Returns 1 when the selection is complete.
/// Original: `TEAM_AWARD::add_candidate` (0x8afa40, team_award.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn team_award_add_candidate(this: usize, person: usize, club: usize, rating: f32, position_mask: u16) -> u32 {
    let t = TeamAward(this);
    let n = t.slot_count();
    let person = Staff(person);
    if person.is_null() || person.player().is_null() || club == 0 {
        return t.complete() as u32;
    }
    covered!("sx05_6 candidate");
    if let Some(k) = (0..n.max(0) as usize).find(|&k| t.slot_id(k) == read_i32(person.addr())) {
        covered!("sx05_6 chosen again");
        if rating > read_f32(t.slot(k) + 0x15) {
            write_f32(t.slot(k) + 0x15, rating);
        }
        return t.complete() as u32;
    }
    let mask = widen_award_positions(if position_mask != 0 { position_mask } else { exe::formation_get_position_mask(person.player()) });
    let mut number = 0u8;
    if exe::squad_manager_club_uses_squad_numbers(read_usize(G_SQUAD_MANAGER), club, 0) != 0 {
        number = (exe::squad_manager_get_squad_number(read_usize(G_SQUAD_MANAGER), person, club) as i32).unsigned_abs() as u8;
    }
    let slots = 0..n.max(0) as usize;
    let place = |k: usize| {
        team_award_place(t, k, person, club, rating, number);
        write_u8(this + 0x252, read_u8(this + 0x252).wrapping_add(1));
        t.complete() as u32
    };
    if let Some(k) = slots.clone().find(|&k| t.slot_id(k) == -1 && t.fits(k, mask)) {
        return place(k);
    }
    for k in slots.clone() {
        if t.slot_id(k) == -1 || !t.fits(k, mask) {
            continue;
        }
        let occupant = widen_award_positions(exe::formation_get_position_mask(staff_at(t.slot_id(k) as isize).player()));
        if let Some(j) = slots.clone().find(|&j| t.slot_id(j) == -1 && t.fits(j, occupant)) {
            covered!("sx05_6 occupant moved");
            core::ptr::copy(t.slot(k) as *const u8, t.slot(j) as *mut u8, TEAM_AWARD_SLOT_SIZE);
            return place(k);
        }
    }
    t.complete() as u32
}

/// Empties the twenty slots (id, name ids and club -1, rating 0; the squad number is left) and the filled count.
/// Original: `TEAM_AWARD::clear_selection` (0x8afd90, team_award.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn team_award_clear_selection(this: usize) {
    let t = TeamAward(this);
    for k in 0..20 {
        for f in [0, 4, 8, 0xc, 0x10] {
            write_i32(t.slot(k) + f, -1);
        }
        write_u32(t.slot(k) + 0x15, 0);
    }
    write_u8(this + 0x252, 0);
}

/// The position masks of the starting eleven by the formation mode (+0x25c), then of the nine substitutes.
const TEAM_AWARD_ELEVENS: [[u16; 11]; 4] = [
    [1, 0x804, 0x84, 0x204, 0x204, 0x210, 0x810, 0x210, 0x240, 0x240, 0x90],
    [1, 0x804, 0x84, 0x204, 0x204, 0x90, 0x210, 0x810, 0xc0, 0x240, 0x840],
    [1, 0x808, 0x88, 0x204, 0x204, 2, 0x210, 0x210, 0x240, 0x240, 0x210],
    [1, 0x804, 0x84, 2, 0x204, 0x208, 0x90, 0x810, 0x220, 0x240, 0x840],
];
const TEAM_AWARD_SUBSTITUTES: [u16; 9] = [1, 0x40, 4, 0x10, 0x40, 4, 0x10, 0x40, 4];

/// Sets the position mask of each slot: the eleven by the formation mode (0, 1 and 2 their own, any other the
/// fourth), the nine substitutes always goalkeeper then three times (0x40, 4, 0x10).
/// Original: `TEAM_AWARD::init_slot_positions` (0x8afdc0, team_award.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn team_award_init_slot_positions(this: usize) {
    let eleven = &TEAM_AWARD_ELEVENS[(read_u8(this + 0x25c) as usize).min(3)];
    for (k, &m) in eleven.iter().chain(TEAM_AWARD_SUBSTITUTES.iter()).enumerate() {
        write_u16(this + 0x26 + 2 * k, m);
    }
}

/// The team award's slot masks, slots and filled count (+0x26..+0x253).
const TEAM_AWARD_SELECTION: (usize, usize) = (0x26, 0x253 - 0x26);

pub(crate) unsafe extern "thiscall" fn compare_team_award_add_candidate(this: usize, person: usize, club: usize, rating: f32, position_mask: u16) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, f32, u16) -> u32>(%%I0%%);
    let (r, v, equal, a, b) = crate::comparison::side_by_side(&[(this + TEAM_AWARD_SELECTION.0, TEAM_AWARD_SELECTION.1)],
        || o(this, person, club, rating, position_mask) & 0xff, || team_award_add_candidate(this, person, club, rating, position_mask));
    SCOREBOARD.record(equal && r == v, || format!("team_award_add_candidate(person {:#x}) original={} rust={} {:?}", person, r, v, first_differences(&a, &b)));
    r
}

pub(crate) unsafe extern "thiscall" fn compare_team_award_clear_selection(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I1%%);
    let (_, _, equal, a, b) = crate::comparison::side_by_side(&[(this + TEAM_AWARD_SELECTION.0, TEAM_AWARD_SELECTION.1)],
        || o(this), || team_award_clear_selection(this));
    SCOREBOARD.record(equal, || format!("team_award_clear_selection({:#x}) {:?}", this, first_differences(&a, &b)));
}

pub(crate) unsafe extern "thiscall" fn compare_team_award_init_slot_positions(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I2%%);
    let (_, _, equal, a, b) = crate::comparison::side_by_side(&[(this + 0x26, 2 * 20)], || o(this), || team_award_init_slot_positions(this));
    SCOREBOARD.record(equal, || format!("team_award_init_slot_positions(mode {}) {:?}", read_u8(this + 0x25c), first_differences(&a, &b)));
}
