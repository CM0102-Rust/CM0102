// ---------- batch 326: the European Championship qualifying draw (euro_champ_qual.cpp) ----------

/// The fixed draw of the 1998 edition (the qualifiers of Euro 2000): for each entrant k, its nation key and its slot in
/// the drawn array (group × 6 + place; 9 groups, groups 1, 3, 6 and 8 with six teams).
const EURO_QUAL_1998_DRAW: [(usize, u8); 49] = [
    (KEY_NATION_BELARUS, 0), (KEY_NATION_WALES, 1), (KEY_NATION_SWITZERLAND, 2), (KEY_NATION_DENMARK, 3), (KEY_NATION_ITALY, 4),
    (KEY_NATION_SLOVENIA, 6), (KEY_NATION_LATVIA, 7), (KEY_NATION_GEORGIA, 8), (KEY_NATION_GREECE, 9), (KEY_NATION_NORWAY, 10),
    (KEY_NATION_ALBANIA, 11), (KEY_NATION_MOLDOVA, 12), (KEY_NATION_NORTHERN_IRELAND, 13), (KEY_NATION_FINLAND, 14),
    (KEY_NATION_TURKEY, 15), (KEY_NATION_GERMANY, 16), (KEY_NATION_ARMENIA, 18), (KEY_NATION_ICELAND, 19),
    (KEY_NATION_UKRAINE, 20), (KEY_NATION_FRANCE, 21), (KEY_NATION_RUSSIA, 22), (KEY_NATION_ANDORRA, 23),
    (KEY_NATION_LUXEMBOURG, 24), (KEY_NATION_POLAND, 25), (KEY_NATION_SWEDEN, 26), (KEY_NATION_BULGARIA, 27),
    (KEY_NATION_ENGLAND, 28), (KEY_NATION_SAN_MARINO, 30), (KEY_NATION_CYPRUS, 31), (KEY_NATION_ISRAEL, 32),
    (KEY_NATION_AUSTRIA, 33), (KEY_NATION_SPAIN, 34), (KEY_NATION_LIECHTENSTEIN, 36), (KEY_NATION_HUNGARY, 37),
    (KEY_NATION_SLOVAKIA, 38), (KEY_NATION_PORTUGAL, 39), (KEY_NATION_ROMANIA, 40), (KEY_NATION_AZERBAIJAN, 41),
    (KEY_NATION_MALTA, 42), (KEY_NATION_FYR_OF_MACEDONIA, 43), (KEY_NATION_REPUBLIC_OF_IRELAND, 44), (KEY_NATION_CROATIA, 45),
    (KEY_NATION_YUGOSLAVIA, 46), (KEY_NATION_SCOTLAND, 48), (KEY_NATION_FAROE_ISLANDS, 49),
    (KEY_NATION_BOSNIA_HERZEGOVINA, 50), (KEY_NATION_LITHUANIA, 51), (KEY_NATION_ESTONIA, 52), (KEY_NATION_CZECH_REPUBLIC, 53),
];

/// The work of `draw_groups` over the drawn array `drawn` (6 dwords per group): returns the indices of the dwords it
/// wrote, in order.
///
/// Edition year (+0x40, word) 1998: the fixed draw `EURO_QUAL_1998_DRAW`: entrant k (the entrants array
/// `+0xee`) becomes the national team of the nation of its key (`exe::database_get_national_team`([`G_NATION_TABLE`] +
/// `nation::SIZE` × key), cdecl) and goes into its slot. Other years: groups = +0xff + 2 (a signed byte; the
/// constructor sets +0xff to 7, so 9); a group order 0..8 and a fill count per group (zero); entrant i = 0. Up to 6 pot
/// rounds, while i is below the entrant count (+0xf2, dword): the order is shuffled by 20 swaps of two positions drawn
/// with `exe::utils_random`(groups) (cdecl); then, for each position j of the order (while i is below the count), its
/// group g (skipped when g ≥ groups) takes entrant i when its fill count is below
/// `EuroChampQual::get_group_team_count`(g − 1) (`exe::eurochampqual_get_group_team_count`: push; ecx = this ->
/// thiscall, ret 4; the byte g − 1, a defect of the original: the capacity of the previous group, so in format 1 the
/// groups of six are 1, 3 and 6, and group 0 asks for group −1), at slot g × 6 + fill; then i + 1 and fill + 1.
pub(crate) unsafe fn draw_groups_into(this: usize, drawn: usize) -> Vec<usize> {
    let mut written = Vec::new();
    let entrants = read_usize(this + 0xee);
    if read_u16(this + 0x40) == 0x7ce {
        covered!("gra_22 the fixed 1998 draw");
        for (k, &(key, slot)) in EURO_QUAL_1998_DRAW.iter().enumerate() {
            let team = exe::database_get_national_team(read_usize(G_NATION_TABLE).wrapping_add((read_u32(key) as usize).wrapping_mul(nation::SIZE)));
            write_usize(entrants + 4 * k, team);
            write_usize(drawn + 4 * slot as usize, read_usize(entrants + 4 * k));
            written.push(slot as usize);
        }
        return written;
    }
    let groups = read_u8(this + 0xff).wrapping_add(2) as i8;
    // ponytail: 16-entry tables; the original has 9 (groups is always 9; above 9 it reads stack leftovers)
    let mut order: [i8; 16] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 0, 0, 0, 0, 0, 0, 0];
    let mut fill = [0i8; 16];
    let mut i: i8 = 0;
    let count = read_i32(this + 0xf2);
    for _pot in 0..6 {
        if i as i32 >= count {
            break;
        }
        for _ in 0..0x14 { // gra22: 20 swaps per pot
            let a = exe::utils_random(groups as i32) as i8 as usize;
            let b = exe::utils_random(groups as i32) as i8 as usize;
            order.swap(a, b);
        }
        covered!("gra_22 pot drawn");
        let mut j: i8 = 0;
        while j < groups {
            if i as i32 >= count {
                break;
            }
            let g = order[j as usize];
            if g < groups {
                let c = fill[g as usize];
                let cap = exe::eurochampqual_get_group_team_count(this, g.wrapping_sub(1) as u8 as u32); // gra22: the capacity asked for g - 1
                if (c as i16) < cap {
                    let slot = (c as i32 + g as i32 * 6) as usize;
                    write_usize(drawn + 4 * slot, read_usize(entrants + 4 * i as usize));
                    written.push(slot);
                    i += 1;
                    fill[g as usize] = c + 1;
                }
            }
            j += 1;
        }
    }
    written
}

/// Rule: the European Championship qualifying entrants are drawn into the groups: in 1998 by a fixed table, otherwise
/// by up to six pots in which the group order is shuffled and each group in turn takes the next entrant while it has
/// room.
///
/// `this.draw_groups()`: allocates the drawn array (`euro_champ_qual::DRAWN_TEAMS`, 6 dwords per group, groups = +0xff
/// + 2: `exe::crt_malloc`, cdecl; out of memory: the error box at line 0x262 and nothing more) and fills it
/// (`draw_groups_into`). The slots no entrant reaches keep what the allocation left.
/// Original: `EuroChampQual::draw_groups` (0x57dab0, euro_champ_qual.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn euro_champ_qual_draw_groups(this: usize) {
    let groups = read_u8(this + 0xff).wrapping_add(2);
    let a = exe::crt_malloc(((groups.wrapping_mul(6) as i8 as i32) << 2) as u32);
    write_usize(this + euro_champ_qual::DRAWN_TEAMS, a);
    if a == 0 {
        show_error_at(0x9c4674, 0x262);
        return;
    }
    draw_groups_into(this, a);
}

/// Comparer for `euro_champ_qual_draw_groups`: the original runs for real (its array stays); from the generator state
/// before it, the copy draws again into a local array without counting the draws; the generator state after both and
/// every dword the copy wrote must equal the original's array. (In 1998 the copy also rewrites the entrants with the same
/// national teams.)
unsafe extern "thiscall" fn compare_euro_champ_qual_draw_groups(this: usize) {
    let rng_before = crate::comparison::MemorySnapshot::read(&crate::rng::STATE_REGIONS);
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_EURO_CHAMP_QUAL_DRAW_GROUPS)(this);
    let a = read_usize(this + euro_champ_qual::DRAWN_TEAMS);
    if a == 0 {
        return;
    }
    let rng_after = crate::comparison::MemorySnapshot::read(&crate::rng::STATE_REGIONS);
    rng_before.write_back();
    let slots = (read_u8(this + 0xff).wrapping_add(2) as usize * 6).max(54);
    let local = vec![0u32; slots];
    let written = crate::rng::pause_trace(|| crate::rng::without_counting(|| draw_groups_into(this, local.as_ptr() as usize)));
    let rng_copy = crate::comparison::MemorySnapshot::read(&crate::rng::STATE_REGIONS);
    rng_after.write_back();
    let bad = written.iter().find(|&&k| local[k] as usize != read_usize(a + 4 * k)).copied();
    SCOREBOARD.record(bad.is_none() && rng_copy == rng_after, || format!("euro_champ_qual_draw_groups({:#x}): {} slots written, first different {:?}, generator equal {}",
        this, written.len(), bad, rng_copy == rng_after));
}
