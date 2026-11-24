// ---------- sx19b_2: the scout manager's static list and its destructor (scout_manager.cpp) ----------

/// `scout_manager_static_destroy_list_b74438` (0x7f6750): the atexit destructor of `G_SCOUT_CLUB_TABLE_LIST`.
const FN_SCOUT_CLUB_TABLE_LIST_DESTROY: usize = 0x7f6750;
/// `PLAYER_SEARCH::~PLAYER_SEARCH` (0x7ae4f0, player_search.cpp; thiscall).
const FN_PLAYER_SEARCH_DESTRUCTOR: usize = 0x7ae4f0;

/// Static initialiser of the exe (in the CRT start-up, before WinMain, with the detours already installed by the DLL
/// load): builds the global list of scouted club records `G_SCOUT_CLUB_TABLE_LIST` empty (`LLIST::LLIST`) and registers
/// its destructor (`scout_manager_static_destroy_list_b74438`, a jump to `LLIST::~LLIST`) with `exe::crt_atexit`.
/// Original: `scout_manager_static_init_list_b74438` (0x7f6730, scout_manager.cpp, cdecl).
pub(crate) unsafe extern "C" fn scout_manager_static_init_list() {
    exe::new_llist(G_SCOUT_CLUB_TABLE_LIST);
    exe::crt_atexit(FN_SCOUT_CLUB_TABLE_LIST_DESTROY);
}

/// Comparer for `scout_manager_static_init_list`: the original runs (it registers the destructor once); the copy builds
/// an empty list into a local and its 14 bytes are compared with the global's.
pub(crate) unsafe extern "C" fn compare_scout_manager_static_init_list() {
    original_fn::<unsafe extern "C" fn()>(%%I0%%)();
    let mut copy = [0xa5u8; 14];
    exe::new_llist(copy.as_mut_ptr() as usize);
    let orig: [u8; 14] = core::array::from_fn(|k| read_u8(G_SCOUT_CLUB_TABLE_LIST + k));
    SCOREBOARD.record(copy == orig, || format!("scout_manager_static_init_list: original={:02x?} rust={:02x?}", orig, copy));
}

/// Destroys the scout manager: the 0x2b0-byte player searches at +0 (their count in the dword before the array) go
/// through `PLAYER_SEARCH::~PLAYER_SEARCH` (`exe::crt_vector_destructor_iterator`) and the block is deleted, +0 = 0;
/// the scout request array (`scout_manager_fields::SCOUT_REQUESTS`) is freed and zeroed.
/// Original: `SCOUT_MANAGER::~SCOUT_MANAGER` (0x7f6aa0, scout_manager.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn scout_manager_destructor(this: usize) {
    let searches = read_usize(this);
    if searches != 0 {
        exe::crt_vector_destructor_iterator(searches, 0x2b0, read_u32(searches - 4), FN_PLAYER_SEARCH_DESTRUCTOR);
        exe::crt_operator_delete(searches - 4);
        write_usize(this, 0);
    }
    let requests = read_usize(this + scout_manager_fields::SCOUT_REQUESTS);
    if requests != 0 {
        exe::crt_free(requests);
        write_usize(this + scout_manager_fields::SCOUT_REQUESTS, 0);
    }
}

/// Comparer for `scout_manager_destructor`: it frees memory, so only the original runs; checked afterwards: both
/// pointers zeroed.
pub(crate) unsafe extern "thiscall" fn compare_scout_manager_destructor(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I1%%)(this);
    let (s, r) = (read_usize(this), read_usize(this + scout_manager_fields::SCOUT_REQUESTS));
    SCOREBOARD.record(s == 0 && r == 0, || format!("scout_manager_destructor({:#x}): searches {:#x} requests {:#x}", this, s, r));
}
