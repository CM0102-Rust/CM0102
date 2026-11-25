// --- rx06_1: the clean-up before a fatal error exit (game.cpp): game_fatal_cleanup (0x5cc7d0) and
// HallOfFame::~HallOfFame (0x5cc830) ---

/// `Config::~Config` (game_config.cpp, thiscall): shuts down what the configuration started.
const FN_CONFIG_DESTRUCTOR: usize = 0x5d7b70;
/// The interface root pointer the message boxes draw on (`gui_show_message_box` reads it).
const G_GUI_ROOT: usize = 0xb67a30;
/// The hall of fame table: 2800 entries of 120 bytes, whose destructor (0x5a53c0) is an empty body.
const HALL_OF_FAME_ENTRIES: u32 = 0xaf0;
const HALL_OF_FAME_ENTRY_SIZE: u32 = 0x78;
const HALL_OF_FAME_ENTRY_DESTRUCTOR: usize = 0x5a53c0;

/// Frees the game data (`free_game_data(1)`), then, when they exist, destroys and deletes the hall of fame
/// (`G_HALL_OF_FAME`) and the configuration (`G_CONFIG`, `Config::~Config`), clearing each pointer; finally clears the
/// interface root pointer, so no later box draws on the screen. Every fatal error box of the exe calls it before
/// `exit(-1)`.
/// Original: `game_fatal_cleanup` (0x5cc7d0, game.cpp, cdecl).
pub(crate) unsafe extern "C" fn game_fatal_cleanup() {
    exe::free_game_data(1);
    let hall = read_usize(G_HALL_OF_FAME);
    if hall != 0 {
        hall_of_fame_destructor(hall);
        exe::crt_operator_delete(hall);
        write_u32(G_HALL_OF_FAME, 0);
    }
    let config = read_usize(G_CONFIG);
    if config != 0 {
        let destroy: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_CONFIG_DESTRUCTOR);
        destroy(config);
        exe::crt_operator_delete(config);
        write_u32(G_CONFIG, 0);
    }
    write_u32(G_GUI_ROOT, 0); // rx06_1: no box draws on the screen after the clean-up
}

/// Comparer for `game_fatal_cleanup`: it frees the game and cannot run twice, so only the original runs (it is only
/// reached on a fatal error exit).
pub(crate) unsafe extern "C" fn compare_game_fatal_cleanup() {
    original_fn::<unsafe extern "C" fn()>(%%I0%%)();
}

/// Runs the entry destructor (an empty body) over the 2800 entries of 120 bytes of the hall of fame table.
/// Original: `HallOfFame::~HallOfFame` (0x5cc830, game.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn hall_of_fame_destructor(this: usize) {
    exe::crt_vector_destructor_iterator(this, HALL_OF_FAME_ENTRY_SIZE, HALL_OF_FAME_ENTRIES, HALL_OF_FAME_ENTRY_DESTRUCTOR);
}

/// Comparer for `HallOfFame::~HallOfFame`: a destructor runs once, so only the original runs.
pub(crate) unsafe extern "thiscall" fn compare_hall_of_fame_destructor(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I1%%)(this);
}
