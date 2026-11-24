// ---------- cx03_2: the destructors of eleven national, continental and staged cups ----------

/// Where a staged cup's destructor frees its own club list (+0xb2): not at all, between the statistics and the stages,
/// or last, after the statistics.
#[derive(Clone, Copy, PartialEq, Eq)]
enum ClubListFree {
    None,
    BeforeStages,
    Last,
}

/// The body of the destructors of the cups run by stages (thiscall, ret 0; SEH frame only for the unwinding): the
/// class vtable back, the team, round and tie tables freed (`free_season_tables`, pointers left as they are), then in
/// the class's order the statistics (`delete_comp_stats`), the club list +0xb2 (freed, left as it is) and the stages
/// with their vector (`delete_stages_and_vector`, the vector cleared), then the Cup base (`exe::drop_cup`).
unsafe fn drop_cup_with_stages(this: usize, vtable: usize, club_list: ClubListFree) {
    let cup = Cup(this);
    write_usize(this, vtable);
    free_season_tables(cup);
    let free_club_list = || {
        let list = read_usize(this + 0xb2); // the club pool (Swedish Cup) or the qualified club list (Asian cups)
        if list != 0 {
            exe::crt_free(list);
        }
    };
    if club_list == ClubListFree::Last {
        delete_stages_and_vector(League(this));
        delete_comp_stats(League(this).comp_stats());
        free_club_list();
    } else {
        delete_comp_stats(League(this).comp_stats());
        if club_list == ClubListFree::BeforeStages {
            free_club_list();
        }
        delete_stages_and_vector(League(this));
    }
    exe::drop_cup(this);
}

/// Destroys the US Open Cup (`drop_national_cup`).
/// Original: `UsaOpenCup::~UsaOpenCup` (0x90b0a0, usa_open_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn drop_usa_open_cup(this: usize) {
    drop_national_cup(this, 0x970998);
}

/// Destroys the Welsh Cup (`drop_national_cup`).
/// Original: `WelCup::~WelCup` (0x924610, wel_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn drop_wel_cup(this: usize) {
    drop_national_cup(this, 0x970ee0);
}

/// Destroys the Asian Super Cup (`drop_national_cup`).
/// Original: `AsiaSuperCup::~AsiaSuperCup` (0x410840, asia_super_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn drop_asia_super_cup(this: usize) {
    drop_national_cup(this, 0x9676c8);
}

/// Destroys the CONCACAF Champions' Cup (`drop_national_cup`).
/// Original: `ConChamp::~ConChamp` (0x4c0e30, con_champ.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn drop_con_champ(this: usize) {
    drop_national_cup(this, 0x968aa0);
}

/// Destroys the European Super Cup (`drop_national_cup`).
/// Original: `EurSuperCup::~EurSuperCup` (0x57a2f0, eur_super_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn drop_eur_super_cup(this: usize) {
    drop_national_cup(this, 0x969f80);
}

/// Destroys the Russian Cup (`drop_cup_with_stages`, without a club list).
/// Original: `RusCup::~RusCup` (0x7e9420, rus_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn drop_rus_cup(this: usize) {
    drop_cup_with_stages(this, 0x96f124, ClubListFree::None);
}

/// Destroys the Swedish Cup (`drop_cup_with_stages`, the club pool freed before the stages).
/// Original: `SweCup::~SweCup` (0x88ce20, swe_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn drop_swe_cup(this: usize) {
    drop_cup_with_stages(this, 0x970044, ClubListFree::BeforeStages);
}

/// Destroys the Asian Club Championship (`drop_cup_with_stages`, the club list freed last).
/// Original: `AsiaClubChamp::~AsiaClubChamp` (0x40ad70, asia_club_champ.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn drop_asia_club_champ(this: usize) {
    drop_cup_with_stages(this, 0x9674d4, ClubListFree::Last);
}

/// Destroys the Asian Cup Winners' Cup (`drop_cup_with_stages`, the club list freed last).
/// Original: `AsiaCupWinner::~AsiaCupWinner` (0x40ce70, asia_cup_winner.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn drop_asia_cup_winner(this: usize) {
    drop_cup_with_stages(this, 0x967574, ClubListFree::Last);
}

/// Destroys the Welsh League Cup (`drop_group_cup` with its entrant list).
/// Original: `WelLgeCup::~WelLgeCup` (0x926100, wel_lge_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn drop_wel_lge_cup(this: usize) {
    drop_group_cup(this, 0x971034, true);
}

/// Destroys the Welsh Premier Cup (`drop_group_cup` with its entrant list).
/// Original: `WelPrmCup::~WelPrmCup` (0x927a50, wel_prm_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn drop_wel_prm_cup(this: usize) {
    drop_group_cup(this, 0x9710e8, true);
}

/// The comparers of the destructors: they free memory (cannot run twice); only the original runs.
pub(crate) unsafe extern "thiscall" fn compare_drop_usa_open_cup(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I0%%)(this)
}

pub(crate) unsafe extern "thiscall" fn compare_drop_wel_cup(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I1%%)(this)
}

pub(crate) unsafe extern "thiscall" fn compare_drop_asia_super_cup(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I2%%)(this)
}

pub(crate) unsafe extern "thiscall" fn compare_drop_con_champ(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I3%%)(this)
}

pub(crate) unsafe extern "thiscall" fn compare_drop_eur_super_cup(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I4%%)(this)
}

pub(crate) unsafe extern "thiscall" fn compare_drop_rus_cup(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I5%%)(this)
}

pub(crate) unsafe extern "thiscall" fn compare_drop_swe_cup(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I6%%)(this)
}

pub(crate) unsafe extern "thiscall" fn compare_drop_asia_club_champ(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I7%%)(this)
}

pub(crate) unsafe extern "thiscall" fn compare_drop_asia_cup_winner(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I8%%)(this)
}

pub(crate) unsafe extern "thiscall" fn compare_drop_wel_lge_cup(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I9%%)(this)
}

pub(crate) unsafe extern "thiscall" fn compare_drop_wel_prm_cup(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I10%%)(this)
}
