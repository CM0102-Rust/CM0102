//! The game's routines rewritten in Rust, and how they replace the originals.
//!
//! # How a routine replaces the original
//! The DLL is loaded by the untouched `cm0102.exe` (3.9.68). For each rewritten routine, `detours.rs` writes a jump at
//! the start of the original routine (a "detour"); the jump goes to our Rust copy. Before writing it, the DLL checks the
//! first bytes of the original (`PROLOGOS`, one `Prologue { address, bytes }` per routine) and the hash of the whole
//! routine (`signatures.rs`): if the exe was changed there (another patch), the routine stays original.
//!
//! Each hooked routine is one line of the `routines!` table below: the original address and first bytes, our copy, its
//! comparer (the log names it by the Rust function: `Prologue { .. } => rust_fn, compare_fn;`); the macro builds PROLOGOS and the
//! lists `instalar` uses, so they cannot drift apart. `original_fn::<F>(IDX_<NAME>)` calls the original routine (through the bytes saved before the jump); the
//! `IDX_` constants are generated from the table by build.rs.
//!
//! # How to read a routine file
//! `motor/<folder>/<file>.rs` holds the routines of the game's source file of the same name (`comp/leagues/eng_prm.rs`
//! for `eng_prm.cpp`). Each rewritten routine has a header: what it does, then `Original: Class::method (address,
//! source file, calling convention)` (the Portuguese notes are in docs/ROTINAS.md). The vocabulary:
//! - `compare_<name>`: the comparer. In comparison mode (`CM_COMPARAR=1`) it runs the original and our copy side by
//!   side (or the original only, checking the outcome it planned) and records any difference (`PLACAR`).
//! - `comparison::branch("...")`: a branch counter; the tests use it to know which paths a career went through.
//! - `read_u32(a)`, `write_u8(a, v)` (`mem.rs`): unaligned access to the game's records; `rec + 0x40` is a field of a
//!   record (see `docs/CAMPOS.en.md` for the known fields).
//! - `KEY_*` (`keys.rs`): the address of a global that holds the pointer to a fixed nation, club... (`read_usize(KEY_..)`).
//! - `FN_*` (`exe_fns.rs`): the address of an exe function the routine still calls (through `transmute`).
//! - `G_*` (`globals.rs`): the address of an exe global (`G_CLUB_TABLE`, `G_CURRENT_DATE`); `club::DIVISION`,
//!   `staff::...` (`fields.rs`): the offset of a named field in a record (`clube + club::DIVISION`).
//! - `show_error_at(file, line)`: the game's error box; `read_f64(addr)`: a float constant read from the exe; `ftol`: the
//!   compiler's float-to-int conversion, as the original does it.
//! - `[inferência]` / `[inference]`: something read from the code but not proven by a test.
//! - Comments like `// cqi4: ...` mark the points the mutation tests change (`engenharia-reversa/mutacoes/`).
//! - Names: hooked routines carry the original method name (docs/NOMES.csv), helpers an English name
//!   (docs/NOMES_AUXILIARES.csv); both files keep the old Portuguese name the batch documentation uses.
//!
//! # Safety
//! One contract covers every `unsafe fn` here, instead of a `# Safety` section repeated on each of them:
//! - it runs inside `cm0102.exe` 3.9.68 (or the 1080 copy), after the DLL checked the routine's bytes (`signatures.rs`),
//!   so every `G_*`, `KEY_*`, `FN_*` address and every record layout (`records.rs`, `fields.rs`) is the one it names;
//! - it runs on the game's own thread, as the original did, never concurrently: the game is single-threaded, which is
//!   also why the engine's flags are atomics with `Relaxed` (they only exist so the access needs no `unsafe`);
//! - a record argument (`Club`, `Staff`...) or a `usize` record address is a live record of the game, as the original
//!   routine would receive it; the routine may read and write it exactly as the original does.
//!
//! What is still `static mut` (23 in the engine) is borrowed (`addr_of!`: the comparers' tapes, the records a dry run
//! builds) or read by the original routine table (`ORIGINAIS`); the rest of the crate's `static mut` is the
//! infrastructure the asm reads (`detours.rs`, `stack_probe.rs`, `allocator.rs`).

use crate::comparison::{Scoreboard, TestRng};
use crate::detours::{self, Prologue};
use crate::registro::linha;
pub(crate) use core::sync::atomic::{AtomicBool, AtomicI32, AtomicU32, AtomicU8, AtomicUsize, Ordering::Relaxed};

/// Builds the tables of the hooked routines from one line per routine (the four used to be separate arrays aligned
/// only by position): `Prologue { .. } => rust_fn, comparer_fn;`. Generates PROLOGOS (the address and the
/// expected first bytes, checked before hooking), `rust_table()` and `compare_table()` (the Rust copy and the comparer
/// of each routine, as addresses) and NAMES (the Rust function's name, the label in the log). Read by the tools through
/// engenharia-reversa/tabela_motor.py.
macro_rules! routines {
    ($( $p:expr => $r:path, $c:path; )*) => {
        const PROLOGOS: [Prologue; N] = [$($p),*];
        fn rust_table() -> [usize; N] {
            [$($r as *const () as usize),*]
        }
        fn compare_table() -> [usize; N] {
            [$($c as *const () as usize),*]
        }
        const NAMES: [&str; N] = [$(stringify!($r)),*];
    };
}

routines! {
    // 0x6c38c0: mov edx, dword ptr [esp + 4]; mov al, byte ptr [edx + 0x19]
    Prologue { address: 0x6c38c0, bytes: &[0x8b, 0x54, 0x24, 0x04, 0x8a, 0x42, 0x19] } => table_a, compare_table_a;
    // 0x6c38f0: mov edx, dword ptr [esp + 4]; mov al, byte ptr [edx + 0x19]
    Prologue { address: 0x6c38f0, bytes: &[0x8b, 0x54, 0x24, 0x04, 0x8a, 0x42, 0x19] } => table_b, compare_table_b;
    // 0x6c4180: cmp byte ptr [esp + 4], 1
    Prologue { address: 0x6c4180, bytes: &[0x80, 0x7c, 0x24, 0x04, 0x01] } => side_difference, compare_diferenca;
    // 0x6cb960: mov al, byte ptr [esp + 4]; cmp al, 1
    Prologue { address: 0x6cb960, bytes: &[0x8a, 0x44, 0x24, 0x04, 0x3c, 0x01] } => band, compare_band;
    // 0x6cb6c0: mov al, byte ptr [esp + 0xc]; mov dl, byte ptr [esp + 4]
    Prologue { address: 0x6cb6c0, bytes: &[0x8a, 0x44, 0x24, 0x0c, 0x8a, 0x54, 0x24, 0x04] } => by_difference, compare_by_difference;
    // 0x6cb930: mov al, byte ptr [esp + 4]; cmp al, 1
    Prologue { address: 0x6cb930, bytes: &[0x8a, 0x44, 0x24, 0x04, 0x3c, 0x01] } => limit, compare_limit;
    // 0x70dfa0: mov al, byte ptr [ecx + 0x27]; mov dl, 6
    Prologue { address: 0x70dfa0, bytes: &[0x8a, 0x41, 0x27, 0xb2, 0x06] } => player_condition, compare_player_condition;
    // 0x717410: mov eax, ecx; mov ecx, dword ptr [eax + 0x19e]
    Prologue { address: 0x717410, bytes: &[0x8b, 0xc1, 0x8b, 0x88, 0x9e, 0x01, 0x00, 0x00] } => player_table, compare_player_table;
    // 0x6c4130: push ebx; push esi; mov esi, ecx; mov ecx, dword ptr [esp + 0xc]
    Prologue { address: 0x6c4130, bytes: &[0x53, 0x56, 0x8b, 0xf1, 0x8b, 0x4c, 0x24, 0x0c] } => player_difference, compare_player_difference;
    // 0x6cfda0: push esi; mov esi, dword ptr [esp + 8]
    Prologue { address: 0x6cfda0, bytes: &[0x56, 0x8b, 0x74, 0x24, 0x08] } => side_byte, compare_side_byte;
    // 0x70df40: mov al, byte ptr [ecx + 0x27]; cmp al, 1
    Prologue { address: 0x70df40, bytes: &[0x8a, 0x41, 0x27, 0x3c, 0x01] } => player_limit, compare_player_limit;
    // 0x6f5b20: mov al, byte ptr [esp + 4]; cmp al, 1
    Prologue { address: 0x6f5b20, bytes: &[0x8a, 0x44, 0x24, 0x04, 0x3c, 0x01] } => player_band, compare_player_band;
    // 0x5b57c0: movsx eax, byte ptr [esp + 4]
    Prologue { address: 0x5b57c0, bytes: &[0x0f, 0xbe, 0x44, 0x24, 0x04] } => position_byte, compare_position_byte;
    // 0x6f4600: push esi; mov esi, ecx; mov al, byte ptr [esi + 0x19]
    Prologue { address: 0x6f4600, bytes: &[0x56, 0x8b, 0xf1, 0x8a, 0x46, 0x19] } => field_by_position, compare_field_by_position;
    // 0x6bce20: push esi; mov esi, ecx; mov ecx, dword ptr [esp + 8]
    Prologue { address: 0x6bce20, bytes: &[0x56, 0x8b, 0xf1, 0x8b, 0x4c, 0x24, 0x08] } => can_choose, compare_can_choose;
    // 0x6f4920: mov dl, byte ptr [esp + 0x10]; push ebx
    Prologue { address: 0x6f4920, bytes: &[0x8a, 0x54, 0x24, 0x10, 0x53] } => aim_set_facing_direction, compare_aim_set_facing_direction;
    // 0x6f56c0: mov ax, word ptr [esp + 8]
    Prologue { address: 0x6f56c0, bytes: &[0x66, 0x8b, 0x44, 0x24, 0x08] } => tire_fatigue, compare_tire_fatigue;
    // 0x6fa140: mov al, byte ptr [esp + 4]; mov dl, byte ptr [ecx + 0x102]
    Prologue { address: 0x6fa140, bytes: &[0x8a, 0x44, 0x24, 0x04, 0x8a, 0x91, 0x02, 0x01, 0x00, 0x00] } => facing, compare_facing;
    // 0x6cd330: sub esp, 8; push ebx; mov ebx, dword ptr [esp + 0x14]
    Prologue { address: 0x6cd330, bytes: &[0x83, 0xec, 0x08, 0x53, 0x8b, 0x5c, 0x24, 0x14] } => search_on_side, compare_search_on_side_recording;
    // 0x6cb9b0: push ebx; push esi; mov esi, dword ptr [esp + 0xc]
    Prologue { address: 0x6cb9b0, bytes: &[0x53, 0x56, 0x8b, 0x74, 0x24, 0x0c] } => reaches, compare_reaches;
    // 0x713e90: push ebx; mov ebx, dword ptr [esp + 0x20]
    Prologue { address: 0x713e90, bytes: &[0x53, 0x8b, 0x5c, 0x24, 0x20] } => classify_move, compare_classificar;
    // 0x6f09b0: movsx eax, byte ptr [ecx + 0x103]
    Prologue { address: 0x6f09b0, bytes: &[0x0f, 0xbe, 0x81, 0x03, 0x01, 0x00, 0x00] } => distance_to_point, compare_distance_to_point;
    // 0x6fa1e0: mov al, byte ptr [ecx + 0x27]; push esi; test al, al
    Prologue { address: 0x6fa1e0, bytes: &[0x8a, 0x41, 0x27, 0x56, 0x84, 0xc0] } => distance_to_goal, compare_distance_to_goal;
    // 0x6cb780: push ebx; push ebp; push esi; push edi; mov edi, dword ptr [esp + 0x14]
    Prologue { address: 0x6cb780, bytes: &[0x53, 0x55, 0x56, 0x57, 0x8b, 0x7c, 0x24, 0x14] } => closest_to_goal, compare_mais_perto;
    // 0x6cbc00: sub esp, 0xc; movsx eax, byte ptr [esp + 0x18]
    Prologue { address: 0x6cbc00, bytes: &[0x83, 0xec, 0x0c, 0x0f, 0xbe, 0x44, 0x24, 0x18] } => players_in_line, compare_players_in_line;
    // 0x6f9f90: push ebx; push ebp; push esi; mov esi, ecx
    Prologue { address: 0x6f9f90, bytes: &[0x53, 0x55, 0x56, 0x8b, 0xf1] } => can_play, compare_consegue;
    // 0x6f4650: mov al, byte ptr [ecx + 0x27]; test al, al
    Prologue { address: 0x6f4650, bytes: &[0x8a, 0x41, 0x27, 0x84, 0xc0] } => in_the_box, compare_in_the_box;
    // 0x5b5270: movsx edx, byte ptr [esp + 4]
    Prologue { address: 0x5b5270, bytes: &[0x0f, 0xbe, 0x54, 0x24, 0x04] } => position_marks, compare_marcas;
    // 0x716270: push esi; mov esi, ecx; mov al, byte ptr [esi + 0x19]
    Prologue { address: 0x716270, bytes: &[0x56, 0x8b, 0xf1, 0x8a, 0x46, 0x19] } => attacking_role, compare_attacking_role;
    // 0x716310: push esi; mov esi, ecx; mov ecx, dword ptr [esi + 0x19e]
    Prologue { address: 0x716310, bytes: &[0x56, 0x8b, 0xf1, 0x8b, 0x8e, 0x9e, 0x01, 0x00, 0x00] } => defending_role, compare_defending_role;
    // 0x6c2be0: sub esp, 8; push ebx; push ebp
    Prologue { address: 0x6c2be0, bytes: &[0x83, 0xec, 0x08, 0x53, 0x55] } => position_place_players, compare_position_place_players;
    // 0x6c3aa0: push ebx; push esi; push edi; mov edi, ecx
    Prologue { address: 0x6c3aa0, bytes: &[0x53, 0x56, 0x57, 0x8b, 0xf9] } => defensive_lines, compare_linhas;
    // 0x6f4aa0: sub esp, 0x10; push ebx; push esi
    Prologue { address: 0x6f4aa0, bytes: &[0x83, 0xec, 0x10, 0x53, 0x56] } => update_player, compare_atualizar;
    // 0x6f44a0: push esi; mov esi, ecx; push edi; fld dword ptr [esi + 0x4d]
    Prologue { address: 0x6f44a0, bytes: &[0x56, 0x8b, 0xf1, 0x57, 0xd9, 0x46, 0x4d] } => draw_moment, compare_draw_moment;
    // 0x6ce110: mov eax, dword ptr [ecx + 0xf59e]
    Prologue { address: 0x6ce110, bytes: &[0x8b, 0x81, 0x9e, 0xf5, 0x00, 0x00] } => hold_position_one, compare_hold_position_one;
    // 0x6bd5b0: push ebx; push esi; mov esi, dword ptr [esp + 0xc]
    Prologue { address: 0x6bd5b0, bytes: &[0x53, 0x56, 0x8b, 0x74, 0x24, 0x0c] } => clear_path, compare_caminho;
    // 0x6cbad0: push ebx; push ebp; push esi; push edi; mov edi, dword ptr [esp + 0x14]
    Prologue { address: 0x6cbad0, bytes: &[0x53, 0x55, 0x56, 0x57, 0x8b, 0x7c, 0x24, 0x14] } => space_ahead, compare_espaco;
    // 0x6cb910: mov al, byte ptr [esp + 4]; cmp al, 2
    Prologue { address: 0x6cb910, bytes: &[0x8a, 0x44, 0x24, 0x04, 0x3c, 0x02] } => off_centre, compare_off_centre;
    // 0x6cb700: push ecx; push ebx; mov bl, byte ptr [esp + 0x10]
    Prologue { address: 0x6cb700, bytes: &[0x51, 0x53, 0x8a, 0x5c, 0x24, 0x10] } => beyond_the_line, compare_alem;
    // 0x6cc4e0: movsx eax, byte ptr [esp + 4]
    Prologue { address: 0x6cc4e0, bytes: &[0x0f, 0xbe, 0x44, 0x24, 0x04] } => distance_between, compare_distance_between;
    // batch 12: match state accessors (match_pl.cpp, 0x700320..0x700410)
    Prologue { address: 0x700320, bytes: &[0x8a, 0x81, 0xa7, 0x8e, 0x00, 0x00] } => ler_8ea7, compare_ler_8ea7;
    Prologue { address: 0x700330, bytes: &[0x8a, 0x81, 0xa8, 0x8e, 0x00, 0x00] } => ler_8ea8, compare_ler_8ea8;
    Prologue { address: 0x700340, bytes: &[0x8a, 0x81, 0xa9, 0x8e, 0x00, 0x00] } => ler_8ea9, compare_ler_8ea9;
    Prologue { address: 0x700380, bytes: &[0x8a, 0x81, 0xaa, 0x8e, 0x00, 0x00] } => ler_8eaa, compare_ler_8eaa;
    Prologue { address: 0x700400, bytes: &[0x8a, 0x81, 0xb2, 0x8e, 0x00, 0x00] } => ler_8eb2, compare_ler_8eb2;
    Prologue { address: 0x700350, bytes: &[0x8a, 0x44, 0x24, 0x04, 0x88, 0x81, 0xa9, 0x8e, 0x00, 0x00] } => gravar_8ea9, compare_gravar_8ea9;
    Prologue { address: 0x700360, bytes: &[0x8a, 0x44, 0x24, 0x04, 0x88, 0x81, 0xa7, 0x8e, 0x00, 0x00] } => gravar_8ea7, compare_gravar_8ea7;
    Prologue { address: 0x700370, bytes: &[0x8a, 0x44, 0x24, 0x04, 0x88, 0x81, 0xa8, 0x8e, 0x00, 0x00] } => gravar_8ea8, compare_gravar_8ea8;
    Prologue { address: 0x700390, bytes: &[0x8a, 0x44, 0x24, 0x04, 0x88, 0x81, 0xaa, 0x8e, 0x00, 0x00] } => gravar_8eaa, compare_gravar_8eaa;
    Prologue { address: 0x7003a0, bytes: &[0x66, 0x8b, 0x44, 0x24, 0x04] } => write_8ed8, compare_write_8ed8;
    Prologue { address: 0x700410, bytes: &[0x8b, 0x81, 0x7a, 0x8f, 0x00, 0x00] } => read_8f7a, compare_read_8f7a;
    Prologue { address: 0x7003b0, bytes: &[0x8a, 0x54, 0x24, 0x04, 0x80, 0xfa, 0xff] } => record_possession, compare_posse;
    Prologue { address: 0x714c60, bytes: &[0x8a, 0x91, 0x01, 0x01, 0x00, 0x00] } => in_list, compare_in_list;
    Prologue { address: 0x6cd660, bytes: &[0x53, 0x55, 0x56, 0x8b, 0xf1] } => store_extremes, compare_extremos;
    // batch 13
    Prologue { address: 0x6cbbb0, bytes: &[0x56, 0x0f, 0xbe, 0x74, 0x24, 0x08] } => count_active, compare_count_active;
    Prologue { address: 0x6cb650, bytes: &[0x0f, 0xbe, 0x44, 0x24, 0x04] } => special_code, compare_special_code;
    Prologue { address: 0x6cd5f0, bytes: &[0x56, 0x57, 0x8b, 0x7c, 0x24, 0x0c] } => behind_the_extreme, compare_atras;
    Prologue { address: 0x6d4280, bytes: &[0x8b, 0x81, 0x92, 0x47, 0x00, 0x00] } => no_match, compare_no_match;
    Prologue { address: 0x6bd6b0, bytes: &[0x8a, 0x44, 0x24, 0x04, 0x53] } => pressure_in_box, compare_pressao_area;
    Prologue { address: 0x6c3010, bytes: &[0x53, 0x55, 0x56, 0x57, 0x8b, 0x7c, 0x24, 0x14] } => nearest_to_point, compare_nearest_to_point;
    Prologue { address: 0x6c3a00, bytes: &[0x0f, 0xbe, 0x54, 0x24, 0x04] } => pressure_at_point, compare_pressao_ponto;
    Prologue { address: 0x6c3950, bytes: &[0x51, 0x0f, 0xbe, 0x54, 0x24, 0x08] } => smallest_distance, compare_smallest_distance;
    // batch 14
    Prologue { address: 0x6c06b0, bytes: &[0x0f, 0xbe, 0x54, 0x24, 0x04] } => split_a, compare_split_a;
    Prologue { address: 0x6c07a0, bytes: &[0x0f, 0xbe, 0x54, 0x24, 0x04] } => split_b, compare_split_b;
    Prologue { address: 0x6c0a70, bytes: &[0x0f, 0xbe, 0x54, 0x24, 0x04] } => split_c, compare_split_c;
    Prologue { address: 0x6c2750, bytes: &[0x83, 0xec, 0x0c, 0x0f, 0xbe, 0x54, 0x24, 0x10] } => choose_by_role, compare_escolher;
    // batch 15
    Prologue { address: 0x6f4a30, bytes: &[0x66, 0x8b, 0x81, 0x9a, 0x01, 0x00, 0x00] } => direction, compare_direction;
    Prologue { address: 0x6ce0f0, bytes: &[0x8a, 0x54, 0x24, 0x04, 0x33, 0xc0] } => clear_stored, compare_clear_stored;
    Prologue { address: 0x70df20, bytes: &[0x8a, 0x81, 0x02, 0x01, 0x00, 0x00] } => x_off_centre, compare_x_fora;
    Prologue { address: 0x7103c0, bytes: &[0x8b, 0x81, 0x9e, 0x01, 0x00, 0x00] } => is_the_stored, compare_is_the_stored;
    Prologue { address: 0x6bba50, bytes: &[0x8a, 0x81, 0x5a, 0x47, 0x00, 0x00] } => side_of_move, compare_side_of_move;
    Prologue { address: 0x6c2fe0, bytes: &[0x0f, 0xbe, 0x44, 0x24, 0x04] } => player_in_block, compare_player_in_block;
    Prologue { address: 0x6c3920, bytes: &[0x8b, 0x54, 0x24, 0x04, 0x8a, 0x42, 0x1a] } => table_a_1a, compare_table_a_1a;
    Prologue { address: 0x6d4250, bytes: &[0x66, 0x8b, 0x81, 0xd4, 0x8e, 0x00, 0x00] } => marked_moment, compare_marked_moment;
    Prologue { address: 0x6f4a40, bytes: &[0x8a, 0x41, 0x27, 0x84, 0xc0] } => side_state, compare_side_state;
    Prologue { address: 0x6f4a70, bytes: &[0x8a, 0x41, 0x27, 0x84, 0xc0] } => opponent_state, compare_opponent_state;
    Prologue { address: 0x70a890, bytes: &[0x66, 0x8b, 0x44, 0x24, 0x04] } => write_19c, compare_write_19c;
    Prologue { address: 0x70a8c0, bytes: &[0x66, 0x8b, 0x44, 0x24, 0x04] } => set_piece_code, compare_set_piece_code;
    Prologue { address: 0x70df70, bytes: &[0x8a, 0x41, 0x27, 0x3c, 0x01] } => near_goal, compare_near_goal;
    Prologue { address: 0x6bba10, bytes: &[0x83, 0xc8, 0xff, 0x33, 0xd2] } => clear_move, compare_clear_move;
    Prologue { address: 0x6cf710, bytes: &[0x0f, 0xbe, 0x54, 0x24, 0x04] } => add_disposition, compare_add_disposition;
    // batch 16
    Prologue { address: 0x6c0890, bytes: &[0x0f, 0xbe, 0x54, 0x24, 0x04] } => split_d, compare_split_d;
    Prologue { address: 0x6c0980, bytes: &[0x0f, 0xbe, 0x54, 0x24, 0x04] } => split_e, compare_split_e;
    Prologue { address: 0x6c0b70, bytes: &[0x0f, 0xbe, 0x54, 0x24, 0x04] } => split_f, compare_split_f;
    Prologue { address: 0x6bb9a0, bytes: &[0x8b, 0x81, 0x61, 0x47, 0x00, 0x00] } => record_move, compare_record_move;
    Prologue { address: 0x714ca0, bytes: &[0x8a, 0x91, 0x01, 0x01, 0x00, 0x00] } => remove_from_list, compare_remove_from_list;
    Prologue { address: 0x6cdd00, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x56, 0x57] } => take_the_field, compare_take_the_field;
    Prologue { address: 0x70d630, bytes: &[0x8a, 0x41, 0x27, 0x56, 0xf6, 0xd8] } => in_danger, compare_in_danger;
    // batch 17
    Prologue { address: 0x6bba80, bytes: &[0x53, 0x56, 0x8b, 0xf1, 0x33, 0xdb] } => check_move, compare_check_move;
    Prologue { address: 0x6c41b0, bytes: &[0x8a, 0x54, 0x24, 0x04, 0x53] } => clear_path_to, compare_clear_path;
    Prologue { address: 0x713d90, bytes: &[0x51, 0x53, 0x8b, 0x5c, 0x24, 0x0c] } => reaches_the_point, compare_reaches_the_point;
    // batch 18
    Prologue { address: 0x6d0f80, bytes: &[0x83, 0xec, 0x20, 0x8b, 0x81, 0x92, 0x47, 0x00, 0x00] } => choose_standout, compare_choose_standout;
    Prologue { address: 0x6c2880, bytes: &[0x83, 0xec, 0x0c, 0x0f, 0xbe, 0x54, 0x24, 0x10] } => choose_by_role_b, compare_choose_by_role_b;
    Prologue { address: 0x6c2a40, bytes: &[0x8a, 0x81, 0xae, 0x8e, 0x00, 0x00] } => formation_square, compare_formation_square;
    // batch 19
    Prologue { address: 0x6ce9c0, bytes: &[0x83, 0xec, 0x10, 0x8a, 0x44, 0x24, 0x14] } => compute_percentages, compare_compute_percentages;
    Prologue { address: 0x6c2050, bytes: &[0x83, 0xec, 0x10, 0x0f, 0xbe, 0x54, 0x24, 0x14] } => choose_by_role_c, compare_choose_by_role_c;
    // batch 20
    Prologue { address: 0x716450, bytes: &[0xd9, 0x44, 0x24, 0x04, 0xd8, 0x89, 0xa2, 0x01, 0x00, 0x00] } => apply_factor, compare_apply_factor;
    Prologue { address: 0x7165b0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x85, 0xc0] } => value_by_condition, compare_value_by_condition;
    Prologue { address: 0x6bb8d0, bytes: &[0x53, 0x8b, 0x5c, 0x24, 0x08] } => apply_to_team, compare_apply_to_team;
    Prologue { address: 0x6bb950, bytes: &[0x0f, 0xbe, 0x54, 0x24, 0x04] } => apply_to_side, compare_apply_to_side;
    Prologue { address: 0x6c2310, bytes: &[0x83, 0xec, 0x0c, 0x0f, 0xbe, 0x54, 0x24, 0x10] } => choose_by_condition, compare_choose_by_condition;
    // batch 21 (routines with no file in the map, called by the engine)
    Prologue { address: 0x717400, bytes: &[0x66, 0x8b, 0x81, 0xbc, 0x01, 0x00, 0x00] } => read_1bc, compare_read_1bc;
    Prologue { address: 0x7160d0, bytes: &[0x8a, 0x44, 0x24, 0x04, 0x8a, 0x51, 0x2b] } => add_2b, compare_add_2b;
    Prologue { address: 0x7160e0, bytes: &[0x8a, 0x44, 0x24, 0x04, 0x88, 0x41, 0x2b] } => write_2b, compare_write_2b;
    Prologue { address: 0x717430, bytes: &[0x8b, 0xc1, 0x8b, 0x88, 0x9e, 0x01, 0x00, 0x00] } => player_table_a_1a, compare_player_table_a_1a;
    Prologue { address: 0x717450, bytes: &[0x8b, 0xc1, 0x8b, 0x88, 0x9e, 0x01, 0x00, 0x00] } => player_table_b, compare_player_table_b;
    Prologue { address: 0x715f30, bytes: &[0x8b, 0xc1, 0x8b, 0x88, 0x9e, 0x01, 0x00, 0x00] } => difference_of_player, compare_difference_of_player;
    Prologue { address: 0x7163c0, bytes: &[0x8a, 0x41, 0x19, 0x50, 0x0f, 0xbe, 0x41, 0x27] } => player_marks_rot, compare_player_marks_rot;
    Prologue { address: 0x7163f0, bytes: &[0x0f, 0xbe, 0x41, 0x27, 0x56] } => mark_800000, compare_mark_800000;
    Prologue { address: 0x70def0, bytes: &[0x8a, 0x41, 0x27, 0x84, 0xc0] } => near_own_goal, compare_near_own_goal;
    Prologue { address: 0x717c70, bytes: &[0x56, 0x8b, 0xf1, 0x8b, 0x8e, 0x9e, 0x01, 0x00, 0x00] } => role_880, compare_role_880;
    Prologue { address: 0x6f0970, bytes: &[0x8b, 0x91, 0x9e, 0x01, 0x00, 0x00] } => release_ball, compare_release_ball;
    Prologue { address: 0x6e1cd0, bytes: &[0x8b, 0x41, 0x23, 0x8b, 0x54, 0x24, 0x04] } => player_by_record, compare_player_by_record;
    Prologue { address: 0x6d5cd0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x25, 0xff, 0x7f, 0x00, 0x00] } => code_byte, compare_code_byte;
    // batch 22
    Prologue { address: 0x715f50, bytes: &[0x56, 0x8b, 0xf1, 0x8b, 0x8e, 0x9e, 0x01, 0x00, 0x00] } => adjusted_rating, compare_adjusted_rating;
    Prologue { address: 0x7172d0, bytes: &[0x83, 0xec, 0x08, 0x53, 0x8a, 0x5c, 0x24, 0x10] } => adjust_17, compare_adjust_17;
    // batch 23
    Prologue { address: 0x6f0630, bytes: &[0x53, 0x56, 0x8b, 0xf1, 0x33, 0xdb] } => take_ball, compare_take_ball;
    // batch 24
    Prologue { address: 0x6e9470, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x56] } => init_strictness, compare_init_strictness;
    Prologue { address: 0x6bab90, bytes: &[0x8a, 0x81, 0x5a, 0x47, 0x00, 0x00] } => move_counts, compare_move_counts;
    Prologue { address: 0x6e9540, bytes: &[0x53, 0x56, 0x8b, 0xf1, 0x57] } => decide_foul, compare_decide_foul;
    // batch 25
    Prologue { address: 0x6e9630, bytes: &[0x53, 0x55, 0x56, 0x8b, 0x74, 0x24, 0x10] } => punish_foul, compare_punish_foul;
    Prologue { address: 0x6ea4e0, bytes: &[0x53, 0x55, 0x56, 0x8b, 0x74, 0x24, 0x10] } => punish_move, compare_punish_move;
    // batch 26
    Prologue { address: 0x6cfca0, bytes: &[0x8b, 0x44, 0x24, 0x10, 0x8b, 0x54, 0x24, 0x0c] } => engine_decide_foul, compare_engine_decide_foul;
    Prologue { address: 0x6cfcd0, bytes: &[0x8b, 0x44, 0x24, 0x20, 0x8b, 0x54, 0x24, 0x1c] } => engine_punish_move, compare_engine_punish_move;
    Prologue { address: 0x6cfd10, bytes: &[0x8b, 0x44, 0x24, 0x18, 0x8b, 0x54, 0x24, 0x14] } => engine_punish_foul, compare_engine_punish_foul;
    // batch 27
    Prologue { address: 0x6ea760, bytes: &[0x81, 0xec, 0x08, 0x02, 0x00, 0x00] } => look_for_offside, compare_look_for_offside;
    Prologue { address: 0x6cfd70, bytes: &[0x8b, 0x44, 0x24, 0x10, 0x8b, 0x54, 0x24, 0x0c] } => engine_look_for_offside, compare_engine_look_for_offside;
    // batch 28
    Prologue { address: 0x6eab60, bytes: &[0x53, 0x56, 0x8b, 0x74, 0x24, 0x0c] } => card_by_severity, compare_card_by_severity;
    Prologue { address: 0x6cfd40, bytes: &[0x8b, 0x44, 0x24, 0x10, 0x8b, 0x54, 0x24, 0x0c] } => engine_card_by_severity, compare_engine_card_by_severity;
    // batch 29
    Prologue { address: 0x6c23f0, bytes: &[0x83, 0xec, 0x0c, 0x0f, 0xbe, 0x54, 0x24, 0x10] } => choose_by_role_e, compare_choose_by_role_e;
    // batch 30
    Prologue { address: 0x6cf750, bytes: &[0x66, 0x8b, 0x81, 0xd4, 0x8e, 0x00, 0x00] } => possession_statistic, compare_possession_statistic;
    // batch 31
    Prologue { address: 0x6ce460, bytes: &[0x83, 0xec, 0x08, 0x53, 0x55, 0x56, 0x57] } => match_summary, compare_match_summary;
    // batch 32
    Prologue { address: 0x6c30e0, bytes: &[0x51, 0x0f, 0xbe, 0x54, 0x24, 0x08] } => closest_on_side, compare_closest_on_side;
    // batch 33
    Prologue { address: 0x700420, bytes: &[0x8b, 0x44, 0x24, 0x18, 0x83, 0xec, 0x08] } => goalkeeper_save, compare_goalkeeper_save;
    // batch 34
    Prologue { address: 0x6f46b0, bytes: &[0x8a, 0x81, 0x02, 0x01, 0x00, 0x00] } => move_to, compare_move_to;
    // batch 35
    Prologue { address: 0x6bce80, bytes: &[0x83, 0xec, 0x2c, 0x55, 0x8b, 0x6c, 0x24, 0x34] } => choose_square, compare_choose_square;
    // batch 36
    Prologue { address: 0x70d690, bytes: &[0x81, 0xec, 0x0c, 0x02, 0x00, 0x00] } => control_of_move, compare_control_of_move;
    // batch 37
    Prologue { address: 0x6ccb60, bytes: &[0x0f, 0xbe, 0x44, 0x24, 0x14] } => trace_move, compare_trace_move;
    // batch 38 (match_day.cpp)
    Prologue { address: 0x6b59a0, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => has_match, compare_has_match;
    Prologue { address: 0x6b7cf0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => won_the_match, compare_won_the_match;
    // batch 39 (commentary: wrappers of 0x6d6f70)
    Prologue { address: 0x7002f0, bytes: &[0x8b, 0x44, 0x24, 0x14, 0x8b, 0x54, 0x24, 0x10] } => commentary_a, compare_commentary_a;
    Prologue { address: 0x70a860, bytes: &[0x8b, 0x44, 0x24, 0x10, 0x8b, 0x54, 0x24, 0x0c] } => commentary_b, compare_commentary_b;
    // batch 40
    Prologue { address: 0x6cd490, bytes: &[0x56, 0x8b, 0xf1, 0x8a, 0x86, 0xb2, 0x8e, 0x00, 0x00] } => ball_out, compare_ball_out;
    // batch 41 (large routine trial)
    Prologue { address: 0x700bd0, bytes: &[0x81, 0xec, 0x14, 0x02, 0x00, 0x00] } => shot_value, compare_shot_value;
    // batch 42 (large integrating routine)
    Prologue { address: 0x6c0c60, bytes: &[0x81, 0xec, 0x20, 0x02, 0x00, 0x00] } => team_strength, compare_team_strength;
    // batch 43
    Prologue { address: 0x6c51e0, bytes: &[0x83, 0xec, 0x24, 0x53, 0x55, 0x8b, 0xe9] } => kick_off, compare_kick_off;
    // batch 44
    Prologue { address: 0x6bbe40, bytes: &[0x56, 0x8b, 0xf1, 0x8b, 0x86, 0x82, 0xf5, 0x00, 0x00] } => roll_ball, compare_roll_ball;
    // batch 45
    Prologue { address: 0x7164b0, bytes: &[0x8b, 0x44, 0x24, 0x08, 0x53] } => condition_test, compare_condition_test;
    // batch 46
    Prologue { address: 0x547810, bytes: &[0x8b, 0x4c, 0x24, 0x04, 0x85, 0xc9] } => likes, compare_likes;
    Prologue { address: 0x547840, bytes: &[0x8b, 0x4c, 0x24, 0x04, 0x85, 0xc9] } => dislikes, compare_dislikes;
    Prologue { address: 0x5b5910, bytes: &[0x8b, 0x44, 0x24, 0x08, 0x33, 0xd2] } => mark_position, compare_mark_position;
    // batch 47
    Prologue { address: 0x6d1770, bytes: &[0x81, 0xec, 0x34, 0x04, 0x00, 0x00] } => fine_positions, compare_fine_positions;
    // batch 48 (leaves of other modules called by the engine)
    Prologue { address: 0x4b0850, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x3d, 0xe8, 0x03, 0x00, 0x00] } => units_digit, compare_units_digit;
    Prologue { address: 0x5c6710, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x8b, 0x51, 0x04] } => is_friendly, compare_is_friendly;
    Prologue { address: 0x4cf5a0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x85, 0xc0] } => contract_of, compare_contract_of;
    Prologue { address: 0x53b900, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => is_national_team, compare_is_national_team;
    // batch 49 (human_manager.cpp and formation.cpp)
    Prologue { address: 0x5fd210, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => is_on_holiday, compare_is_on_holiday;
    Prologue { address: 0x5fd480, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => manager_9a05, compare_manager_9a05;
    Prologue { address: 0x5b5a60, bytes: &[0x0f, 0xbf, 0x44, 0x24, 0x04, 0x3d, 0x08, 0x02, 0x00, 0x00] } => square_bit, compare_square_bit;
    Prologue { address: 0x5b73a0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => split_mask, compare_split_mask;
    // batch 50
    Prologue { address: 0x5409b0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => is_second_national_team, compare_is_second_national_team;
    Prologue { address: 0x6ce310, bytes: &[0x51, 0x8b, 0x44, 0x24, 0x0c] } => aggregate_score, compare_aggregate_score;
    // batch 51
    Prologue { address: 0x6d4330, bytes: &[0x83, 0xec, 0x0c, 0x53, 0x8a, 0x5c, 0x24, 0x14] } => direct_marker, compare_direct_marker;
    // batch 52
    Prologue { address: 0x6cdd50, bytes: &[0x81, 0xec, 0x10, 0x02, 0x00, 0x00] } => take_off_pitch, compare_take_off_pitch;
    // batch 53
    Prologue { address: 0x6bbf60, bytes: &[0x83, 0xec, 0x10, 0x53, 0x55] } => individual_marking, compare_individual_marking;
    // batch 54
    Prologue { address: 0x6d3de0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x83, 0xec, 0x0c] } => new_reputation, compare_new_reputation;
    // batch 55
    Prologue { address: 0x6d3610, bytes: &[0x83, 0xec, 0x3c, 0xb8, 0x02, 0x00, 0x00, 0x00] } => player_reputation, compare_player_reputation;
    // batch 56
    Prologue { address: 0x6ceb90, bytes: &[0x81, 0xec, 0x24, 0x02, 0x00, 0x00] } => choose_taker, compare_choose_taker;
    // batch 57
    Prologue { address: 0x6c4290, bytes: &[0x81, 0xec, 0x0c, 0x02, 0x00, 0x00] } => throw_in, compare_throw_in;
    Prologue { address: 0x6c4470, bytes: &[0x81, 0xec, 0x18, 0x02, 0x00, 0x00] } => corner, compare_corner;
    Prologue { address: 0x6c50d0, bytes: &[0x51, 0x53, 0x55, 0x56, 0x57] } => penalty, compare_penalty;
    // batch 58
    Prologue { address: 0x5b5290, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => set_square, compare_set_square;
    Prologue { address: 0x6d10a0, bytes: &[0x51, 0x53, 0x55, 0x56, 0x57] } => mark_stars, compare_mark_stars;
    // batch 59
    Prologue { address: 0x5b58e0, bytes: &[0x0f, 0xbe, 0x44, 0x24, 0x04] } => designated_for_position, compare_designated_for_position;
    Prologue { address: 0x7160f0, bytes: &[0x53, 0x55, 0x56, 0x8b, 0xf1] } => designated_marked, compare_designated_marked;
    Prologue { address: 0x53b3d0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => main_national_team, compare_main_national_team;
    Prologue { address: 0x53b4f0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => b_national_team, compare_b_national_team;
    Prologue { address: 0x6bc030, bytes: &[0x83, 0xec, 0x54, 0x53, 0x55] } => who_to_mark, compare_who_to_mark;
    Prologue { address: 0x5fe040, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => manager_9ace, compare_manager_9ace;
    Prologue { address: 0x5fd6b0, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => manager_9a06, compare_manager_9a06;
    Prologue { address: 0x600a50, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => human_club, compare_human_club;
    Prologue { address: 0x6d42a0, bytes: &[0x53, 0x56, 0x8b, 0xf1, 0x8b, 0x86, 0x92, 0x47, 0x00, 0x00] } => side_under_control, compare_side_under_control;
    Prologue { address: 0x8982b0, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => tactics_row, compare_tactics_row;
    Prologue { address: 0x898040, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => tactics_byte, compare_tactics_byte;
    Prologue { address: 0x6be520, bytes: &[0x83, 0xec, 0x0c, 0x53, 0x55] } => penalty_order, compare_penalty_order;
    Prologue { address: 0x6bd760, bytes: &[0x53, 0x56, 0x8b, 0xf1, 0x57] } => player_order, compare_player_order;
    Prologue { address: 0x897590, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => marked_slot, compare_marked_slot;
    Prologue { address: 0x897760, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => mark_slot, compare_mark_slot;
    Prologue { address: 0x897dc0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => row_person, compare_row_person;
    Prologue { address: 0x48db80, bytes: &[0x83, 0xec, 0x0c, 0xa1, 0x5c, 0x23, 0xae, 0x00] } => club_in_competition, compare_club_in_competition;
    Prologue { address: 0x76d940, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x85, 0xc0] } => special_club, compare_special_club;
    Prologue { address: 0x6e9820, bytes: &[0x83, 0xec, 0x14, 0x53, 0x55] } => foul_decision, compare_foul_decision;
    Prologue { address: 0x6c6a30, bytes: &[0x83, 0xec, 0x7c, 0x56, 0x8b, 0xf1] } => choose_teammate, compare_choose_teammate;
    Prologue { address: 0x6cfc60, bytes: &[0x8b, 0x44, 0x24, 0x1c, 0x8b, 0x54, 0x24, 0x18] } => foul_in_engine, compare_foul_in_engine;
    Prologue { address: 0x5fea50, bytes: &[0x8b, 0x44, 0x24, 0x08, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => manager_9b23, compare_manager_9b23;
    Prologue { address: 0x547d30, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x85, 0xc0] } => dislikes_club, compare_dislikes_club;
    Prologue { address: 0x547d60, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x85, 0xc0] } => likes_club, compare_likes_club;
    Prologue { address: 0x540a50, bytes: &[0x8b, 0x4c, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => partner_club, compare_partner_club;
    Prologue { address: 0x4cf5f0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x53, 0x55, 0x56] } => contract_with_club, compare_contract_with_club;
    Prologue { address: 0x53ce90, bytes: &[0x8b, 0x44, 0x24, 0x08, 0x85, 0xc0] } => contract_level, compare_contract_level;
    Prologue { address: 0x544410, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x0f, 0xbe, 0x54, 0x24, 0x08] } => chance_to_know, compare_chance_to_know;
    Prologue { address: 0x5b09d0, bytes: &[0x83, 0xec, 0x08, 0x53, 0x56] } => know_person, compare_know_person;
    Prologue { address: 0x5afe00, bytes: &[0x51, 0x53, 0x55, 0x8b, 0x6c, 0x24, 0x10] } => fog_after_match, compare_fog_after_match;
    Prologue { address: 0x849ce0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x8b, 0x91, 0xdc, 0x00, 0x00, 0x00] } => shortlist_entry, compare_shortlist_entry;
    // 4 bytes of routine (mov + ret) and the first alignment nop: the ret runs before the jump back
    Prologue { address: 0x8676f0, bytes: &[0x8b, 0x41, 0x40, 0xc3, 0x90] } => contract_flags, compare_contract_flags;
    Prologue { address: 0x68d1b0, bytes: &[0x8b, 0x51, 0x0a, 0x85, 0xd2] } => next_list_item, compare_next_list_item;
    Prologue { address: 0x62c3d0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x8b, 0x15, 0x64, 0x23, 0xae, 0x00] } => word_0c, compare_word_0c;
    Prologue { address: 0x4b0870, bytes: &[0x8b, 0x4c, 0x24, 0x04, 0x33, 0xd2] } => round_below_1000, compare_round_below_1000;
    Prologue { address: 0x7e6f20, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x85, 0xc0] } => group_nation, compare_group_nation;
    Prologue { address: 0x6d59b0, bytes: &[0x53, 0x55, 0x8b, 0x6c, 0x24, 0x0c] } => how_many_mark, compare_how_many_mark;
    Prologue { address: 0x6d5a10, bytes: &[0x53, 0x56, 0x57, 0x8b, 0x7c, 0x24, 0x10] } => who_marks, compare_who_marks;
    Prologue { address: 0x4b0700, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x8b, 0x0d, 0xb0, 0xf6, 0x9c, 0x00] } => competition_counts, compare_competition_counts;
    Prologue { address: 0x4b0780, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x8b, 0x4c, 0x24, 0x08] } => order_by_5f, compare_order_by_5f;
    Prologue { address: 0x4b07b0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x8b, 0x08] } => order_by_80, compare_order_by_80;
    Prologue { address: 0x4b07d0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x8b, 0x54, 0x24, 0x08] } => order_by_80_desc, compare_order_by_80_desc;
    Prologue { address: 0x4b0810, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x8b, 0x54, 0x24, 0x08] } => order_by_80_asc, compare_order_by_80_asc;
    Prologue { address: 0x6ebd70, bytes: &[0x55, 0x8b, 0xec, 0x83, 0xe4, 0xf8] } => player_rating, compare_player_rating;
    Prologue { address: 0x5afd00, bytes: &[0x55, 0x8b, 0x6c, 0x24, 0x08] } => forget_person, compare_forget_person;
    Prologue { address: 0x5afde0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x85, 0xc0] } => manager_table, compare_manager_table;
    Prologue { address: 0x5b2e80, bytes: &[0x51, 0x8b, 0x54, 0x24, 0x08] } => sector_slot_taken, compare_sector_slot_taken;
    Prologue { address: 0x5b2f10, bytes: &[0x66, 0x8b, 0x54, 0x24, 0x04] } => slot_of_code, compare_slot_of_code;
    Prologue { address: 0x5b5960, bytes: &[0x56, 0x8d, 0x81, 0xb4, 0x05, 0x00, 0x00] } => clear_assigned, compare_clear_assigned;
    Prologue { address: 0x5b59a0, bytes: &[0x53, 0x56, 0x8b, 0xf1, 0x57] } => formation_mask, compare_formation_mask;
    Prologue { address: 0x5b2ae0, bytes: &[0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => slot_code, compare_slot_code;
    Prologue { address: 0x4bff70, bytes: &[0x8b, 0x44, 0x24, 0x0c, 0x66, 0x85, 0xc0] } => competition_criterion, compare_competition_criterion;
    Prologue { address: 0x53b610, bytes: &[0x8a, 0x4c, 0x24, 0x04, 0x33, 0xc0] } => type_not_11, compare_type_not_11;
    Prologue { address: 0x53b620, bytes: &[0x8a, 0x44, 0x24, 0x04, 0x3c, 0x0b] } => type_11_to_15, compare_type_11_to_15;
    Prologue { address: 0x76d780, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x50] } => national_side_class, compare_national_side_class;
    Prologue { address: 0x76d7b0, bytes: &[0x0f, 0xbe, 0x44, 0x24, 0x08] } => read_person_mark, compare_read_person_mark;
    Prologue { address: 0x76d7e0, bytes: &[0x0f, 0xbe, 0x44, 0x24, 0x08] } => clear_person_mark, compare_clear_person_mark;
    Prologue { address: 0x53a5b0, bytes: &[0xa1, 0xb4, 0x59, 0x9b, 0x00] } => free_record, compare_free_record;
    Prologue { address: 0x76c0f0, bytes: &[0x0f, 0xbe, 0x44, 0x24, 0x08] } => set_person_44, compare_set_person_44;
    Prologue { address: 0x76e010, bytes: &[0x0f, 0xbe, 0x44, 0x24, 0x08] } => person_byte_44, compare_person_byte_44;
    Prologue { address: 0x76e040, bytes: &[0x0f, 0xbe, 0x44, 0x24, 0x08] } => person_field_3a, compare_person_field_3a;
    Prologue { address: 0x76e070, bytes: &[0x56, 0x8b, 0x74, 0x24, 0x08] } => class_by_national_side, compare_class_by_national_side;
    Prologue { address: 0x535920, bytes: &[0xa1, 0x84, 0x59, 0x9b, 0x00] } => database_global_9b5984, compare_database_global_9b5984;
    Prologue { address: 0x7751e0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x85, 0xc0] } => person_competition, compare_person_competition;
    Prologue { address: 0x7754c0, bytes: &[0x53, 0x56, 0x8b, 0x74, 0x24, 0x0c] } => national_side_byte_42, compare_national_side_byte_42;
    Prologue { address: 0x775520, bytes: &[0x53, 0x56, 0x8b, 0x74, 0x24, 0x0c] } => national_side_byte_43, compare_national_side_byte_43;
    Prologue { address: 0x775790, bytes: &[0x56, 0x8b, 0x74, 0x24, 0x08] } => national_or_special, compare_national_or_special;
    Prologue { address: 0x776380, bytes: &[0x56, 0x8b, 0x74, 0x24, 0x08] } => dislikes_national_side, compare_dislikes_national_side;
    Prologue { address: 0x777590, bytes: &[0x83, 0xec, 0x08, 0x8d, 0x44, 0x24, 0x00] } => copy_date_and_mark, compare_copy_date_and_mark;
    Prologue { address: 0x6ec010, bytes: &[0x83, 0xec, 0x54, 0x53, 0x55] } => update_attributes, compare_update_attributes;
    Prologue { address: 0x6eecb0, bytes: &[0x83, 0xec, 0x08, 0x53, 0x55] } => match_performance, compare_match_performance;
    Prologue { address: 0x6bbb60, bytes: &[0x83, 0xec, 0x14, 0x53, 0x55] } => side_strength, compare_side_strength;
    // these two end in a relative instruction, rebuilt in the trampoline (RELATIVAS in detours.rs)
    Prologue { address: 0x714d00, bytes: &[0x80, 0x79, 0x27, 0x01, 0x75, 0x15] } => ball_in_attack, compare_ball_in_attack;
    Prologue { address: 0x6f55c0, bytes: &[0x51, 0x56, 0x8b, 0xf1, 0xe8, 0xd7, 0xee, 0xff, 0xff] } => evolve_moment, compare_evolve_moment;
    Prologue { address: 0x6b61e0, bytes: &[0x53, 0x8b, 0x15, 0x68, 0xf6, 0x9c, 0x00] } => match_of_interest, compare_match_of_interest;
    Prologue { address: 0x6854a0, bytes: &[0x8b, 0x44, 0x24, 0x08, 0x53, 0x55] } => group_decided, compare_group_decided;
    Prologue { address: 0x6be120, bytes: &[0x83, 0xec, 0x10, 0x53, 0x56] } => narrate_qualification, compare_narrate_qualification;
    Prologue { address: 0x6cd3d0, bytes: &[0x83, 0xec, 0x08, 0x8b, 0x44, 0x24, 0x10] } => look_for_play, compare_look_for_play;
    Prologue { address: 0x6cc070, bytes: &[0x83, 0xec, 0x0c, 0x0f, 0xbe, 0x44, 0x24, 0x18] } => players_on_path, compare_players_on_path;
    Prologue { address: 0x6cc540, bytes: &[0x83, 0xec, 0x14, 0x8a, 0x44, 0x24, 0x24] } => intercept, compare_intercept;
    Prologue { address: 0x6f5b80, bytes: &[0x51, 0x8a, 0x51, 0x27, 0x8a, 0x81, 0x03, 0x01, 0x00, 0x00] } => advance_row, compare_advance_row;
    Prologue { address: 0x6c4ec0, bytes: &[0x81, 0xec, 0x14, 0x02, 0x00, 0x00] } => line_up_kickoff, compare_line_up_kickoff;
    Prologue { address: 0x6c4820, bytes: &[0x81, 0xec, 0x1c, 0x02, 0x00, 0x00] } => line_up_set_piece, compare_line_up_set_piece;
    Prologue { address: 0x713fc0, bytes: &[0x81, 0xec, 0x24, 0x02, 0x00, 0x00] } => decide_off_ball, compare_decide_off_ball;
    Prologue { address: 0x714d40, bytes: &[0x55, 0x8b, 0xec, 0x83, 0xe4, 0xf8] } => move_off_ball, compare_move_off_ball;
    Prologue { address: 0x7103e0, bytes: &[0x83, 0xec, 0x14, 0x53, 0x56] } => decide_player, compare_decide_player;
    Prologue { address: 0x6fa250, bytes: &[0x53, 0x55, 0x8b, 0x44, 0x24, 0x18] } => pass_ball, compare_pass_ball;
    Prologue { address: 0x6fa490, bytes: &[0x83, 0xec, 0x14, 0x53, 0x55] } => long_ball, compare_long_ball;
    Prologue { address: 0x6cd770, bytes: &[0x56, 0x8b, 0xf1, 0x57, 0x8b, 0x8e, 0x82, 0xf5, 0x00, 0x00] } => deflect_ball, compare_deflect_ball;
    Prologue { address: 0x70d430, bytes: &[0x66, 0x81, 0x7c, 0x24, 0x04, 0xe8, 0x03] } => save_shot, compare_save_shot;
    Prologue { address: 0x6f5810, bytes: &[0x83, 0xec, 0x08, 0x56, 0x8b, 0xf1] } => recover_stamina, compare_recover_stamina;
    Prologue { address: 0x6bd930, bytes: &[0x51, 0x53, 0x55, 0x56, 0x8b, 0xf1] } => period_completed, compare_period_completed;
    Prologue { address: 0x6bcbd0, bytes: &[0x83, 0xec, 0x28, 0x53, 0x8b, 0x5c, 0x24, 0x34] } => pressure, compare_pressure;
    Prologue { address: 0x6c87e0, bytes: &[0x81, 0xec, 0x3c, 0x02, 0x00, 0x00] } => opposition_challenge, compare_opposition_challenge;
    Prologue { address: 0x6d26b0, bytes: &[0x83, 0xec, 0x34, 0xb8, 0x01, 0x00, 0x00, 0x00] } => develop_player, compare_develop_player;
    Prologue { address: 0x6d47e0, bytes: &[0x81, 0xec, 0x0c, 0x02, 0x00, 0x00] } => kit, compare_kit;
    Prologue { address: 0x62ec00, bytes: &[0x0f, 0xbe, 0x44, 0x24, 0x04] } => injury_severity, compare_injury_severity;
    Prologue { address: 0x62ece0, bytes: &[0x8b, 0x54, 0x24, 0x04, 0x56] } => injury_grade, compare_injury_grade;
    Prologue { address: 0x6cfe10, bytes: &[0x83, 0xec, 0x18, 0x53, 0x55] } => tackle_injury, compare_tackle_injury;
    Prologue { address: 0x6e1cf0, bytes: &[0x55, 0x8b, 0xec, 0x83, 0xe4, 0xf8] } => rate_player, compare_rate_player;
    Prologue { address: 0x6e1a40, bytes: &[0x51, 0x53, 0x55, 0x56, 0x57] } => best_rated, compare_best_rated;
    Prologue { address: 0x6e1940, bytes: &[0x83, 0xec, 0x0c, 0x8a, 0x44, 0x24, 0x14] } => best_in_list, compare_best_in_list;
    Prologue { address: 0x6cda20, bytes: &[0x51, 0x8a, 0x44, 0x24, 0x0c] } => player_left, compare_player_left;
    Prologue { address: 0x6e1aa0, bytes: &[0x81, 0xec, 0x04, 0x01, 0x00, 0x00] } => redo_team, compare_redo_team;
    Prologue { address: 0x6e1090, bytes: &[0x81, 0xec, 0xc4, 0x04, 0x00, 0x00] } => substitute, compare_substitute;
    Prologue { address: 0x6e2000, bytes: &[0x81, 0xec, 0x24, 0x01, 0x00, 0x00] } => pick, compare_pick;
    Prologue { address: 0x6df460, bytes: &[0x55, 0x8b, 0xec, 0x83, 0xe4, 0xf8] } => stance, compare_stance;
    Prologue { address: 0x6e0230, bytes: &[0x55, 0x8b, 0xec, 0x83, 0xe4, 0xf8] } => choose_formation, compare_choose_formation;
    Prologue { address: 0x6ddab0, bytes: &[0x55, 0x8b, 0xec, 0x83, 0xe4, 0xf8] } => instructions, compare_instructions;
    Prologue { address: 0x6e0d00, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => put_in_place, compare_put_in_place;
    Prologue { address: 0x6e0df0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x8b, 0x4c, 0x24, 0x08] } => order_by_table, compare_order_by_table;
    Prologue { address: 0x6e0fe0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x8b, 0x4c, 0x24, 0x08] } => order_by_condition, compare_order_by_condition;
    Prologue { address: 0x6dbb80, bytes: &[0x83, 0xec, 0x08, 0x53, 0x56] } => starters, compare_starters;
    Prologue { address: 0x6dbb10, bytes: &[0x56, 0x8b, 0xf1, 0x8b, 0x46, 0x2f] } => store_tactic, compare_store_tactic;
    Prologue { address: 0x6d0b60, bytes: &[0x83, 0xec, 0x28, 0x53, 0x55] } => apply_substitutions, compare_apply_substitutions;
    Prologue { address: 0x6d1210, bytes: &[0x83, 0xec, 0x10, 0x56, 0x8b, 0xf1] } => punishments, compare_punishments;
    Prologue { address: 0x6be710, bytes: &[0x83, 0xec, 0x18, 0x53, 0x56] } => penalties, compare_penalties;
    Prologue { address: 0x6bac40, bytes: &[0x83, 0xec, 0x08, 0x55, 0x56] } => apply_foul, compare_apply_foul;
    Prologue { address: 0x70ec80, bytes: &[0x83, 0xec, 0x28, 0x53, 0x55] } => tackle, compare_tackle;
    Prologue { address: 0x6c9cd0, bytes: &[0x83, 0xec, 0x4c, 0x8a, 0x44, 0x24, 0x50] } => contest, compare_contest;
    Prologue { address: 0x6f5c30, bytes: &[0x81, 0xec, 0x18, 0x02, 0x00, 0x00] } => respond, respond_hook;
    Prologue { address: 0x70dfd0, bytes: &[0x81, 0xec, 0x20, 0x02, 0x00, 0x00] } => play, gate_try_block_shot;
    Prologue { address: 0x6b74a0, bytes: &[0xa1, 0xc8, 0x7d, 0xb6, 0x00] } => post_match_events, compare_post_match_events;
    Prologue { address: 0x6b6cc0, bytes: &[0x55, 0x8b, 0xec, 0x83, 0xe4, 0xf8] } => post_match_players, compare_post_match_players;
    Prologue { address: 0x6b79a0, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => post_match_club, compare_post_match_club;
    Prologue { address: 0x6b66a0, bytes: &[0xa1, 0x04, 0x57, 0xa1, 0x00] } => match_without_human, compare_match_without_human;
    Prologue { address: 0x6b5040, bytes: &[0x83, 0xec, 0x1c, 0x53, 0x8b, 0xd9] } => match_day_interest, compare_match_day_interest;
    Prologue { address: 0x6c54a0, bytes: &[0x81, 0xec, 0x18, 0x02, 0x00, 0x00] } => restart_play, compare_restart_play;
    Prologue { address: 0x6f6c00, bytes: &[0x83, 0xec, 0x3c, 0x53, 0x55] } => shot, compare_shot;
    Prologue { address: 0x6d4a20, bytes: &[0x83, 0xec, 0x08, 0x56, 0x8b, 0xf1] } => referee_grade, compare_referee_grade;
    Prologue { address: 0x79ce10, bytes: &[0x8b, 0x11, 0x56, 0x85, 0xd2] } => record_referee_grade, compare_record_referee_grade;
    Prologue { address: 0x897990, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => stored_taker, compare_stored_taker;
    Prologue { address: 0x896fe0, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => store_instructions, compare_store_instructions;
    Prologue { address: 0x6b6570, bytes: &[0x0f, 0xbf, 0x44, 0x24, 0x04] } => competition_number, compare_competition_number;
    Prologue { address: 0x547790, bytes: &[0x8b, 0x4c, 0x24, 0x04, 0x85, 0xc9] } => favourite_of, compare_favourite_of;
    Prologue { address: 0x5477d0, bytes: &[0x8b, 0x4c, 0x24, 0x04, 0x85, 0xc9] } => rejected_by, compare_rejected_by;
    Prologue { address: 0x6d1cc0, bytes: &[0x83, 0xec, 0x44, 0x53, 0x55] } => match_wrap_up, compare_match_wrap_up;
    Prologue { address: 0x8983f0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => read_tactics_row, compare_read_tactics_row;
    Prologue { address: 0x8984d0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => write_tactics_row, compare_write_tactics_row;
    Prologue { address: 0x6beeb0, bytes: &[0x83, 0xec, 0x38, 0x53, 0x55] } => quick_simulation, compare_quick_simulation;
    Prologue { address: 0x6d0980, bytes: &[0x51, 0x57, 0x8b, 0xf9, 0xc7, 0x44, 0x24, 0x04, 0x00, 0x00, 0x00, 0x00] } => injure, compare_injure;
    Prologue { address: 0x62be00, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] } => register_injury, compare_register_injury;
    Prologue { address: 0x717470, bytes: &[0x6a, 0xff, 0x68, 0x90, 0xd4, 0x95, 0x00] } => player_highlight, compare_player_highlight;
    Prologue { address: 0x70c040, bytes: &[0x81, 0xec, 0x18, 0x02, 0x00, 0x00] } => shot_outcome, compare_shot_outcome;
    Prologue { address: 0x70a910, bytes: &[0x81, 0xec, 0x14, 0x02, 0x00, 0x00] } => shot_on_goal, compare_shot_on_goal;
    Prologue { address: 0x6f09f0, bytes: &[0x81, 0xec, 0x5c, 0x02, 0x00, 0x00] } => player_move, compare_mover;
    Prologue { address: 0x7109f0, bytes: &[0x6a, 0xff, 0x68, 0x6b, 0xd4, 0x95, 0x00] } => dribble, compare_dribble;
    Prologue { address: 0x6b98c0, bytes: &[0x81, 0xec, 0x10, 0x02, 0x00, 0x00] } => engine_loop, compare_engine_loop;
    Prologue { address: 0x6fad40, bytes: &[0x81, 0xec, 0x6c, 0x02, 0x00, 0x00] } => execute_pass, compare_execute_pass;
    Prologue { address: 0x6d5950, bytes: &[0x51, 0x8d, 0x44, 0x24, 0x02] } => behind_on_aggregate, compare_behind_on_aggregate;
    Prologue { address: 0x6d5a70, bytes: &[0x53, 0x56, 0x57, 0x8b, 0xf9] } => free_teammate, compare_free_teammate;
    Prologue { address: 0x702050, bytes: &[0x81, 0xec, 0x84, 0x06, 0x00, 0x00] } => shoot, compare_shoot;
    Prologue { address: 0x6b7e50, bytes: &[0x6a, 0xff, 0x68, 0x0c, 0xd3, 0x95, 0x00] } => build_engine, compare_build_engine;
    Prologue { address: 0x6b7f20, bytes: &[0x55, 0x8b, 0xec, 0x83, 0xe4, 0xf8] } => prepare_match, compare_prepare_match;
    Prologue { address: 0x6b97a0, bytes: &[0x6a, 0xff, 0x68, 0xda, 0xd3, 0x95, 0x00] } => destroy_engine, compare_destroy_engine;
    Prologue { address: 0x6db500, bytes: &[0x8b, 0x54, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => select_side, compare_select_side;
    Prologue { address: 0x6b4190, bytes: &[0x83, 0xec, 0x08, 0x55, 0x56] } => end_day, compare_end_day;
    Prologue { address: 0x6b42a0, bytes: &[0x83, 0xec, 0x08, 0x53, 0x8b, 0xd9] } => pick_day_match, compare_pick_day_match;
    Prologue { address: 0x6dbc50, bytes: &[0x6a, 0xff, 0x68, 0x52, 0xd4, 0x95, 0x00] } => select_automatically, compare_select_automatically;
    Prologue { address: 0x6b5a70, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] } => start_next_match, compare_start_next_match;
    Prologue { address: 0x6b5ce0, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] } => step_day_match, compare_step_day_match;
    Prologue { address: 0x6b5f00, bytes: &[0x81, 0xec, 0x58, 0x02, 0x00, 0x00] } => close_day_match, compare_close_day_match;
    Prologue { address: 0x6b4360, bytes: &[0x6a, 0xff, 0x68, 0xdc, 0xd1, 0x95, 0x00] } => play_the_day, compare_play_the_day;
    Prologue { address: 0x6b3c10, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] } => set_up_the_day, compare_set_up_the_day;
    Prologue { address: 0x714d30, bytes: &[0x8a, 0x41, 0x27, 0xc3, 0x90] } => player_side, compare_player_side;
    Prologue { address: 0x6d8c70, bytes: &[0x51, 0x53, 0x55, 0x56, 0x57] } => sounds_finished, compare_sounds_finished;
    Prologue { address: 0x6d98b0, bytes: &[0x81, 0xec, 0x18, 0x02, 0x00, 0x00] } => record_events, compare_record_events;
    Prologue { address: 0x6d9c40, bytes: &[0x81, 0xec, 0x08, 0x02, 0x00, 0x00] } => read_events, compare_read_events;
    Prologue { address: 0x6b55c0, bytes: &[0x51, 0x53, 0x8b, 0x5c, 0x24, 0x0c] } => network_step, compare_network_step;
    Prologue { address: 0x6d9e50, bytes: &[0x81, 0xec, 0x0c, 0x06, 0x00, 0x00] } => read_events_cfg, compare_read_events_cfg;
    Prologue { address: 0x6d8d10, bytes: &[0xb8, 0xd8, 0x1b, 0x00, 0x00] } => event_text, compare_event_text;
    Prologue { address: 0x6b5260, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] } => confirm_network_match, compare_confirm_network_match;
    Prologue { address: 0x6d7b00, bytes: &[0xb8, 0x18, 0x26, 0x00, 0x00] } => commentate_on_panel, compare_commentate_on_panel;
    Prologue { address: 0x9326e0, bytes: &[0x81, 0xec, 0x08, 0x02, 0x00, 0x00] } => abrir_no_conteiner, compare_abrir_no_conteiner;
    Prologue { address: 0x933080, bytes: &[0x53, 0x56, 0x8b, 0x74, 0x24, 0x0c] } => buscar_no_conteiner, compare_buscar_no_conteiner;
    Prologue { address: 0x933170, bytes: &[0x56, 0x8b, 0x74, 0x24, 0x08] } => position_in_container, compare_position_in_container;
    Prologue { address: 0x5b7d20, bytes: &[0x8b, 0x4c, 0x24, 0x04, 0x81, 0xec, 0x04, 0x02, 0x00, 0x00] } => position_mask, compare_position_mask;
    Prologue { address: 0x5b7e60, bytes: &[0x8a, 0x44, 0x24, 0x08, 0x81, 0xec, 0x10, 0x02, 0x00, 0x00] } => pitch_cell, compare_pitch_cell;
    Prologue { address: 0x5b8960, bytes: &[0x81, 0xec, 0x04, 0x0e, 0x00, 0x00] } => slots_without_conflict, compare_slots_without_conflict;
    Prologue { address: 0x7c2150, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x08, 0x02, 0x00, 0x00] } => player_statistic, compare_player_statistic;
    Prologue { address: 0x7c1950, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => ratings_average, compare_ratings_average;
    Prologue { address: 0x5b8dc0, bytes: &[0x81, 0xec, 0x04, 0x0e, 0x00, 0x00] } => centre_backs_without_conflict, compare_centre_backs_without_conflict;
    Prologue { address: 0x844300, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => shortlist_slot_fits, compare_shortlist_slot_fits;
    Prologue { address: 0x83bd00, bytes: &[0x0f, 0xbf, 0x44, 0x24, 0x04] } => position_code, compare_position_code;
    Prologue { address: 0x866f60, bytes: &[0x83, 0xec, 0x0c, 0x53, 0x55] } => staff_criterion, compare_staff_criterion;
    Prologue { address: 0x896a90, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => set_tactic_slot, compare_set_tactic_slot;
    Prologue { address: 0x8968f0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => tactic_slot_person, compare_tactic_slot_person;
    Prologue { address: 0x8967d0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => set_tactic_formation, compare_set_tactic_formation;
    Prologue { address: 0x5b3c70, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => stored_cell, compare_stored_cell;
    Prologue { address: 0x5b50d0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => set_formation_flag, compare_set_formation_flag;
    Prologue { address: 0x452fe0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => reset_records, compare_reset_records;
    Prologue { address: 0x68a030, bytes: &[0x8b, 0x44, 0x24, 0x0c, 0x8b, 0x0d, 0xe8, 0xf6, 0x9c, 0x00] } => rotate_round, compare_rotate_round;
    Prologue { address: 0x68a1c0, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => store_round_match, compare_store_round_match;
    Prologue { address: 0x5af180, bytes: &[0xa1, 0x5c, 0x2a, 0xae, 0x00] } => mark_calendar_year, compare_mark_calendar_year;
    Prologue { address: 0x897ee0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => extra_tactic_slot, compare_extra_tactic_slot;
    Prologue { address: 0x76d430, bytes: &[0x8b, 0x54, 0x24, 0x0c, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => national_team_ages, compare_national_team_ages;
    Prologue { address: 0x5fdd30, bytes: &[0x8b, 0x44, 0x24, 0x08, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => set_human_field, compare_set_human_field;
    Prologue { address: 0x7f6cd0, bytes: &[0x8a, 0x54, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => scout_record, compare_scout_record;
    Prologue { address: 0x600c90, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => human_runs_club, compare_human_runs_club;
    Prologue { address: 0x4ca470, bytes: &[0x8b, 0x54, 0x24, 0x04, 0x53] } => contract_category, compare_contract_category;
    Prologue { address: 0x6d58f0, bytes: &[0x8a, 0x81, 0xa8, 0x8e, 0x00, 0x00] } => ball_goal_end, compare_ball_goal_end;
    Prologue { address: 0x6d5930, bytes: &[0x8a, 0x81, 0x38, 0xf6, 0x00, 0x00] } => match_abandoned, compare_match_abandoned;
    Prologue { address: 0x6d5af0, bytes: &[0x8a, 0x44, 0x24, 0x04, 0x84, 0xc0] } => side_slots_without_conflict, compare_side_slots_without_conflict;
    Prologue { address: 0x6d5b20, bytes: &[0x8a, 0x44, 0x24, 0x04, 0x84, 0xc0] } => side_centre_backs_without_conflict, compare_side_centre_backs_without_conflict;
    Prologue { address: 0x6e9460, bytes: &[0x8b, 0xc1, 0xc7, 0x00, 0x00, 0x00, 0x00, 0x00] } => construct_referee, compare_construct_referee;
    Prologue { address: 0x6ebd60, bytes: &[0xc7, 0x81, 0x9e, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00] } => destroy_match_player, compare_destroy_match_player;
    Prologue { address: 0x6d5d00, bytes: &[0x66, 0x8b, 0x41, 0x08, 0x66, 0x85, 0xc0] } => event_at_front, compare_event_at_front;
    Prologue { address: 0x6d5bf0, bytes: &[0x53, 0x56, 0x8b, 0xf1, 0x83, 0xc8, 0xff] } => construct_commentary_queue, compare_construct_commentary_queue;
    Prologue { address: 0x6d5b50, bytes: &[0x53, 0x56, 0x57, 0xbe, 0x90, 0x87, 0xb6, 0x00] } => free_event_tables, compare_free_event_tables;
    Prologue { address: 0x6b6510, bytes: &[0x8b, 0x4c, 0x24, 0x04, 0x8b, 0x54, 0x24, 0x08] } => day_match_order, compare_day_match_order;
    Prologue { address: 0x6bd840, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x8b, 0x4c, 0x24, 0x08] } => action_order, compare_action_order;
    Prologue { address: 0x6d5d80, bytes: &[0x51, 0x53, 0x56, 0x6a, 0x03] } => event_variant, compare_event_variant;
    Prologue { address: 0x6d60b0, bytes: &[0x83, 0xec, 0x08, 0x53, 0x55] } => combine_with_previous, compare_combine_with_previous;
    Prologue { address: 0x6d4b70, bytes: &[0x6a, 0xff, 0x68, 0x00, 0xd4, 0x95, 0x00] } => context_commentary, compare_context_commentary;
    Prologue { address: 0x6eada0, bytes: &[0x8b, 0xc1, 0x83, 0xca, 0xff] } => construct_match_player, compare_construct_match_player;
    Prologue { address: 0x6d78b0, bytes: &[0x51, 0x8b, 0xd1, 0x55, 0x56] } => next_event, compare_next_event;
    Prologue { address: 0x6db2d0, bytes: &[0x8a, 0x44, 0x24, 0x18, 0x81, 0xec, 0x58, 0x02, 0x00, 0x00] } => commentary_player_name, compare_commentary_player_name;
    Prologue { address: 0x6e86d0, bytes: &[0x51, 0x8b, 0x15, 0x20, 0x71, 0xdd, 0x00] } => perceived_ability, compare_perceived_ability;
    Prologue { address: 0x6e8910, bytes: &[0x81, 0xec, 0x70, 0x04, 0x00, 0x00] } => apply_squad_rule, compare_apply_squad_rule;
    Prologue { address: 0x6e6340, bytes: &[0x83, 0xec, 0x50, 0x53, 0x55] } => off_match_score, compare_off_match_score;
    Prologue { address: 0x6e2f20, bytes: &[0x83, 0xec, 0x50, 0x53, 0x8b, 0x5c, 0x24, 0x58] } => lineup_slot_score, compare_lineup_slot_score;
    Prologue { address: 0x69fc60, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => person_in_list, compare_person_in_list;
    Prologue { address: 0x4cf6d0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x85, 0xc0] } => contract_record, compare_contract_record;
    Prologue { address: 0x62c2f0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x53] } => daily_wear, compare_daily_wear;
    Prologue { address: 0x896570, bytes: &[0x66, 0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => formation_record, compare_formation_record;
    Prologue { address: 0x7c6930, bytes: &[0x53, 0x55, 0x8b, 0x6c, 0x24, 0x0c] } => competition_weight, compare_competition_weight;
    Prologue { address: 0x785790, bytes: &[0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => set_news_field, compare_set_news_field;
    Prologue { address: 0x62bd10, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x56] } => recover_injury, compare_recover_injury;
    Prologue { address: 0x8c1870, bytes: &[0x8b, 0x81, 0x2e, 0x09, 0x00, 0x00] } => person_in_transfer_list, compare_person_in_transfer_list;
    Prologue { address: 0x55cae0, bytes: &[0x0f, 0xbe, 0x44, 0x24, 0x08] } => table_penalty, compare_table_penalty;
    Prologue { address: 0x7a7eb0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x8b, 0x15, 0x0c, 0x2d, 0xae, 0x00] } => record_bits, compare_record_bits;
    Prologue { address: 0x684790, bytes: &[0x8a, 0x81, 0xbe, 0x00, 0x00, 0x00] } => competition_flagged, compare_competition_flagged;
    Prologue { address: 0x6826d0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x8d, 0x14, 0xc5, 0x00, 0x00, 0x00, 0x00] } => league_record, compare_league_record;
    Prologue { address: 0x5d5a60, bytes: &[0x8a, 0x44, 0x24, 0x04, 0x84, 0xc0] } => game_average, compare_game_average;
    Prologue { address: 0x4d0c20, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x85, 0xc0] } => contract_value, compare_contract_value;
    Prologue { address: 0x76d810, bytes: &[0x8a, 0x44, 0x24, 0x0c, 0x84, 0xc0] } => national_team_level, compare_national_team_level;
    Prologue { address: 0x4cc9e0, bytes: &[0x53, 0x56, 0x8b, 0xf1, 0x57] } => contract_offer, compare_contract_offer;
    Prologue { address: 0x4cca30, bytes: &[0x56, 0x8b, 0xf1, 0x57, 0x8b, 0x46, 0x34] } => agreed_contract_offer, compare_agreed_contract_offer;
    Prologue { address: 0x8d0d90, bytes: &[0x8b, 0x4c, 0x24, 0x04, 0x8a, 0x41, 0x2c] } => transfer_offer_closed, compare_transfer_offer_closed;
    Prologue { address: 0x725a70, bytes: &[0x8b, 0x4c, 0x24, 0x08, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => match_control_label, compare_match_control_label;
    Prologue { address: 0x71b120, bytes: &[0x56, 0x6a, 0x2d, 0xb9, 0x50, 0x44, 0xb7, 0x00] } => close_match_screen, compare_close_match_screen;
    Prologue { address: 0x727f00, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => open_waiting_screen, compare_open_waiting_screen;
    Prologue { address: 0x735780, bytes: &[0x56, 0x8b, 0x71, 0x08, 0x33, 0xd2] } => story_with_response_two, compare_story_with_response_two;
    Prologue { address: 0x735840, bytes: &[0x56, 0x8b, 0x71, 0x08, 0x33, 0xd2] } => has_story, compare_has_story;
    Prologue { address: 0x5221f0, bytes: &[0x8a, 0x51, 0x51, 0x53, 0x8a, 0x59, 0x3c] } => cup_match_importance, compare_cup_match_importance;
    Prologue { address: 0x71b1b0, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => open_latest_scores, compare_open_latest_scores;
    Prologue { address: 0x71b300, bytes: &[0x83, 0xec, 0x08, 0x6a, 0x0e] } => build_latest_scores, compare_build_latest_scores;
    Prologue { address: 0x71b3d0, bytes: &[0xa1, 0x84, 0x7c, 0xb7, 0x00] } => latest_scores_input, compare_latest_scores_input;
    Prologue { address: 0x71b4a0, bytes: &[0x6a, 0x00, 0x68, 0x70, 0xc1, 0x71, 0x00] } => open_match_details, compare_open_match_details;
    Prologue { address: 0x71bdc0, bytes: &[0x51, 0xa1, 0x84, 0x7c, 0xb7, 0x00] } => match_details_input, compare_match_details_input;
    Prologue { address: 0x71c170, bytes: &[0x56, 0x6a, 0x2d, 0xb9, 0x50, 0x44, 0xb7, 0x00] } => close_match_details, compare_close_match_details;
    Prologue { address: 0x556210, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x8b, 0x10] } => discipline_new_season, compare_discipline_new_season;
    Prologue { address: 0x556240, bytes: &[0x53, 0x55, 0x8b, 0x6c, 0x24, 0x0c] } => discipline_daily_update, compare_discipline_daily_update;
    Prologue { address: 0x55cb20, bytes: &[0x0f, 0xbe, 0x44, 0x24, 0x08] } => discipline_set_slot, compare_discipline_set_slot;
    Prologue { address: 0x8c5410, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x8b, 0x10] } => set_agreed_contract_flag, compare_set_agreed_contract_flag;
    Prologue { address: 0x8c5430, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x8b, 0x10] } => clear_agreed_contract_flag, compare_clear_agreed_contract_flag;
    Prologue { address: 0x8c61e0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x8b, 0x51, 0x04] } => offer_slot, compare_offer_slot;
    Prologue { address: 0x718aa0, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => open_match_screen, compare_open_match_screen;
    Prologue { address: 0x725860, bytes: &[0x51, 0x53, 0x56, 0x57, 0x6a, 0x11] } => match_screens_handle_latest_scores_cycle, compare_match_screens_handle_latest_scores_cycle;
    Prologue { address: 0x722dc0, bytes: &[0x53, 0x56, 0x6a, 0x00, 0xb9, 0x50, 0x44, 0xb7, 0x00] } => handle_team_stats_input, compare_handle_team_stats_input;
    Prologue { address: 0x723d20, bytes: &[0x83, 0xec, 0x24, 0x53, 0x55] } => draw_latest_scores_panel, compare_draw_latest_scores_panel;
    Prologue { address: 0x7febb0, bytes: &[0x0f, 0xbf, 0x81, 0x2a, 0x38, 0x00, 0x00] } => is_forward_unavailable, compare_is_forward_unavailable;
    Prologue { address: 0x839590, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x85, 0xc0] } => get_heading_colour, compare_get_heading_colour;
    Prologue { address: 0x5d5ab0, bytes: &[0x8b, 0x81, 0x56, 0x08, 0x00, 0x00] } => transfer_manager_get_shortlist_manager, compare_transfer_manager_get_shortlist_manager;
    Prologue { address: 0x5d5ac0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x8b, 0x89, 0x1d, 0x65, 0x00, 0x00] } => training_manager_get_player_entry, compare_training_manager_get_player_entry;
    Prologue { address: 0x62c660, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x8b, 0x00] } => set_training_load, compare_set_training_load;
    Prologue { address: 0x5c2990, bytes: &[0x8b, 0x54, 0x24, 0x04, 0x56] } => is_own_comp_record, compare_is_own_comp_record;
    Prologue { address: 0x5cb9b0, bytes: &[0x8a, 0x44, 0x24, 0x04, 0x8a, 0x54, 0x24, 0x08] } => friendly_international_date_set, compare_friendly_international_date_set;
    Prologue { address: 0x86fc20, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x57] } => clear_manager_record, compare_clear_manager_record;
    Prologue { address: 0x4d0bd0, bytes: &[0x8b, 0x54, 0x24, 0x04, 0x56] } => remove_loan_contract, compare_remove_loan_contract;
    Prologue { address: 0x84aeb0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x56] } => clear_bid_markers_for_player, compare_clear_bid_markers_for_player;
    Prologue { address: 0x600e90, bytes: &[0x53, 0x56, 0x8b, 0x74, 0x24, 0x0c] } => human_manager_get_stand_in_manager, compare_human_manager_get_stand_in_manager;
    Prologue { address: 0x8c5450, bytes: &[0x56, 0x57, 0x8b, 0x7c, 0x24, 0x0c] } => get_agreed_contract_club, compare_get_agreed_contract_club;
    Prologue { address: 0x8ce920, bytes: &[0x8b, 0x54, 0x24, 0x04, 0x53] } => record_international_appearance, compare_record_international_appearance;
    Prologue { address: 0x55cb60, bytes: &[0x53, 0x55, 0x8b, 0x51, 0x14] } => find_held_ban, compare_find_held_ban;
    Prologue { address: 0x7a7ed0, bytes: &[0x8b, 0x41, 0x04, 0x8b, 0x49, 0x08] } => find_retirement_entry, compare_find_retirement_entry;
    Prologue { address: 0x5ca030, bytes: &[0xa1, 0xfc, 0xad, 0xad, 0x00] } => register_comp_objects, compare_register_comp_objects;
    Prologue { address: 0x785830, bytes: &[0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => get_slot, compare_get_slot;
    Prologue { address: 0x8cb4f0, bytes: &[0x8b, 0x54, 0x24, 0x04, 0x53] } => remove_next_sale_percentage, compare_remove_next_sale_percentage;
    Prologue { address: 0x8cb590, bytes: &[0x8b, 0x54, 0x24, 0x04, 0x53] } => remove_first_team_appearance_money, compare_remove_first_team_appearance_money;
    Prologue { address: 0x8cb630, bytes: &[0x8b, 0x54, 0x24, 0x04, 0x53] } => remove_international_team_appearance_money, compare_remove_international_team_appearance_money;
    Prologue { address: 0x7a82b0, bytes: &[0x8b, 0x51, 0x20, 0x53, 0x33, 0xc0] } => remove_spare_non_player_record, compare_remove_spare_non_player_record;
    Prologue { address: 0x7a8470, bytes: &[0x8b, 0x51, 0x40, 0x53, 0x33, 0xc0] } => remove_player_data_from_pool, compare_remove_player_data_from_pool;
    Prologue { address: 0x682f70, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => calc_prize_fund, compare_calc_prize_fund;
    Prologue { address: 0x7c1a60, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x04, 0x02, 0x00, 0x00] } => get_last_match_rating, compare_get_last_match_rating;
    Prologue { address: 0x7ae640, bytes: &[0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => set_slot_index, compare_set_slot_index;
    Prologue { address: 0x7ae6e0, bytes: &[0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => set_list_type, compare_set_list_type;
    Prologue { address: 0x6834f0, bytes: &[0x53, 0x55, 0x56, 0x8b, 0x74, 0x24, 0x10] } => league_compare_h2h_rows, compare_league_compare_h2h_rows;
    Prologue { address: 0x6869a0, bytes: &[0x8b, 0x44, 0x24, 0x08, 0x85, 0xc0] } => update_head_to_head, compare_update_head_to_head;
    Prologue { address: 0x8c1f80, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => note_terms_offered_to_main_player, compare_note_terms_offered_to_main_player;
    Prologue { address: 0x5fdc30, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => get_holiday_stamp, compare_get_holiday_stamp;
    Prologue { address: 0x5fed50, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => get_saved_lineup, compare_get_saved_lineup;
    Prologue { address: 0x7ae570, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => set_manager, compare_set_manager;
    Prologue { address: 0x717cc0, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => open_day_fixtures_screen, compare_open_day_fixtures_screen;
    Prologue { address: 0x718930, bytes: &[0x56, 0x8b, 0x74, 0x24, 0x08] } => handle_day_fixtures_input, compare_handle_day_fixtures_input;
    Prologue { address: 0x727e20, bytes: &[0x81, 0xec, 0x04, 0x02, 0x00, 0x00] } => match_screens_find_best_rated_starter_slot, compare_match_screens_find_best_rated_starter_slot;
    Prologue { address: 0x7c47b0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x04, 0x02, 0x00, 0x00] } => get_club_league_stat, compare_get_club_league_stat;
    Prologue { address: 0x729500, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => fixture_entry_matches, compare_fixture_entry_matches;
    Prologue { address: 0x729bd0, bytes: &[0x53, 0x56, 0x8b, 0x74, 0x24, 0x0c] } => match_stats_get_record_bucket, compare_match_stats_get_record_bucket;
    Prologue { address: 0x71dd70, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x53] } => match_team_block_assign, compare_match_team_block_assign;
    Prologue { address: 0x71df00, bytes: &[0x83, 0xec, 0x08, 0x56, 0x57] } => match_player_row_assign, compare_match_player_row_assign;
    Prologue { address: 0x728e90, bytes: &[0x8b, 0x44, 0x24, 0x0c, 0x53] } => match_screens_tally_live_score, compare_match_screens_tally_live_score;
    Prologue { address: 0x725ca0, bytes: &[0x83, 0xec, 0x44, 0x53, 0x55] } => match_screens_handle_continue_command, compare_match_screens_handle_continue_command;
    Prologue { address: 0x725f70, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => match_screens_open_team_selection, compare_match_screens_open_team_selection;
    Prologue { address: 0x727c30, bytes: &[0x53, 0x55, 0x56, 0x57, 0x6a, 0x00] } => match_screens_handle_team_selection_input, compare_match_screens_handle_team_selection_input;
    Prologue { address: 0x7217a0, bytes: &[0x83, 0xec, 0x34, 0x53, 0x56] } => draw_team_stats_panel, compare_draw_team_stats_panel;
    Prologue { address: 0x722ed0, bytes: &[0x81, 0xec, 0x0c, 0x01, 0x00, 0x00] } => draw_player_ratings_panel, compare_draw_player_ratings_panel;
    Prologue { address: 0x71e040, bytes: &[0x51, 0x8b, 0x44, 0x24, 0x0c] } => draw_match_stats_panel, compare_draw_match_stats_panel;
    Prologue { address: 0x71e750, bytes: &[0x6a, 0xff, 0x68, 0x5b, 0xd5, 0x95, 0x00] } => refresh_match_stats_values, compare_refresh_match_stats_values;
    Prologue { address: 0x720320, bytes: &[0x6a, 0xff, 0x68, 0x7b, 0xd5, 0x95, 0x00] } => refresh_action_zones_values, compare_refresh_action_zones_values;
    Prologue { address: 0x723350, bytes: &[0x6a, 0xff, 0x68, 0xbb, 0xd5, 0x95, 0x00] } => refresh_player_ratings_rows, compare_refresh_player_ratings_rows;
    Prologue { address: 0x71f500, bytes: &[0xa0, 0xb4, 0xb7, 0xb6, 0x00] } => draw_action_zones_panel, compare_draw_action_zones_panel;
    Prologue { address: 0x721000, bytes: &[0x6a, 0xff, 0x68, 0x9e, 0xd5, 0x95, 0x00] } => draw_match_report_panel, compare_draw_match_report_panel;
    Prologue { address: 0x717db0, bytes: &[0x81, 0xec, 0x9c, 0x00, 0x00, 0x00] } => build_day_fixtures_screen, compare_build_day_fixtures_screen;
    Prologue { address: 0x721cd0, bytes: &[0x81, 0xec, 0x18, 0x01, 0x00, 0x00] } => fill_team_stats_rows, compare_fill_team_stats_rows;
    Prologue { address: 0x724520, bytes: &[0x81, 0xec, 0x3c, 0x04, 0x00, 0x00] } => refresh_latest_scores_rows, compare_refresh_latest_scores_rows;
    Prologue { address: 0x727fb0, bytes: &[0x83, 0xec, 0x44, 0x53, 0x56] } => match_screens_handle_waiting_screen, compare_match_screens_handle_waiting_screen;
    Prologue { address: 0x729600, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => read_match_record, compare_read_match_record;
    Prologue { address: 0x7293f0, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => match_screens_record_match, compare_match_screens_record_match;
    Prologue { address: 0x729820, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => write_match_record, compare_write_match_record;
    Prologue { address: 0x7260f0, bytes: &[0x6a, 0xff, 0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] } => match_screens_draw_team_selection, compare_match_screens_draw_team_selection;
    Prologue { address: 0x72aa80, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x04, 0x02, 0x00, 0x00] } => match_screens_load, compare_match_screens_load;
    Prologue { address: 0x72adf0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x04, 0x02, 0x00, 0x00] } => match_screens_save, compare_match_screens_save;
    Prologue { address: 0x403350, bytes: &[0x81, 0xec, 0x0c, 0x02, 0x00, 0x00] } => area_layout, compare_area_layout;
    Prologue { address: 0x402ac0, bytes: &[0x66, 0x83, 0x7c, 0x24, 0x38, 0xff] } => init, compare_init;
    Prologue { address: 0x402e70, bytes: &[0x81, 0xec, 0x10, 0x02, 0x00, 0x00] } => draw, compare_draw;
    Prologue { address: 0x718e90, bytes: &[0x6a, 0xff, 0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] } => build_match_screen, compare_build_match_screen;
    Prologue { address: 0x71a240, bytes: &[0xa1, 0x84, 0x7c, 0xb7, 0x00] } => handle_match_screen_input, compare_handle_match_screen_input;
    Prologue { address: 0x71b630, bytes: &[0x6a, 0xff, 0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] } => build_match_details_screen, compare_build_match_details_screen;
    Prologue { address: 0x71d9e0, bytes: &[0x6a, 0xff, 0x68, 0x3b, 0xd5, 0x95, 0x00] } => refresh_possession_bar, compare_refresh_possession_bar;
    Prologue { address: 0x71c270, bytes: &[0x6a, 0xff, 0x68, 0x1b, 0xd5, 0x95, 0x00] } => draw_match_overview_panel, compare_draw_match_overview_panel;
    Prologue { address: 0x7280e0, bytes: &[0x6a, 0xff, 0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] } => match_screens_draw_live_league_table, compare_match_screens_draw_live_league_table;
    Prologue { address: 0x7357c0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x53, 0x55] } => get_finable_story_kind, compare_get_finable_story_kind;
    Prologue { address: 0x62d140, bytes: &[0x51, 0x56, 0x57, 0x8b, 0x7c, 0x24, 0x18] } => choose_injury_type, compare_choose_injury_type;
    Prologue { address: 0x785670, bytes: &[0x8b, 0x01, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => get_category, compare_get_category;
    Prologue { address: 0x76c060, bytes: &[0x51, 0x53, 0x55, 0x32, 0xc0] } => national_teams_remove_duplicate_squad_entries, compare_national_teams_remove_duplicate_squad_entries;
    Prologue { address: 0x8972c0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => find_selection_slot, compare_find_selection_slot;
    Prologue { address: 0x72b810, bytes: &[0x51, 0x53, 0x55, 0x8b, 0x6c, 0x24, 0x14] } => pick_former_international_pundit, compare_pick_former_international_pundit;
    Prologue { address: 0x521d60, bytes: &[0x55, 0x8b, 0x6c, 0x24, 0x08] } => apply_round_rules_to_match, compare_apply_round_rules_to_match;
    Prologue { address: 0x519c90, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => find_tree_slot_for_fixture, compare_find_tree_slot_for_fixture;
    Prologue { address: 0x5c4680, bytes: &[0x51, 0x8b, 0x44, 0x24, 0x08] } => pick_tour_nation, compare_pick_tour_nation;
    Prologue { address: 0x8c6db0, bytes: &[0x56, 0x8b, 0x74, 0x24, 0x08] } => transfer_manager_close_offer_contract_offers, compare_transfer_manager_close_offer_contract_offers;
    Prologue { address: 0x8c16b0, bytes: &[0x81, 0xec, 0x0c, 0x02, 0x00, 0x00] } => queue_free_transfer, compare_queue_free_transfer;
    Prologue { address: 0x8c5fc0, bytes: &[0x83, 0xec, 0x0c, 0x8b, 0x44, 0x24, 0x10] } => get_exchange_negotiations_for_player, compare_get_exchange_negotiations_for_player;
    Prologue { address: 0x775ba0, bytes: &[0x83, 0xec, 0x08, 0x53, 0x56, 0x57] } => national_teams_remove_player, compare_national_teams_remove_player;
    Prologue { address: 0x5af2f0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x85, 0xc0] } => get_time_slice, compare_get_time_slice;
    Prologue { address: 0x84b230, bytes: &[0x0f, 0xbe, 0x44, 0x24, 0x0c] } => shortlist_manager_max_fee_for_squad_status, compare_shortlist_manager_max_fee_for_squad_status;
    Prologue { address: 0x8c5cf0, bytes: &[0x51, 0x8b, 0x44, 0x24, 0x08] } => get_offers_for_player, compare_get_offers_for_player;
    Prologue { address: 0x83bf40, bytes: &[0x56, 0x8b, 0x91, 0xdc, 0x00, 0x00, 0x00] } => update_club_target_reputation, compare_update_club_target_reputation;
    Prologue { address: 0x4afeb0, bytes: &[0x53, 0x55, 0x8b, 0x6c, 0x24, 0x14] } => comp_util_write_fixture_to_news_slots, compare_comp_util_write_fixture_to_news_slots;
    Prologue { address: 0x729ff0, bytes: &[0xa0, 0xc4, 0xb7, 0xb6, 0x00] } => fill_match_record, compare_fill_match_record;
    Prologue { address: 0x72b160, bytes: &[0x8b, 0x54, 0x24, 0x08, 0x56] } => match_stats_compare_events, compare_match_stats_compare_events;
    Prologue { address: 0x7c6bf0, bytes: &[0x83, 0xec, 0x08, 0x56, 0x8b, 0xf1] } => remove_language, compare_remove_language;
    Prologue { address: 0x8ced40, bytes: &[0x53, 0x8b, 0x5c, 0x24, 0x08] } => can_sign_player, compare_can_sign_player;
    Prologue { address: 0x8c5e40, bytes: &[0x51, 0x53, 0x55, 0x8b, 0x6c, 0x24, 0x10] } => get_exchange_offers_for_player, compare_get_exchange_offers_for_player;
    Prologue { address: 0x786fe0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x04, 0x02, 0x00, 0x00] } => count_unread, compare_count_unread;
    Prologue { address: 0x786d30, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => set_item_slot, compare_set_item_slot;
    Prologue { address: 0x7875e0, bytes: &[0x56, 0x8b, 0x74, 0x24, 0x08] } => news_club_has_human_manager, compare_news_club_has_human_manager;
    Prologue { address: 0x521e60, bytes: &[0x8b, 0x4c, 0x24, 0x14, 0x8b, 0x44, 0x24, 0x18] } => cup_set_round_draw_date, compare_cup_set_round_draw_date;
    Prologue { address: 0x521eb0, bytes: &[0x8b, 0x4c, 0x24, 0x14, 0x8b, 0x44, 0x24, 0x18] } => cup_set_round_match_date, compare_cup_set_round_match_date;
    Prologue { address: 0x5c64e0, bytes: &[0x56, 0x8b, 0xf1, 0x57, 0x8b, 0x7c, 0x24, 0x0c] } => remove_club_from_ai_plans, compare_remove_club_from_ai_plans;
    Prologue { address: 0x683210, bytes: &[0x51, 0x53, 0x8b, 0x1d, 0xfc, 0x79, 0xb6, 0x00] } => league_compare_table_rows, compare_league_compare_table_rows;
    Prologue { address: 0x6844b0, bytes: &[0x8b, 0x4c, 0x24, 0x08, 0x83, 0xec, 0x10] } => league_build_round_robin_grid, compare_league_build_round_robin_grid;
    Prologue { address: 0x68a4d0, bytes: &[0x0f, 0xbf, 0x81, 0xa9, 0x00, 0x00, 0x00] } => final_day_margin_needed, compare_final_day_margin_needed;
    Prologue { address: 0x4b0010, bytes: &[0x53, 0x55, 0x8b, 0x6c, 0x24, 0x14] } => comp_util_read_fixture_from_news_slots, compare_comp_util_read_fixture_from_news_slots;
    Prologue { address: 0x4b0890, bytes: &[0x53, 0x56, 0x8b, 0x74, 0x24, 0x10] } => comp_util_classify_tie_result_for_news, compare_comp_util_classify_tie_result_for_news;
    Prologue { address: 0x521f10, bytes: &[0x66, 0x8b, 0x4c, 0x24, 0x0c] } => cup_set_round_match_day, compare_cup_set_round_match_day;
    Prologue { address: 0x5b7160, bytes: &[0x83, 0xec, 0x48, 0x53, 0x55] } => build_position_lookup, compare_build_position_lookup;
    Prologue { address: 0x7ad220, bytes: &[0x53, 0x55, 0x8b, 0x6c, 0x24, 0x0c] } => player_regen_score_club_for_player, compare_player_regen_score_club_for_player;
    Prologue { address: 0x5fe290, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => is_watching_fixture, compare_is_watching_fixture;
    Prologue { address: 0x6b6970, bytes: &[0x6a, 0xff, 0x68, 0x8c, 0xd2, 0x95, 0x00] } => match_day_drop_viewer_and_finish_match, compare_match_day_drop_viewer_and_finish_match;
    Prologue { address: 0x729e30, bytes: &[0x6a, 0xff, 0x68, 0x3b, 0xd6, 0x95, 0x00] } => find_record_fixture, compare_find_record_fixture;
    Prologue { address: 0x6847b0, bytes: &[0x8a, 0x91, 0xc1, 0x00, 0x00, 0x00] } => has_relegation, compare_has_relegation;
    Prologue { address: 0x68a160, bytes: &[0x8b, 0x4c, 0x24, 0x14, 0x8b, 0x44, 0x24, 0x18] } => league_round_set_date, compare_league_round_set_date;
    Prologue { address: 0x68a3e0, bytes: &[0x51, 0x8b, 0x44, 0x24, 0x10] } => get_match_importance, compare_get_match_importance;
    Prologue { address: 0x448490, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x83, 0xec, 0x08] } => record_holder_assign, compare_record_holder_assign;
    Prologue { address: 0x448110, bytes: &[0x83, 0xec, 0x08, 0x8d, 0x44, 0x24, 0x00] } => match_record_assign, compare_match_record_assign;
    Prologue { address: 0x448170, bytes: &[0x83, 0xec, 0x08, 0x8d, 0x54, 0x24, 0x00] } => run_record_assign, compare_run_record_assign;
    Prologue { address: 0x5fe420, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => is_fixture_watched, compare_is_fixture_watched;
    Prologue { address: 0x600740, bytes: &[0x81, 0xec, 0x20, 0x02, 0x00, 0x00] } => has_nickname, compare_has_nickname;
    Prologue { address: 0x7a8780, bytes: &[0x83, 0xec, 0x08, 0x56, 0x57] } => retire_record_assign, compare_retire_record_assign;
    Prologue { address: 0x729000, bytes: &[0x6a, 0xff, 0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] } => construct_match_stats, compare_construct_match_stats;
    Prologue { address: 0x7293c0, bytes: &[0x56, 0x8b, 0xf1, 0x8b, 0x06] } => destroy_match_stats, compare_destroy_match_stats;
    Prologue { address: 0x933a00, bytes: &[0x51, 0x53, 0x55, 0x8b, 0x6c, 0x24, 0x14] } => zipdir_match_filename_pattern, compare_zipdir_match_filename_pattern;
    Prologue { address: 0x9331c0, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => zget_archive_id, compare_zget_archive_id;
    Prologue { address: 0x933860, bytes: &[0x81, 0xec, 0x04, 0x02, 0x00, 0x00] } => zget_matching_size, compare_zget_matching_size;
    Prologue { address: 0x933610, bytes: &[0x81, 0xec, 0x20, 0x04, 0x00, 0x00] } => zadd_matching_files, compare_zadd_matching_files;
    Prologue { address: 0x933320, bytes: &[0x81, 0xec, 0x14, 0x03, 0x00, 0x00] } => zextract_matching_files, compare_zextract_matching_files;
    Prologue { address: 0x90cd50, bytes: &[0xb8, 0x1c, 0x68, 0x00, 0x00] } => utils_clear_temp_folder, compare_utils_clear_temp_folder;
    Prologue { address: 0x932c30, bytes: &[0x53, 0x55, 0x56, 0x8b, 0x74, 0x24, 0x1c] } => zipdir_zread, compare_zipdir_zread;
    Prologue { address: 0x932e10, bytes: &[0x53, 0x55, 0x56, 0x8b, 0x74, 0x24, 0x1c] } => zipdir_zwrite, compare_zipdir_zwrite;
    Prologue { address: 0x932b00, bytes: &[0x56, 0x8b, 0x74, 0x24, 0x08] } => zclose, compare_zclose;
    Prologue { address: 0x838e10, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => check_saved_game_file, compare_check_saved_game_file;
    Prologue { address: 0x839290, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => check_quick_start_file, compare_check_quick_start_file;
    Prologue { address: 0x838eb0, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] } => valid_saved_game_version, compare_valid_saved_game_version;
    Prologue { address: 0x8395b0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x04, 0x02, 0x00, 0x00] } => save_fifa_transfer_rules, compare_save_fifa_transfer_rules;
    Prologue { address: 0x839760, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x04, 0x02, 0x00, 0x00] } => load_fifa_transfer_rules, compare_load_fifa_transfer_rules;
    Prologue { address: 0x591f80, bytes: &[0x83, 0xec, 0x70, 0x8b, 0x44, 0x24, 0x74] } => build_save_game_summary, compare_build_save_game_summary;
    Prologue { address: 0x524d30, bytes: &[0x8b, 0x44, 0x24, 0x0c, 0x8b, 0x4c, 0x24, 0x08] } => save_database_for_game, compare_save_database_for_game;
    Prologue { address: 0x524d10, bytes: &[0x8b, 0x44, 0x24, 0x08, 0x8b, 0x4c, 0x24, 0x04] } => load_database_from_stream, compare_load_database_from_stream;
    Prologue { address: 0x82ff40, bytes: &[0x53, 0xb9, 0x50, 0x44, 0xb7, 0x00] } => free_game_data, compare_free_game_data;
    Prologue { address: 0x827110, bytes: &[0x81, 0xec, 0x00, 0x08, 0x00, 0x00] } => create_game_from_setup_file, compare_create_game_from_setup_file;
    Prologue { address: 0x82e430, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] } => save_game_data, compare_save_game_data;
    Prologue { address: 0x82a9c0, bytes: &[0x6a, 0xff, 0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] } => load_game_data, compare_load_game_data;
    Prologue { address: 0x5408c0, bytes: &[0x56, 0x8b, 0x74, 0x24, 0x0c] } => database_get_reputation_scope, compare_database_get_reputation_scope;
    Prologue { address: 0x5407e0, bytes: &[0x8b, 0x44, 0x24, 0x0c, 0x56] } => database_get_staff_reputation_for_viewer, compare_database_get_staff_reputation_for_viewer;
    Prologue { address: 0x53d980, bytes: &[0x8b, 0x44, 0x24, 0x08, 0x83, 0xec, 0x10] } => database_is_staff_foreign, compare_database_is_staff_foreign;
    Prologue { address: 0x53ab90, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => database_get_staff_age, compare_database_get_staff_age;
    Prologue { address: 0x544670, bytes: &[0x80, 0x3d, 0x3c, 0x23, 0xae, 0x00, 0x01] } => nation_allows_free_transfers, compare_nation_allows_free_transfers;
    Prologue { address: 0x53d740, bytes: &[0x8b, 0x44, 0x24, 0x04, 0xb1, 0x02] } => database_is_staff_eu_national, compare_database_is_staff_eu_national;
    Prologue { address: 0x544520, bytes: &[0x56, 0x8b, 0x74, 0x24, 0x08] } => staff_is_transfer_listed_or_requested, compare_staff_is_transfer_listed_or_requested;
    Prologue { address: 0x543540, bytes: &[0x8b, 0x54, 0x24, 0x04, 0x81, 0xec, 0x04, 0x02, 0x00, 0x00] } => staff_compute_player_attribute, compare_staff_compute_player_attribute;
    Prologue { address: 0x5431e0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => staff_get_player_attribute, compare_staff_get_player_attribute;
    Prologue { address: 0x540780, bytes: &[0x8b, 0x54, 0x24, 0x04, 0x56] } => database_compare_comp_history_rows, compare_database_compare_comp_history_rows;
    Prologue { address: 0x53a610, bytes: &[0x8b, 0x44, 0x24, 0x0c, 0xba, 0x3f, 0x00, 0x00, 0x00] } => database_player_to_attribute_vector, compare_database_player_to_attribute_vector;
    Prologue { address: 0x5446d0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x8b, 0x0d, 0xe4, 0xf2, 0x9c, 0x00] } => nation_is_uk_or_ireland, compare_nation_is_uk_or_ireland;
    Prologue { address: 0x53d7f0, bytes: &[0x8b, 0x4c, 0x24, 0x04, 0x85, 0xc9] } => database_is_basque_club, compare_database_is_basque_club;
    Prologue { address: 0x5444a0, bytes: &[0x56, 0x57, 0x8b, 0x7c, 0x24, 0x0c] } => staff_is_transfer_listed, compare_staff_is_transfer_listed;
    Prologue { address: 0x544590, bytes: &[0x56, 0x8b, 0x74, 0x24, 0x08] } => staff_is_loan_listed, compare_staff_is_loan_listed;
    Prologue { address: 0x544600, bytes: &[0x56, 0x8b, 0x74, 0x24, 0x08] } => staff_is_listed_by_request, compare_staff_is_listed_by_request;
    Prologue { address: 0x544380, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x0f, 0xbe, 0x54, 0x24, 0x08] } => player_attribute_rating, compare_player_attribute_rating;
    Prologue { address: 0x542740, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => database_club_has_reserve_squad, compare_database_club_has_reserve_squad;
    Prologue { address: 0x539c60, bytes: &[0x83, 0xec, 0x14, 0x53, 0x55] } => database_pick_first_name_for_nation, compare_database_pick_first_name_for_nation;
    Prologue { address: 0x53a1b0, bytes: &[0x51, 0x53, 0x55, 0x56, 0x57] } => database_pick_second_name_for_nation, compare_database_pick_second_name_for_nation;
    Prologue { address: 0x53dba0, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => database_format_staff_name, compare_database_format_staff_name;
    Prologue { address: 0x53b640, bytes: &[0x66, 0xa1, 0xf0, 0xbd, 0xae, 0x00] } => database_get_club_kit_colours, compare_database_get_club_kit_colours;
    Prologue { address: 0x53ca90, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => database_get_club_manager, compare_database_get_club_manager;
    Prologue { address: 0x545880, bytes: &[0x6a, 0xff, 0x68, 0xfb, 0x79, 0x95, 0x00] } => club_get_english_name, compare_club_get_english_name;
    Prologue { address: 0x544710, bytes: &[0x8a, 0x4c, 0x24, 0x08, 0x84, 0xc9] } => staff_adjust_morale, compare_staff_adjust_morale;
    Prologue { address: 0x53a8b0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x8b, 0x4c, 0x24, 0x0c] } => database_non_player_to_attribute_vector, compare_database_non_player_to_attribute_vector;
    Prologue { address: 0x540980, bytes: &[0x8a, 0x4c, 0x24, 0x08, 0x8b, 0x44, 0x24, 0x04] } => staff_set_squad_flags, compare_staff_set_squad_flags;
    Prologue { address: 0x53c590, bytes: &[0x81, 0xec, 0x0c, 0x02, 0x00, 0x00] } => database_count_club_squad, compare_database_count_club_squad;
    Prologue { address: 0x541f90, bytes: &[0x81, 0xec, 0x04, 0x02, 0x00, 0x00] } => database_collect_club_squad, compare_database_collect_club_squad;
    Prologue { address: 0x544820, bytes: &[0x56, 0x8b, 0x74, 0x24, 0x08] } => club_can_be_managed, compare_club_can_be_managed;
    Prologue { address: 0x53a980, bytes: &[0x8b, 0x44, 0x24, 0x0c, 0x8b, 0x4c, 0x24, 0x04] } => database_vector_to_non_player, compare_database_vector_to_non_player;
    Prologue { address: 0x53aa10, bytes: &[0x8b, 0x44, 0x24, 0x0c, 0x8b, 0x4c, 0x24, 0x04] } => database_vector_to_player, compare_database_vector_to_player;
    Prologue { address: 0x539490, bytes: &[0x81, 0xec, 0x6c, 0x02, 0x00, 0x00] } => staff_normalise_non_player_attributes, compare_staff_normalise_non_player_attributes;
    Prologue { address: 0x540520, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x8b, 0x0d, 0xe4, 0xf1, 0x9c, 0x00] } => database_get_colour_family, compare_database_get_colour_family;
    Prologue { address: 0x540490, bytes: &[0x53, 0x56, 0x57, 0x8b, 0x7c, 0x24, 0x10] } => database_colours_clash, compare_database_colours_clash;
    Prologue { address: 0x543fd0, bytes: &[0x8b, 0x54, 0x24, 0x04, 0x81, 0xec, 0x04, 0x02, 0x00, 0x00] } => staff_compute_non_player_attribute, compare_staff_compute_non_player_attribute;
    Prologue { address: 0x545a60, bytes: &[0x8b, 0x4c, 0x24, 0x08, 0x0c, 0xff] } => get_adjective_form_index, compare_get_adjective_form_index;
    Prologue { address: 0x543f30, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => staff_get_non_player_attribute, compare_staff_get_non_player_attribute;
    Prologue { address: 0x5442f0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x0f, 0xbe, 0x54, 0x24, 0x08] } => player_attribute_rating_with_scaled_ability, compare_player_attribute_rating_with_scaled_ability;
    Prologue { address: 0x539b90, bytes: &[0x51, 0x8b, 0x44, 0x24, 0x08] } => database_pick_first_name_by_weight, compare_database_pick_first_name_by_weight;
    Prologue { address: 0x53b7c0, bytes: &[0x66, 0xa1, 0x84, 0x31, 0xae, 0x00] } => database_get_comp_colours, compare_database_get_comp_colours;
    Prologue { address: 0x53b9a0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => database_club_has_room_for_job, compare_database_club_has_room_for_job;
    Prologue { address: 0x53bd10, bytes: &[0x81, 0xec, 0x08, 0x02, 0x00, 0x00] } => database_add_staff_to_club, compare_database_add_staff_to_club;
    Prologue { address: 0x6146b0, bytes: &[0x56, 0x8b, 0x74, 0x24, 0x08] } => match_club_key_id, compare_match_club_key_id;
    Prologue { address: 0x60ba80, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x8b, 0x0d, 0xa4, 0x00, 0x9d, 0x00] } => remap_club_key_id, compare_remap_club_key_id;
    Prologue { address: 0x60d810, bytes: &[0x56, 0x8b, 0x74, 0x24, 0x08] } => match_nation_key_id, compare_match_nation_key_id;
    Prologue { address: 0x60efd0, bytes: &[0x56, 0x8b, 0x74, 0x24, 0x08] } => match_club_comp_key_id, compare_match_club_comp_key_id;
    Prologue { address: 0x6115e0, bytes: &[0x56, 0x8b, 0x74, 0x24, 0x08] } => match_staff_comp_key_id, compare_match_staff_comp_key_id;
    Prologue { address: 0x54d5e0, bytes: &[0x8b, 0x44, 0x24, 0x10, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => db_files_read_continents, compare_db_files_read_continents;
    Prologue { address: 0x54d740, bytes: &[0x8b, 0x44, 0x24, 0x10, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => db_files_read_nations, compare_db_files_read_nations;
    Prologue { address: 0x54d8a0, bytes: &[0x8b, 0x44, 0x24, 0x10, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => db_files_read_cities, compare_db_files_read_cities;
    Prologue { address: 0x54da00, bytes: &[0x8b, 0x44, 0x24, 0x10, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => db_files_read_stadiums, compare_db_files_read_stadiums;
    Prologue { address: 0x54e780, bytes: &[0x8b, 0x44, 0x24, 0x10, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => db_files_read_staff_v1, compare_db_files_read_staff_v1;
    Prologue { address: 0x54fd60, bytes: &[0x8b, 0x44, 0x24, 0x10, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => db_files_read_staff_history, compare_db_files_read_staff_history;
    Prologue { address: 0x550230, bytes: &[0x8b, 0x44, 0x24, 0x10, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => db_files_read_comp_history, compare_db_files_read_comp_history;
    Prologue { address: 0x550380, bytes: &[0x8b, 0x44, 0x24, 0x10, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => db_files_read_staff_comps, compare_db_files_read_staff_comps;
    Prologue { address: 0x5504d0, bytes: &[0x8b, 0x44, 0x24, 0x10, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => db_files_read_staff_comp_history, compare_db_files_read_staff_comp_history;
    Prologue { address: 0x5505c0, bytes: &[0x8b, 0x44, 0x24, 0x10, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => db_files_read_officials, compare_db_files_read_officials;
    Prologue { address: 0x550710, bytes: &[0x8b, 0x44, 0x24, 0x10, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => db_files_read_colours, compare_db_files_read_colours;
    Prologue { address: 0x550860, bytes: &[0x8b, 0x44, 0x24, 0x10, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => db_files_read_names, compare_db_files_read_names;
    Prologue { address: 0x54db50, bytes: &[0x8b, 0x44, 0x24, 0x10, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => db_files_read_clubs, compare_db_files_read_clubs;
    Prologue { address: 0x54feb0, bytes: &[0x8b, 0x44, 0x24, 0x10, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => db_files_read_comps, compare_db_files_read_comps;
    Prologue { address: 0x54e880, bytes: &[0x8b, 0x44, 0x24, 0x10, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => db_files_read_players, compare_db_files_read_players;
    Prologue { address: 0x54f030, bytes: &[0x8b, 0x44, 0x24, 0x10, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => db_files_read_non_players, compare_db_files_read_non_players;
    Prologue { address: 0x54e630, bytes: &[0x8b, 0x44, 0x24, 0x10, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => db_files_read_staff, compare_db_files_read_staff;
    Prologue { address: 0x54fc10, bytes: &[0x8b, 0x44, 0x24, 0x10, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => db_files_read_staff_extra, compare_db_files_read_staff_extra;
    Prologue { address: 0x54d6e0, bytes: &[0x8b, 0x44, 0x24, 0x08, 0x56] } => db_files_write_continents, compare_db_files_write_continents;
    Prologue { address: 0x54d840, bytes: &[0x8b, 0x44, 0x24, 0x08, 0x56] } => db_files_write_nations, compare_db_files_write_nations;
    Prologue { address: 0x54d9a0, bytes: &[0x8b, 0x44, 0x24, 0x08, 0x56] } => db_files_write_cities, compare_db_files_write_cities;
    Prologue { address: 0x54daf0, bytes: &[0x8b, 0x44, 0x24, 0x08, 0x56] } => db_files_write_stadiums, compare_db_files_write_stadiums;
    Prologue { address: 0x54e5d0, bytes: &[0x8b, 0x44, 0x24, 0x08, 0x56] } => db_files_write_clubs, compare_db_files_write_clubs;
    Prologue { address: 0x54e720, bytes: &[0x8b, 0x44, 0x24, 0x08, 0x56] } => db_files_write_staff, compare_db_files_write_staff;
    Prologue { address: 0x54efd0, bytes: &[0x8b, 0x44, 0x24, 0x08, 0x56] } => db_files_write_players, compare_db_files_write_players;
    Prologue { address: 0x54fbb0, bytes: &[0x8b, 0x44, 0x24, 0x08, 0x56] } => db_files_write_non_players, compare_db_files_write_non_players;
    Prologue { address: 0x54fd00, bytes: &[0x8b, 0x44, 0x24, 0x08, 0x56] } => db_files_write_staff_extra, compare_db_files_write_staff_extra;
    Prologue { address: 0x54fe50, bytes: &[0x8b, 0x44, 0x24, 0x08, 0x56] } => db_files_write_staff_history, compare_db_files_write_staff_history;
    Prologue { address: 0x5501d0, bytes: &[0x8b, 0x44, 0x24, 0x08, 0x56] } => db_files_write_comps, compare_db_files_write_comps;
    Prologue { address: 0x550320, bytes: &[0x8b, 0x44, 0x24, 0x08, 0x56] } => db_files_write_comp_history, compare_db_files_write_comp_history;
    Prologue { address: 0x550470, bytes: &[0x8b, 0x44, 0x24, 0x08, 0x56] } => db_files_write_staff_comps, compare_db_files_write_staff_comps;
    Prologue { address: 0x5506b0, bytes: &[0x8b, 0x44, 0x24, 0x08, 0x56] } => db_files_write_officials, compare_db_files_write_officials;
    Prologue { address: 0x550800, bytes: &[0x8b, 0x44, 0x24, 0x08, 0x56] } => db_files_write_records_0x3a, compare_db_files_write_records_0x3a;
    Prologue { address: 0x550950, bytes: &[0x8b, 0x44, 0x24, 0x08, 0x56] } => db_files_write_or_read_records_0x3c, compare_db_files_write_or_read_records_0x3c;
    Prologue { address: 0x54d580, bytes: &[0x8b, 0x44, 0x24, 0x08, 0x8b, 0x4c, 0x24, 0x04] } => db_files_open_for_read, compare_db_files_open_for_read;
    Prologue { address: 0x54d5b0, bytes: &[0x8b, 0x44, 0x24, 0x08, 0x8b, 0x4c, 0x24, 0x04] } => db_files_open_for_write, compare_db_files_open_for_write;
    Prologue { address: 0x527ee0, bytes: &[0xa1, 0xcc, 0x23, 0xae, 0x00] } => database_free_staff_history_table, compare_database_free_staff_history_table;
    Prologue { address: 0x527f20, bytes: &[0xa1, 0xe4, 0x23, 0xae, 0x00] } => database_free_staff_comp_history_table, compare_database_free_staff_comp_history_table;
    Prologue { address: 0x527f60, bytes: &[0xa1, 0x7c, 0x23, 0xae, 0x00] } => database_grow_comp_history_table, compare_database_grow_comp_history_table;
    Prologue { address: 0x540020, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x8b, 0x4c, 0x24, 0x08] } => qsort_comparator_club_ids_by_reputation, compare_qsort_comparator_club_ids_by_reputation;
    Prologue { address: 0x5402f0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x8b, 0x4c, 0x24, 0x08] } => qsort_comparator_squad_by_player_0xb, compare_qsort_comparator_squad_by_player_0xb;
    Prologue { address: 0x549c30, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x8b, 0x4c, 0x24, 0x08] } => compare_staff_by_player_ability_desc, compare_compare_staff_by_player_ability_desc;
    Prologue { address: 0x538ec0, bytes: &[0x81, 0xec, 0x78, 0x02, 0x00, 0x00] } => staff_normalise_player_physical_attributes, compare_staff_normalise_player_physical_attributes;
    Prologue { address: 0x53a0e0, bytes: &[0x51, 0x8b, 0x44, 0x24, 0x08] } => database_pick_second_name_by_weight, compare_database_pick_second_name_by_weight;
    Prologue { address: 0x538bc0, bytes: &[0x81, 0xec, 0x74, 0x02, 0x00, 0x00] } => staff_normalise_player_attributes, compare_staff_normalise_player_attributes;
    Prologue { address: 0x54dc90, bytes: &[0x51, 0x8b, 0x44, 0x24, 0x14] } => db_files_read_clubs_v1, compare_db_files_read_clubs_v1;
    Prologue { address: 0x54e9a0, bytes: &[0x8b, 0x44, 0x24, 0x10, 0x53] } => db_files_read_players_v1, compare_db_files_read_players_v1;
    Prologue { address: 0x54f150, bytes: &[0x8b, 0x44, 0x24, 0x10, 0x53] } => db_files_read_non_players_v1, compare_db_files_read_non_players_v1;
    Prologue { address: 0x54ffd0, bytes: &[0x8b, 0x44, 0x24, 0x10, 0x53] } => db_files_read_comps_v1, compare_db_files_read_comps_v1;
    Prologue { address: 0x5453f0, bytes: &[0x6a, 0xff, 0x68, 0x5b, 0x79, 0x95, 0x00] } => nation_get_english_name, compare_nation_get_english_name;
    Prologue { address: 0x545770, bytes: &[0x6a, 0xff, 0x68, 0xdb, 0x79, 0x95, 0x00] } => club_get_english_short_name, compare_club_get_english_short_name;
    Prologue { address: 0x541e60, bytes: &[0xa1, 0x5c, 0x23, 0xae, 0x00] } => database_trim_senior_and_b_team_squads, compare_database_trim_senior_and_b_team_squads;
    Prologue { address: 0x544970, bytes: &[0x83, 0xec, 0x10, 0xa1, 0x5c, 0x23, 0xae, 0x00] } => database_merge_b_team_staff, compare_database_merge_b_team_staff;
    Prologue { address: 0x539750, bytes: &[0xa1, 0x50, 0x23, 0xae, 0x00] } => database_build_nation_name_index, compare_database_build_nation_name_index;
    Prologue { address: 0x53c190, bytes: &[0x81, 0xec, 0x08, 0x02, 0x00, 0x00] } => database_remove_staff_from_club, compare_database_remove_staff_from_club;
    Prologue { address: 0x53cd60, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => database_get_club_scouts, compare_database_get_club_scouts;
    Prologue { address: 0x547770, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x85, 0xc0] } => club_is_in_season, compare_club_is_in_season;
    Prologue { address: 0x549e50, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x85, 0xc0] } => club_is_in_major_nation, compare_club_is_in_major_nation;
    Prologue { address: 0x549ea0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x85, 0xc0] } => nation_is_major_nation, compare_nation_is_major_nation;
    Prologue { address: 0x549c90, bytes: &[0x8b, 0x54, 0x24, 0x04, 0x85, 0xd2] } => staff_clear_opinion_links, compare_staff_clear_opinion_links;
    Prologue { address: 0x549cf0, bytes: &[0x8b, 0x54, 0x24, 0x04, 0x56] } => staff_add_opinion_link, compare_staff_add_opinion_link;
    Prologue { address: 0x549d60, bytes: &[0x8b, 0x4c, 0x24, 0x04, 0x85, 0xc9] } => staff_remove_opinion_link, compare_staff_remove_opinion_link;
    Prologue { address: 0x53cb80, bytes: &[0x8b, 0x4c, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => database_get_club_assistant, compare_database_get_club_assistant;
    Prologue { address: 0x53cc30, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => database_get_club_coaches, compare_database_get_club_coaches;
    Prologue { address: 0x53c8d0, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => database_count_club_coaches, compare_database_count_club_coaches;
    Prologue { address: 0x5454c0, bytes: &[0x6a, 0xff, 0x68, 0x7b, 0x79, 0x95, 0x00] } => nation_get_english_three_letter_name, compare_nation_get_english_three_letter_name;
    Prologue { address: 0x5455a0, bytes: &[0x6a, 0xff, 0x68, 0x9b, 0x79, 0x95, 0x00] } => comp_get_english_name, compare_comp_get_english_name;
    Prologue { address: 0x5456a0, bytes: &[0x6a, 0xff, 0x68, 0xbb, 0x79, 0x95, 0x00] } => staff_comp_get_english_name, compare_staff_comp_get_english_name;
    Prologue { address: 0x545990, bytes: &[0x6a, 0xff, 0x68, 0x1b, 0x7a, 0x95, 0x00] } => stadium_get_english_name, compare_stadium_get_english_name;
    Prologue { address: 0x535930, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x85, 0xc0] } => database_language_file_exists, compare_database_language_file_exists;
    Prologue { address: 0x544f30, bytes: &[0x56, 0x8b, 0xf1, 0x8b, 0x46, 0x20] } => eng_lng_free_tables, compare_eng_lng_free_tables;
    Prologue { address: 0x527240, bytes: &[0x81, 0xec, 0x18, 0x02, 0x00, 0x00] } => database_generate_extra_non_playing_staff, compare_database_generate_extra_non_playing_staff;
    Prologue { address: 0x5267c0, bytes: &[0x81, 0xec, 0x28, 0x06, 0x00, 0x00] } => database_trim_club_table, compare_database_trim_club_table;
    Prologue { address: 0x539ae0, bytes: &[0x8b, 0x0d, 0x7c, 0x23, 0xae, 0x00] } => database_prune_comp_history_table, compare_database_prune_comp_history_table;
    Prologue { address: 0x5315a0, bytes: &[0x81, 0xec, 0x04, 0x06, 0x00, 0x00] } => resolve_database_indexes, compare_resolve_database_indexes;
    Prologue { address: 0x53db40, bytes: &[0x8a, 0x44, 0x24, 0x04, 0x84, 0xc0] } => database_sort_clubs_by_reputation, compare_database_sort_clubs_by_reputation;
    Prologue { address: 0x549100, bytes: &[0x51, 0x56, 0x8b, 0x74, 0x24, 0x0c] } => club_has_enough_defenders, compare_club_has_enough_defenders;
    Prologue { address: 0x549200, bytes: &[0x51, 0x56, 0x8b, 0x74, 0x24, 0x0c] } => club_has_enough_midfielders, compare_club_has_enough_midfielders;
    Prologue { address: 0x5492c0, bytes: &[0x51, 0x56, 0x8b, 0x74, 0x24, 0x0c] } => club_has_enough_strikers, compare_club_has_enough_strikers;
    Prologue { address: 0x5493c0, bytes: &[0x51, 0x56, 0x8b, 0x74, 0x24, 0x0c] } => club_has_enough_hard_workers, compare_club_has_enough_hard_workers;
    Prologue { address: 0x549470, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x55] } => club_has_fit_goalkeeper, compare_club_has_fit_goalkeeper;
    Prologue { address: 0x549850, bytes: &[0x51, 0x56, 0x8b, 0x74, 0x24, 0x0c] } => club_has_ball_winning_midfielders, compare_club_has_ball_winning_midfielders;
    Prologue { address: 0x5494d0, bytes: &[0x51, 0x56, 0x8b, 0x74, 0x24, 0x0c] } => club_lacks_aerial_strikers, compare_club_lacks_aerial_strikers;
    Prologue { address: 0x5495b0, bytes: &[0x51, 0x56, 0x8b, 0x74, 0x24, 0x0c] } => club_lacks_aerial_defenders, compare_club_lacks_aerial_defenders;
    Prologue { address: 0x549690, bytes: &[0x51, 0x56, 0x8b, 0x74, 0x24, 0x0c] } => club_lacks_fast_defenders, compare_club_lacks_fast_defenders;
    Prologue { address: 0x549770, bytes: &[0x51, 0x56, 0x8b, 0x74, 0x24, 0x0c] } => club_has_fast_striker, compare_club_has_fast_striker;
    Prologue { address: 0x549940, bytes: &[0x51, 0x56, 0x8b, 0x74, 0x24, 0x0c] } => club_has_two_creative_midfielders, compare_club_has_two_creative_midfielders;
    Prologue { address: 0x549a30, bytes: &[0x81, 0xec, 0xcc, 0x00, 0x00, 0x00] } => club_squad_strength_band, compare_club_squad_strength_band;
    Prologue { address: 0x545e90, bytes: &[0x83, 0xec, 0x08, 0x53, 0x55] } => club_reputation_for_staff, compare_club_reputation_for_staff;
    Prologue { address: 0x547c90, bytes: &[0x8b, 0x4c, 0x24, 0x04, 0x32, 0xc0] } => staff_count_disliked_staff_at_club, compare_staff_count_disliked_staff_at_club;
    Prologue { address: 0x547ce0, bytes: &[0x8b, 0x4c, 0x24, 0x04, 0x32, 0xc0] } => staff_count_favoured_staff_at_club, compare_staff_count_favoured_staff_at_club;
    Prologue { address: 0x547f50, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x33, 0xd2] } => staff_remove_favoured_club, compare_staff_remove_favoured_club;
    Prologue { address: 0x547bd0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x33, 0xd2] } => club_remove_favoured_staff, compare_club_remove_favoured_staff;
    Prologue { address: 0x547c10, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x33, 0xd2] } => staff_remove_favoured_staff, compare_staff_remove_favoured_staff;
    Prologue { address: 0x547c50, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x33, 0xd2] } => staff_remove_disliked_staff, compare_staff_remove_disliked_staff;
    Prologue { address: 0x548090, bytes: &[0x8b, 0x44, 0x24, 0x0c, 0x56] } => staff_get_favoured_staff, compare_staff_get_favoured_staff;
    Prologue { address: 0x547870, bytes: &[0x56, 0x8b, 0x74, 0x24, 0x08] } => club_add_favoured_staff, compare_club_add_favoured_staff;
    Prologue { address: 0x547940, bytes: &[0x56, 0x8b, 0x74, 0x24, 0x08] } => club_add_disliked_staff, compare_club_add_disliked_staff;
    Prologue { address: 0x547a10, bytes: &[0x53, 0x8b, 0x5c, 0x24, 0x08] } => staff_add_favoured_staff, compare_staff_add_favoured_staff;
    Prologue { address: 0x547e70, bytes: &[0x56, 0x57, 0x8b, 0x7c, 0x24, 0x0c] } => staff_add_disliked_club, compare_staff_add_disliked_club;
    Prologue { address: 0x547f90, bytes: &[0x56, 0x8b, 0x74, 0x24, 0x08] } => staff_fade_staff_relationships, compare_staff_fade_staff_relationships;
    Prologue { address: 0x5480f0, bytes: &[0x8b, 0x44, 0x24, 0x0c, 0x56] } => staff_get_disliked_staff, compare_staff_get_disliked_staff;
    Prologue { address: 0x5481a0, bytes: &[0x51, 0x8b, 0x4c, 0x24, 0x08] } => staff_get_natural_position_mask, compare_staff_get_natural_position_mask;
    Prologue { address: 0x548150, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x53] } => club_count_squad_with_position_mask, compare_club_count_squad_with_position_mask;
    Prologue { address: 0x548da0, bytes: &[0x83, 0xec, 0x10, 0x53, 0x8b, 0x5c, 0x24, 0x18] } => staff_pick_favoured_team_mate, compare_staff_pick_favoured_team_mate;
    Prologue { address: 0x5487b0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x85, 0xc0] } => club_nation_is_not_argentina, compare_club_nation_is_not_argentina;
    Prologue { address: 0x5487d0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x85, 0xc0] } => club_nation_not_argentina_france_croatia, compare_club_nation_not_argentina_france_croatia;
    Prologue { address: 0x53d900, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x8b, 0x0d, 0x1c, 0xf4, 0x9c, 0x00] } => database_is_melanesian_nation, compare_database_is_melanesian_nation;
    Prologue { address: 0x53d940, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x8b, 0x0d, 0xd0, 0xf4, 0x9c, 0x00] } => database_is_polynesian_nation, compare_database_is_polynesian_nation;
    Prologue { address: 0x53ac40, bytes: &[0x0f, 0xbe, 0x44, 0x24, 0x04] } => database_get_coaching_job_name, compare_database_get_coaching_job_name;
    Prologue { address: 0x540340, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => database_format_years_between_dates, compare_database_format_years_between_dates;
    Prologue { address: 0x540080, bytes: &[0x81, 0xec, 0xa0, 0x0f, 0x00, 0x00] } => database_compare_clubs_by_short_name, compare_database_compare_clubs_by_short_name;
    Prologue { address: 0x540270, bytes: &[0x81, 0xec, 0xa0, 0x0f, 0x00, 0x00] } => database_compare_staff_by_full_name, compare_database_compare_staff_by_full_name;
    Prologue { address: 0x53e990, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => database_staff_matches_names, compare_database_staff_matches_names;
    Prologue { address: 0x53fd40, bytes: &[0x81, 0xec, 0x08, 0x02, 0x00, 0x00] } => database_compare_clubs_by_latitude, compare_database_compare_clubs_by_latitude;
    Prologue { address: 0x53fe50, bytes: &[0x81, 0xec, 0x08, 0x02, 0x00, 0x00] } => database_compare_clubs_by_longitude, compare_database_compare_clubs_by_longitude;
    Prologue { address: 0x53e140, bytes: &[0x8b, 0x44, 0x24, 0x10, 0x81, 0xec, 0x08, 0x02, 0x00, 0x00] } => database_format_name_from_ids, compare_database_format_name_from_ids;
    Prologue { address: 0x53eab0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => database_format_official_name, compare_database_format_official_name;
    Prologue { address: 0x5482f0, bytes: &[0x53, 0x57, 0x8b, 0x7c, 0x24, 0x10] } => position_mask_get_text, compare_position_mask_get_text;
    Prologue { address: 0x5461b0, bytes: &[0x81, 0xec, 0x04, 0x02, 0x00, 0x00] } => write_position_name, compare_write_position_name;
    Prologue { address: 0x5465a0, bytes: &[0x81, 0xec, 0x04, 0x02, 0x00, 0x00] } => write_player_description, compare_write_player_description;
    Prologue { address: 0x548810, bytes: &[0x83, 0xec, 0x08, 0x53, 0x8b, 0x5c, 0x24, 0x10] } => staff_is_best_favoured_candidate, compare_staff_is_best_favoured_candidate;
    Prologue { address: 0x546ab0, bytes: &[0x83, 0xec, 0x0c, 0x8b, 0x44, 0x24, 0x14] } => staff_get_player_strength_flags, compare_staff_get_player_strength_flags;
    Prologue { address: 0x5470d0, bytes: &[0x83, 0xec, 0x0c, 0x8b, 0x44, 0x24, 0x14] } => staff_get_player_weakness_flags, compare_staff_get_player_weakness_flags;
    Prologue { address: 0x53c9b0, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => database_count_club_physios, compare_database_count_club_physios;
    Prologue { address: 0x5448c0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x04, 0x02, 0x00, 0x00] } => club_is_b_team, compare_club_is_b_team;
    Prologue { address: 0x544c70, bytes: &[0x8b, 0x4c, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => club_collect_non_playing_staff, compare_club_collect_non_playing_staff;
    Prologue { address: 0x543280, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x56] } => staff_is_player_attribute_visible, compare_staff_is_player_attribute_visible;
    Prologue { address: 0x543440, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => staff_get_player_attribute_for_viewer, compare_staff_get_player_attribute_for_viewer;
    Prologue { address: 0x543b10, bytes: &[0x8a, 0x4c, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => database_get_non_player_attribute_name, compare_database_get_non_player_attribute_name;
    Prologue { address: 0x53ff60, bytes: &[0x81, 0xec, 0xa0, 0x0f, 0x00, 0x00] } => database_compare_staff_by_surname, compare_database_compare_staff_by_surname;
    Prologue { address: 0x53ffe0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x8b, 0x4c, 0x24, 0x08] } => database_compare_clubs_by_reputation, compare_database_compare_clubs_by_reputation;
    Prologue { address: 0x53d7d0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x8b, 0x40, 0x53] } => database_club_nation_is_eec, compare_database_club_nation_is_eec;
    Prologue { address: 0x545dc0, bytes: &[0xb0, 0x01, 0xb9, 0x04, 0x21, 0xae, 0x00] } => get_regions_sorted_by_name, compare_get_regions_sorted_by_name;
    Prologue { address: 0x545e00, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x90, 0x01, 0x00, 0x00] } => compare_region_names, compare_compare_region_names;
    Prologue { address: 0x549dd0, bytes: &[0x8b, 0x4c, 0x24, 0x04, 0x33, 0xc0] } => staff_get_opinion_link_staff, compare_staff_get_opinion_link_staff;
    Prologue { address: 0x53e2e0, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => database_format_name_from_strings, compare_database_format_name_from_strings;
    Prologue { address: 0x53ad00, bytes: &[0x0f, 0xbe, 0x44, 0x24, 0x04] } => database_get_job_name, compare_database_get_job_name;
    Prologue { address: 0x53cee0, bytes: &[0x0f, 0xbe, 0x44, 0x24, 0x04] } => database_get_language_name, compare_database_get_language_name;
    Prologue { address: 0x542100, bytes: &[0x8b, 0x44, 0x24, 0x10, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => club_get_national_team_name, compare_club_get_national_team_name;
    Prologue { address: 0x5428a0, bytes: &[0x8a, 0x4c, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => database_get_player_attribute_name, compare_database_get_player_attribute_name;
    Prologue { address: 0x53ee50, bytes: &[0xb8, 0xa4, 0x11, 0x00, 0x00] } => database_format_player_positions, compare_database_format_player_positions;
    Prologue { address: 0x616f20, bytes: &[0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => get_region_name, compare_get_region_name;
    Prologue { address: 0x6177f0, bytes: &[0x55, 0x8b, 0x6c, 0x24, 0x0c] } => get_continent_adjective, compare_get_continent_adjective;
    Prologue { address: 0x601a40, bytes: &[0x81, 0xec, 0x20, 0x02, 0x00, 0x00] } => set_key_ids, compare_set_key_ids;
    Prologue { address: 0x603834, bytes: &[0x81, 0xec, 0x20, 0x02, 0x00, 0x00] } => set_key_ids_1080, compare_set_key_ids_1080;
    Prologue { address: 0x535a80, bytes: &[0x81, 0xec, 0x40, 0x0e, 0x00, 0x00] } => database_player_initialisation, compare_database_player_initialisation;
    Prologue { address: 0x6181d0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x55] } => nation_get_nationality_adjective, compare_nation_get_nationality_adjective;
    Prologue { address: 0x6827d0, bytes: &[0x83, 0xec, 0x4c, 0x53, 0x55] } => sort_table, compare_sort_table;
    Prologue { address: 0x682b30, bytes: &[0x81, 0xec, 0x10, 0x02, 0x00, 0x00] } => build_h2h_table, compare_build_h2h_table;
    Prologue { address: 0x687970, bytes: &[0x51, 0x53, 0x55, 0x8b, 0x6c, 0x24, 0x10] } => free_table, compare_free_table;
    Prologue { address: 0x689ad0, bytes: &[0x53, 0x56, 0x8b, 0xf1, 0x57] } => league_check_complete, compare_league_check_complete;
    Prologue { address: 0x683010, bytes: &[0xa1, 0x38, 0x2a, 0xae, 0x00] } => pay_tv_rights, compare_pay_tv_rights;
    Prologue { address: 0x684640, bytes: &[0x83, 0xec, 0x0c, 0x53, 0x55] } => league_record_season_honours, compare_league_record_season_honours;
    Prologue { address: 0x4afce0, bytes: &[0xa1, 0x7c, 0x23, 0xae, 0x00] } => comp_util_add_history_record, compare_comp_util_add_history_record;
    Prologue { address: 0x4b02e0, bytes: &[0x55, 0x8b, 0xec, 0x83, 0xe4, 0xf8] } => comp_util_get_league_clash_type, compare_comp_util_get_league_clash_type;
    Prologue { address: 0x4c0720, bytes: &[0x81, 0xec, 0xf0, 0x01, 0x00, 0x00] } => comp_util_get_club_strength_score, compare_comp_util_get_club_strength_score;
    Prologue { address: 0x4afd80, bytes: &[0x81, 0xec, 0x1c, 0x02, 0x00, 0x00] } => comp_util_get_latest_placed_club, compare_comp_util_get_latest_placed_club;
    Prologue { address: 0x5c88f0, bytes: &[0x53, 0x55, 0x56, 0x57, 0x8b, 0xf9] } => find_friendly_match, compare_find_friendly_match;
    Prologue { address: 0x5ae6a0, bytes: &[0x8b, 0x54, 0x24, 0x0c, 0x8b, 0x44, 0x24, 0x04] } => get_fixture_list_for_date, compare_get_fixture_list_for_date;
    Prologue { address: 0x5ac920, bytes: &[0x83, 0xec, 0x14, 0x8b, 0x44, 0x24, 0x18] } => find_next_fixture, compare_find_next_fixture;
    Prologue { address: 0x5ae870, bytes: &[0x83, 0xec, 0x14, 0x8b, 0x44, 0x24, 0x18] } => is_stadium_in_use, compare_is_stadium_in_use;
    Prologue { address: 0x5cbdd0, bytes: &[0x66, 0x0f, 0xbe, 0x01, 0x8b, 0x54, 0x24, 0x04] } => friendly_international_date_resolve, compare_friendly_international_date_resolve;
    Prologue { address: 0x774f30, bytes: &[0x83, 0xec, 0x08, 0x56, 0x57] } => end_duty_for_competition, compare_end_duty_for_competition;
    Prologue { address: 0x774f90, bytes: &[0x83, 0xec, 0x08, 0x56, 0x57] } => end_duty_for_nation_in_competition, compare_end_duty_for_nation_in_competition;
    Prologue { address: 0x7774b0, bytes: &[0xa1, 0x50, 0x23, 0xae, 0x00] } => prune_club_call_up_records, compare_prune_club_call_up_records;
    Prologue { address: 0x776250, bytes: &[0x83, 0xec, 0x08, 0x53, 0x8a, 0x5c, 0x24, 0x14] } => add_player_to_squad, compare_add_player_to_squad;
    Prologue { address: 0x7762f0, bytes: &[0x8a, 0x44, 0x24, 0x08, 0x56] } => national_teams_remove_player_from_squad, compare_national_teams_remove_player_from_squad;
    Prologue { address: 0x76d880, bytes: &[0x51, 0x56, 0x8b, 0x74, 0x24, 0x0c] } => release_overage_u21_players, compare_release_overage_u21_players;
    Prologue { address: 0x775000, bytes: &[0x8b, 0x54, 0x24, 0x04, 0x56] } => clear_current_competition, compare_clear_current_competition;
    Prologue { address: 0x775070, bytes: &[0x51, 0xa1, 0x50, 0x23, 0xae, 0x00] } => release_competitions_of_continent, compare_release_competitions_of_continent;
    Prologue { address: 0x775f90, bytes: &[0x83, 0xec, 0x18, 0x8b, 0x44, 0x24, 0x28] } => call_up_replacement, compare_call_up_replacement;
    Prologue { address: 0x76abf0, bytes: &[0x83, 0xec, 0x28, 0xa1, 0x50, 0x23, 0xae, 0x00] } => build_selection_pools, compare_build_selection_pools;
    Prologue { address: 0x778ed0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x8b, 0x08] } => compare_by_rating, compare_compare_by_rating;
    Prologue { address: 0x778ef0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x56] } => compare_by_club, compare_compare_by_club;
    Prologue { address: 0x5c4cb0, bytes: &[0x83, 0xec, 0x1c, 0x89, 0x4c, 0x24, 0x00] } => find_friendly_opponent, compare_find_friendly_opponent;
    Prologue { address: 0x5c5040, bytes: &[0x83, 0xec, 0x20, 0x8b, 0x44, 0x24, 0x24] } => find_tour_opponent, compare_find_tour_opponent;
    Prologue { address: 0x685f60, bytes: &[0x83, 0xec, 0x0c, 0x8b, 0xd1] } => apply_result_to_table, compare_apply_result_to_table;
    Prologue { address: 0x68aad0, bytes: &[0x8b, 0x44, 0x24, 0x08, 0x53] } => points_for_result, compare_points_for_result;
    Prologue { address: 0x683410, bytes: &[0xa1, 0xfc, 0x79, 0xb6, 0x00] } => league_compare_table_rows_h2h, compare_league_compare_table_rows_h2h;
    Prologue { address: 0x685cf0, bytes: &[0x8b, 0x54, 0x24, 0x04, 0x56] } => league_compare_dates, compare_league_compare_dates;
    Prologue { address: 0x68ab70, bytes: &[0x53, 0x56, 0x8b, 0x74, 0x24, 0x10] } => find_club_row, compare_find_club_row;
    Prologue { address: 0x68abd0, bytes: &[0x53, 0x56, 0x8b, 0xb1, 0xb1, 0x00, 0x00, 0x00] } => contains_club, compare_contains_club;
    Prologue { address: 0x448230, bytes: &[0x56, 0x57, 0x8b, 0x7c, 0x24, 0x0c] } => rating_record_assign, compare_rating_record_assign;
    Prologue { address: 0x4484d0, bytes: &[0x56, 0x57, 0x8b, 0x7c, 0x24, 0x0c] } => man_of_match_record_assign, compare_man_of_match_record_assign;
    Prologue { address: 0x448500, bytes: &[0x56, 0x57, 0x8b, 0x7c, 0x24, 0x0c] } => discipline_record_assign, compare_discipline_record_assign;
    Prologue { address: 0x448530, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x56] } => count_record_assign, compare_count_record_assign;
    Prologue { address: 0x4485b0, bytes: &[0x56, 0x57, 0x8b, 0x7c, 0x24, 0x0c] } => fans_player_record_assign, compare_fans_player_record_assign;
    Prologue { address: 0x450c60, bytes: &[0x56, 0x8b, 0xf1, 0x8d, 0x4e, 0x11] } => tally_record_construct, compare_tally_record_construct;
    Prologue { address: 0x450d30, bytes: &[0x56, 0x8b, 0xf1, 0x8d, 0x4e, 0x12] } => count_record_construct, compare_count_record_construct;
    Prologue { address: 0x448550, bytes: &[0x83, 0xec, 0x08, 0x56, 0x57] } => transfer_fee_record_assign, compare_transfer_fee_record_assign;
    Prologue { address: 0x451570, bytes: &[0x56, 0x57, 0x8b, 0x7c, 0x24, 0x0c] } => goals_in_match_record_assign, compare_goals_in_match_record_assign;
    Prologue { address: 0x786010, bytes: &[0x83, 0xec, 0x08, 0xba, 0x32, 0x00, 0x00, 0x00] } => news_operator, compare_news_operator;
    Prologue { address: 0x8cb940, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x83, 0xec, 0x08] } => past_transfer_copy_construct, compare_past_transfer_copy_construct;
    Prologue { address: 0x8cbf40, bytes: &[0x83, 0xec, 0x08, 0x8d, 0x44, 0x24, 0x00] } => future_transfer_assign, compare_future_transfer_assign;
    Prologue { address: 0x7a8b70, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x83, 0xec, 0x08] } => regen_queue_entry_assign, compare_regen_queue_entry_assign;
    Prologue { address: 0x842080, bytes: &[0x8b, 0x01, 0x83, 0xec, 0x08] } => is_on_shortlist, compare_is_on_shortlist;
    Prologue { address: 0x7a6e60, bytes: &[0x51, 0xa1, 0x64, 0x23, 0xae, 0x00] } => player_regen_pick_staff_for_non_player_data, compare_player_regen_pick_staff_for_non_player_data;
    Prologue { address: 0x556c70, bytes: &[0x83, 0xec, 0x08, 0x55, 0x56] } => discipline_operator, compare_discipline_operator;
    Prologue { address: 0x8626e0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x56] } => set_default_wage, compare_set_default_wage;
    Prologue { address: 0x62c510, bytes: &[0x56, 0x8b, 0x74, 0x24, 0x08] } => get_match_condition, compare_get_match_condition;
    Prologue { address: 0x85fa00, bytes: &[0xa1, 0x94, 0x2c, 0xae, 0x00] } => days_to_end_date, compare_days_to_end_date;
    Prologue { address: 0x867520, bytes: &[0x56, 0x8b, 0xf1, 0x8b, 0x06] } => clear_all_grievances, compare_clear_all_grievances;
    Prologue { address: 0x867240, bytes: &[0x56, 0x8b, 0xf1, 0x57, 0x8b, 0x06] } => check_transfer_request_withdrawal, compare_check_transfer_request_withdrawal;
    Prologue { address: 0x4cbb60, bytes: &[0x51, 0x53, 0x55, 0x56, 0x8b, 0x74, 0x24, 0x14] } => get_squad_placement, compare_get_squad_placement;
    Prologue { address: 0x4cc0b0, bytes: &[0x53, 0x8b, 0x5c, 0x24, 0x08] } => move_to_first_team, compare_move_to_first_team;
    Prologue { address: 0x4cc0f0, bytes: &[0x53, 0x8b, 0x5c, 0x24, 0x08] } => move_to_reserves, compare_move_to_reserves;
    Prologue { address: 0x4cc810, bytes: &[0x83, 0xec, 0x08, 0x53, 0x55] } => contract_manager_operator, compare_contract_manager_operator;
    Prologue { address: 0x4cb0a0, bytes: &[0x56, 0x8b, 0x74, 0x24, 0x10] } => contract_manager_raise_match_morale, compare_contract_manager_raise_match_morale;
    Prologue { address: 0x4ca1c0, bytes: &[0x51, 0x8b, 0xc1, 0x53, 0x33, 0xdb] } => remove_talks_records, compare_remove_talks_records;
    Prologue { address: 0x7549b0, bytes: &[0x51, 0x8b, 0xc1, 0x33, 0xd2] } => remove_transfer_interest, compare_remove_transfer_interest;
    Prologue { address: 0x7f8ee0, bytes: &[0x83, 0xec, 0x1c, 0x53, 0x8b, 0xd9] } => remove_scout_request, compare_remove_scout_request;
    Prologue { address: 0x5fff30, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x83, 0xec, 0x24] } => remove_nickname_from_all, compare_remove_nickname_from_all;
    Prologue { address: 0x7a8690, bytes: &[0x8b, 0x54, 0x24, 0x04, 0x53] } => remove_retirement, compare_remove_retirement;
    Prologue { address: 0x7a8050, bytes: &[0xa1, 0x54, 0x1f, 0xde, 0x00] } => player_regen_check_unemployed_retirement, compare_player_regen_check_unemployed_retirement;
    Prologue { address: 0x8c7470, bytes: &[0x53, 0x56, 0x8b, 0x74, 0x24, 0x0c] } => transfer_manager_offer_needs_work_permit, compare_transfer_manager_offer_needs_work_permit;
    Prologue { address: 0x8c7330, bytes: &[0x51, 0x55, 0x56, 0x8b, 0xf1] } => update_work_permits, compare_update_work_permits;
    Prologue { address: 0x7ae950, bytes: &[0x56, 0x8b, 0xf1, 0x8b, 0x86, 0x80, 0x01, 0x00, 0x00] } => get_scout, compare_get_scout;
    Prologue { address: 0x690c30, bytes: &[0x8b, 0x54, 0x24, 0x04, 0x53] } => get_current_run, compare_get_current_run;
    Prologue { address: 0x450d40, bytes: &[0x56, 0x8b, 0xf1, 0x8d, 0x4e, 0x10] } => record_holder_construct, compare_record_holder_construct;
    Prologue { address: 0x72b940, bytes: &[0x83, 0xec, 0x50, 0x53, 0x56] } => pick_rival_manager_id, compare_pick_rival_manager_id;
    Prologue { address: 0x4485d0, bytes: &[0x6a, 0xff, 0x68, 0x5b, 0x54, 0x95, 0x00] } => transfer_fee_record_construct, compare_transfer_fee_record_construct;
    Prologue { address: 0x448620, bytes: &[0x6a, 0xff, 0x68, 0x7b, 0x54, 0x95, 0x00] } => transfer_fee_record_construct_2, compare_transfer_fee_record_construct_2;
    Prologue { address: 0x448670, bytes: &[0x6a, 0xff, 0x68, 0x9b, 0x54, 0x95, 0x00] } => run_record_construct, compare_run_record_construct;
    Prologue { address: 0x450ad0, bytes: &[0x6a, 0xff, 0x68, 0x9b, 0x56, 0x95, 0x00] } => won_run_record_construct, compare_won_run_record_construct;
    Prologue { address: 0x450b20, bytes: &[0x6a, 0xff, 0x68, 0xbb, 0x56, 0x95, 0x00] } => lost_run_record_construct, compare_lost_run_record_construct;
    Prologue { address: 0x450b70, bytes: &[0x6a, 0xff, 0x68, 0xdb, 0x56, 0x95, 0x00] } => unbeaten_run_record_construct, compare_unbeaten_run_record_construct;
    Prologue { address: 0x450bc0, bytes: &[0x6a, 0xff, 0x68, 0xfb, 0x56, 0x95, 0x00] } => winless_run_record_construct, compare_winless_run_record_construct;
    Prologue { address: 0x450c10, bytes: &[0x6a, 0xff, 0x68, 0x1b, 0x57, 0x95, 0x00] } => goals_in_match_record_construct, compare_goals_in_match_record_construct;
    Prologue { address: 0x450c70, bytes: &[0x6a, 0xff, 0x68, 0x43, 0x57, 0x95, 0x00] } => youngest_player_record_construct, compare_youngest_player_record_construct;
    Prologue { address: 0x8e05a0, bytes: &[0x56, 0x57, 0x8b, 0x7c, 0x24, 0x0c] } => write_deadline_news_text, compare_write_deadline_news_text;
    Prologue { address: 0x8d0530, bytes: &[0x53, 0x8b, 0x5c, 0x24, 0x0c] } => can_end_loan, compare_can_end_loan;
    Prologue { address: 0x5d5a20, bytes: &[0xdd, 0x44, 0x24, 0x04, 0xdd, 0x44, 0x24, 0x0c] } => pow_double, compare_pow_double;
    Prologue { address: 0x4cf560, bytes: &[0x56, 0x8b, 0xf1, 0x8b, 0x46, 0x38] } => remove_held_offers, compare_remove_held_offers;
    Prologue { address: 0x5563f0, bytes: &[0x81, 0xec, 0x10, 0x02, 0x00, 0x00] } => hold_bans_for_loan, compare_hold_bans_for_loan;
    Prologue { address: 0x8cb6d0, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => fill_vacated_staff_job, compare_fill_vacated_staff_job;
    Prologue { address: 0x786520, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => send_to_nation_club_managers, compare_send_to_nation_club_managers;
    Prologue { address: 0x7aefd0, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => get_scout_assignment, compare_get_scout_assignment;
    Prologue { address: 0x521e00, bytes: &[0x56, 0x8b, 0xf1, 0x57, 0x8a, 0x86, 0xb1, 0x00, 0x00, 0x00] } => cup_check_complete, compare_cup_check_complete;
    Prologue { address: 0x450cd0, bytes: &[0x6a, 0xff, 0x68, 0x63, 0x57, 0x95, 0x00] } => oldest_player_record_construct, compare_oldest_player_record_construct;
    Prologue { address: 0x8c7260, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x83, 0xec, 0x10] } => find_work_permit, compare_find_work_permit;
    Prologue { address: 0x84b180, bytes: &[0x56, 0x8b, 0x74, 0x24, 0x08] } => passes_work_permit_check, compare_passes_work_permit_check;
    Prologue { address: 0x5567d0, bytes: &[0x53, 0x8b, 0x5c, 0x24, 0x08] } => drop_held_bans, compare_drop_held_bans;
    Prologue { address: 0x4ccbf0, bytes: &[0x56, 0x57, 0x8b, 0xf9, 0x8b, 0x47, 0x34] } => remove_pending_offers_for_staff, compare_remove_pending_offers_for_staff;
    Prologue { address: 0x4cc930, bytes: &[0x56, 0x57, 0x8b, 0xf9, 0x8b, 0x47, 0x34] } => schedule_decision_for_transfer, compare_schedule_decision_for_transfer;
    Prologue { address: 0x786780, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => send_to_all_managers, compare_send_to_all_managers;
    Prologue { address: 0x7862d0, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => send_to_division_managers, compare_send_to_division_managers;
    Prologue { address: 0x7863f0, bytes: &[0x81, 0xec, 0x04, 0x02, 0x00, 0x00] } => send_to_nation_managers, compare_send_to_nation_managers;
    Prologue { address: 0x4486c0, bytes: &[0x53, 0x56, 0x57, 0x8b, 0x7c, 0x24, 0x10] } => offer_breaks_fee_record, compare_offer_breaks_fee_record;
    Prologue { address: 0x62c590, bytes: &[0x56, 0x8b, 0x74, 0x24, 0x08] } => get_condition, compare_get_condition;
    Prologue { address: 0x8cf7e0, bytes: &[0x83, 0xec, 0x2c, 0x33, 0xc0] } => get_selection_limit, compare_get_selection_limit;
    Prologue { address: 0x5fe8d0, bytes: &[0x51, 0x53, 0x55, 0x56, 0x33, 0xdb] } => clear_invalid_watched_fixtures, compare_clear_invalid_watched_fixtures;
    Prologue { address: 0x7f9690, bytes: &[0x51, 0x53, 0x8b, 0x5c, 0x24, 0x10] } => collect_club_scout_targets, compare_collect_club_scout_targets;
    Prologue { address: 0x7ae8a0, bytes: &[0x53, 0x56, 0x8b, 0xf1, 0x83, 0xcb, 0xff] } => set_scout, compare_set_scout;
    Prologue { address: 0x6951e0, bytes: &[0x53, 0x56, 0x8b, 0x74, 0x24, 0x0c] } => update_board_after_fixture, compare_update_board_after_fixture;
    Prologue { address: 0x55ca00, bytes: &[0x53, 0x56, 0x8b, 0x74, 0x24, 0x0c] } => process_match, compare_process_match;
    Prologue { address: 0x8420e0, bytes: &[0x83, 0xec, 0x08, 0x53, 0x8b, 0x5c, 0x24, 0x10] } => collect_club_shortlist_entries, compare_collect_club_shortlist_entries;
    Prologue { address: 0x8cc930, bytes: &[0x51, 0x53, 0x8b, 0xd9, 0x8b, 0x4c, 0x24, 0x0c] } => find_highest_offer_for_player, compare_find_highest_offer_for_player;
    Prologue { address: 0x452710, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => get_all_time_record, compare_get_all_time_record;
    Prologue { address: 0x4528d0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => get_season_record, compare_get_season_record;
    Prologue { address: 0x452cb0, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => queue_record_news, compare_queue_record_news;
    Prologue { address: 0x7a81a0, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => add_spare_non_player_record, compare_add_spare_non_player_record;
    Prologue { address: 0x7a8510, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => add_retirement, compare_add_retirement;
    Prologue { address: 0x7a87d0, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => queue_staff_for_regen, compare_queue_staff_for_regen;
    Prologue { address: 0x4cd030, bytes: &[0x83, 0xec, 0x0c, 0x53, 0x8b, 0x5c, 0x24, 0x14] } => trigger_relegation_clauses, compare_trigger_relegation_clauses;
    Prologue { address: 0x4cca70, bytes: &[0x8b, 0x44, 0x24, 0x08, 0x53] } => remove_contract_offer, compare_remove_contract_offer;
    Prologue { address: 0x55d270, bytes: &[0x53, 0x55, 0x56, 0x8b, 0x74, 0x24, 0x10] } => is_player_suspended, compare_is_player_suspended;
    Prologue { address: 0x8d2fa0, bytes: &[0x8b, 0x15, 0x84, 0xf6, 0x9c, 0x00] } => get_fixture_rules_scope, compare_get_fixture_rules_scope;
    Prologue { address: 0x8cf5e0, bytes: &[0x83, 0xec, 0x08, 0x53, 0x55] } => get_ineligibility_code, compare_get_ineligibility_code;
    Prologue { address: 0x5c6740, bytes: &[0x6a, 0xff, 0x68, 0xf8, 0xa0, 0x95, 0x00] } => find_scheduled_friendly_slot, compare_find_scheduled_friendly_slot;
    Prologue { address: 0x786640, bytes: &[0x81, 0xec, 0x04, 0x02, 0x00, 0x00] } => send_to_continent_managers, compare_send_to_continent_managers;
    Prologue { address: 0x786860, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => send_to_comp_region, compare_send_to_comp_region;
    Prologue { address: 0x784eb0, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => is_important_for, compare_is_important_for;
    Prologue { address: 0x785a20, bytes: &[0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => get_slot_string, compare_get_slot_string;
    Prologue { address: 0x74ae60, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] } => post_financial_trouble_news, compare_post_financial_trouble_news;
    Prologue { address: 0x7f9190, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => can_club_scout_nation, compare_can_club_scout_nation;
    Prologue { address: 0x7c6830, bytes: &[0x83, 0xec, 0x08, 0x53, 0x56] } => get_language_fluency, compare_get_language_fluency;
    Prologue { address: 0x86fb80, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => rewind_staff_history, compare_rewind_staff_history;
    Prologue { address: 0x5b2ba0, bytes: &[0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x1c, 0x02, 0x00, 0x00] } => set_slot_position, compare_set_slot_position;
    Prologue { address: 0x8cdc90, bytes: &[0x6a, 0xff, 0x68, 0x18, 0x3f, 0x96, 0x00] } => club_has_offer_for_player, compare_club_has_offer_for_player;
    Prologue { address: 0x8cdd80, bytes: &[0x6a, 0xff, 0x68, 0x38, 0x3f, 0x96, 0x00] } => find_club_offer_for_player, compare_find_club_offer_for_player;
    Prologue { address: 0x8cd150, bytes: &[0x6a, 0xff, 0x68, 0xd8, 0x3e, 0x96, 0x00] } => player_has_pending_offer, compare_player_has_pending_offer;
    Prologue { address: 0x8cde70, bytes: &[0x6a, 0xff, 0x68, 0x58, 0x3f, 0x96, 0x00] } => is_player_under_offer, compare_is_player_under_offer;
    Prologue { address: 0x8d0a70, bytes: &[0x6a, 0xff, 0x68, 0x5b, 0x40, 0x96, 0x00] } => player_club_uses_bosman, compare_player_club_uses_bosman;
    Prologue { address: 0x8cdb80, bytes: &[0x56, 0x8b, 0x74, 0x24, 0x08] } => check_board_veto, compare_check_board_veto;
    Prologue { address: 0x8cb810, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => add_past_transfer, compare_add_past_transfer;
    Prologue { address: 0x8c7070, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => add_work_permit, compare_add_work_permit;
    Prologue { address: 0x8cac40, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => add_first_team_appearance_money, compare_add_first_team_appearance_money;
    Prologue { address: 0x4c79a0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => get_free_staff_contract_space, compare_get_free_staff_contract_space;
    Prologue { address: 0x4c7950, bytes: &[0x56, 0x8b, 0xf1, 0x6a, 0x01] } => add_staff_contract, compare_add_staff_contract;
    Prologue { address: 0x4cd6d0, bytes: &[0x81, 0xec, 0x10, 0x02, 0x00, 0x00] } => add_decision_date, compare_add_decision_date;
    Prologue { address: 0x866ee0, bytes: &[0x53, 0x56, 0x8b, 0xf1, 0x33, 0xdb] } => reset_morale_state, compare_reset_morale_state;
    Prologue { address: 0x85e590, bytes: &[0x81, 0xec, 0x08, 0x02, 0x00, 0x00] } => init_from_current_contract, compare_init_from_current_contract;
    Prologue { address: 0x867d30, bytes: &[0x56, 0x57, 0x8b, 0xf9, 0x8a, 0x47, 0x35] } => get_expiry_band, compare_get_expiry_band;
    Prologue { address: 0x757a50, bytes: &[0x81, 0xec, 0x08, 0x02, 0x00, 0x00] } => log_story, compare_log_story;
    Prologue { address: 0x757c70, bytes: &[0x83, 0xec, 0x24, 0x53, 0x8b, 0xd9] } => expire_story_log, compare_expire_story_log;
    Prologue { address: 0x757da0, bytes: &[0x83, 0xec, 0x20, 0x55, 0x56] } => has_logged_story, compare_has_logged_story;
    Prologue { address: 0x754740, bytes: &[0x81, 0xec, 0x24, 0x02, 0x00, 0x00] } => add_transfer_interest, compare_add_transfer_interest;
    Prologue { address: 0x757ea0, bytes: &[0x8b, 0x54, 0x24, 0x04, 0x56] } => compare_id_pair_keys, compare_compare_id_pair_keys;
    Prologue { address: 0x757ee0, bytes: &[0x8b, 0x54, 0x24, 0x04, 0x56] } => compare_story_log_records, compare_compare_story_log_records;
    Prologue { address: 0x51c020, bytes: &[0x81, 0xec, 0xe8, 0x05, 0x00, 0x00] } => decide_fixture_winner, compare_decide_fixture_winner;
    Prologue { address: 0x51f2f0, bytes: &[0x81, 0xec, 0x08, 0x02, 0x00, 0x00] } => decide_tie_winner, compare_decide_tie_winner;
    Prologue { address: 0x51ebc0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x8b, 0x54, 0x24, 0x08] } => cup_compare_draw_entries_by_seed, compare_cup_compare_draw_entries_by_seed;
    Prologue { address: 0x687430, bytes: &[0x81, 0xec, 0x08, 0x02, 0x00, 0x00] } => init_table_row, compare_init_table_row;
    Prologue { address: 0x687a20, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => league_get_club_array, compare_league_get_club_array;
    Prologue { address: 0x68aa30, bytes: &[0x53, 0x33, 0xdb, 0x66, 0x39, 0x59, 0x3e] } => record_final_positions, compare_record_final_positions;
    Prologue { address: 0x6830b0, bytes: &[0x8b, 0x0d, 0xf0, 0x7e, 0xdd, 0x00] } => move_club_to_division, compare_move_club_to_division;
    Prologue { address: 0x4afb40, bytes: &[0x56, 0x57, 0x0f, 0xbf, 0x74, 0x24, 0x10] } => comp_util_shuffle_pointer_array, compare_comp_util_shuffle_pointer_array;
    Prologue { address: 0x4afbd0, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => comp_util_sort_and_shuffle_top_clubs, compare_comp_util_sort_and_shuffle_top_clubs;
    Prologue { address: 0x556e90, bytes: &[0x83, 0xec, 0x10, 0x0f, 0xbe, 0x44, 0x24, 0x18] } => update_ban_progress, compare_update_ban_progress;
    Prologue { address: 0x556d00, bytes: &[0x81, 0xec, 0x0c, 0x02, 0x00, 0x00] } => release_ban, compare_release_ban;
    Prologue { address: 0x55e1d0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => clear_ban_history, compare_clear_ban_history;
    Prologue { address: 0x62e500, bytes: &[0x83, 0xec, 0x10, 0x53, 0x55] } => roll_recurring_injury, compare_roll_recurring_injury;
    Prologue { address: 0x631f90, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => reset_player_history, compare_reset_player_history;
    Prologue { address: 0x62bfa0, bytes: &[0x83, 0xec, 0x08, 0x53, 0x55] } => injury_reset_player, compare_injury_reset_player;
    Prologue { address: 0x448ad0, bytes: &[0x51, 0x8b, 0x44, 0x24, 0x0c] } => add_transfer_totals, compare_add_transfer_totals;
    Prologue { address: 0x448cb0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => club_records_update_career_records, compare_club_records_update_career_records;
    Prologue { address: 0x452a90, bytes: &[0x81, 0xec, 0x14, 0x02, 0x00, 0x00] } => update_season_rating_record, compare_update_season_rating_record;
    Prologue { address: 0x5c29b0, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] } => is_comp_current_on_date, compare_is_comp_current_on_date;
    Prologue { address: 0x5c45a0, bytes: &[0x6a, 0xff, 0x68, 0x00, 0xa0, 0x95, 0x00] } => candidate_has_nearby_date, compare_candidate_has_nearby_date;
    Prologue { address: 0x5c4b60, bytes: &[0x81, 0xec, 0x08, 0x02, 0x00, 0x00] } => add_planned_friendly, compare_add_planned_friendly;
    Prologue { address: 0x786090, bytes: &[0x0f, 0xbe, 0x44, 0x24, 0x18] } => send_by_scope, compare_send_by_scope;
    Prologue { address: 0x7861e0, bytes: &[0x8b, 0x54, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => send_to_club_manager, compare_send_to_club_manager;
    Prologue { address: 0x785d40, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => deliver_to_manager, compare_deliver_to_manager;
    Prologue { address: 0x7874b0, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => auto_respond_all, compare_auto_respond_all;
    Prologue { address: 0x787350, bytes: &[0x8b, 0x44, 0x24, 0x08, 0x81, 0xec, 0x04, 0x02, 0x00, 0x00] } => has_urgent_items, compare_has_urgent_items;
    Prologue { address: 0x7858c0, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => set_slot_string, compare_set_slot_string;
    Prologue { address: 0x736720, bytes: &[0x83, 0xec, 0x10, 0x55, 0x8b, 0x6c, 0x24, 0x1c] } => rate_player_form_for_story, compare_rate_player_form_for_story;
    Prologue { address: 0x74af60, bytes: &[0x8b, 0x44, 0x24, 0x08, 0x53] } => write_financial_trouble_news_text, compare_write_financial_trouble_news_text;
    Prologue { address: 0x786960, bytes: &[0x81, 0xec, 0x08, 0x02, 0x00, 0x00] } => get_inbox_items, compare_get_inbox_items;
    Prologue { address: 0x7aace0, bytes: &[0x53, 0x56, 0x8b, 0x74, 0x24, 0x0c] } => retire_from_playing_staff, compare_retire_from_playing_staff;
    Prologue { address: 0x7ad0e0, bytes: &[0x83, 0xec, 0x14, 0x89, 0x4c, 0x24, 0x10] } => pick_best_club_for_player, compare_pick_best_club_for_player;
    Prologue { address: 0x7ad590, bytes: &[0x81, 0xec, 0x18, 0x02, 0x00, 0x00] } => pick_club_from_nation_list, compare_pick_club_from_nation_list;
    Prologue { address: 0x69d060, bytes: &[0x81, 0xec, 0x08, 0x02, 0x00, 0x00] } => fix_missing_jobs, compare_fix_missing_jobs;
    Prologue { address: 0x69c540, bytes: &[0x83, 0xec, 0x08, 0x53, 0x55] } => best_manager_candidate_score, compare_best_manager_candidate_score;
    Prologue { address: 0x8c6220, bytes: &[0x81, 0xec, 0x10, 0x02, 0x00, 0x00] } => schedule_offer, compare_schedule_offer;
    Prologue { address: 0x849f00, bytes: &[0x53, 0x56, 0x57, 0x8b, 0x7c, 0x24, 0x14] } => get_squad_status_for_club, compare_get_squad_status_for_club;
    Prologue { address: 0x849d60, bytes: &[0x83, 0xec, 0x08, 0x53, 0x55] } => shortlist_manager_estimate_squad_status, compare_shortlist_manager_estimate_squad_status;
    Prologue { address: 0x844660, bytes: &[0x83, 0xec, 0x08, 0x53, 0x8b, 0x5c, 0x24, 0x14] } => shortlist_manager_release_club_slot_targets, compare_shortlist_manager_release_club_slot_targets;
    Prologue { address: 0x7b7490, bytes: &[0x83, 0xec, 0x0c, 0x53, 0x55] } => scout_covers_nation, compare_scout_covers_nation;
    Prologue { address: 0x5c87b0, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => remove_friendly_match, compare_remove_friendly_match;
    Prologue { address: 0x5cc2d0, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => init_banned_nation_pairs, compare_init_banned_nation_pairs;
    Prologue { address: 0x696570, bytes: &[0x83, 0xec, 0x08, 0x89, 0x4c, 0x24, 0x00] } => assess_transfer_deal, compare_assess_transfer_deal;
    Prologue { address: 0x6978f0, bytes: &[0x51, 0x53, 0x56, 0x57, 0x8b, 0x7c, 0x24, 0x14] } => apply_transfer_confidence_changes, compare_apply_transfer_confidence_changes;
    Prologue { address: 0x6ac470, bytes: &[0x83, 0xec, 0x08, 0x53, 0x56] } => resolve_board_ultimatum, compare_resolve_board_ultimatum;
    Prologue { address: 0x4d0b50, bytes: &[0x56, 0x8b, 0x74, 0x24, 0x08] } => remove_contract, compare_remove_contract;
    Prologue { address: 0x4cf4a0, bytes: &[0x55, 0x56, 0x57, 0x8b, 0xf9] } => resubmit_held_offers, compare_resubmit_held_offers;
    Prologue { address: 0x4d45f0, bytes: &[0x6a, 0xff, 0x68, 0x40, 0x6e, 0x95, 0x00] } => contract_manager_season_end_month, compare_contract_manager_season_end_month;
    Prologue { address: 0x86dda0, bytes: &[0x56, 0x57, 0x8b, 0x7c, 0x24, 0x0c] } => staff_records_daily_update, compare_staff_records_daily_update;
    Prologue { address: 0x86d6e0, bytes: &[0x8b, 0x54, 0x24, 0x04, 0x53] } => staff_records_end_season_for_nation, compare_staff_records_end_season_for_nation;
    Prologue { address: 0x86f950, bytes: &[0x81, 0xec, 0x20, 0x02, 0x00, 0x00] } => find_manager_of_the_month, compare_find_manager_of_the_month;
    Prologue { address: 0x69b650, bytes: &[0x81, 0xec, 0x04, 0x02, 0x00, 0x00] } => process_pending_departures, compare_process_pending_departures;
    Prologue { address: 0x690730, bytes: &[0x53, 0x56, 0x8b, 0x74, 0x24, 0x0c] } => manager_manager_remove_player_from_squad, compare_manager_manager_remove_player_from_squad;
    Prologue { address: 0x690890, bytes: &[0x8b, 0x0d, 0x50, 0x10, 0xae, 0x00] } => remove_staff_from_club, compare_remove_staff_from_club;
    Prologue { address: 0x77bbe0, bytes: &[0x6a, 0xff, 0x68, 0xfe, 0xe0, 0x95, 0x00] } => construct_news_item, compare_construct_news_item;
    Prologue { address: 0x738340, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] } => adjust_supporter_confidence, compare_adjust_supporter_confidence;
    Prologue { address: 0x7386f0, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] } => adjust_board_confidence, compare_adjust_board_confidence;
    Prologue { address: 0x735870, bytes: &[0x51, 0x53, 0x55, 0x56, 0x8b, 0x74, 0x24, 0x14] } => pick_ai_manager_response, compare_pick_ai_manager_response;
    Prologue { address: 0x787140, bytes: &[0x81, 0xec, 0x04, 0x02, 0x00, 0x00] } => mark_item_read, compare_mark_item_read;
    Prologue { address: 0x786b70, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => find_item_by_id, compare_find_item_by_id;
    Prologue { address: 0x732c80, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] } => queue_player_gratitude_news, compare_queue_player_gratitude_news;
    Prologue { address: 0x743370, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] } => post_player_backing_request_news, compare_post_player_backing_request_news;
    Prologue { address: 0x743a50, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] } => post_player_criticism_reaction_news, compare_post_player_criticism_reaction_news;
    Prologue { address: 0x839e60, bytes: &[0x81, 0xec, 0x04, 0x02, 0x00, 0x00] } => build_position_lists, compare_build_position_lists;
    Prologue { address: 0x83a780, bytes: &[0x51, 0x53, 0x55, 0x56, 0x57] } => sort_position_lists, compare_sort_position_lists;
    Prologue { address: 0x842180, bytes: &[0x83, 0xec, 0x14, 0x53, 0x55] } => collect_interested_clubs, compare_collect_interested_clubs;
    Prologue { address: 0x8423f0, bytes: &[0x81, 0xec, 0x08, 0x02, 0x00, 0x00] } => add_shortlist_entry, compare_add_shortlist_entry;
    Prologue { address: 0x84af00, bytes: &[0x8b, 0x0d, 0x50, 0x10, 0xae, 0x00] } => shortlist_manager_predict_selling_club_response, compare_shortlist_manager_predict_selling_club_response;
    Prologue { address: 0x8d0590, bytes: &[0x83, 0xec, 0x10, 0x32, 0xd2] } => transfer_manager_get_loan_period, compare_transfer_manager_get_loan_period;
    Prologue { address: 0x8cc5b0, bytes: &[0x81, 0xec, 0x14, 0x02, 0x00, 0x00] } => get_offer_response_delay, compare_get_offer_response_delay;
    Prologue { address: 0x8caee0, bytes: &[0x83, 0xec, 0x0c, 0xa1, 0x5c, 0x23, 0xae, 0x00] } => pay_transfer_instalments, compare_pay_transfer_instalments;
    Prologue { address: 0x8d0f60, bytes: &[0x53, 0x56, 0x8b, 0x74, 0x24, 0x0c] } => remove_offer, compare_remove_offer;
    Prologue { address: 0x556940, bytes: &[0x81, 0xec, 0x04, 0x02, 0x00, 0x00] } => add_ban, compare_add_ban;
    Prologue { address: 0x556270, bytes: &[0x81, 0xec, 0x08, 0x02, 0x00, 0x00] } => clear_bans_invalid_for_club, compare_clear_bans_invalid_for_club;
    Prologue { address: 0x556570, bytes: &[0x83, 0xec, 0x20, 0x53, 0x55] } => restore_held_bans, compare_restore_held_bans;
    Prologue { address: 0x7c6a30, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => add_language, compare_add_language;
    Prologue { address: 0x7c6ca0, bytes: &[0x83, 0xec, 0x08, 0x53, 0x56] } => improve_language, compare_improve_language;
    Prologue { address: 0x7c6650, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => player_stats_reset_player, compare_player_stats_reset_player;
    Prologue { address: 0x775220, bytes: &[0x8b, 0x54, 0x24, 0x04, 0x81, 0xec, 0x04, 0x02, 0x00, 0x00] } => set_current_competition, compare_set_current_competition;
    Prologue { address: 0x7775c0, bytes: &[0xa1, 0x50, 0x23, 0xae, 0x00] } => mark_clubs_with_fixtures, compare_mark_clubs_with_fixtures;
    Prologue { address: 0x556830, bytes: &[0x83, 0xec, 0x08, 0x53, 0x55] } => discipline_remove_player, compare_discipline_remove_player;
    Prologue { address: 0x4dc910, bytes: &[0x53, 0x8b, 0x5c, 0x24, 0x08] } => compare_decision_dates, compare_compare_decision_dates;
    Prologue { address: 0x4cd9c0, bytes: &[0x53, 0x55, 0x56, 0x57, 0x8b, 0xf9] } => process_due_decision_dates, compare_process_due_decision_dates;
    Prologue { address: 0x4ca300, bytes: &[0x81, 0xec, 0x08, 0x02, 0x00, 0x00] } => get_current_contract_offers, compare_get_current_contract_offers;
    Prologue { address: 0x4d01e0, bytes: &[0x6a, 0xff, 0x68, 0x1b, 0x6e, 0x95, 0x00] } => sign_loan_contract, compare_sign_loan_contract;
    Prologue { address: 0x4d07e0, bytes: &[0x81, 0xec, 0x08, 0x02, 0x00, 0x00] } => end_loan, compare_end_loan;
    Prologue { address: 0x4cbd20, bytes: &[0x51, 0x53, 0x8b, 0x5c, 0x24, 0x10] } => toggle_squad_placement, compare_toggle_squad_placement;
    Prologue { address: 0x694c80, bytes: &[0x83, 0xec, 0x08, 0x53, 0x55] } => reset_board_for_new_manager, compare_reset_board_for_new_manager;
    Prologue { address: 0x69b350, bytes: &[0x83, 0xec, 0x14, 0x53, 0x55] } => get_departure_reason, compare_get_departure_reason;
    Prologue { address: 0x86f5b0, bytes: &[0x81, 0xec, 0x10, 0x02, 0x00, 0x00] } => get_manager_record, compare_get_manager_record;
    Prologue { address: 0x69f810, bytes: &[0x55, 0x56, 0x8b, 0x74, 0x24, 0x0c] } => is_staff_suitable_for_job, compare_is_staff_suitable_for_job;
    Prologue { address: 0x4cb990, bytes: &[0x0f, 0xbe, 0x44, 0x24, 0x04] } => staff_can_take_job, compare_staff_can_take_job;
    Prologue { address: 0x4cb250, bytes: &[0x83, 0xec, 0x08, 0x56, 0x57] } => contract_manager_lower_match_morale, compare_contract_manager_lower_match_morale;
    Prologue { address: 0x83c090, bytes: &[0x51, 0x8b, 0x44, 0x24, 0x08] } => shortlist_set_up_formation_slots, compare_shortlist_set_up_formation_slots;
    Prologue { address: 0x83a730, bytes: &[0x56, 0x57, 0x8b, 0x7c, 0x24, 0x0c] } => update_club_shortlist, compare_update_club_shortlist;
    Prologue { address: 0x842b00, bytes: &[0x6a, 0xff, 0x68, 0xdb, 0x24, 0x96, 0x00] } => suspend_clubs_shortlisting_player, compare_suspend_clubs_shortlisting_player;
    Prologue { address: 0x842c00, bytes: &[0x83, 0xec, 0x14, 0x53, 0x55] } => add_player_to_club_shortlists, compare_add_player_to_club_shortlists;
    Prologue { address: 0x842320, bytes: &[0x53, 0x55, 0x56, 0x57, 0x8b, 0x7c, 0x24, 0x14] } => transfer_list_player, compare_transfer_list_player;
    Prologue { address: 0x8cb340, bytes: &[0x83, 0xec, 0x10, 0x53, 0x55] } => pay_appearance_money, compare_pay_appearance_money;
    Prologue { address: 0x8c36b0, bytes: &[0x56, 0x8b, 0xf1, 0x8b, 0x4c, 0x24, 0x08] } => club_rejects_offer, compare_club_rejects_offer;
    Prologue { address: 0x8c6ae0, bytes: &[0x6a, 0xff, 0x68, 0x18, 0x3d, 0x96, 0x00] } => detach_offer_players, compare_detach_offer_players;
    Prologue { address: 0x8c8740, bytes: &[0x6a, 0xff, 0x68, 0xce, 0x3d, 0x96, 0x00] } => build_cash_offer_option, compare_build_cash_offer_option;
    Prologue { address: 0x62e820, bytes: &[0x81, 0xec, 0x08, 0x02, 0x00, 0x00] } => add_illness_outbreak, compare_add_illness_outbreak;
    Prologue { address: 0x62e9e0, bytes: &[0x6a, 0xff, 0x68, 0x88, 0xb1, 0x95, 0x00] } => spread_illness_outbreaks, compare_spread_illness_outbreaks;
    Prologue { address: 0x62ce30, bytes: &[0x6a, 0xff, 0x68, 0x20, 0xb1, 0x95, 0x00] } => injury_player_in_close_season, compare_injury_player_in_close_season;
    Prologue { address: 0x62a920, bytes: &[0x6a, 0xff, 0x68, 0x90, 0xb0, 0x95, 0x00] } => init_squad_fitness, compare_init_squad_fitness;
    Prologue { address: 0x62cf70, bytes: &[0x81, 0xec, 0x1c, 0x02, 0x00, 0x00] } => init_records, compare_init_records;
    Prologue { address: 0x7acfc0, bytes: &[0x6a, 0xff, 0x68, 0x6b, 0xf2, 0x95, 0x00] } => build_nation_club_lists, compare_build_nation_club_lists;
    Prologue { address: 0x452da0, bytes: &[0x55, 0x8b, 0xec, 0x83, 0xe4, 0xf8] } => pick_fans_player_of_year, compare_pick_fans_player_of_year;
    Prologue { address: 0x7c1350, bytes: &[0x81, 0xec, 0x0c, 0x02, 0x00, 0x00] } => end_season_for_key_nation, compare_end_season_for_key_nation;
    Prologue { address: 0x448760, bytes: &[0x83, 0xec, 0x10, 0x53, 0x8b, 0x5c, 0x24, 0x18] } => update_transfer_fee_records, compare_update_transfer_fee_records;
    Prologue { address: 0x5cc3e0, bytes: &[0x51, 0x8a, 0x81, 0x06, 0x35, 0x00, 0x00] } => is_banned_nation_pair, compare_is_banned_nation_pair;
    Prologue { address: 0x5cc5f0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x8b, 0x08] } => compare_nations_by_reputation_desc, compare_compare_nations_by_reputation_desc;
    Prologue { address: 0x5cc610, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x56] } => compare_fixture_records_by_date, compare_compare_fixture_records_by_date;
    Prologue { address: 0x776fc0, bytes: &[0x6a, 0xff, 0x68, 0x48, 0xdf, 0x95, 0x00] } => can_schedule_club_fixture, compare_can_schedule_club_fixture;
    Prologue { address: 0x778e90, bytes: &[0x56, 0x8b, 0x74, 0x24, 0x0c] } => compare_call_up_records_by_date, compare_compare_call_up_records_by_date;
    Prologue { address: 0x7779b0, bytes: &[0x8b, 0x54, 0x24, 0x04, 0x83, 0xec, 0x08] } => add_qualified_nation, compare_add_qualified_nation;
    Prologue { address: 0x522190, bytes: &[0x8b, 0x4c, 0x24, 0x04, 0x8b, 0x54, 0x24, 0x08] } => cup_compare_entries_by_level_desc, compare_cup_compare_entries_by_level_desc;
    Prologue { address: 0x5221c0, bytes: &[0x8b, 0x4c, 0x24, 0x04, 0x8b, 0x54, 0x24, 0x08] } => cup_compare_entries_by_level_asc, compare_cup_compare_entries_by_level_asc;
    Prologue { address: 0x68a980, bytes: &[0x56, 0x57, 0x8b, 0xf9, 0x33, 0xf6] } => limit_club_division_tier, compare_limit_club_division_tier;
    Prologue { address: 0x69aba0, bytes: &[0x83, 0xec, 0x08, 0xa1, 0xcc, 0x6e, 0xdd, 0x00] } => consider_sacking_manager, compare_consider_sacking_manager;
    Prologue { address: 0x6ac560, bytes: &[0x6a, 0xff, 0x68, 0x88, 0xd0, 0x95, 0x00] } => issue_board_ultimatum, compare_issue_board_ultimatum;
    Prologue { address: 0x69b790, bytes: &[0x83, 0xec, 0x08, 0x53, 0x55] } => remove_club_manager, compare_remove_club_manager;
    Prologue { address: 0x6a5610, bytes: &[0x83, 0xec, 0x08, 0x53, 0x55] } => review_national_team_performance, compare_review_national_team_performance;
    Prologue { address: 0x69eb50, bytes: &[0x56, 0x8b, 0x74, 0x24, 0x08] } => is_club_in_financial_trouble, compare_is_club_in_financial_trouble;
    Prologue { address: 0x690b70, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x8b, 0x09] } => get_board_confidence_band, compare_get_board_confidence_band;
    Prologue { address: 0x869480, bytes: &[0x6a, 0xff, 0x68, 0x96, 0x2b, 0x96, 0x00] } => add_promotion_record, compare_add_promotion_record;
    Prologue { address: 0x86a1c0, bytes: &[0x6a, 0xff, 0x68, 0x06, 0x2c, 0x96, 0x00] } => add_relegation_record, compare_add_relegation_record;
    Prologue { address: 0x86bdd0, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] } => add_qualified_for_round_record, compare_add_qualified_for_round_record;
    Prologue { address: 0x86cd70, bytes: &[0x6a, 0xff, 0x68, 0x5b, 0x2d, 0x96, 0x00] } => add_transfer_records, compare_add_transfer_records;
    Prologue { address: 0x86c000, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] } => add_knocked_out_record, compare_add_knocked_out_record;
    Prologue { address: 0x86c1d0, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] } => add_failed_to_qualify_record, compare_add_failed_to_qualify_record;
    Prologue { address: 0x8cee70, bytes: &[0x81, 0xec, 0x10, 0x02, 0x00, 0x00] } => transfer_manager_get_offer_completion_date, compare_transfer_manager_get_offer_completion_date;
    Prologue { address: 0x8cf0e0, bytes: &[0x6a, 0xff, 0x68, 0xa8, 0x3f, 0x96, 0x00] } => find_next_allowed_transfer_date, compare_find_next_allowed_transfer_date;
    Prologue { address: 0x8d0bb0, bytes: &[0x6a, 0xff, 0x68, 0x90, 0x40, 0x96, 0x00] } => transfer_manager_get_next_window_dates, compare_transfer_manager_get_next_window_dates;
    Prologue { address: 0x8d0780, bytes: &[0x6a, 0xff, 0x68, 0x3b, 0x40, 0x96, 0x00] } => transfer_manager_calc_compensation_fee, compare_transfer_manager_calc_compensation_fee;
    Prologue { address: 0x8ca8a0, bytes: &[0x83, 0xec, 0x18, 0x53, 0x55] } => pay_fee_and_record_clauses, compare_pay_fee_and_record_clauses;
    Prologue { address: 0x8cd240, bytes: &[0x6a, 0xff, 0x68, 0x00, 0x3f, 0x96, 0x00] } => transfer_manager_selling_club_must_accept, compare_transfer_manager_selling_club_must_accept;
    Prologue { address: 0x8c7e20, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] } => update_work_permit_on_move, compare_update_work_permit_on_move;
    Prologue { address: 0x8c38b0, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] } => complete_accepted_contract_offer, compare_complete_accepted_contract_offer;
    Prologue { address: 0x8c1e40, bytes: &[0x6a, 0xff, 0x68, 0xc8, 0x3b, 0x96, 0x00] } => offer_contract_to_offer_player, compare_offer_contract_to_offer_player;
    Prologue { address: 0x8c44e0, bytes: &[0x56, 0x8b, 0x74, 0x24, 0x08] } => collapse_offer, compare_collapse_offer;
    Prologue { address: 0x8bed60, bytes: &[0x6a, 0xff, 0x68, 0xc3, 0x3a, 0x96, 0x00] } => make_loan_offer, compare_make_loan_offer;
    Prologue { address: 0x8cbbe0, bytes: &[0x81, 0xec, 0x0c, 0x02, 0x00, 0x00] } => process_due_future_transfers, compare_process_due_future_transfers;
    Prologue { address: 0x7a7150, bytes: &[0x83, 0xec, 0x08, 0x56, 0x8b, 0x74, 0x24, 0x10] } => player_regen_score_non_player_candidate, compare_player_regen_score_non_player_candidate;
    Prologue { address: 0x7ae2a0, bytes: &[0x53, 0x56, 0x57, 0x8b, 0x7c, 0x24, 0x10] } => player_regen_compare_retirements, compare_player_regen_compare_retirements;
    Prologue { address: 0x7ae240, bytes: &[0x56, 0x8b, 0x74, 0x24, 0x08] } => player_regen_compare_queue_entries, compare_player_regen_compare_queue_entries;
    Prologue { address: 0x7c3920, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => get_category_record, compare_get_category_record;
    Prologue { address: 0x7c5d80, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => create_stats, compare_create_stats;
    Prologue { address: 0x7ae350, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x56] } => is_retirement_pending, compare_is_retirement_pending;
    Prologue { address: 0x51e220, bytes: &[0x81, 0xec, 0x28, 0x02, 0x00, 0x00] } => randomize_round_teams, compare_randomize_round_teams;
    Prologue { address: 0x522610, bytes: &[0x83, 0xec, 0x14, 0x56, 0x57] } => separate_shared_stadium_ties, compare_separate_shared_stadium_ties;
    Prologue { address: 0x4bf850, bytes: &[0x66, 0x8b, 0x4c, 0x24, 0x10] } => comp_util_scale_by_round, compare_comp_util_scale_by_round;
    Prologue { address: 0x4afa80, bytes: &[0x56, 0x8b, 0x74, 0x24, 0x0c] } => comp_util_get_fixture_venue_side, compare_comp_util_get_fixture_venue_side;
    Prologue { address: 0x4afba0, bytes: &[0x8b, 0x4c, 0x24, 0x04, 0x8b, 0x54, 0x24, 0x08] } => compare_nation_coefficients_desc, compare_compare_nation_coefficients_desc;
    Prologue { address: 0x7f8d70, bytes: &[0x6a, 0xff, 0x68, 0xfb, 0x0f, 0x96, 0x00] } => has_scout_request, compare_has_scout_request;
    Prologue { address: 0x896e90, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => clear_selection_slot, compare_clear_selection_slot;
    Prologue { address: 0x897bb0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => set_set_piece_taker, compare_set_set_piece_taker;
    Prologue { address: 0x8cf6b0, bytes: &[0x51, 0x53, 0x55, 0x56, 0x8b, 0x74, 0x24, 0x18] } => fill_squad_restrictions, compare_fill_squad_restrictions;
    Prologue { address: 0x8cf880, bytes: &[0x83, 0xec, 0x0c, 0x55, 0x8b, 0x6c, 0x24, 0x18] } => transfer_manager_check_matchday_squad_limit, compare_transfer_manager_check_matchday_squad_limit;
    Prologue { address: 0x8c7560, bytes: &[0x8b, 0x44, 0x24, 0x08, 0x81, 0xec, 0x10, 0x02, 0x00, 0x00] } => player_needs_work_permit, compare_player_needs_work_permit;
    Prologue { address: 0x8ce990, bytes: &[0x6a, 0xff, 0x68, 0x78, 0x3f, 0x96, 0x00] } => can_club_bid_for_player, compare_can_club_bid_for_player;
    Prologue { address: 0x8c2060, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => note_terms_offered_to_exchange_player, compare_note_terms_offered_to_exchange_player;
    Prologue { address: 0x8c1780, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => count_club_free_transfer_queue, compare_count_club_free_transfer_queue;
    Prologue { address: 0x8cca10, bytes: &[0x6a, 0xff, 0x68, 0x98, 0x3e, 0x96, 0x00] } => get_future_transfer, compare_get_future_transfer;
    Prologue { address: 0x8cbf70, bytes: &[0x53, 0x56, 0x57, 0x8b, 0x7c, 0x24, 0x10] } => remove_future_transfer, compare_remove_future_transfer;
    Prologue { address: 0x8cb9a0, bytes: &[0x81, 0xec, 0x08, 0x02, 0x00, 0x00] } => add_future_transfer, compare_add_future_transfer;
    Prologue { address: 0x8c6100, bytes: &[0x51, 0x55, 0x56, 0x8b, 0xf1] } => get_open_offers_for_club_players, compare_get_open_offers_for_club_players;
    Prologue { address: 0x8c47d0, bytes: &[0x6a, 0xff, 0x68, 0x88, 0x3c, 0x96, 0x00] } => collapse_offers_for_player, compare_collapse_offers_for_player;
    Prologue { address: 0x8caaf0, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => add_next_sale_percentage, compare_add_next_sale_percentage;
    Prologue { address: 0x68f130, bytes: &[0x55, 0x8b, 0xec, 0x83, 0xe4, 0xf8] } => init_club_board_records, compare_init_club_board_records;
    Prologue { address: 0x6abef0, bytes: &[0x81, 0xec, 0x08, 0x02, 0x00, 0x00] } => process_ban_appeals, compare_process_ban_appeals;
    Prologue { address: 0x5fe700, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] } => can_watch_fixture, compare_can_watch_fixture;
    Prologue { address: 0x7a79c0, bytes: &[0x83, 0xec, 0x0c, 0x57, 0x8b, 0x7c, 0x24, 0x14] } => player_regen_check_initial_retirement, compare_player_regen_check_initial_retirement;
    Prologue { address: 0x7a7310, bytes: &[0x83, 0xec, 0x14, 0x8b, 0x0d, 0x50, 0x10, 0xae, 0x00] } => player_regen_check_retirement, compare_player_regen_check_retirement;
    Prologue { address: 0x7a58f0, bytes: &[0x6a, 0xff, 0x68, 0x88, 0xf1, 0x95, 0x00] } => build_initial_retirement_list, compare_build_initial_retirement_list;
    Prologue { address: 0x7a5790, bytes: &[0x6a, 0xff, 0x68, 0x58, 0xf1, 0x95, 0x00] } => find_new_retirements, compare_find_new_retirements;
    Prologue { address: 0x7aaeb0, bytes: &[0x53, 0x56, 0x8b, 0x74, 0x24, 0x0c] } => player_regen_retire_from_non_playing_staff, compare_player_regen_retire_from_non_playing_staff;
    Prologue { address: 0x86d800, bytes: &[0x6a, 0xff, 0x68, 0x7b, 0x2d, 0x96, 0x00] } => add_free_release_records, compare_add_free_release_records;
    Prologue { address: 0x62c400, bytes: &[0x81, 0xec, 0x08, 0x02, 0x00, 0x00] } => get_player_injury_status, compare_get_player_injury_status;
    Prologue { address: 0x4cd230, bytes: &[0x6a, 0xff, 0x68, 0x50, 0x6d, 0x95, 0x00] } => schedule_offer_decision, compare_schedule_offer_decision;
    Prologue { address: 0x86fab0, bytes: &[0x6a, 0xff, 0x68, 0x08, 0x2e, 0x96, 0x00] } => get_manager_of_year_candidates, compare_get_manager_of_year_candidates;
    Prologue { address: 0x68ac40, bytes: &[0x83, 0xec, 0x0c, 0x53, 0x55, 0x56] } => update_projected_positions, compare_update_projected_positions;
    Prologue { address: 0x68afb0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x8b, 0x08] } => compare_projection_by_reputation_desc, compare_compare_projection_by_reputation_desc;
    Prologue { address: 0x68afd0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x8b, 0x54, 0x24, 0x08] } => compare_projection_by_points_desc, compare_compare_projection_by_points_desc;
    Prologue { address: 0x5acbe0, bytes: &[0x81, 0xec, 0x24, 0x02, 0x00, 0x00] } => find_fixtures, compare_find_fixtures;
    Prologue { address: 0x5abd90, bytes: &[0x81, 0xec, 0x08, 0x02, 0x00, 0x00] } => collect_comp_fixtures_on_day, compare_collect_comp_fixtures_on_day;
    Prologue { address: 0x5ae6d0, bytes: &[0x81, 0xec, 0x04, 0x02, 0x00, 0x00] } => fix_man_comp_has_fixture_on_date, compare_fix_man_comp_has_fixture_on_date;
    Prologue { address: 0x5c6b70, bytes: &[0x6a, 0xff, 0x68, 0x31, 0xa1, 0x95, 0x00] } => friendly_is_club_free_on_date, compare_friendly_is_club_free_on_date;
    Prologue { address: 0x522910, bytes: &[0x56, 0x66, 0x8b, 0x71, 0x3e] } => has_club, compare_has_club;
    Prologue { address: 0x5af5b0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x53] } => fix_man_get_partner_key_nation_index, compare_fix_man_get_partner_key_nation_index;
    Prologue { address: 0x4d8610, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] } => handle_transfer_request, compare_handle_transfer_request;
    Prologue { address: 0x4cbb20, bytes: &[0x8a, 0x44, 0x24, 0x04, 0x3c, 0x0b] } => is_contract_staff_job, compare_is_contract_staff_job;
    Prologue { address: 0x4cb960, bytes: &[0x8b, 0x4c, 0x24, 0x04, 0x8a, 0x41, 0x3d] } => staff_is_not_board_member, compare_staff_is_not_board_member;
    Prologue { address: 0x8c6e60, bytes: &[0x6a, 0xff, 0x68, 0x68, 0x3d, 0x96, 0x00] } => init_work_permits, compare_init_work_permits;
    Prologue { address: 0x8c3e50, bytes: &[0x8a, 0x44, 0x24, 0x08, 0x56] } => get_clause_count, compare_get_clause_count;
    Prologue { address: 0x8d0ea0, bytes: &[0x83, 0xec, 0x10, 0x56, 0x57] } => find_first_team_appearance_money, compare_find_first_team_appearance_money;
    Prologue { address: 0x8c18c0, bytes: &[0x6a, 0xff, 0x68, 0xb0, 0x3b, 0x96, 0x00] } => process_free_transfer_queue, compare_process_free_transfer_queue;
    Prologue { address: 0x8cf470, bytes: &[0x6a, 0xff, 0x68, 0xd6, 0x3f, 0x96, 0x00] } => find_next_window_open_date, compare_find_next_window_open_date;
    Prologue { address: 0x8c72d0, bytes: &[0x53, 0x56, 0x8b, 0xf1, 0x57] } => find_negotiated_loan_contract, compare_find_negotiated_loan_contract;
    Prologue { address: 0x8c8fd0, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] } => build_clause_offer_option, compare_build_clause_offer_option;
    Prologue { address: 0x8c5560, bytes: &[0x53, 0x56, 0x57, 0x8b, 0x7c, 0x24, 0x14] } => transfer_manager_is_rejection_reason_valid, compare_transfer_manager_is_rejection_reason_valid;
    Prologue { address: 0x8c54c0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x56] } => transfer_manager_get_squad_status_reason, compare_transfer_manager_get_squad_status_reason;
    Prologue { address: 0x5d6d10, bytes: &[0x83, 0xec, 0x08, 0x53, 0x55, 0x56] } => game_update_squad_relationships, compare_game_update_squad_relationships;
    Prologue { address: 0x5d7190, bytes: &[0x83, 0xec, 0x08, 0x53, 0x55, 0x56] } => game_check_club_dressing_room_unrest, compare_game_check_club_dressing_room_unrest;
    Prologue { address: 0x5ce0b0, bytes: &[0x83, 0xec, 0x1c, 0xa1, 0x5c, 0x23, 0xae, 0x00] } => game_spread_squad_morale_by_leadership, compare_game_spread_squad_morale_by_leadership;
    Prologue { address: 0x5d6aa0, bytes: &[0x51, 0x8b, 0x0d, 0x64, 0x23, 0xae, 0x00] } => game_check_players_missing_training, compare_game_check_players_missing_training;
    Prologue { address: 0x5ce060, bytes: &[0xa1, 0xf8, 0xad, 0xad, 0x00] } => game_update_comps_for_date, compare_game_update_comps_for_date;
    Prologue { address: 0x5cdde0, bytes: &[0x8b, 0x54, 0x24, 0x04, 0x85, 0xd2] } => manager_manager_adjust_board_confidence, compare_manager_manager_adjust_board_confidence;
    Prologue { address: 0x85b820, bytes: &[0x83, 0xec, 0x14, 0x53, 0x8b, 0x5c, 0x24, 0x1c] } => stadium_daily_update, compare_stadium_daily_update;
    Prologue { address: 0x85de20, bytes: &[0x81, 0xec, 0x08, 0x02, 0x00, 0x00] } => add_booking, compare_add_booking;
    Prologue { address: 0x85ddb0, bytes: &[0x83, 0xec, 0x14, 0x56, 0x8b, 0xf1] } => is_stadium_free_on_date, compare_is_stadium_free_on_date;
    Prologue { address: 0x7f9720, bytes: &[0x83, 0xec, 0x0c, 0x53, 0x8b, 0x5c, 0x24, 0x14] } => fixture_has_scouted_player, compare_fixture_has_scouted_player;
    Prologue { address: 0x7f7490, bytes: &[0x8b, 0x54, 0x24, 0x04, 0x81, 0xec, 0x04, 0x02, 0x00, 0x00] } => is_player_shortlisted, compare_is_player_shortlisted;
    Prologue { address: 0x86cc50, bytes: &[0x6a, 0xff, 0x68, 0x3b, 0x2d, 0x96, 0x00] } => add_first_cap_record, compare_add_first_cap_record;
    Prologue { address: 0x7c3d20, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x04, 0x02, 0x00, 0x00] } => player_stats_get_career_goals, compare_player_stats_get_career_goals;
    Prologue { address: 0x86da20, bytes: &[0x6a, 0xff, 0x68, 0x9b, 0x2d, 0x96, 0x00] } => add_loan_records, compare_add_loan_records;
    Prologue { address: 0x632040, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x56] } => injury_compare_entry_dates, compare_injury_compare_entry_dates;
    Prologue { address: 0x7c3bb0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x04, 0x02, 0x00, 0x00] } => player_stats_get_goals_for_club, compare_player_stats_get_goals_for_club;
    Prologue { address: 0x86c730, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] } => add_team_of_year_record, compare_add_team_of_year_record;
    Prologue { address: 0x870220, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x8b, 0x4c, 0x24, 0x08] } => staff_records_compare_clubs_by_manager_points, compare_staff_records_compare_clubs_by_manager_points;
    Prologue { address: 0x5ac7a0, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] } => find_stored_fixture, compare_find_stored_fixture;
    Prologue { address: 0x5ab230, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] } => insert_fixture, compare_insert_fixture;
    Prologue { address: 0x5abc30, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] } => remove_fixture, compare_remove_fixture;
    Prologue { address: 0x5c2ae0, bytes: &[0x6a, 0xff, 0x68, 0x10, 0x9e, 0x95, 0x00] } => create_due_planned_friendlies, compare_create_due_planned_friendlies;
    Prologue { address: 0x5c6f00, bytes: &[0x6a, 0xff, 0x68, 0x5c, 0xa1, 0x95, 0x00] } => add_friendly_match, compare_add_friendly_match;
    Prologue { address: 0x5c4380, bytes: &[0x6a, 0xff, 0x68, 0xe4, 0x9f, 0x95, 0x00] } => plan_friendly_on_free_date, compare_plan_friendly_on_free_date;
    Prologue { address: 0x51f890, bytes: &[0x6a, 0xff, 0x68, 0x98, 0x77, 0x95, 0x00] } => cup_update_for_date, compare_cup_update_for_date;
    Prologue { address: 0x51c800, bytes: &[0x6a, 0xff, 0x68, 0xd9, 0x76, 0x95, 0x00] } => set_up_round, compare_set_up_round;
    Prologue { address: 0x5ab3c0, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] } => add_away_trip_illness_risk, compare_add_away_trip_illness_risk;
    Prologue { address: 0x58d710, bytes: &[0x8b, 0x54, 0x24, 0x0c, 0x53] } => count_results_in_period, compare_count_results_in_period;
    Prologue { address: 0x58d460, bytes: &[0x83, 0xec, 0x18, 0x53, 0x8b, 0x5c, 0x24, 0x20] } => update_rankings, compare_update_rankings;
    Prologue { address: 0x58e220, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x8b, 0x54, 0x24, 0x08] } => compare_result_points_desc, compare_compare_result_points_desc;
    Prologue { address: 0x58dea0, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] } => append_result, compare_append_result;
    Prologue { address: 0x58e190, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x56] } => get_nation_strength_tier, compare_get_nation_strength_tier;
    Prologue { address: 0x58dfc0, bytes: &[0x83, 0xec, 0x1c, 0xa1, 0x50, 0x23, 0xae, 0x00] } => recalculate_scores, compare_recalculate_scores;
    Prologue { address: 0x48dd70, bytes: &[0x8b, 0x0d, 0x74, 0x2c, 0xae, 0x00] } => comp_active_key_nation_of_fixture, compare_comp_active_key_nation_of_fixture;
    Prologue { address: 0x669a10, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => key_nation_is_world_cup_comp, compare_key_nation_is_world_cup_comp;
    Prologue { address: 0x669ae0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => key_nation_is_international_comp, compare_key_nation_is_international_comp;
    Prologue { address: 0x669470, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => key_nation_index_of_comp, compare_key_nation_index_of_comp;
    Prologue { address: 0x58d610, bytes: &[0x81, 0xec, 0x04, 0x02, 0x00, 0x00] } => collect_ranked_order, compare_collect_ranked_order;
    Prologue { address: 0x58e240, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x8b, 0x54, 0x24, 0x08] } => compare_ranking_score_desc, compare_compare_ranking_score_desc;
    Prologue { address: 0x922ee0, bytes: &[0x51, 0x56, 0x57, 0x8b, 0xf9] } => update_city_weather, compare_update_city_weather;
    Prologue { address: 0x922c40, bytes: &[0x81, 0xec, 0xd0, 0x07, 0x00, 0x00] } => read_region, compare_read_region;
    Prologue { address: 0x922d20, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0xe0, 0x0a, 0x00, 0x00] } => read_region_period, compare_read_region_period;
    Prologue { address: 0x5d5f20, bytes: &[0x55, 0x8b, 0xec, 0x83, 0xe4, 0xf8] } => game_update_nation_coefficients, compare_game_update_nation_coefficients;
    Prologue { address: 0x5d6250, bytes: &[0x66, 0x8b, 0x44, 0x24, 0x04] } => is_player_regen_day, compare_is_player_regen_day;
    Prologue { address: 0x5d61d0, bytes: &[0x66, 0x0f, 0xbe, 0x4c, 0x24, 0x04] } => clamp_char_sum, compare_clamp_char_sum;
    Prologue { address: 0x85da00, bytes: &[0x83, 0xec, 0x0c, 0xa1, 0xe4, 0xf2, 0x9c, 0x00] } => score_venue_for_fixture, compare_score_venue_for_fixture;
    Prologue { address: 0x85d8a0, bytes: &[0x83, 0xec, 0x10, 0x8b, 0x4c, 0x24, 0x14] } => get_travel_distance_factor, compare_get_travel_distance_factor;
    Prologue { address: 0x922810, bytes: &[0x81, 0xec, 0xf0, 0x05, 0x00, 0x00] } => load_config, compare_load_config;
    Prologue { address: 0x8b41d0, bytes: &[0x83, 0xec, 0x18, 0x8b, 0x44, 0x24, 0x1c] } => get_coaching_factor, compare_get_coaching_factor;
    Prologue { address: 0x8b4cd0, bytes: &[0x8b, 0x54, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => get_coach_assignments, compare_get_coach_assignments;
    Prologue { address: 0x8b4f60, bytes: &[0x51, 0x53, 0x56, 0x57, 0x8b, 0xf9] } => get_training_load, compare_get_training_load;
    Prologue { address: 0x8b2be0, bytes: &[0x81, 0xec, 0x0c, 0x02, 0x00, 0x00] } => apply_training_progress, compare_apply_training_progress;
    Prologue { address: 0x8b4b20, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x08, 0x02, 0x00, 0x00] } => find_position_training, compare_find_position_training;
    Prologue { address: 0x8b4db0, bytes: &[0x51, 0x8b, 0x54, 0x24, 0x08] } => has_fast_position_training, compare_has_fast_position_training;
    Prologue { address: 0x8b1730, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x56] } => advance_week_counter, compare_advance_week_counter;
    Prologue { address: 0x7a6e30, bytes: &[0x8b, 0x41, 0x1c, 0x56, 0x8b, 0x74, 0x24, 0x0c] } => take_spare_non_player_record, compare_take_spare_non_player_record;
    Prologue { address: 0x62ed30, bytes: &[0x8b, 0x54, 0x24, 0x04, 0x56] } => get_recurring_injury_type, compare_get_recurring_injury_type;
    Prologue { address: 0x5c2460, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] } => friendly_update_for_date, compare_friendly_update_for_date;
    Prologue { address: 0x5c27a0, bytes: &[0x81, 0xec, 0x08, 0x02, 0x00, 0x00] } => plan_ai_preseason_friendlies, compare_plan_ai_preseason_friendlies;
    Prologue { address: 0x5c2d40, bytes: &[0x6a, 0xff, 0x68, 0x4e, 0x9e, 0x95, 0x00] } => set_preseason_windows, compare_set_preseason_windows;
    Prologue { address: 0x5c4040, bytes: &[0x6a, 0xff, 0x68, 0x9e, 0x9f, 0x95, 0x00] } => schedule_tour_matches, compare_schedule_tour_matches;
    Prologue { address: 0x5c5270, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x8b, 0x54, 0x24, 0x08] } => compare_friendly_candidates_by_band, compare_compare_friendly_candidates_by_band;
    Prologue { address: 0x5c5290, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x8b, 0x54, 0x24, 0x08] } => compare_friendly_candidate_ptrs_by_nation, compare_compare_friendly_candidate_ptrs_by_nation;
    Prologue { address: 0x5223a0, bytes: &[0x81, 0xec, 0x10, 0x02, 0x00, 0x00] } => cup_rank_teams, compare_cup_rank_teams;
    Prologue { address: 0x4c08f0, bytes: &[0x81, 0xec, 0x04, 0x02, 0x00, 0x00] } => comp_util_sort_clubs_by_strength, compare_comp_util_sort_clubs_by_strength;
    Prologue { address: 0x4a2540, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => set_club_final_position, compare_set_club_final_position;
    Prologue { address: 0x669340, bytes: &[0x8b, 0x54, 0x24, 0x04, 0x32, 0xc0] } => key_nation_index_of_nation, compare_key_nation_index_of_nation;
    Prologue { address: 0x669360, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => key_nation_index_of_club, compare_key_nation_index_of_club;
    Prologue { address: 0x669950, bytes: &[0x51, 0x56, 0x8b, 0x74, 0x24, 0x0c] } => key_nation_get_club_season_year, compare_key_nation_get_club_season_year;
    Prologue { address: 0x669bd0, bytes: &[0x6a, 0xff, 0x68, 0x58, 0xc6, 0x95, 0x00] } => key_nation_club_in_season, compare_key_nation_club_in_season;
    Prologue { address: 0x6695c0, bytes: &[0xa1, 0x08, 0x2d, 0xae, 0x00] } => key_nation_get_main_continent, compare_key_nation_get_main_continent;
    Prologue { address: 0x669600, bytes: &[0xa1, 0x08, 0x2d, 0xae, 0x00] } => key_nation_get_main_nation, compare_key_nation_get_main_nation;
    Prologue { address: 0x7c1e30, bytes: &[0x81, 0xec, 0x0c, 0x02, 0x00, 0x00] } => get_stat, compare_get_stat;
    Prologue { address: 0x7c6d40, bytes: &[0x8b, 0x54, 0x24, 0x04, 0x56] } => player_stats_compare_language_entries, compare_player_stats_compare_language_entries;
    Prologue { address: 0x7c6d90, bytes: &[0x56, 0x8b, 0x74, 0x24, 0x08] } => player_stats_compare_history_by_club, compare_player_stats_compare_history_by_club;
    Prologue { address: 0x867b50, bytes: &[0x8a, 0x51, 0x1c, 0xb0, 0x02] } => has_active_release_clause, compare_has_active_release_clause;
    Prologue { address: 0x868210, bytes: &[0x6a, 0xff, 0x68, 0x08, 0x2b, 0x96, 0x00] } => check_fifa_protected_period, compare_check_fifa_protected_period;
    Prologue { address: 0x867aa0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x8b, 0x40, 0x61] } => staff_contract_is_bonus_type_allowed, compare_staff_contract_is_bonus_type_allowed;
    Prologue { address: 0x923630, bytes: &[0x53, 0x56, 0x8b, 0x74, 0x24, 0x0c] } => check_postponement, compare_check_postponement;
    Prologue { address: 0x5a0590, bytes: &[0x81, 0xec, 0x04, 0x02, 0x00, 0x00] } => get_club_finance, compare_get_club_finance;
    Prologue { address: 0x85e380, bytes: &[0x53, 0x8b, 0x5c, 0x24, 0x08] } => stadium_compare_bookings, compare_stadium_compare_bookings;
    Prologue { address: 0x923010, bytes: &[0x51, 0x53, 0x55, 0x8b, 0xe9] } => get_match_weather, compare_get_match_weather;
    Prologue { address: 0x9232c0, bytes: &[0x0f, 0xbe, 0x4c, 0x24, 0x04] } => get_temperature, compare_get_temperature;
    Prologue { address: 0x923970, bytes: &[0x8b, 0x4c, 0x24, 0x08, 0x53] } => get_pitch_condition, compare_get_pitch_condition;
    Prologue { address: 0x5986b0, bytes: &[0x53, 0x56, 0x8b, 0xf1, 0x8b, 0x15, 0xb4, 0x23, 0xae, 0x00] } => get_wealth_band, compare_get_wealth_band;
    Prologue { address: 0x5d6220, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x8b, 0x4c, 0x24, 0x08] } => compare_players_by_ability, compare_compare_players_by_ability;
    Prologue { address: 0x85e350, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x8b, 0x48, 0x0c] } => stadium_compare_venues_by_capacity, compare_stadium_compare_venues_by_capacity;
    Prologue { address: 0x8b3d40, bytes: &[0x81, 0xec, 0x0c, 0x0c, 0x00, 0x00] } => set_default_schedules, compare_set_default_schedules;
    Prologue { address: 0x8b4e50, bytes: &[0x0f, 0xbe, 0x44, 0x24, 0x04] } => set_type, compare_set_type;
    Prologue { address: 0x8b3930, bytes: &[0x8b, 0x54, 0x24, 0x04, 0xa1, 0x64, 0x23, 0xae, 0x00] } => get_manager_schedule, compare_get_manager_schedule;
    Prologue { address: 0x8b3c50, bytes: &[0xa1, 0x6c, 0x23, 0xae, 0x00] } => init_player_entries, compare_init_player_entries;
    Prologue { address: 0x7a8350, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => add_player_data_to_pool, compare_add_player_data_to_pool;
    Prologue { address: 0x7a6e20, bytes: &[0x8b, 0x41, 0x20, 0x85, 0xc0] } => has_spare_non_player_record, compare_has_spare_non_player_record;
    Prologue { address: 0x7e8e10, bytes: &[0x83, 0xec, 0x08, 0x53, 0x55] } => make_points_ban, compare_make_points_ban;
    Prologue { address: 0x7e7ed0, bytes: &[0x83, 0xec, 0x08, 0x56, 0x8b, 0xf1] } => ruling_body_start_new_season, compare_ruling_body_start_new_season;
    Prologue { address: 0x7e9050, bytes: &[0x81, 0xec, 0x04, 0x02, 0x00, 0x00] } => compact_entries, compare_compact_entries;
    Prologue { address: 0x5b3560, bytes: &[0x81, 0xec, 0x80, 0x02, 0x00, 0x00] } => set_slot_coordinate, compare_set_slot_coordinate;
    Prologue { address: 0x5b3460, bytes: &[0x8a, 0x54, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => check_duplicate_position, compare_check_duplicate_position;
    Prologue { address: 0x5b57d0, bytes: &[0x8a, 0x44, 0x24, 0x08, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => set_marking_type, compare_set_marking_type;
    Prologue { address: 0x686a70, bytes: &[0x81, 0xec, 0x68, 0x02, 0x00, 0x00] } => draw_team_order, compare_draw_team_order;
    Prologue { address: 0x521790, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => get_fixture_side_teams, compare_get_fixture_side_teams;
    Prologue { address: 0x5219b0, bytes: &[0x8b, 0x54, 0x24, 0x0c, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => collect_possible_teams, compare_collect_possible_teams;
    Prologue { address: 0x7bca90, bytes: &[0x83, 0xec, 0x08, 0x53, 0x8b, 0x5c, 0x24, 0x10] } => player_search_get_club_target_rating, compare_player_search_get_club_target_rating;
    Prologue { address: 0x7bd830, bytes: &[0x8a, 0x54, 0x24, 0x04, 0x8a, 0x81, 0xa1, 0x01, 0x00, 0x00] } => player_search_slot_get_match_entry, compare_player_search_slot_get_match_entry;
    Prologue { address: 0x7bd870, bytes: &[0x8a, 0x81, 0xa1, 0x01, 0x00, 0x00] } => player_search_slot_sum_goals, compare_player_search_slot_sum_goals;
    Prologue { address: 0x8c2f50, bytes: &[0x83, 0xec, 0x0c, 0x53, 0x55] } => advance_offer_to_club_approval, compare_advance_offer_to_club_approval;
    Prologue { address: 0x8d76d0, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] } => post_offer_accepted_news, compare_post_offer_accepted_news;
    Prologue { address: 0x8e1780, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] } => post_loan_offer_news, compare_post_loan_offer_news;
    Prologue { address: 0x8c28c0, bytes: &[0x81, 0xec, 0x14, 0x02, 0x00, 0x00] } => handle_player_rejected_terms, compare_handle_player_rejected_terms;
    Prologue { address: 0x6a4c40, bytes: &[0x6a, 0xff, 0x68, 0x6b, 0xce, 0x95, 0x00] } => post_board_confidence_update_news, compare_post_board_confidence_update_news;
    Prologue { address: 0x6a4a20, bytes: &[0x6a, 0xff, 0x68, 0x4b, 0xce, 0x95, 0x00] } => post_board_expectations_news, compare_post_board_expectations_news;
    Prologue { address: 0x8c4e10, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x83, 0xec, 0x0c] } => hold_offers_at_window_close, compare_hold_offers_at_window_close;
    Prologue { address: 0x6a44a0, bytes: &[0x6a, 0xff, 0x68, 0x2b, 0xce, 0x95, 0x00] } => post_fa_expectations_news, compare_post_fa_expectations_news;
    Prologue { address: 0x6a5d10, bytes: &[0x6a, 0xff, 0x68, 0xeb, 0xce, 0x95, 0x00] } => post_supporter_protest_news, compare_post_supporter_protest_news;
    Prologue { address: 0x8be9d0, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] } => add_transfer_negotiation, compare_add_transfer_negotiation;
    Prologue { address: 0x69fd50, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] } => post_job_rumour_news, compare_post_job_rumour_news;
    Prologue { address: 0x8d8310, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] } => post_player_rejects_move_news, compare_post_player_rejects_move_news;
    Prologue { address: 0x59b7e0, bytes: &[0x51, 0x53, 0x56, 0x8b, 0x74, 0x24, 0x10] } => add_weekly_merchandising_income, compare_add_weekly_merchandising_income;
    Prologue { address: 0x599a20, bytes: &[0x83, 0xec, 0x14, 0x8b, 0x15, 0xb4, 0x23, 0xae, 0x00] } => sell_season_tickets, compare_sell_season_tickets;
    Prologue { address: 0x59b630, bytes: &[0x53, 0x56, 0x57, 0x8b, 0x7c, 0x24, 0x10] } => upgrade_training_facilities, compare_upgrade_training_facilities;
    Prologue { address: 0x59ed70, bytes: &[0x81, 0xec, 0x08, 0x02, 0x00, 0x00] } => process_club_finance_events, compare_process_club_finance_events;
    Prologue { address: 0x59dd40, bytes: &[0x55, 0x8b, 0xec, 0x83, 0xe4, 0xf8] } => try_board_debt_payment, compare_try_board_debt_payment;
    Prologue { address: 0x59e5a0, bytes: &[0x83, 0xec, 0x0c, 0x53, 0x55] } => try_cash_injection, compare_try_cash_injection;
    Prologue { address: 0x597db0, bytes: &[0x83, 0xec, 0x18, 0x8b, 0x41, 0x08] } => get_max_staff_wage, compare_get_max_staff_wage;
    Prologue { address: 0x599960, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x8b, 0x91, 0x38, 0x01, 0x00, 0x00] } => pay_bonuses, compare_pay_bonuses;
    Prologue { address: 0x598340, bytes: &[0x0f, 0xbe, 0x44, 0x24, 0x04] } => adjust_max_wage, compare_adjust_max_wage;
    Prologue { address: 0x62c080, bytes: &[0x83, 0xec, 0x20, 0x53, 0x55] } => apply_match_effects, compare_apply_match_effects;
    Prologue { address: 0x62bf60, bytes: &[0x56, 0x8b, 0xf1, 0x57, 0x8b, 0x46, 0x28] } => find_match_injury, compare_find_match_injury;
    Prologue { address: 0x631550, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x8b, 0x0d, 0x28, 0x3d, 0xb6, 0x00] } => is_player_on_holiday, compare_is_player_on_holiday;
    Prologue { address: 0x7a6f10, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] } => player_regen_sign_for_club, compare_player_regen_sign_for_club;
    Prologue { address: 0x7a7f20, bytes: &[0x6a, 0xff, 0x68, 0xf0, 0xf1, 0x95, 0x00] } => schedule_injury_retirement, compare_schedule_injury_retirement;
    Prologue { address: 0x7aa340, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] } => player_regen_post_retirement_announcement_news, compare_player_regen_post_retirement_announcement_news;
    Prologue { address: 0x62e700, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => end_player_injury, compare_end_player_injury;
    Prologue { address: 0x72bbf0, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] } => media_queue_player_apology_news, compare_media_queue_player_apology_news;
    Prologue { address: 0x787630, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => clear_inbox, compare_clear_inbox;
    Prologue { address: 0x775cd0, bytes: &[0x81, 0xec, 0x10, 0x02, 0x00, 0x00] } => withdraw_injured_player, compare_withdraw_injured_player;
    Prologue { address: 0x76f7d0, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] } => national_teams_post_player_withdrawn_news, compare_national_teams_post_player_withdrawn_news;
    Prologue { address: 0x7701f0, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] } => national_teams_post_replacement_needed_news, compare_national_teams_post_replacement_needed_news;
    Prologue { address: 0x76db00, bytes: &[0x6a, 0xff, 0x68, 0xdb, 0xdd, 0x95, 0x00] } => national_teams_can_change_squad, compare_national_teams_can_change_squad;
    Prologue { address: 0x76da80, bytes: &[0x56, 0x8b, 0x74, 0x24, 0x08] } => national_teams_player_in_squad, compare_national_teams_player_in_squad;
    Prologue { address: 0x5c57d0, bytes: &[0x8b, 0x54, 0x24, 0x04, 0x53] } => schedule_assign_opponent, compare_schedule_assign_opponent;
    Prologue { address: 0x86c8c0, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] } => add_month_award_record, compare_add_month_award_record;
    Prologue { address: 0x5c52d0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x66, 0x8b, 0x80, 0x80, 0x00, 0x00, 0x00] } => friendly_club_reputation_tier, compare_friendly_club_reputation_tier;
    Prologue { address: 0x5c74d0, bytes: &[0x0f, 0xbe, 0x44, 0x24, 0x04] } => friendly_get_rest_days_for_kind, compare_friendly_get_rest_days_for_kind;
    Prologue { address: 0x866620, bytes: &[0x6a, 0xff, 0x68, 0x90, 0x2a, 0x96, 0x00] } => get_contract_satisfaction, compare_get_contract_satisfaction;
    Prologue { address: 0x4d9f80, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] } => post_wants_new_contract_news, compare_post_wants_new_contract_news;
    Prologue { address: 0x4d9460, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] } => post_transfer_request_withdrawn_news, compare_post_transfer_request_withdrawn_news;
    Prologue { address: 0x8c3310, bytes: &[0x6a, 0xff, 0x68, 0x10, 0x3c, 0x96, 0x00] } => confirm_offer_by_club, compare_confirm_offer_by_club;
    Prologue { address: 0x8c34e0, bytes: &[0x83, 0xec, 0x08, 0x53, 0x55] } => try_complete_offer, compare_try_complete_offer;
    Prologue { address: 0x8c91e0, bytes: &[0x53, 0x55, 0x56, 0x57, 0x8b, 0x7c, 0x24, 0x14] } => settle_previous_transfer_clauses, compare_settle_previous_transfer_clauses;
    Prologue { address: 0x842740, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] } => try_loan_offer_for_player, compare_try_loan_offer_for_player;
    Prologue { address: 0x4d6790, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] } => post_contract_expiry_news, compare_post_contract_expiry_news;
    Prologue { address: 0x4d6d70, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] } => post_bosman_departure_news, compare_post_bosman_departure_news;
    Prologue { address: 0x4cc490, bytes: &[0x6a, 0xff, 0x68, 0x29, 0x6d, 0x95, 0x00] } => add_contract_offer, compare_add_contract_offer;
    Prologue { address: 0x4ccb30, bytes: &[0x53, 0x55, 0x56, 0x57, 0x8b, 0xf9] } => remove_contract_offer_for_transfer, compare_remove_contract_offer_for_transfer;
    Prologue { address: 0x4ce450, bytes: &[0x6a, 0xff, 0x68, 0x88, 0x6d, 0x95, 0x00] } => contract_manager_lowest_wage_demand, compare_contract_manager_lowest_wage_demand;
    Prologue { address: 0x767ee0, bytes: &[0x8b, 0x44, 0x24, 0x08, 0x53] } => record_match_ratings, compare_record_match_ratings;
    Prologue { address: 0x767fa0, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => add_player_rating, compare_add_player_rating;
    Prologue { address: 0x768130, bytes: &[0x83, 0xec, 0x0c, 0x53, 0x55] } => find_best_player, compare_find_best_player;
    Prologue { address: 0x767150, bytes: &[0x56, 0x57, 0x8b, 0xf9, 0x8b, 0x47, 0x09] } => award_best_rated_player, compare_award_best_rated_player;
    Prologue { address: 0x7671c0, bytes: &[0x51, 0x53, 0x8b, 0x5c, 0x24, 0x14] } => set_winner, compare_set_winner;
    Prologue { address: 0x412f00, bytes: &[0x53, 0x55, 0x8b, 0x4c, 0x24, 0x1c] } => award_raise_winner_reputation, compare_award_raise_winner_reputation;
    Prologue { address: 0x62c680, bytes: &[0x6a, 0xff, 0x68, 0xf8, 0xb0, 0x95, 0x00] } => update_player_fitness, compare_update_player_fitness;
    Prologue { address: 0x690bd0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x8b, 0x09] } => get_league_confidence_band, compare_get_league_confidence_band;
    Prologue { address: 0x69f7c0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x56] } => set_board_ratings, compare_set_board_ratings;
    Prologue { address: 0x62b7b0, bytes: &[0x83, 0xec, 0x1c, 0x53, 0x8b, 0xd9] } => injury_daily_update, compare_injury_daily_update;
    Prologue { address: 0x62e010, bytes: &[0x6a, 0xff, 0x68, 0x68, 0xb1, 0x95, 0x00] } => post_due_recovery_news, compare_post_due_recovery_news;
    Prologue { address: 0x55d360, bytes: &[0x56, 0x8b, 0x74, 0x24, 0x08] } => is_one_booking_from_ban, compare_is_one_booking_from_ban;
    Prologue { address: 0x7a65d0, bytes: &[0x81, 0xec, 0x10, 0x02, 0x00, 0x00] } => process_regen_queue, compare_process_regen_queue;
    Prologue { address: 0x757f30, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x66, 0x81, 0xb8, 0x80, 0x00, 0x00, 0x00, 0x6a, 0x18] } => media_pick_outlet_type_for_club, compare_media_pick_outlet_type_for_club;
    Prologue { address: 0x757fa0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x8b, 0x48, 0x61] } => media_pick_outlet_type_for_person, compare_media_pick_outlet_type_for_person;
    Prologue { address: 0x685d30, bytes: &[0x81, 0xec, 0x10, 0x02, 0x00, 0x00] } => league_add_result, compare_league_add_result;
    Prologue { address: 0x687710, bytes: &[0x81, 0xec, 0x04, 0x02, 0x00, 0x00] } => copy_table, compare_copy_table;
    Prologue { address: 0x68a850, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => league_set_predicted_positions, compare_league_set_predicted_positions;
    Prologue { address: 0x76b600, bytes: &[0x83, 0xec, 0x60, 0x56, 0x57] } => record_international_result, compare_record_international_result;
    Prologue { address: 0x775610, bytes: &[0x8b, 0x44, 0x24, 0x10, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => format_squad_label, compare_format_squad_label;
    Prologue { address: 0x48d410, bytes: &[0x8b, 0x54, 0x24, 0x04, 0x8b, 0xc1] } => are_round_fixtures_known, compare_are_round_fixtures_known;
    Prologue { address: 0x7771c0, bytes: &[0x6a, 0xff, 0x68, 0x6b, 0xdf, 0x95, 0x00] } => record_club_call_ups, compare_record_club_call_ups;
    Prologue { address: 0x48e190, bytes: &[0x8b, 0x54, 0x24, 0x04, 0x56] } => get_fixture_match_importance, compare_get_fixture_match_importance;
    Prologue { address: 0x48d200, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => get_stage, compare_get_stage;
    Prologue { address: 0x4190d0, bytes: &[0x83, 0xec, 0x1c, 0x53, 0x55] } => awol_daily_update, compare_awol_daily_update;
    Prologue { address: 0x418ff0, bytes: &[0x56, 0x8b, 0x74, 0x24, 0x08] } => has_missed_training, compare_has_missed_training;
    Prologue { address: 0x419070, bytes: &[0x53, 0x8b, 0x5c, 0x24, 0x08] } => has_fitness_incident, compare_has_fitness_incident;
    Prologue { address: 0x41a360, bytes: &[0x6a, 0xff, 0x68, 0xe8, 0x3e, 0x95, 0x00] } => miss_training, compare_miss_training;
    Prologue { address: 0x419710, bytes: &[0x56, 0x57, 0x8b, 0x7c, 0x24, 0x0c] } => end_absence, compare_end_absence;
    Prologue { address: 0x867560, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => set_flags, compare_set_flags;
    Prologue { address: 0x8bf680, bytes: &[0x51, 0x53, 0x8b, 0x5c, 0x24, 0x0c] } => reject_transfer_offer, compare_reject_transfer_offer;
    Prologue { address: 0x8bfcb0, bytes: &[0x51, 0x53, 0x55, 0x56, 0x57] } => withdraw_transfer_offer, compare_withdraw_transfer_offer;
    Prologue { address: 0x8d5170, bytes: &[0x6a, 0xff, 0x68, 0x2b, 0x43, 0x96, 0x00] } => post_offer_rejected_news, compare_post_offer_rejected_news;
    Prologue { address: 0x8c1290, bytes: &[0x83, 0xec, 0x20, 0x53, 0x8b, 0x5c, 0x24, 0x28] } => unsettle_player_after_bid, compare_unsettle_player_after_bid;
    Prologue { address: 0x8de790, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] } => post_offer_collapsed_news, compare_post_offer_collapsed_news;
    Prologue { address: 0x8db8a0, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] } => post_appearance_money_news, compare_post_appearance_money_news;
    Prologue { address: 0x8c9390, bytes: &[0x51, 0x53, 0x55, 0x56, 0x8b, 0x74, 0x24, 0x14] } => finalise_transfer, compare_finalise_transfer;
    Prologue { address: 0x8dbef0, bytes: &[0x6a, 0xff, 0x68, 0x5b, 0x45, 0x96, 0x00] } => post_transfer_completed_news, compare_post_transfer_completed_news;
    Prologue { address: 0x8e05d0, bytes: &[0x6a, 0xff, 0x68, 0x9b, 0x46, 0x96, 0x00] } => post_transfer_agreed_news, compare_post_transfer_agreed_news;
    Prologue { address: 0x8bf400, bytes: &[0x6a, 0xff, 0x68, 0x28, 0x3b, 0x96, 0x00] } => add_config_loan, compare_add_config_loan;
    Prologue { address: 0x4da590, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] } => post_contract_end_news, compare_post_contract_end_news;
    Prologue { address: 0x8dda90, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] } => post_transfer_delay_request_news, compare_post_transfer_delay_request_news;
    Prologue { address: 0x896660, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => get_club_tactic, compare_get_club_tactic;
    Prologue { address: 0x5fbe20, bytes: &[0x8b, 0x44, 0x24, 0x0c, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => select_tactic_slot, compare_select_tactic_slot;
    Prologue { address: 0x5fbd00, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => get_active_tactic, compare_get_active_tactic;
    Prologue { address: 0x896c60, bytes: &[0x81, 0xec, 0x0c, 0x02, 0x00, 0x00] } => remove_player_from_selection, compare_remove_player_from_selection;
    Prologue { address: 0x8973e0, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => swap_selection_slots, compare_swap_selection_slots;
    Prologue { address: 0x897250, bytes: &[0x8a, 0x81, 0xf0, 0x05, 0x00, 0x00] } => has_selection_undo, compare_has_selection_undo;
    Prologue { address: 0x6a0e70, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] } => post_manager_under_pressure_news, compare_post_manager_under_pressure_news;
    Prologue { address: 0x6a1b00, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] } => post_manager_departure_news, compare_post_manager_departure_news;
    Prologue { address: 0x6a2e00, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] } => post_manager_appointment_news, compare_post_manager_appointment_news;
    Prologue { address: 0x682e10, bytes: &[0x81, 0xec, 0x04, 0x02, 0x00, 0x00] } => pay_prize_money, compare_pay_prize_money;
    Prologue { address: 0x686940, bytes: &[0x8a, 0x44, 0x24, 0x08, 0x3c, 0x05] } => league_update_staff_records_for_status, compare_league_update_staff_records_for_status;
    Prologue { address: 0x686670, bytes: &[0x6a, 0xff, 0x68, 0x8b, 0xca, 0x95, 0x00] } => announce_team_status, compare_announce_team_status;
    Prologue { address: 0x7c0d70, bytes: &[0x55, 0x8b, 0xec, 0x83, 0xe4, 0xf8] } => player_stats_record_match, compare_player_stats_record_match;
    Prologue { address: 0x447430, bytes: &[0x81, 0xec, 0x20, 0x02, 0x00, 0x00] } => update_league_position_records, compare_update_league_position_records;
    Prologue { address: 0x448e30, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] } => send_record_news, compare_send_record_news;
    Prologue { address: 0x76bab0, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] } => select_squads_for_fixture, compare_select_squads_for_fixture;
    Prologue { address: 0x76fd10, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] } => national_teams_post_player_called_up_news, compare_national_teams_post_player_called_up_news;
    Prologue { address: 0x519690, bytes: &[0x6a, 0xff, 0x68, 0xdb, 0x75, 0x95, 0x00] } => add_team_result_news, compare_add_team_result_news;
    Prologue { address: 0x844540, bytes: &[0x83, 0xec, 0x14, 0x53, 0x8b, 0x5c, 0x24, 0x1c] } => count_slots_with_bids, compare_count_slots_with_bids;
    Prologue { address: 0x844280, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x8b, 0x0d, 0x50, 0x10, 0xae, 0x00] } => shortlist_manager_remove_from_transfer_list, compare_shortlist_manager_remove_from_transfer_list;
    Prologue { address: 0x8442d0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x8b, 0x0d, 0x50, 0x10, 0xae, 0x00] } => shortlist_manager_remove_from_loan_list, compare_shortlist_manager_remove_from_loan_list;
    Prologue { address: 0x843e10, bytes: &[0x6a, 0xff, 0x68, 0x18, 0x25, 0x96, 0x00] } => update_staff_status, compare_update_staff_status;
    Prologue { address: 0x8442b0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x8b, 0x0d, 0x50, 0x10, 0xae, 0x00] } => shortlist_manager_add_to_loan_list, compare_shortlist_manager_add_to_loan_list;
    Prologue { address: 0x84b1f0, bytes: &[0x8b, 0x44, 0x24, 0x08, 0x56] } => shortlist_manager_max_fee_for_player_status, compare_shortlist_manager_max_fee_for_player_status;
    Prologue { address: 0x83b6e0, bytes: &[0x83, 0xec, 0x2c, 0x53, 0x55] } => allocate_transfer_budget, compare_allocate_transfer_budget;
    Prologue { address: 0x84b4d0, bytes: &[0x8b, 0x4c, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => shortlist_manager_squad_status_allowed, compare_shortlist_manager_squad_status_allowed;
    Prologue { address: 0x7fc150, bytes: &[0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => scout_manager_write_list_name, compare_scout_manager_write_list_name;
    Prologue { address: 0x5cbb00, bytes: &[0x6a, 0xff, 0x68, 0xa0, 0xa3, 0x95, 0x00] } => update_international_friendlies, compare_update_international_friendlies;
    Prologue { address: 0x7f6e30, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => get_selected_list, compare_get_selected_list;
    Prologue { address: 0x7f6f20, bytes: &[0x8b, 0x44, 0x24, 0x08, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => set_selected_list, compare_set_selected_list;
    Prologue { address: 0x584fa0, bytes: &[0x81, 0xec, 0x14, 0x02, 0x00, 0x00] } => european_cup_collect_entrants, compare_european_cup_collect_entrants;
    Prologue { address: 0x584990, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => build_qualifying_entry_table, compare_build_qualifying_entry_table;
    Prologue { address: 0x5851d0, bytes: &[0x8b, 0x51, 0x2c, 0x8b, 0x41, 0x30] } => european_cup_advance_stage, compare_european_cup_advance_stage;
    Prologue { address: 0x583b10, bytes: &[0x53, 0x56, 0x8b, 0x74, 0x24, 0x0c] } => process_fixture, compare_process_fixture;
    Prologue { address: 0x586c00, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x8b, 0x49, 0x08] } => european_cup_set_club_progress, compare_european_cup_set_club_progress;
    Prologue { address: 0x48e360, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x0f, 0xbf, 0x48, 0x30] } => counts_for_fixture_clash, compare_counts_for_fixture_clash;
    Prologue { address: 0x79c7b0, bytes: &[0x83, 0xec, 0x08, 0x53, 0x55, 0x56] } => choose_match_official, compare_choose_match_official;
    Prologue { address: 0x79bf40, bytes: &[0x8b, 0x51, 0x04, 0x33, 0xc0] } => clear_busy_flags, compare_clear_busy_flags;
    Prologue { address: 0x79cee0, bytes: &[0x81, 0xec, 0x08, 0x02, 0x00, 0x00] } => renew_officials, compare_renew_officials;
    Prologue { address: 0x79c5c0, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => officials_manager_add_random_officials, compare_officials_manager_add_random_officials;
    Prologue { address: 0x79caa0, bytes: &[0x8b, 0x0d, 0x90, 0x23, 0xae, 0x00] } => officials_manager_validate_officials, compare_officials_manager_validate_officials;
    Prologue { address: 0x79c530, bytes: &[0x83, 0xec, 0x08, 0x56, 0x57] } => officials_manager_operator, compare_officials_manager_operator;
    Prologue { address: 0x86c390, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] } => add_year_award_record, compare_add_year_award_record;
    Prologue { address: 0x7e4a30, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => match_record_is_bigger_win, compare_match_record_is_bigger_win;
    Prologue { address: 0x7e4b00, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => match_record_is_bigger_defeat, compare_match_record_is_bigger_defeat;
    Prologue { address: 0x8c39f0, bytes: &[0x81, 0xec, 0xc8, 0x01, 0x00, 0x00] } => update_appearance_clauses_after_match, compare_update_appearance_clauses_after_match;
    Prologue { address: 0x8cad90, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => add_international_team_appearance_money, compare_add_international_team_appearance_money;
    Prologue { address: 0x8da5e0, bytes: &[0x6a, 0xff, 0x68, 0x96, 0x44, 0x96, 0x00] } => post_transfer_finalised_news, compare_post_transfer_finalised_news;
    Prologue { address: 0x696b10, bytes: &[0x83, 0xec, 0x14, 0x8a, 0x44, 0x24, 0x24] } => check_board_transfer_veto, compare_check_board_transfer_veto;
    Prologue { address: 0x6ac700, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] } => post_board_ultimatum_news, compare_post_board_ultimatum_news;
    Prologue { address: 0x6ac880, bytes: &[0x81, 0xec, 0x08, 0x02, 0x00, 0x00] } => news_board_ultimatum_text, compare_news_board_ultimatum_text;
    Prologue { address: 0x8c1af0, bytes: &[0x81, 0xec, 0x08, 0x02, 0x00, 0x00] } => free_transfer_staff, compare_free_transfer_staff;
    Prologue { address: 0x5fd2f0, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => set_on_holiday, compare_set_on_holiday;
    Prologue { address: 0x8d7cd0, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] } => post_contract_talks_news, compare_post_contract_talks_news;
    Prologue { address: 0x573080, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] } => eng_first_create_playoff_stage, compare_eng_first_create_playoff_stage;
    Prologue { address: 0x576a70, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] } => eng_second_create_play_off_stage, compare_eng_second_create_play_off_stage;
    Prologue { address: 0x5783c0, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] } => eng_third_create_play_off_stage, compare_eng_third_create_play_off_stage;
    Prologue { address: 0x576c50, bytes: &[0x8a, 0x44, 0x24, 0x0c, 0x53] } => eng_second_update_staff_records_for_status, compare_eng_second_update_staff_records_for_status;
    Prologue { address: 0x573260, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x8b, 0x49, 0x08] } => eng_first_set_club_progress, compare_eng_first_set_club_progress;
    Prologue { address: 0x578660, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x8b, 0x49, 0x08] } => eng_third_set_club_progress, compare_eng_third_set_club_progress;
    Prologue { address: 0x684cd0, bytes: &[0x81, 0xec, 0x14, 0x02, 0x00, 0x00] } => update_positions_and_status, compare_update_positions_and_status;
    Prologue { address: 0x572ff0, bytes: &[0x8b, 0x51, 0x2c, 0x8b, 0x41, 0x30] } => eng_first_advance_stage, compare_eng_first_advance_stage;
    Prologue { address: 0x5769e0, bytes: &[0x8b, 0x51, 0x2c, 0x8b, 0x41, 0x30] } => eng_second_advance_stage, compare_eng_second_advance_stage;
    Prologue { address: 0x584ae0, bytes: &[0x8a, 0x44, 0x24, 0x0c, 0x53] } => european_cup_update_staff_records_for_status, compare_european_cup_update_staff_records_for_status;
    Prologue { address: 0x586330, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] } => draw_quarter_finals, compare_draw_quarter_finals;
    Prologue { address: 0x586660, bytes: &[0x8a, 0x44, 0x24, 0x08, 0x81, 0xec, 0x04, 0x02, 0x00, 0x00] } => european_cup_collect_club_list, compare_european_cup_collect_club_list;
    Prologue { address: 0x896290, bytes: &[0x55, 0x8b, 0xec, 0x83, 0xe4, 0xf8] } => record_tactic_result, compare_record_tactic_result;
    Prologue { address: 0x8976a0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => get_captain, compare_get_captain;
    Prologue { address: 0x897880, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => set_captain, compare_set_captain;
    Prologue { address: 0x7757d0, bytes: &[0x6a, 0xff, 0x68, 0xf0, 0xde, 0x95, 0x00] } => get_international_duty_club, compare_get_international_duty_club;
    Prologue { address: 0x76d9b0, bytes: &[0x8b, 0x44, 0x24, 0x08, 0x56] } => national_teams_player_is_eligible, compare_national_teams_player_is_eligible;
    Prologue { address: 0x775580, bytes: &[0x53, 0x55, 0x8b, 0xe9, 0x32, 0xdb] } => collect_duty_competitions, compare_collect_duty_competitions;
    Prologue { address: 0x7e4750, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] } => fill_match_record_from_fixture, compare_fill_match_record_from_fixture;
    Prologue { address: 0x7e4920, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x08, 0x02, 0x00, 0x00] } => fill_staff_name_ref, compare_fill_staff_name_ref;
    Prologue { address: 0x7e56d0, bytes: &[0x8b, 0x4c, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => match_record_result_code, compare_match_record_result_code;
    Prologue { address: 0x5ae9d0, bytes: &[0x6a, 0xff, 0x68, 0x46, 0x98, 0x95, 0x00] } => move_nearby_club_fixtures, compare_move_nearby_club_fixtures;
    Prologue { address: 0x5abb10, bytes: &[0x6a, 0xff, 0x68, 0x40, 0x97, 0x95, 0x00] } => reschedule_club_fixtures_near_date, compare_reschedule_club_fixtures_near_date;
    Prologue { address: 0x5aac70, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] } => queue_fixture_moved_news, compare_queue_fixture_moved_news;
    Prologue { address: 0x41add0, bytes: &[0x6a, 0xff, 0x68, 0x36, 0x3f, 0x95, 0x00] } => report_poor_fitness, compare_report_poor_fitness;
    Prologue { address: 0x5a74b0, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] } => post_fine_reaction_news, compare_post_fine_reaction_news;
    Prologue { address: 0x5a7f20, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] } => post_coach_fine_feedback_news, compare_post_coach_fine_feedback_news;
    Prologue { address: 0x7dab90, bytes: &[0xb8, 0x94, 0x10, 0x00, 0x00] } => pick_random_pundit, compare_pick_random_pundit;
    Prologue { address: 0x7dad20, bytes: &[0x53, 0x55, 0x56, 0x8b, 0xf1] } => get_pundit_name, compare_get_pundit_name;
    Prologue { address: 0x758de0, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] } => queue_manager_fa_complaint_story, compare_queue_manager_fa_complaint_story;
    Prologue { address: 0x598370, bytes: &[0x83, 0xec, 0x0c, 0x53, 0x55, 0x56] } => update_wage_bill, compare_update_wage_bill;
    Prologue { address: 0x59cf50, bytes: &[0xa1, 0x5c, 0x23, 0xae, 0x00] } => refresh_all_wage_bills, compare_refresh_all_wage_bills;
    Prologue { address: 0x59b4c0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => finance_get_days_to_season_start, compare_finance_get_days_to_season_start;
    Prologue { address: 0x599800, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x8b, 0x54, 0x24, 0x08] } => add_players_sold, compare_add_players_sold;
    Prologue { address: 0x599890, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x8b, 0x54, 0x24, 0x08] } => pay_players_bought, compare_pay_players_bought;
    Prologue { address: 0x5998e0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x8b, 0x91, 0x3c, 0x01, 0x00, 0x00] } => pay_sign_on_fee, compare_pay_sign_on_fee;
    Prologue { address: 0x5999a0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x8b, 0x91, 0x48, 0x01, 0x00, 0x00] } => add_prize_money, compare_add_prize_money;
    Prologue { address: 0x5987c0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x8b, 0x91, 0x58, 0x01, 0x00, 0x00] } => add_tv_revenue, compare_add_tv_revenue;
    Prologue { address: 0x599920, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x8b, 0x91, 0x30, 0x01, 0x00, 0x00] } => pay_wages, compare_pay_wages;
    Prologue { address: 0x522240, bytes: &[0x83, 0xec, 0x08, 0x8b, 0x44, 0x24, 0x0c] } => pay_round_prize_money, compare_pay_round_prize_money;
    Prologue { address: 0x51e870, bytes: &[0x8b, 0x44, 0x24, 0x08, 0x83, 0xec, 0x08] } => set_home_teams_for_round, compare_set_home_teams_for_round;
    Prologue { address: 0x521b30, bytes: &[0x66, 0x8b, 0x41, 0x3c, 0x53] } => record_team_progress, compare_record_team_progress;
    Prologue { address: 0x5c5f40, bytes: &[0x6a, 0xff, 0x68, 0xbb, 0xa0, 0x95, 0x00] } => process_friendly_reply, compare_process_friendly_reply;
    Prologue { address: 0x5c5570, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] } => send_friendly_request, compare_send_friendly_request;
    Prologue { address: 0x5ca3e0, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => build_nation_list, compare_build_nation_list;
    Prologue { address: 0x51c410, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] } => draw_round, compare_draw_round;
    Prologue { address: 0x4bffc0, bytes: &[0x8b, 0x44, 0x24, 0x0c, 0x81, 0xec, 0xc8, 0x02, 0x00, 0x00] } => comp_util_format_stage_round_name, compare_comp_util_format_stage_round_name;
    Prologue { address: 0x522360, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x8b, 0x48, 0x57] } => club_in_higher_division, compare_club_in_higher_division;
    Prologue { address: 0x519890, bytes: &[0x53, 0x56, 0x8b, 0xf1, 0x57] } => cup_update_staff_records_for_status, compare_cup_update_staff_records_for_status;
    Prologue { address: 0x519a90, bytes: &[0x83, 0xec, 0x14, 0x53, 0x55] } => cup_record_season_honours, compare_cup_record_season_honours;
    Prologue { address: 0x519fc0, bytes: &[0x83, 0xec, 0x10, 0x53, 0x55] } => register_semi_final_losers_for_uefa, compare_register_semi_final_losers_for_uefa;
    Prologue { address: 0x8bfa30, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x55] } => reject_loan_offer, compare_reject_loan_offer;
    Prologue { address: 0x8cd450, bytes: &[0x81, 0xec, 0x20, 0x02, 0x00, 0x00] } => check_squad_limits, compare_check_squad_limits;
    Prologue { address: 0x8d0dd0, bytes: &[0x83, 0xec, 0x10, 0x56, 0x57] } => find_next_sale_percentage, compare_find_next_sale_percentage;
    Prologue { address: 0x7af600, bytes: &[0x6a, 0xff, 0x68, 0x86, 0xf3, 0x95, 0x00] } => contains_player, compare_contains_player;
    Prologue { address: 0x7f7270, bytes: &[0x81, 0xec, 0x30, 0x02, 0x00, 0x00] } => sync_scout_lists, compare_sync_scout_lists;
    Prologue { address: 0x7bd700, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => player_search_find_or_claim_player_slot, compare_player_search_find_or_claim_player_slot;
    Prologue { address: 0x840d80, bytes: &[0x55, 0x8b, 0xec, 0x83, 0xe4, 0xf8] } => value_player_for_club, compare_value_player_for_club;
    Prologue { address: 0x7f8610, bytes: &[0x0f, 0xbf, 0x05, 0x7a, 0x7c, 0xb7, 0x00] } => find_club_search_entry, compare_find_club_search_entry;
    Prologue { address: 0x8d0f00, bytes: &[0x83, 0xec, 0x10, 0x56, 0x57] } => find_international_appearance_money, compare_find_international_appearance_money;
    Prologue { address: 0x4ca6d0, bytes: &[0x55, 0x8b, 0xec, 0x83, 0xe4, 0xf8] } => update_after_match, compare_update_after_match;
    Prologue { address: 0x4cc2d0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x8b, 0x48, 0x61] } => contract_manager_get_player_morale, compare_contract_manager_get_player_morale;
    Prologue { address: 0x4cc2e0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x8b, 0x48, 0x61] } => contract_manager_write_morale_text, compare_contract_manager_write_morale_text;
    Prologue { address: 0x4d0c60, bytes: &[0x55, 0x8b, 0xec, 0x83, 0xe4, 0xf8] } => calc_player_value, compare_calc_player_value;
    Prologue { address: 0x4cc3a0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x8b, 0x54, 0x24, 0x08] } => contract_manager_write_morale_comparison_text, compare_contract_manager_write_morale_comparison_text;
    Prologue { address: 0x4cc130, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => get_squad_flags_for_club, compare_get_squad_flags_for_club;
    Prologue { address: 0x6aaee0, bytes: &[0x81, 0xec, 0xdc, 0x00, 0x00, 0x00] } => board_named_players_after_match, compare_board_named_players_after_match;
    Prologue { address: 0x6ab670, bytes: &[0x6a, 0xff, 0x68, 0x1b, 0xd0, 0x95, 0x00] } => get_player_match_rating_verdict, compare_get_player_match_rating_verdict;
    Prologue { address: 0x6ab2d0, bytes: &[0x8b, 0x4c, 0x24, 0x04, 0x81, 0xec, 0x64, 0x02, 0x00, 0x00] } => build_board_player_performance_text, compare_build_board_player_performance_text;
    Prologue { address: 0x868830, bytes: &[0x83, 0xec, 0x08, 0x53, 0x55] } => record_match_result, compare_record_match_result;
    Prologue { address: 0x86a070, bytes: &[0x6a, 0xff, 0x68, 0xdb, 0x2b, 0x96, 0x00] } => add_league_finish_records, compare_add_league_finish_records;
    Prologue { address: 0x86dc30, bytes: &[0x6a, 0xff, 0x68, 0xbb, 0x2d, 0x96, 0x00] } => add_supporters_player_of_year_record, compare_add_supporters_player_of_year_record;
    Prologue { address: 0x86b0b0, bytes: &[0x6a, 0xff, 0x68, 0x46, 0x2c, 0x96, 0x00] } => add_cup_runners_up_record, compare_add_cup_runners_up_record;
    Prologue { address: 0x86b710, bytes: &[0x6a, 0xff, 0x68, 0x66, 0x2c, 0x96, 0x00] } => add_cup_third_place_record, compare_add_cup_third_place_record;
    Prologue { address: 0x869aa0, bytes: &[0x6a, 0xff, 0x68, 0xc1, 0x2b, 0x96, 0x00] } => record_manager_job_change, compare_record_manager_job_change;
    Prologue { address: 0x758200, bytes: &[0x83, 0xec, 0x18, 0x53, 0x55] } => process_transfer_squad_reactions, compare_process_transfer_squad_reactions;
    Prologue { address: 0x75b6e0, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] } => queue_signing_celebration_story, compare_queue_signing_celebration_story;
    Prologue { address: 0x75cfc0, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] } => queue_sale_unrest_story, compare_queue_sale_unrest_story;
    Prologue { address: 0x59a5d0, bytes: &[0x81, 0xec, 0x1c, 0x02, 0x00, 0x00] } => process_match_income, compare_process_match_income;
    Prologue { address: 0x59af40, bytes: &[0x81, 0xec, 0x08, 0x02, 0x00, 0x00] } => calc_gate_receipts, compare_calc_gate_receipts;
    Prologue { address: 0x599850, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x8b, 0x91, 0x28, 0x01, 0x00, 0x00] } => pay_league_fine, compare_pay_league_fine;
    Prologue { address: 0x59e050, bytes: &[0x51, 0x53, 0x55, 0x56, 0x8b, 0x74, 0x24, 0x14] } => try_finance_boost_event, compare_try_finance_boost_event;
    Prologue { address: 0x59e940, bytes: &[0x55, 0x8b, 0xec, 0x83, 0xe4, 0xf8] } => try_board_takeover, compare_try_board_takeover;
    Prologue { address: 0x5999e0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x8b, 0x91, 0x20, 0x01, 0x00, 0x00] } => pay_other_costs, compare_pay_other_costs;
    Prologue { address: 0x5ca4f0, bytes: &[0x81, 0xec, 0x04, 0x02, 0x00, 0x00] } => build_international_date_table, compare_build_international_date_table;
    Prologue { address: 0x5cc510, bytes: &[0x51, 0x66, 0xa1, 0x92, 0x2c, 0xae, 0x00] } => resolve_international_dates, compare_resolve_international_dates;
    Prologue { address: 0x5c6830, bytes: &[0x0f, 0xbf, 0x05, 0x7a, 0x7c, 0xb7, 0x00] } => find_human_club_tournament, compare_find_human_club_tournament;
    Prologue { address: 0x5c4770, bytes: &[0x81, 0xec, 0x50, 0x02, 0x00, 0x00] } => plan_club_friendlies, compare_plan_club_friendlies;
    Prologue { address: 0x5c6e00, bytes: &[0x83, 0xec, 0x08, 0x53, 0x56, 0x57] } => add_tournament, compare_add_tournament;
    Prologue { address: 0x5c68b0, bytes: &[0x56, 0x8b, 0xf1, 0x57, 0x8b, 0x86, 0xc8, 0x34, 0x00, 0x00] } => find_human_club_open_invitation, compare_find_human_club_open_invitation;
    Prologue { address: 0x62d250, bytes: &[0x81, 0xec, 0x20, 0x02, 0x00, 0x00] } => add_injury, compare_add_injury;
    Prologue { address: 0x62dca0, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] } => injury_post_injury_news, compare_injury_post_injury_news;
    Prologue { address: 0x418f60, bytes: &[0x56, 0x8b, 0x74, 0x24, 0x08] } => is_player_awol, compare_is_player_awol;
    Prologue { address: 0x8e4f20, bytes: &[0x6a, 0xff, 0x68, 0xab, 0x48, 0x96, 0x00] } => pay_former_club_fee_cuts, compare_pay_former_club_fee_cuts;
    Prologue { address: 0x8cb0f0, bytes: &[0x83, 0xec, 0x14, 0x53, 0x55] } => pay_next_sale_percentage, compare_pay_next_sale_percentage;
    Prologue { address: 0x8db3a0, bytes: &[0x6a, 0xff, 0x68, 0xdb, 0x44, 0x96, 0x00] } => post_sale_percentage_news, compare_post_sale_percentage_news;
    Prologue { address: 0x8c7950, bytes: &[0x83, 0xec, 0x14, 0x53, 0x55] } => is_work_permit_granted, compare_is_work_permit_granted;
    Prologue { address: 0x8c5800, bytes: &[0x8b, 0x0d, 0x50, 0x10, 0xae, 0x00] } => transfer_manager_get_squad_status_for_reason, compare_transfer_manager_get_squad_status_for_reason;
    Prologue { address: 0x8c6200, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x8b, 0x51, 0x10] } => get_negotiation_slot, compare_get_negotiation_slot;
    Prologue { address: 0x755340, bytes: &[0x81, 0xec, 0xf4, 0x00, 0x00, 0x00] } => apply_squad_reaction_to_player_story, compare_apply_squad_reaction_to_player_story;
    Prologue { address: 0x758010, bytes: &[0x83, 0xec, 0x10, 0x53, 0x57] } => warn_squad_of_position_rival, compare_warn_squad_of_position_rival;
    Prologue { address: 0x735160, bytes: &[0x8b, 0x44, 0x24, 0x0c, 0x85, 0xc0] } => media_format_response_option_label, compare_media_format_response_option_label;
    Prologue { address: 0x557120, bytes: &[0x6a, 0xff, 0x68, 0xc8, 0x7c, 0x95, 0x00] } => serve_international_bans, compare_serve_international_bans;
    Prologue { address: 0x557000, bytes: &[0x53, 0x55, 0x8b, 0x5c, 0x24, 0x0c] } => apply_appeal_result, compare_apply_appeal_result;
    Prologue { address: 0x55d3c0, bytes: &[0x8b, 0x54, 0x24, 0x04, 0x56] } => get_slot_text, compare_get_slot_text;
    Prologue { address: 0x68d1f0, bytes: &[0x8b, 0xc1, 0x53, 0x55, 0x56] } => manager_manager_remove, compare_manager_manager_remove;
    Prologue { address: 0x68d080, bytes: &[0x56, 0x8b, 0xf1, 0x8b, 0x46, 0x02] } => manager_manager_clear, compare_manager_manager_clear;
    Prologue { address: 0x68d2e0, bytes: &[0x8b, 0x41, 0x0a, 0x85, 0xc0] } => get_current, compare_get_current;
    Prologue { address: 0x51d2d0, bytes: &[0x81, 0xec, 0x2c, 0x02, 0x00, 0x00] } => sort_round_teams, compare_sort_round_teams;
    Prologue { address: 0x51d9f0, bytes: &[0x81, 0xec, 0x30, 0x02, 0x00, 0x00] } => check_round_teams_for_nation, compare_check_round_teams_for_nation;
    Prologue { address: 0x518780, bytes: &[0xc6, 0x81, 0xb1, 0x00, 0x00, 0x00, 0x00] } => cup_start_new_season, compare_cup_start_new_season;
    Prologue { address: 0x48dee0, bytes: &[0x53, 0x56, 0x8b, 0x74, 0x24, 0x0c] } => comp_active_key_nation_of_club, compare_comp_active_key_nation_of_club;
    Prologue { address: 0x48df40, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x53] } => comp_active_key_nation_of_stadium, compare_comp_active_key_nation_of_stadium;
    Prologue { address: 0x48dcd0, bytes: &[0x8a, 0x54, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => comp_key_nation_entry_from_active_index, compare_comp_key_nation_entry_from_active_index;
    Prologue { address: 0x5ae380, bytes: &[0x6a, 0xff, 0x68, 0x16, 0x98, 0x95, 0x00] } => resolve_same_day_clash, compare_resolve_same_day_clash;
    Prologue { address: 0x5aee40, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x8b, 0x15, 0x5c, 0x2a, 0xae, 0x00] } => get_club_first_fixture_year, compare_get_club_first_fixture_year;
    Prologue { address: 0x5aee60, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x8b, 0x15, 0x5c, 0x2a, 0xae, 0x00] } => get_club_last_fixture_year, compare_get_club_last_fixture_year;
    Prologue { address: 0x59b3b0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x04, 0x02, 0x00, 0x00] } => save_finance_dat, compare_save_finance_dat;
    Prologue { address: 0x59b2a0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x04, 0x02, 0x00, 0x00] } => load_from_save, compare_load_from_save;
    Prologue { address: 0x859cc0, bytes: &[0x81, 0xec, 0x08, 0x02, 0x00, 0x00] } => squad_manager_save, compare_squad_manager_save;
    Prologue { address: 0x905510, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => uefa_seeding_save_to_temp_file, compare_uefa_seeding_save_to_temp_file;
    Prologue { address: 0x4c6b70, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => conmebol_seeding_save_to_temp_file, compare_conmebol_seeding_save_to_temp_file;
    Prologue { address: 0x5fb320, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => save_temp_file, compare_save_temp_file;
    Prologue { address: 0x443f50, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x04, 0x02, 0x00, 0x00] } => club_history_save, compare_club_history_save;
    Prologue { address: 0x79bb80, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x04, 0x02, 0x00, 0x00] } => save_official_stats, compare_save_official_stats;
    Prologue { address: 0x84c680, bytes: &[0x81, 0xec, 0x04, 0x02, 0x00, 0x00] } => simulated_stats_save, compare_simulated_stats_save;
    Prologue { address: 0x5af040, bytes: &[0xa1, 0x5c, 0x2a, 0xae, 0x00] } => fix_man_save_club_dates, compare_fix_man_save_club_dates;
    Prologue { address: 0x5aee90, bytes: &[0xa1, 0x5c, 0x2a, 0xae, 0x00] } => fix_man_load_club_dates, compare_fix_man_load_club_dates;
    Prologue { address: 0x5af4a0, bytes: &[0x81, 0xec, 0x04, 0x02, 0x00, 0x00] } => fix_man_save_fixture_leg_checks, compare_fix_man_save_fixture_leg_checks;
    Prologue { address: 0x59cdd0, bytes: &[0x81, 0xec, 0x08, 0x02, 0x00, 0x00] } => fin_prm_end_season_for_nation, compare_fin_prm_end_season_for_nation;
    Prologue { address: 0x59cfa0, bytes: &[0x81, 0xec, 0x1c, 0x02, 0x00, 0x00] } => season_end_club_update, compare_season_end_club_update;
    Prologue { address: 0x59fe10, bytes: &[0x6a, 0xff, 0x68, 0x2b, 0x91, 0x95, 0x00] } => news_dividend, compare_news_dividend;
    Prologue { address: 0x5968d0, bytes: &[0x83, 0xec, 0x34, 0x53, 0x56] } => get_max_wage, compare_get_max_wage;
    Prologue { address: 0x59ffb0, bytes: &[0x6a, 0xff, 0x68, 0x6b, 0x91, 0x95, 0x00] } => news_ground_works, compare_news_ground_works;
    Prologue { address: 0x59fae0, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] } => news_request_refused, compare_news_request_refused;
    Prologue { address: 0x5adba0, bytes: &[0x6a, 0xff, 0x68, 0xeb, 0x97, 0x95, 0x00] } => collect_day_clashes, compare_collect_day_clashes;
    Prologue { address: 0x5ad780, bytes: &[0x6a, 0xff, 0x68, 0xc9, 0x97, 0x95, 0x00] } => find_fixture_clashes, compare_find_fixture_clashes;
    Prologue { address: 0x5af390, bytes: &[0x81, 0xec, 0x14, 0x02, 0x00, 0x00] } => fix_man_load_fixture_leg_checks, compare_fix_man_load_fixture_leg_checks;
    Prologue { address: 0x5ab510, bytes: &[0x6a, 0xff, 0x68, 0x07, 0x97, 0x95, 0x00] } => find_new_fixture_date, compare_find_new_fixture_date;
    Prologue { address: 0x5aa680, bytes: &[0x6a, 0xff, 0x68, 0x24, 0x96, 0x95, 0x00] } => add_fixture, compare_add_fixture;
    Prologue { address: 0x5aa2d0, bytes: &[0x6a, 0xff, 0x68, 0xea, 0x95, 0x95, 0x00] } => set_year, compare_set_year;
    Prologue { address: 0x51cb90, bytes: &[0x6a, 0xff, 0x68, 0x11, 0x77, 0x95, 0x00] } => create_tie_fixtures, compare_create_tie_fixtures;
    Prologue { address: 0x522950, bytes: &[0x6a, 0xff, 0x68, 0xfb, 0x77, 0x95, 0x00] } => post_draw_news, compare_post_draw_news;
    Prologue { address: 0x8c6bc0, bytes: &[0x6a, 0xff, 0x68, 0x48, 0x3d, 0x96, 0x00] } => update_transfer_window_news, compare_update_transfer_window_news;
    Prologue { address: 0x51ebe0, bytes: &[0x6a, 0xff, 0x68, 0x5a, 0x77, 0x95, 0x00] } => move_round_fixtures_to_set_dates, compare_move_round_fixtures_to_set_dates;
    Prologue { address: 0x8c41c0, bytes: &[0x6a, 0xff, 0x68, 0x68, 0x3c, 0x96, 0x00] } => remove_exchange_player, compare_remove_exchange_player;
    Prologue { address: 0x8c7690, bytes: &[0x51, 0x53, 0x55, 0x56, 0x8b, 0x74, 0x24, 0x14] } => check_offer_work_permits, compare_check_offer_work_permits;
    Prologue { address: 0x695240, bytes: &[0x83, 0xec, 0x2c, 0x8b, 0x44, 0x24, 0x3c] } => update_board_confidence_after_match, compare_update_board_confidence_after_match;
    Prologue { address: 0x6a5fa0, bytes: &[0x6a, 0xff, 0x68, 0x16, 0xcf, 0x95, 0x00] } => post_board_match_reaction_news, compare_post_board_match_reaction_news;
    Prologue { address: 0x6a5a60, bytes: &[0x6a, 0xff, 0x68, 0xcb, 0xce, 0x95, 0x00] } => post_fa_performance_news, compare_post_fa_performance_news;
    Prologue { address: 0x6a3770, bytes: &[0x83, 0xec, 0x10, 0x53, 0x55, 0x56] } => set_season_expectations, compare_set_season_expectations;
    Prologue { address: 0x69ba50, bytes: &[0x6a, 0xff, 0x68, 0xa0, 0xcc, 0x95, 0x00] } => add_job_vacancy, compare_add_job_vacancy;
    Prologue { address: 0x6a0540, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] } => post_job_offer_news, compare_post_job_offer_news;
    Prologue { address: 0x754a30, bytes: &[0x6a, 0xff, 0x68, 0xbb, 0xd8, 0x95, 0x00] } => process_transfer_interest, compare_process_transfer_interest;
    Prologue { address: 0x587600, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] } => news_create_not_for_sale_story, compare_news_create_not_for_sale_story;
    Prologue { address: 0x75c390, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] } => queue_trophy_praise_story, compare_queue_trophy_praise_story;
    Prologue { address: 0x5cbdf0, bytes: &[0x6a, 0xff, 0x68, 0xc6, 0xa3, 0x95, 0x00] } => arrange_international_friendlies, compare_arrange_international_friendlies;
    Prologue { address: 0x5ca090, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] } => friendly_check_match_invitation, compare_friendly_check_match_invitation;
    Prologue { address: 0x5ca290, bytes: &[0x81, 0xec, 0x10, 0x02, 0x00, 0x00] } => friendly_check_tournament_invitation, compare_friendly_check_tournament_invitation;
    Prologue { address: 0x76b100, bytes: &[0x6a, 0xff, 0x68, 0x08, 0xdd, 0x95, 0x00] } => national_teams_daily_update, compare_national_teams_daily_update;
    Prologue { address: 0x7f8420, bytes: &[0x0f, 0xbf, 0x05, 0x7a, 0x7c, 0xb7, 0x00] } => current_search_has_no_club, compare_current_search_has_no_club;
    Prologue { address: 0x7f8500, bytes: &[0x0f, 0xbf, 0x05, 0x7a, 0x7c, 0xb7, 0x00] } => is_club_in_session_searches, compare_is_club_in_session_searches;
    Prologue { address: 0x76c120, bytes: &[0x6a, 0xff, 0x68, 0x7a, 0xdd, 0x95, 0x00] } => select_squad, compare_select_squad;
    Prologue { address: 0x7f8c00, bytes: &[0x6a, 0xff, 0x68, 0xd8, 0x0f, 0x96, 0x00] } => scout_manager_scout_match, compare_scout_manager_scout_match;
    Prologue { address: 0x7f6be0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => get_edit_session, compare_get_edit_session;
    Prologue { address: 0x7f6af0, bytes: &[0x8b, 0x44, 0x24, 0x08, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => set_edit_session, compare_set_edit_session;
    Prologue { address: 0x8c5880, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] } => transfer_manager_daily_update, compare_transfer_manager_daily_update;
    Prologue { address: 0x8c63a0, bytes: &[0x6a, 0xff, 0x68, 0x00, 0x3d, 0x96, 0x00] } => process_due_offer, compare_process_due_offer;
    Prologue { address: 0x8cfc90, bytes: &[0x6a, 0xff, 0x68, 0xf8, 0x3f, 0x96, 0x00] } => check_offer_restrictions, compare_check_offer_restrictions;
    Prologue { address: 0x8c0080, bytes: &[0x6a, 0xff, 0x68, 0x48, 0x3b, 0x96, 0x00] } => accept_transfer_bid, compare_accept_transfer_bid;
    Prologue { address: 0x8d3090, bytes: &[0x6a, 0xff, 0x68, 0xc6, 0x42, 0x96, 0x00] } => post_offer_made_news, compare_post_offer_made_news;
    Prologue { address: 0x8d0040, bytes: &[0x6a, 0xff, 0x68, 0x1b, 0x40, 0x96, 0x00] } => transfer_manager_can_loan_player, compare_transfer_manager_can_loan_player;
    Prologue { address: 0x6847c0, bytes: &[0x6a, 0xff, 0x68, 0x21, 0xca, 0x95, 0x00] } => league_update_for_date, compare_league_update_for_date;
    Prologue { address: 0x6835c0, bytes: &[0x6a, 0xff, 0x68, 0x81, 0xc9, 0x95, 0x00] } => create_fixtures, compare_create_fixtures;
    Prologue { address: 0x689c80, bytes: &[0x81, 0xec, 0x10, 0x02, 0x00, 0x00] } => league_promote_and_relegate, compare_league_promote_and_relegate;
    Prologue { address: 0x5aaf00, bytes: &[0x6a, 0xff, 0x68, 0x70, 0x96, 0x95, 0x00] } => fix_man_daily_update, compare_fix_man_daily_update;
    Prologue { address: 0x5abf10, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] } => get_fixture_list, compare_get_fixture_list;
    Prologue { address: 0x5ac3b0, bytes: &[0x81, 0xec, 0x54, 0x02, 0x00, 0x00] } => load_fixtures, compare_load_fixtures;
    Prologue { address: 0x518790, bytes: &[0x81, 0xec, 0x18, 0x02, 0x00, 0x00] } => map_fixture_tree, compare_map_fixture_tree;
    Prologue { address: 0x48d2b0, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => get_first_stage_of_group, compare_get_first_stage_of_group;
    Prologue { address: 0x48cec0, bytes: &[0x81, 0xec, 0x00, 0x04, 0x00, 0x00] } => get_current_stage, compare_get_current_stage;
    Prologue { address: 0x8b1750, bytes: &[0x81, 0xec, 0x34, 0x04, 0x00, 0x00] } => update_player_training, compare_update_player_training;
    Prologue { address: 0x8b39f0, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => get_player_schedule, compare_get_player_schedule;
    Prologue { address: 0x8b4f50, bytes: &[0x0f, 0xbe, 0x44, 0x24, 0x04] } => get_category_level, compare_get_category_level;
    Prologue { address: 0x8b3640, bytes: &[0x81, 0xec, 0x08, 0x02, 0x00, 0x00] } => remove_or_restore_training_progress, compare_remove_or_restore_training_progress;
    Prologue { address: 0x8b37c0, bytes: &[0x81, 0xec, 0x08, 0x02, 0x00, 0x00] } => get_training_attribute_change, compare_get_training_attribute_change;
    Prologue { address: 0x8b4770, bytes: &[0x0f, 0xbf, 0x44, 0x24, 0x08] } => get_position_aptitude, compare_get_position_aptitude;
    Prologue { address: 0x7a59e0, bytes: &[0x6a, 0xff, 0x68, 0xa8, 0xf1, 0x95, 0x00] } => process_retirements, compare_process_retirements;
    Prologue { address: 0x7a92e0, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] } => player_regen_post_retirement_news, compare_player_regen_post_retirement_news;
    Prologue { address: 0x7a8ba0, bytes: &[0x6a, 0xff, 0x68, 0x0b, 0xf2, 0x95, 0x00] } => player_regen_post_youth_promotion_news, compare_player_regen_post_youth_promotion_news;
    Prologue { address: 0x4196a0, bytes: &[0x55, 0x8b, 0x6c, 0x24, 0x08] } => clear_awol, compare_clear_awol;
    Prologue { address: 0x4197c0, bytes: &[0x55, 0x8b, 0x6c, 0x24, 0x08] } => clear_missed_training, compare_clear_missed_training;
    Prologue { address: 0x41a630, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] } => awol_post_missed_training_news, compare_awol_post_missed_training_news;
    Prologue { address: 0x84b6f0, bytes: &[0x53, 0x8b, 0x5c, 0x24, 0x14] } => shortlist_manager_is_compensation_deal, compare_shortlist_manager_is_compensation_deal;
    Prologue { address: 0x83b6c0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x8b, 0x4c, 0x24, 0x08] } => shortlist_compare_slots_by_index, compare_shortlist_compare_slots_by_index;
    Prologue { address: 0x84b580, bytes: &[0x6a, 0xff, 0x68, 0xe8, 0x25, 0x96, 0x00] } => mark_player_under_offer, compare_mark_player_under_offer;
    Prologue { address: 0x868c50, bytes: &[0x6a, 0xff, 0x68, 0x76, 0x2b, 0x96, 0x00] } => add_league_champions_record, compare_add_league_champions_record;
    Prologue { address: 0x86a800, bytes: &[0x6a, 0xff, 0x68, 0x26, 0x2c, 0x96, 0x00] } => add_cup_winners_record, compare_add_cup_winners_record;
    Prologue { address: 0x86fc70, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x8b, 0x15, 0x64, 0x23, 0xae, 0x00] } => clear_human_manager_record, compare_clear_human_manager_record;
    Prologue { address: 0x8c8090, bytes: &[0x6a, 0xff, 0x68, 0xab, 0x3d, 0x96, 0x00] } => ai_respond_to_bid, compare_ai_respond_to_bid;
    Prologue { address: 0x8bee50, bytes: &[0x6a, 0xff, 0x68, 0xee, 0x3a, 0x96, 0x00] } => add_loan_negotiation, compare_add_loan_negotiation;
    Prologue { address: 0x8c8860, bytes: &[0x6a, 0xff, 0x68, 0x07, 0x3e, 0x96, 0x00] } => make_exchange_option, compare_make_exchange_option;
    Prologue { address: 0x4ccca0, bytes: &[0x81, 0xec, 0x14, 0x02, 0x00, 0x00] } => schedule_loan_end, compare_schedule_loan_end;
    Prologue { address: 0x84a020, bytes: &[0x55, 0x8b, 0xec, 0x83, 0xe4, 0xf8] } => make_club_bids, compare_make_club_bids;
    Prologue { address: 0x846920, bytes: &[0x55, 0x8b, 0xec, 0x83, 0xe4, 0xf8] } => staff_interest_in_joining_club, compare_staff_interest_in_joining_club;
    Prologue { address: 0x85e410, bytes: &[0x6a, 0xff, 0x68, 0xeb, 0x29, 0x96, 0x00] } => construct_staff_contract, compare_construct_staff_contract;
    Prologue { address: 0x579d60, bytes: &[0x83, 0xec, 0x6c, 0x55, 0x56] } => england_rules_can_loan_player, compare_england_rules_can_loan_player;
    Prologue { address: 0x579fd0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x8b, 0x15, 0xb4, 0x23, 0xae, 0x00] } => record_loan, compare_record_loan;
    Prologue { address: 0x57a020, bytes: &[0x83, 0xec, 0x08, 0x56, 0x6a, 0xff] } => england_rules_daily_update, compare_england_rules_daily_update;
    Prologue { address: 0x579c70, bytes: &[0x6a, 0xff, 0x68, 0x08, 0x87, 0x95, 0x00] } => england_rules_get_loan_period, compare_england_rules_get_loan_period;
    Prologue { address: 0x7f6650, bytes: &[0x8b, 0x0d, 0x50, 0x10, 0xae, 0x00] } => scotland_rules_can_loan_player, compare_scotland_rules_can_loan_player;
    Prologue { address: 0x7f6530, bytes: &[0x56, 0x57, 0xb9, 0x90, 0x2c, 0xae, 0x00] } => scotland_rules_get_loan_period, compare_scotland_rules_get_loan_period;
    Prologue { address: 0x661770, bytes: &[0x83, 0xec, 0x64, 0x55, 0x56] } => italy_rules_can_loan_player, compare_italy_rules_can_loan_player;
    Prologue { address: 0x63c3d0, bytes: &[0x53, 0x56, 0x57, 0x8b, 0x7c, 0x24, 0x10] } => ireland_rules_can_loan_player, compare_ireland_rules_can_loan_player;
    Prologue { address: 0x859630, bytes: &[0x83, 0xec, 0x64, 0x56, 0x57] } => spain_rules_can_loan_player, compare_spain_rules_can_loan_player;
    Prologue { address: 0x685640, bytes: &[0x6a, 0xff, 0x68, 0x72, 0xca, 0x95, 0x00] } => move_fixtures_for_tv, compare_move_fixtures_for_tv;
    Prologue { address: 0x684230, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] } => send_group_draw_news, compare_send_group_draw_news;
    Prologue { address: 0x683ee0, bytes: &[0x6a, 0xff, 0x68, 0xbc, 0xc9, 0x95, 0x00] } => send_fixture_list_news, compare_send_fixture_list_news;
    Prologue { address: 0x8be300, bytes: &[0x6a, 0xff, 0x68, 0x6a, 0x3a, 0x96, 0x00] } => add_transfer_offer, compare_add_transfer_offer;
    Prologue { address: 0x8c0660, bytes: &[0x8b, 0x44, 0x24, 0x08, 0x55] } => accept_loan_offer, compare_accept_loan_offer;
    Prologue { address: 0x8c2180, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] } => handle_player_accepted_terms, compare_handle_player_accepted_terms;
    Prologue { address: 0x4d71b0, bytes: &[0x6a, 0xff, 0x68, 0x16, 0x6f, 0x95, 0x00] } => post_transfer_listed_news, compare_post_transfer_listed_news;
    Prologue { address: 0x4cda70, bytes: &[0x6a, 0xff, 0x68, 0x68, 0x6d, 0x95, 0x00] } => contract_manager_offer_contract, compare_contract_manager_offer_contract;
    Prologue { address: 0x92e940, bytes: &[0x81, 0xec, 0x1c, 0x02, 0x00, 0x00] } => draw_groups, compare_draw_groups;
    Prologue { address: 0x92e7f0, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => run_group_draw, compare_run_group_draw;
    Prologue { address: 0x92f1a0, bytes: &[0x53, 0x8b, 0x5c, 0x24, 0x0c] } => world_cup_update_for_date, compare_world_cup_update_for_date;
    Prologue { address: 0x92d0f0, bytes: &[0x81, 0xec, 0x04, 0x02, 0x00, 0x00] } => world_cup_create_group_table, compare_world_cup_create_group_table;
    Prologue { address: 0x92d540, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] } => world_cup_create_group_stage, compare_world_cup_create_group_stage;
    Prologue { address: 0x92f680, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => world_cup_set_predicted_positions, compare_world_cup_set_predicted_positions;
    Prologue { address: 0x92c290, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => world_cup_create_schedule, compare_world_cup_create_schedule;
    Prologue { address: 0x92e740, bytes: &[0x66, 0x8b, 0x41, 0x36, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => add_qualified_team, compare_add_qualified_team;
    Prologue { address: 0x92f230, bytes: &[0x81, 0xec, 0x1c, 0x02, 0x00, 0x00] } => seed_teams, compare_seed_teams;
    Prologue { address: 0x8b2880, bytes: &[0x83, 0xec, 0x34, 0x53, 0x56] } => update_coach_assignments, compare_update_coach_assignments;
    Prologue { address: 0x8b2470, bytes: &[0x81, 0xec, 0xd0, 0x04, 0x00, 0x00] } => reassign_squad_schedules, compare_reassign_squad_schedules;
    Prologue { address: 0x8b2230, bytes: &[0x56, 0x57, 0x8b, 0xf1, 0xbf, 0xa7, 0x00, 0x00, 0x00] } => training_manager_cpp_0x8b2230, compare_training_manager_cpp_0x8b2230;
    Prologue { address: 0x7c3e40, bytes: &[0x81, 0xec, 0x48, 0x02, 0x00, 0x00] } => build_career_history, compare_build_career_history;
    Prologue { address: 0x7c6e30, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x04, 0x02, 0x00, 0x00] } => player_stats_years_since_at_club, compare_player_stats_years_since_at_club;
    Prologue { address: 0x7c6de0, bytes: &[0x56, 0x8b, 0x74, 0x24, 0x08] } => player_stats_compare_history_by_staff, compare_player_stats_compare_history_by_staff;
    Prologue { address: 0x7a32e0, bytes: &[0x56, 0x8b, 0x74, 0x24, 0x08] } => player_setup_injury_from_name, compare_player_setup_injury_from_name;
    Prologue { address: 0x630ee0, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] } => post_recovery_news, compare_post_recovery_news;
    Prologue { address: 0x630c50, bytes: &[0x8b, 0x44, 0x24, 0x0c, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => injury_get_category_name, compare_injury_get_category_name;
    Prologue { address: 0x7b5be0, bytes: &[0x6a, 0xff, 0x68, 0x68, 0xf5, 0x95, 0x00] } => player_search_scout_match, compare_player_search_scout_match;
    Prologue { address: 0x7bd310, bytes: &[0x81, 0xec, 0x14, 0x02, 0x00, 0x00] } => player_search_record_scouted_club_match, compare_player_search_record_scouted_club_match;
    Prologue { address: 0x7706c0, bytes: &[0x6a, 0xff, 0x68, 0xa6, 0xde, 0x95, 0x00] } => send_club_international_report_news, compare_send_club_international_report_news;
    Prologue { address: 0x76e0b0, bytes: &[0x6a, 0xff, 0x68, 0xfb, 0xdd, 0x95, 0x00] } => national_teams_post_call_up_news, compare_national_teams_post_call_up_news;
    Prologue { address: 0x76d560, bytes: &[0x6a, 0xff, 0x68, 0xbb, 0xdd, 0x95, 0x00] } => national_teams_collect_fixture_opponents, compare_national_teams_collect_fixture_opponents;
    Prologue { address: 0x7f8750, bytes: &[0x6a, 0xff, 0x68, 0xbe, 0x0f, 0x96, 0x00] } => scout_manager_daily_update, compare_scout_manager_daily_update;
    Prologue { address: 0x7f9890, bytes: &[0x81, 0xec, 0x10, 0x02, 0x00, 0x00] } => send_scout_reports, compare_send_scout_reports;
    Prologue { address: 0x7f9a60, bytes: &[0x6a, 0xff, 0x68, 0x3b, 0x10, 0x96, 0x00] } => send_scout_report_news, compare_send_scout_report_news;
    Prologue { address: 0x4cf720, bytes: &[0x81, 0xec, 0x18, 0x02, 0x00, 0x00] } => sign_contract, compare_sign_contract;
    Prologue { address: 0x7abac0, bytes: &[0x83, 0xec, 0x08, 0x56, 0x8b, 0x74, 0x24, 0x10] } => regenerate_as_youth_player, compare_regenerate_as_youth_player;
    Prologue { address: 0x56a090, bytes: &[0x53, 0x8b, 0x5c, 0x24, 0x08] } => remove_dispute, compare_remove_dispute;
    Prologue { address: 0x5b5d10, bytes: &[0x81, 0xec, 0x08, 0x04, 0x00, 0x00] } => load_from_disk, compare_load_from_disk;
    Prologue { address: 0x43f2d0, bytes: &[0x8b, 0x44, 0x24, 0x08, 0x8b, 0xc8] } => swap_bytes_16_array, compare_swap_bytes_16_array;
    Prologue { address: 0x43f300, bytes: &[0x8b, 0x44, 0x24, 0x08, 0x8b, 0xc8] } => swap_bytes_32_array, compare_swap_bytes_32_array;
    Prologue { address: 0x79bf60, bytes: &[0x81, 0xec, 0x0c, 0x02, 0x00, 0x00] } => set_up_officials, compare_set_up_officials;
    Prologue { address: 0x79d410, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x8b, 0x4c, 0x24, 0x08] } => officials_manager_compare_comp_level, compare_officials_manager_compare_comp_level;
    Prologue { address: 0x5c6570, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] } => on_friendly_fixture_cancelled, compare_on_friendly_fixture_cancelled;
    Prologue { address: 0x5cb9e0, bytes: &[0x6a, 0xff, 0x68, 0x80, 0xa3, 0x95, 0x00] } => friendly_cancel_national_fixture, compare_friendly_cancel_national_fixture;
    Prologue { address: 0x5c8260, bytes: &[0x6a, 0xff, 0x68, 0x77, 0xa2, 0x95, 0x00] } => cancel_tournament, compare_cancel_tournament;
    Prologue { address: 0x79d4e0, bytes: &[0x8b, 0x54, 0x24, 0x04, 0x56] } => officials_manager_compare_nation_then_quality_desc, compare_officials_manager_compare_nation_then_quality_desc;
    Prologue { address: 0x79d480, bytes: &[0x8b, 0x54, 0x24, 0x04, 0x56] } => officials_manager_compare_nation_then_quality_asc, compare_officials_manager_compare_nation_then_quality_asc;
    Prologue { address: 0x7c5fe0, bytes: &[0x81, 0xec, 0x3c, 0x02, 0x00, 0x00] } => archive_player_stats, compare_archive_player_stats;
    Prologue { address: 0x7a50f0, bytes: &[0x53, 0x8b, 0x5c, 0x24, 0x14] } => player_setup_find_staff, compare_player_setup_find_staff;
    Prologue { address: 0x7a5090, bytes: &[0x55, 0x8b, 0x6c, 0x24, 0x0c] } => player_setup_read_quoted_token, compare_player_setup_read_quoted_token;
    Prologue { address: 0x7a3140, bytes: &[0x56, 0x8b, 0x74, 0x24, 0x08] } => player_setup_type_from_name, compare_player_setup_type_from_name;
    Prologue { address: 0x585290, bytes: &[0x6a, 0xff, 0x68, 0xde, 0x8a, 0x95, 0x00] } => draw_phase1_groups, compare_draw_phase1_groups;
    Prologue { address: 0x586f60, bytes: &[0x8b, 0x44, 0x24, 0x08, 0x8b, 0x54, 0x24, 0x04] } => european_cup_compare_clubs_by_seed, compare_european_cup_compare_clubs_by_seed;
    Prologue { address: 0x586900, bytes: &[0x81, 0xec, 0x20, 0x02, 0x00, 0x00] } => european_cup_rank_teams, compare_european_cup_rank_teams;
    Prologue { address: 0x585ab0, bytes: &[0x6a, 0xff, 0x68, 0xfe, 0x8a, 0x95, 0x00] } => draw_phase2_groups, compare_draw_phase2_groups;
    Prologue { address: 0x5871e0, bytes: &[0x8b, 0x4c, 0x24, 0x04, 0x8b, 0x54, 0x24, 0x08] } => european_cup_compare_phase2_draw_entries, compare_european_cup_compare_phase2_draw_entries;
    Prologue { address: 0x4b01d0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x3d, 0xe8, 0x03, 0x00, 0x00] } => comp_util_get_parent_stage_id, compare_comp_util_get_parent_stage_id;
    Prologue { address: 0x583c50, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] } => european_cup_create_round_table, compare_european_cup_create_round_table;
    Prologue { address: 0x68aa80, bytes: &[0xa1, 0x44, 0x7a, 0xb6, 0x00] } => set_club_season_expectations, compare_set_club_season_expectations;
    Prologue { address: 0x6831a0, bytes: &[0x56, 0x8b, 0x74, 0x24, 0x08] } => relegate_club_to_division, compare_relegate_club_to_division;
    Prologue { address: 0x48ce10, bytes: &[0x56, 0x8b, 0xf1, 0x83, 0x7e, 0x1c, 0xff] } => all_stages_complete, compare_all_stages_complete;
    Prologue { address: 0x48e1c0, bytes: &[0x81, 0xec, 0x08, 0x02, 0x00, 0x00] } => comp_collect_club_list, compare_comp_collect_club_list;
    Prologue { address: 0x48cc70, bytes: &[0x83, 0xec, 0x0c, 0x53, 0x55] } => find_club_progress, compare_find_club_progress;
    Prologue { address: 0x903b50, bytes: &[0x55, 0x8b, 0xec, 0x83, 0xe4, 0xf8] } => roll_coefficients, compare_roll_coefficients;
    Prologue { address: 0x903cc0, bytes: &[0x81, 0xec, 0x04, 0x02, 0x00, 0x00] } => set_place_counts, compare_set_place_counts;
    Prologue { address: 0x9050d0, bytes: &[0x83, 0xec, 0x0c, 0x0f, 0xbf, 0x44, 0x24, 0x14] } => uefa_find_nth_unplaced_club_of_nation, compare_uefa_find_nth_unplaced_club_of_nation;
    Prologue { address: 0x904040, bytes: &[0x81, 0xec, 0x08, 0x02, 0x00, 0x00] } => collect_european_clubs, compare_collect_european_clubs;
    Prologue { address: 0x904210, bytes: &[0x81, 0xec, 0x64, 0x02, 0x00, 0x00] } => assign_champions_league_places, compare_assign_champions_league_places;
    Prologue { address: 0x904e30, bytes: &[0x83, 0xec, 0x10, 0x53, 0x55] } => assign_intertoto_places, compare_assign_intertoto_places;
    Prologue { address: 0x904580, bytes: &[0x81, 0xec, 0x48, 0x02, 0x00, 0x00] } => assign_uefa_cup_places, compare_assign_uefa_cup_places;
    Prologue { address: 0x903890, bytes: &[0x6a, 0xff, 0x68, 0x3b, 0x52, 0x96, 0x00] } => pick_english_intertoto_club, compare_pick_english_intertoto_club;
    Prologue { address: 0x9035a0, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] } => update_season, compare_update_season;
    Prologue { address: 0x4c6430, bytes: &[0xa1, 0x3c, 0xf6, 0x9c, 0x00] } => assign_libertadores_places, compare_assign_libertadores_places;
    Prologue { address: 0x4c6730, bytes: &[0x83, 0xec, 0x0c, 0x0f, 0xbf, 0x44, 0x24, 0x14] } => find_nth_unqualified_club_of_nation, compare_find_nth_unqualified_club_of_nation;
    Prologue { address: 0x4c6890, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x85, 0xc0] } => is_mercosur_invited_club, compare_is_mercosur_invited_club;
    Prologue { address: 0x4c6280, bytes: &[0xa1, 0x5c, 0x23, 0xae, 0x00] } => collect_conmebol_clubs, compare_collect_conmebol_clubs;
    Prologue { address: 0x4c61b0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x04, 0x02, 0x00, 0x00] } => update_for_new_season, compare_update_for_new_season;
    Prologue { address: 0x4c6960, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x8b, 0x10] } => set_bra_champ_cup_winner, compare_set_bra_champ_cup_winner;
    Prologue { address: 0x5fa730, bytes: &[0x8b, 0x01, 0x53, 0x55, 0x56] } => find_hosts, compare_find_hosts;
    Prologue { address: 0x5fa7d0, bytes: &[0x8b, 0x01, 0x56, 0x66, 0x8b, 0x71, 0x04] } => find_next_hosts, compare_find_next_hosts;
    Prologue { address: 0x5fa850, bytes: &[0x6a, 0xff, 0x68, 0x78, 0xaf, 0x95, 0x00] } => host_country_daily_update, compare_host_country_daily_update;
    Prologue { address: 0x579b30, bytes: &[0x6a, 0xff, 0x68, 0xeb, 0x86, 0x95, 0x00] } => construct_england_rules, compare_construct_england_rules;
    Prologue { address: 0x90b940, bytes: &[0x6a, 0xff, 0x68, 0x3b, 0x55, 0x96, 0x00] } => construct_usa_rules, compare_construct_usa_rules;
    Prologue { address: 0x412c80, bytes: &[0x6a, 0xff, 0x68, 0x1b, 0x3b, 0x95, 0x00] } => australia_rules, compare_australia_rules;
    Prologue { address: 0x859470, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => spain_rules_init_transfer_windows, compare_spain_rules_init_transfer_windows;
    Prologue { address: 0x6613f0, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => italy_rules_init_transfer_windows, compare_italy_rules_init_transfer_windows;
    Prologue { address: 0x5c15e0, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => france_rules_init_transfer_windows, compare_france_rules_init_transfer_windows;
    Prologue { address: 0x43f0e0, bytes: &[0x6a, 0xff, 0x68, 0xdb, 0x4e, 0x95, 0x00] } => construct_brazil_rules, compare_construct_brazil_rules;
    Prologue { address: 0x5dffe0, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => germany_rules_init_transfer_windows, compare_germany_rules_init_transfer_windows;
    Prologue { address: 0x7f6390, bytes: &[0x6a, 0xff, 0x68, 0xeb, 0x0e, 0x96, 0x00] } => construct_scotland_rules, compare_construct_scotland_rules;
    Prologue { address: 0x633d40, bytes: &[0x81, 0xec, 0x04, 0x02, 0x00, 0x00] } => intertoto_cup_update_for_date, compare_intertoto_cup_update_for_date;
    Prologue { address: 0x633c10, bytes: &[0x8a, 0x44, 0x24, 0x08, 0x53] } => intertoto_cup_update_staff_records_for_status, compare_intertoto_cup_update_staff_records_for_status;
    Prologue { address: 0x9030e0, bytes: &[0x8b, 0x44, 0x24, 0x08, 0x8b, 0x54, 0x24, 0x04] } => uefa_cup_update_staff_records_for_status, compare_uefa_cup_update_staff_records_for_status;
    Prologue { address: 0x4c11a0, bytes: &[0x81, 0xec, 0xa0, 0x09, 0x00, 0x00] } => con_champ_select_qualified_clubs, compare_con_champ_select_qualified_clubs;
    Prologue { address: 0x4c1620, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => con_champ_update_for_date, compare_con_champ_update_for_date;
    Prologue { address: 0x4c15f0, bytes: &[0x56, 0x8b, 0xf1, 0x6a, 0x00] } => con_champ_set_up_rounds, compare_con_champ_set_up_rounds;
    Prologue { address: 0x63f350, bytes: &[0x53, 0x56, 0x8b, 0xf1, 0xb0, 0x04] } => ita_cup_set_up_rules, compare_ita_cup_set_up_rules;
    Prologue { address: 0x63d6e0, bytes: &[0x53, 0x56, 0x8b, 0xf1, 0xb0, 0x04] } => ita_c_cup_set_up_rules, compare_ita_c_cup_set_up_rules;
    Prologue { address: 0x426230, bytes: &[0x53, 0x56, 0x8b, 0xf1, 0xb0, 0x03] } => bra_champ_cup_set_up_rules, compare_bra_champ_cup_set_up_rules;
    Prologue { address: 0x401ba0, bytes: &[0x8a, 0x44, 0x24, 0x0c, 0x56] } => african_nations_update_staff_records_for_status, compare_african_nations_update_staff_records_for_status;
    Prologue { address: 0x5e13b0, bytes: &[0x8a, 0x44, 0x24, 0x0c, 0x56] } => goldcup_update_staff_records_for_status, compare_goldcup_update_staff_records_for_status;
    Prologue { address: 0x91e840, bytes: &[0x8a, 0x44, 0x24, 0x0c, 0x56] } => wc_oceania_league_update_staff_records_for_status, compare_wc_oceania_league_update_staff_records_for_status;
    Prologue { address: 0x4c3890, bytes: &[0x6a, 0xff, 0x68, 0x78, 0x6a, 0x95, 0x00] } => conmebol_liber_set_up_rules, compare_conmebol_liber_set_up_rules;
    Prologue { address: 0x4c1ed0, bytes: &[0x6a, 0xff, 0x68, 0xc8, 0x69, 0x95, 0x00] } => con_merc_cup_set_up_rules, compare_con_merc_cup_set_up_rules;
    Prologue { address: 0x7979b0, bytes: &[0x6a, 0xff, 0x68, 0xe8, 0xea, 0x95, 0x00] } => oceania_club_champ_set_up_rules, compare_oceania_club_champ_set_up_rules;
    Prologue { address: 0x9191e0, bytes: &[0x6a, 0xff, 0x68, 0x98, 0x59, 0x96, 0x00] } => wc_europe_league_set_up_rules, compare_wc_europe_league_set_up_rules;
    Prologue { address: 0x921040, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] } => wc_south_american_league_set_up_rules, compare_wc_south_american_league_set_up_rules;
    Prologue { address: 0x912eb0, bytes: &[0x6a, 0xff, 0x68, 0x48, 0x57, 0x96, 0x00] } => wc_asia_league_set_up_rules, compare_wc_asia_league_set_up_rules;
    Prologue { address: 0x5e12d0, bytes: &[0x6a, 0xff, 0x68, 0x68, 0xa8, 0x95, 0x00] } => goldcup_set_up_rules, compare_goldcup_set_up_rules;
    Prologue { address: 0x401ab0, bytes: &[0x6a, 0xff, 0x68, 0xd8, 0x34, 0x95, 0x00] } => african_nations_set_up_rules, compare_african_nations_set_up_rules;
    Prologue { address: 0x91ecf0, bytes: &[0x6a, 0xff, 0x68, 0xf8, 0x5a, 0x96, 0x00] } => wc_oceania_league_set_up_rules, compare_wc_oceania_league_set_up_rules;
    Prologue { address: 0x4c3f40, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => conmebol_liber_select_qualified_clubs, compare_conmebol_liber_select_qualified_clubs;
    Prologue { address: 0x4c4080, bytes: &[0x8a, 0x44, 0x24, 0x0c, 0x56] } => conmebol_liber_update_staff_records_for_status, compare_conmebol_liber_update_staff_records_for_status;
    Prologue { address: 0x4c3a80, bytes: &[0x8b, 0x51, 0x2c, 0x8b, 0x41, 0x30] } => conmebol_liber_advance_stage, compare_conmebol_liber_advance_stage;
    Prologue { address: 0x401e10, bytes: &[0x8b, 0x51, 0x2c, 0x8b, 0x41, 0x30] } => african_nations_advance_stage, compare_african_nations_advance_stage;
    Prologue { address: 0x4019c0, bytes: &[0x81, 0xec, 0x04, 0x02, 0x00, 0x00] } => init_group_a_table, compare_init_group_a_table;
    Prologue { address: 0x401b90, bytes: &[0x8b, 0x41, 0x0c, 0x8b, 0x48, 0x0c] } => african_nations_record_season_honours, compare_african_nations_record_season_honours;
    Prologue { address: 0x42a5a0, bytes: &[0x53, 0x56, 0x8b, 0xf1, 0xbb, 0x01, 0x00, 0x00, 0x00] } => bra_nat_first_set_up_rules, compare_bra_nat_first_set_up_rules;
    Prologue { address: 0x42cb70, bytes: &[0x53, 0x56, 0x8b, 0xf1, 0xbb, 0x01, 0x00, 0x00, 0x00] } => bra_nat_second_set_up_rules, compare_bra_nat_second_set_up_rules;
    Prologue { address: 0x42de50, bytes: &[0x56, 0x8b, 0xf1, 0xb8, 0x02, 0x00, 0x00, 0x00] } => bra_nat_third_set_up_rules, compare_bra_nat_third_set_up_rules;
    Prologue { address: 0x430ff0, bytes: &[0x53, 0x56, 0x8b, 0xf1, 0x33, 0xc0] } => bra_reg_central_set_up_rules, compare_bra_reg_central_set_up_rules;
    Prologue { address: 0x4322d0, bytes: &[0x53, 0x56, 0x8b, 0xf1, 0x33, 0xc0] } => bra_reg_gaucho_set_up_rules, compare_bra_reg_gaucho_set_up_rules;
    Prologue { address: 0x43bc70, bytes: &[0x53, 0x56, 0x8b, 0xf1, 0xb8, 0x02, 0x00, 0x00, 0x00] } => bra_reg_santa_set_up_rules, compare_bra_reg_santa_set_up_rules;
    Prologue { address: 0x437fc0, bytes: &[0x56, 0x8b, 0xf1, 0xb8, 0x02, 0x00, 0x00, 0x00] } => bra_reg_parana_set_up_rules, compare_bra_reg_parana_set_up_rules;
    Prologue { address: 0x439430, bytes: &[0x56, 0x8b, 0xf1, 0x33, 0xc0] } => bra_reg_pern_set_up_rules, compare_bra_reg_pern_set_up_rules;
    Prologue { address: 0x43a600, bytes: &[0x56, 0x8b, 0xf1, 0x33, 0xc0] } => bra_reg_rio_set_up_rules, compare_bra_reg_rio_set_up_rules;
    Prologue { address: 0x433640, bytes: &[0x53, 0x56, 0x8b, 0xf1, 0x33, 0xc0] } => bra_reg_goias_set_up_rules, compare_bra_reg_goias_set_up_rules;
    Prologue { address: 0x663620, bytes: &[0x56, 0x8b, 0xf1, 0xb9, 0x02, 0x00, 0x00, 0x00] } => jap_j1_set_up_rules, compare_jap_j1_set_up_rules;
    Prologue { address: 0x665140, bytes: &[0x56, 0x8b, 0xf1, 0xb2, 0x03] } => jap_j2_set_up_rules, compare_jap_j2_set_up_rules;
    Prologue { address: 0x411ce0, bytes: &[0x56, 0x8b, 0xf1, 0x33, 0xc0] } => aus_nsl_set_up_rules, compare_aus_nsl_set_up_rules;
    Prologue { address: 0x908660, bytes: &[0xb2, 0x02, 0xb8, 0x01, 0x00, 0x00, 0x00] } => usa_mls_set_up_rules, compare_usa_mls_set_up_rules;
    Prologue { address: 0x432970, bytes: &[0x8b, 0x41, 0x0c, 0x8b, 0x08] } => bra_reg_gaucho_record_season_honours, compare_bra_reg_gaucho_record_season_honours;
    Prologue { address: 0x408eb0, bytes: &[0x8b, 0x41, 0x0c, 0x8b, 0x08] } => arg_second_record_season_honours, compare_arg_second_record_season_honours;
    Prologue { address: 0x42abc0, bytes: &[0x8a, 0x44, 0x24, 0x0c, 0x56] } => bra_nat_first_update_staff_records_for_status, compare_bra_nat_first_update_staff_records_for_status;
    Prologue { address: 0x42eb00, bytes: &[0x51, 0x8a, 0x44, 0x24, 0x10] } => bra_nat_third_update_staff_records_for_status, compare_bra_nat_third_update_staff_records_for_status;
    Prologue { address: 0x430210, bytes: &[0x8a, 0x44, 0x24, 0x0c, 0x53] } => bra_reg_bahia_update_staff_records_for_status, compare_bra_reg_bahia_update_staff_records_for_status;
    Prologue { address: 0x4314e0, bytes: &[0x8a, 0x44, 0x24, 0x0c, 0x53] } => bra_reg_central_update_staff_records_for_status, compare_bra_reg_central_update_staff_records_for_status;
    Prologue { address: 0x434d90, bytes: &[0x8a, 0x44, 0x24, 0x0c, 0x53] } => bra_reg_minas_gerais_update_staff_records_for_status, compare_bra_reg_minas_gerais_update_staff_records_for_status;
    Prologue { address: 0x411980, bytes: &[0x8a, 0x44, 0x24, 0x0c, 0x3c, 0xff] } => aus_nsl_update_staff_records_for_status, compare_aus_nsl_update_staff_records_for_status;
    Prologue { address: 0x4350d0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x8b, 0x49, 0x08] } => bra_reg_minas_gerais_set_club_progress, compare_bra_reg_minas_gerais_set_club_progress;
    Prologue { address: 0x909dd0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x8b, 0x49, 0x08] } => usa_mls_set_club_progress, compare_usa_mls_set_club_progress;
    Prologue { address: 0x42e100, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x8b, 0x49, 0x08] } => bra_nat_third_set_club_progress, compare_bra_nat_third_set_club_progress;
    Prologue { address: 0x908200, bytes: &[0x8a, 0x44, 0x24, 0x0c, 0x56] } => usa_mls_update_staff_records_for_status, compare_usa_mls_update_staff_records_for_status;
    Prologue { address: 0x9099d0, bytes: &[0x55, 0x56, 0x57, 0x8b, 0x7c, 0x24, 0x10] } => usa_mls_add_result, compare_usa_mls_add_result;
    Prologue { address: 0x90a000, bytes: &[0x8b, 0x4c, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => usa_mls_get_club_all_star_conference, compare_usa_mls_get_club_all_star_conference;
    Prologue { address: 0x574b70, bytes: &[0x56, 0x8b, 0xf1, 0xb9, 0x02, 0x00, 0x00, 0x00] } => eng_prm_set_up_rules, compare_eng_prm_set_up_rules;
    Prologue { address: 0x572d90, bytes: &[0x56, 0x8b, 0xf1, 0xb8, 0x02, 0x00, 0x00, 0x00] } => eng_first_set_up_rules, compare_eng_first_set_up_rules;
    Prologue { address: 0x576780, bytes: &[0x56, 0x8b, 0xf1, 0xb8, 0x02, 0x00, 0x00, 0x00] } => eng_second_set_up_rules, compare_eng_second_set_up_rules;
    Prologue { address: 0x5780c0, bytes: &[0x53, 0x56, 0x8b, 0xf1, 0xb0, 0x03] } => eng_third_set_up_rules, compare_eng_third_set_up_rules;
    Prologue { address: 0x574a20, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => eng_prm_create_table, compare_eng_prm_create_table;
    Prologue { address: 0x576630, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => eng_second_create_table, compare_eng_second_create_table;
    Prologue { address: 0x577f70, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => eng_third_create_table, compare_eng_third_create_table;
    Prologue { address: 0x572bf0, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => eng_first_create_table, compare_eng_first_create_table;
    Prologue { address: 0x641640, bytes: &[0x53, 0x56, 0x8b, 0xf1, 0xbb, 0x01, 0x00, 0x00, 0x00] } => ita_ser_a_set_up_rules, compare_ita_ser_a_set_up_rules;
    Prologue { address: 0x6468a0, bytes: &[0x53, 0x56, 0x8b, 0xf1, 0xbb, 0x01, 0x00, 0x00, 0x00] } => ita_ser_b_set_up_rules, compare_ita_ser_b_set_up_rules;
    Prologue { address: 0x64b770, bytes: &[0x53, 0x56, 0x8b, 0xf1, 0xb2, 0x03] } => ita_ser_c1a_set_up_rules, compare_ita_ser_c1a_set_up_rules;
    Prologue { address: 0x64f920, bytes: &[0x53, 0x56, 0x8b, 0xf1, 0xb8, 0x02, 0x00, 0x00, 0x00] } => ita_ser_c1b_set_up_rules, compare_ita_ser_c1b_set_up_rules;
    Prologue { address: 0x641510, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => ita_ser_a_create_table, compare_ita_ser_a_create_table;
    Prologue { address: 0x646780, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => ita_ser_b_create_table, compare_ita_ser_b_create_table;
    Prologue { address: 0x64b670, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => ita_ser_c1a_create_table, compare_ita_ser_c1a_create_table;
    Prologue { address: 0x64f820, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => ita_ser_c1b_create_table, compare_ita_ser_c1b_create_table;
    Prologue { address: 0x5bc410, bytes: &[0x56, 0x8b, 0xf1, 0xb9, 0x02, 0x00, 0x00, 0x00] } => fra_first_set_up_rules, compare_fra_first_set_up_rules;
    Prologue { address: 0x5bf0a0, bytes: &[0x56, 0x8b, 0xf1, 0xb8, 0x02, 0x00, 0x00, 0x00] } => fra_second_set_up_rules, compare_fra_second_set_up_rules;
    Prologue { address: 0x5c0700, bytes: &[0x56, 0x8b, 0xf1, 0xb9, 0x02, 0x00, 0x00, 0x00] } => fra_third_set_up_rules, compare_fra_third_set_up_rules;
    Prologue { address: 0x5bc2b0, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => fra_first_create_table, compare_fra_first_create_table;
    Prologue { address: 0x5bef10, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => fra_second_create_table, compare_fra_second_create_table;
    Prologue { address: 0x5c05d0, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => fra_third_create_table, compare_fra_third_create_table;
    Prologue { address: 0x84fdf0, bytes: &[0x53, 0x56, 0x8b, 0xf1, 0x57] } => spa_first_set_up_rules, compare_spa_first_set_up_rules;
    Prologue { address: 0x855cc0, bytes: &[0x56, 0x8b, 0xf1, 0xb9, 0x02, 0x00, 0x00, 0x00] } => spa_second_set_up_rules, compare_spa_second_set_up_rules;
    Prologue { address: 0x853790, bytes: &[0x56, 0x8b, 0xf1, 0xb8, 0x02, 0x00, 0x00, 0x00] } => spa_lower_set_up_rules, compare_spa_lower_set_up_rules;
    Prologue { address: 0x84fc70, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => spa_first_create_table, compare_spa_first_create_table;
    Prologue { address: 0x654180, bytes: &[0x8a, 0x44, 0x24, 0x0c, 0x53] } => ita_ser_c2a_update_staff_records_for_status, compare_ita_ser_c2a_update_staff_records_for_status;
    Prologue { address: 0x850880, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x8b, 0x49, 0x08] } => spa_first_set_club_progress, compare_spa_first_set_club_progress;
    Prologue { address: 0x5b9fa0, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => fra_cfa_create_group_table, compare_fra_cfa_create_group_table;
    Prologue { address: 0x5be230, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => fra_lower_create_table, compare_fra_lower_create_table;
    Prologue { address: 0x64bd80, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x8b, 0x49, 0x08] } => ita_ser_c1a_set_club_progress, compare_ita_ser_c1a_set_club_progress;
    Prologue { address: 0x64ff40, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x8b, 0x49, 0x08] } => ita_ser_c1b_set_club_progress, compare_ita_ser_c1b_set_club_progress;
    Prologue { address: 0x56c570, bytes: &[0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => eng_cc_cup_create_round_table, compare_eng_cc_cup_create_round_table;
    Prologue { address: 0x56f320, bytes: &[0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => eng_fa_cup_create_round_table, compare_eng_fa_cup_create_round_table;
    Prologue { address: 0x56d640, bytes: &[0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => eng_charity_create_round_table, compare_eng_charity_create_round_table;
    Prologue { address: 0x5ba3f0, bytes: &[0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => fra_cup_create_round_table, compare_fra_cup_create_round_table;
    Prologue { address: 0x5bcc40, bytes: &[0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => fra_lge_cup_create_round_table, compare_fra_lge_cup_create_round_table;
    Prologue { address: 0x84d220, bytes: &[0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => spa_cup_create_round_table, compare_spa_cup_create_round_table;
    Prologue { address: 0x5bf5d0, bytes: &[0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x0c, 0x02, 0x00, 0x00] } => fra_super_create_round_table, compare_fra_super_create_round_table;
    Prologue { address: 0x858960, bytes: &[0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => spa_super_create_round_table, compare_spa_super_create_round_table;
    Prologue { address: 0x666020, bytes: &[0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => jap_super_create_round_table, compare_jap_super_create_round_table;
    Prologue { address: 0x56ccc0, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => eng_cc_cup_collect_entrants, compare_eng_cc_cup_collect_entrants;
    Prologue { address: 0x56b3c0, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => eng_auto_cup_collect_entrants, compare_eng_auto_cup_collect_entrants;
    Prologue { address: 0x56d780, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => eng_charity_select_teams, compare_eng_charity_select_teams;
    Prologue { address: 0x5baa50, bytes: &[0x81, 0xec, 0x04, 0x02, 0x00, 0x00] } => fra_cup_collect_entrants, compare_fra_cup_collect_entrants;
    Prologue { address: 0x5bd160, bytes: &[0x81, 0xec, 0x04, 0x02, 0x00, 0x00] } => fra_lge_cup_collect_entrants, compare_fra_lge_cup_collect_entrants;
    Prologue { address: 0x5bf770, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => fra_super_select_teams, compare_fra_super_select_teams;
    Prologue { address: 0x858aa0, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => spa_super_select_teams, compare_spa_super_select_teams;
    Prologue { address: 0x666160, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => jap_super_select_teams, compare_jap_super_select_teams;
    Prologue { address: 0x90a400, bytes: &[0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => usa_mls_all_stars_create_round_table, compare_usa_mls_all_stars_create_round_table;
    Prologue { address: 0x56caf0, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => eng_cc_cup_update_for_date, compare_eng_cc_cup_update_for_date;
    Prologue { address: 0x570550, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] } => eng_fa_cup_update_for_date, compare_eng_fa_cup_update_for_date;
    Prologue { address: 0x90a900, bytes: &[0x6a, 0xff, 0x68, 0xa8, 0x54, 0x96, 0x00] } => usa_mls_all_stars_update_for_date, compare_usa_mls_all_stars_update_for_date;
    Prologue { address: 0x665640, bytes: &[0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => jap_j_cup_create_round_table, compare_jap_j_cup_create_round_table;
    Prologue { address: 0x661ff0, bytes: &[0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => jap_emp_cup_create_round_table, compare_jap_emp_cup_create_round_table;
    Prologue { address: 0x90b170, bytes: &[0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => usa_open_cup_create_round_table, compare_usa_open_cup_create_round_table;
    Prologue { address: 0x665a00, bytes: &[0x81, 0xec, 0x04, 0x02, 0x00, 0x00] } => jap_j_cup_collect_entrants, compare_jap_j_cup_collect_entrants;
    Prologue { address: 0x90b530, bytes: &[0x81, 0xec, 0x98, 0x03, 0x00, 0x00] } => usa_open_cup_collect_entrants, compare_usa_open_cup_collect_entrants;
    Prologue { address: 0x6624f0, bytes: &[0x81, 0xec, 0xc8, 0x04, 0x00, 0x00] } => jap_emp_cup_collect_entrants, compare_jap_emp_cup_collect_entrants;
    Prologue { address: 0x85a760, bytes: &[0x53, 0x8b, 0x5c, 0x24, 0x08] } => get_squad_number, compare_get_squad_number;
    Prologue { address: 0x85add0, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => club_uses_squad_numbers, compare_club_uses_squad_numbers;
    Prologue { address: 0x85a850, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => set_squad_number, compare_set_squad_number;
    Prologue { address: 0x85a470, bytes: &[0x81, 0xec, 0x08, 0x01, 0x00, 0x00] } => allocate_squad_number, compare_allocate_squad_number;
    Prologue { address: 0x859fe0, bytes: &[0x81, 0xec, 0x90, 0x02, 0x00, 0x00] } => assign_club_squad_numbers, compare_assign_club_squad_numbers;
    Prologue { address: 0x859dc0, bytes: &[0x55, 0x8b, 0xe9, 0x57, 0x33, 0xff] } => init_new_game, compare_init_new_game;
    Prologue { address: 0x859e20, bytes: &[0x81, 0xec, 0xd8, 0x00, 0x00, 0x00] } => assign_league_squad_numbers, compare_assign_league_squad_numbers;
    Prologue { address: 0x85aae0, bytes: &[0x81, 0xec, 0x9c, 0x02, 0x00, 0x00] } => select_euro_registered_players, compare_select_euro_registered_players;
    Prologue { address: 0x5f9690, bytes: &[0x6a, 0xff, 0x68, 0x3b, 0xaf, 0x95, 0x00] } => construct_holland_rules, compare_construct_holland_rules;
    Prologue { address: 0x518500, bytes: &[0x6a, 0xff, 0x68, 0xbb, 0x75, 0x95, 0x00] } => construct_croatia_rules, compare_construct_croatia_rules;
    Prologue { address: 0x63c210, bytes: &[0x6a, 0xff, 0x68, 0xab, 0xb7, 0x95, 0x00] } => construct_ireland_rules, compare_construct_ireland_rules;
    Prologue { address: 0x7944f0, bytes: &[0x6a, 0xff, 0x68, 0x2b, 0xe9, 0x95, 0x00] } => construct_norway_rules, compare_construct_norway_rules;
    Prologue { address: 0x7936d0, bytes: &[0x6a, 0xff, 0x68, 0xbb, 0xe7, 0x95, 0x00] } => construct_northern_ireland_rules, compare_construct_northern_ireland_rules;
    Prologue { address: 0x90f080, bytes: &[0x6a, 0xff, 0x68, 0x4b, 0x56, 0x96, 0x00] } => construct_wales_rules, compare_construct_wales_rules;
    Prologue { address: 0x9020a0, bytes: &[0x6a, 0xff, 0x68, 0xab, 0x51, 0x96, 0x00] } => construct_turkey_rules, compare_construct_turkey_rules;
    Prologue { address: 0x7cd830, bytes: &[0x6a, 0xff, 0x68, 0xab, 0xf9, 0x95, 0x00] } => construct_poland_rules, compare_construct_poland_rules;
    Prologue { address: 0x5a9950, bytes: &[0x6a, 0xff, 0x68, 0x4b, 0x95, 0x95, 0x00] } => construct_finland_rules, compare_construct_finland_rules;
    Prologue { address: 0x40a620, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => argentina_rules_init_transfer_windows, compare_argentina_rules_init_transfer_windows;
    Prologue { address: 0x7d3df0, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => portugal_rules_init_transfer_windows, compare_portugal_rules_init_transfer_windows;
    Prologue { address: 0x66e630, bytes: &[0x6a, 0xff, 0x68, 0x9b, 0xc8, 0x95, 0x00] } => construct_korea_rules, compare_construct_korea_rules;
    Prologue { address: 0x5ecf20, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => greece_rules_init_transfer_windows, compare_greece_rules_init_transfer_windows;
    Prologue { address: 0x7ec7e0, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => russia_rules_init_transfer_windows, compare_russia_rules_init_transfer_windows;
    Prologue { address: 0x424c00, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => belgium_rules_init_transfer_windows, compare_belgium_rules_init_transfer_windows;
    Prologue { address: 0x85a9b0, bytes: &[0x83, 0xec, 0x10, 0x8a, 0x44, 0x24, 0x14] } => process_euro_registration, compare_process_euro_registration;
    Prologue { address: 0x58dd50, bytes: &[0xa1, 0x50, 0x23, 0xae, 0x00] } => create_default_table, compare_create_default_table;
    Prologue { address: 0x85dfc0, bytes: &[0x81, 0xec, 0x30, 0x02, 0x00, 0x00] } => build_venue_list, compare_build_venue_list;
    Prologue { address: 0x42a480, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => bra_nat_first_create_table, compare_bra_nat_first_create_table;
    Prologue { address: 0x42ca70, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => bra_nat_second_create_table, compare_bra_nat_second_create_table;
    Prologue { address: 0x42dc90, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => bra_nat_third_create_table, compare_bra_nat_third_create_table;
    Prologue { address: 0x42fd10, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => bra_reg_bahia_create_table, compare_bra_reg_bahia_create_table;
    Prologue { address: 0x430ef0, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => bra_reg_central_load_region_clubs, compare_bra_reg_central_load_region_clubs;
    Prologue { address: 0x4321d0, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => bra_reg_gaucho_load_region_clubs, compare_bra_reg_gaucho_load_region_clubs;
    Prologue { address: 0x433540, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => bra_reg_goias_create_table, compare_bra_reg_goias_create_table;
    Prologue { address: 0x434810, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => bra_reg_minas_gerais_create_table, compare_bra_reg_minas_gerais_create_table;
    Prologue { address: 0x435d80, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => bra_reg_north_create_table, compare_bra_reg_north_create_table;
    Prologue { address: 0x436d90, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => bra_reg_northeast_create_table, compare_bra_reg_northeast_create_table;
    Prologue { address: 0x437ec0, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => bra_reg_parana_create_table, compare_bra_reg_parana_create_table;
    Prologue { address: 0x439330, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => bra_reg_pern_create_table, compare_bra_reg_pern_create_table;
    Prologue { address: 0x43a500, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => bra_reg_rio_create_table, compare_bra_reg_rio_create_table;
    Prologue { address: 0x43bb70, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => bra_reg_santa_create_table, compare_bra_reg_santa_create_table;
    Prologue { address: 0x43cc40, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => bra_reg_sp_create_table, compare_bra_reg_sp_create_table;
    Prologue { address: 0x6634e0, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => jap_j1_create_table, compare_jap_j1_create_table;
    Prologue { address: 0x665030, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => jap_j2_create_table, compare_jap_j2_create_table;
    Prologue { address: 0x411b70, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => aus_nsl_create_table, compare_aus_nsl_create_table;
    Prologue { address: 0x430050, bytes: &[0x6a, 0xff, 0x68, 0x2e, 0x46, 0x95, 0x00] } => bra_reg_bahia_create_semi_final_stage, compare_bra_reg_bahia_create_semi_final_stage;
    Prologue { address: 0x4360c0, bytes: &[0x6a, 0xff, 0x68, 0x7e, 0x48, 0x95, 0x00] } => bra_reg_north_create_semi_final_stage, compare_bra_reg_north_create_semi_final_stage;
    Prologue { address: 0x4370d0, bytes: &[0x6a, 0xff, 0x68, 0xee, 0x48, 0x95, 0x00] } => bra_reg_northeast_create_semi_final_stage, compare_bra_reg_northeast_create_semi_final_stage;
    Prologue { address: 0x439710, bytes: &[0x6a, 0xff, 0x68, 0xee, 0x49, 0x95, 0x00] } => bra_reg_pern_create_semi_final_stage, compare_bra_reg_pern_create_semi_final_stage;
    Prologue { address: 0x43bf50, bytes: &[0x6a, 0xff, 0x68, 0xee, 0x4a, 0x95, 0x00] } => bra_reg_santa_create_semi_final_stage, compare_bra_reg_santa_create_semi_final_stage;
    Prologue { address: 0x43cf80, bytes: &[0x6a, 0xff, 0x68, 0x5e, 0x4b, 0x95, 0x00] } => bra_reg_sp_create_semi_final_stage, compare_bra_reg_sp_create_semi_final_stage;
    Prologue { address: 0x4325b0, bytes: &[0x6a, 0xff, 0x68, 0x0e, 0x47, 0x95, 0x00] } => bra_reg_gaucho_create_knockout_stage, compare_bra_reg_gaucho_create_knockout_stage;
    Prologue { address: 0x4312d0, bytes: &[0x6a, 0xff, 0x68, 0x9e, 0x46, 0x95, 0x00] } => bra_reg_central_create_knockout_stage, compare_bra_reg_central_create_knockout_stage;
    Prologue { address: 0x433930, bytes: &[0x6a, 0xff, 0x68, 0x7e, 0x47, 0x95, 0x00] } => bra_reg_goias_create_knockout_stage, compare_bra_reg_goias_create_knockout_stage;
    Prologue { address: 0x642880, bytes: &[0x83, 0xec, 0x1c, 0x8b, 0x44, 0x24, 0x20] } => ita_ser_a_update_for_date, compare_ita_ser_a_update_for_date;
    Prologue { address: 0x647050, bytes: &[0x83, 0xec, 0x18, 0xb0, 0x02] } => ita_ser_b_update_for_date, compare_ita_ser_b_update_for_date;
    Prologue { address: 0x6471a0, bytes: &[0x53, 0x56, 0x8b, 0xf1, 0x57] } => ita_league_sort_clubs_level_on_points, compare_ita_league_sort_clubs_level_on_points;
    Prologue { address: 0x641dc0, bytes: &[0x8a, 0x44, 0x24, 0x0c, 0x83, 0xec, 0x3c] } => ita_ser_a_update_staff_records_for_status, compare_ita_ser_a_update_staff_records_for_status;
    Prologue { address: 0x64b9e0, bytes: &[0x8b, 0x51, 0x2c, 0x8b, 0x41, 0x30] } => ita_ser_c1a_advance_stage, compare_ita_ser_c1a_advance_stage;
    Prologue { address: 0x64ba70, bytes: &[0x6a, 0xff, 0x68, 0xec, 0xba, 0x95, 0x00] } => create_playoff_stages, compare_create_playoff_stages;
    Prologue { address: 0x64fba0, bytes: &[0x8b, 0x51, 0x2c, 0x8b, 0x41, 0x30] } => ita_ser_c1b_advance_stage, compare_ita_ser_c1b_advance_stage;
    Prologue { address: 0x64fc30, bytes: &[0x6a, 0xff, 0x68, 0x8c, 0xbb, 0x95, 0x00] } => create_play_off_stages, compare_create_play_off_stages;
    Prologue { address: 0x578330, bytes: &[0x8b, 0x51, 0x2c, 0x8b, 0x41, 0x30] } => eng_third_advance_stage, compare_eng_third_advance_stage;
    Prologue { address: 0x573380, bytes: &[0x6a, 0xff, 0x68, 0xc9, 0x83, 0x95, 0x00] } => construct_eng_prm, compare_construct_eng_prm;
    Prologue { address: 0x571680, bytes: &[0x6a, 0xff, 0x68, 0x59, 0x83, 0x95, 0x00] } => construct_eng_first, compare_construct_eng_first;
    Prologue { address: 0x5754a0, bytes: &[0x6a, 0xff, 0x68, 0x19, 0x84, 0x95, 0x00] } => construct_eng_second, compare_construct_eng_second;
    Prologue { address: 0x576dd0, bytes: &[0x6a, 0xff, 0x68, 0x89, 0x84, 0x95, 0x00] } => construct_eng_third, compare_construct_eng_third;
    Prologue { address: 0x6406d0, bytes: &[0x6a, 0xff, 0x68, 0x59, 0xb9, 0x95, 0x00] } => construct_ita_ser_a, compare_construct_ita_ser_a;
    Prologue { address: 0x645810, bytes: &[0x6a, 0xff, 0x68, 0x09, 0xba, 0x95, 0x00] } => construct_ita_ser_b, compare_construct_ita_ser_b;
    Prologue { address: 0x64aa70, bytes: &[0x6a, 0xff, 0x68, 0x99, 0xba, 0x95, 0x00] } => construct_ita_ser_c1a, compare_construct_ita_ser_c1a;
    Prologue { address: 0x64ec20, bytes: &[0x6a, 0xff, 0x68, 0x39, 0xbb, 0x95, 0x00] } => construct_ita_ser_c1b, compare_construct_ita_ser_c1b;
    Prologue { address: 0x5be330, bytes: &[0x6a, 0xff, 0x68, 0x89, 0x9a, 0x95, 0x00] } => construct_fra_second, compare_construct_fra_second;
    Prologue { address: 0x5bfb40, bytes: &[0x6a, 0xff, 0x68, 0x29, 0x9b, 0x95, 0x00] } => construct_fra_third, compare_construct_fra_third;
    Prologue { address: 0x8539a0, bytes: &[0x6a, 0xff, 0x68, 0x59, 0x27, 0x96, 0x00] } => construct_spa_second, compare_construct_spa_second;
    Prologue { address: 0x84e130, bytes: &[0x6a, 0xff, 0x68, 0xa7, 0x26, 0x96, 0x00] } => construct_spa_first, compare_construct_spa_first;
    Prologue { address: 0x851020, bytes: &[0x6a, 0xff, 0x68, 0x0b, 0x27, 0x96, 0x00] } => construct_spa_lower, compare_construct_spa_lower;
    Prologue { address: 0x855b40, bytes: &[0x81, 0xec, 0x08, 0x02, 0x00, 0x00] } => spa_second_create_table, compare_spa_second_create_table;
    Prologue { address: 0x8533d0, bytes: &[0x81, 0xec, 0x9c, 0x03, 0x00, 0x00] } => spa_lower_create_table, compare_spa_lower_create_table;
    Prologue { address: 0x5ba0a0, bytes: &[0x56, 0x8b, 0xf1, 0xb8, 0x02, 0x00, 0x00, 0x00] } => fra_cfa_set_up_rules, compare_fra_cfa_set_up_rules;
    Prologue { address: 0x5b91c0, bytes: &[0x6a, 0xff, 0x68, 0xab, 0x98, 0x95, 0x00] } => construct_fra_cfa, compare_construct_fra_cfa;
    Prologue { address: 0x5bd600, bytes: &[0x6a, 0xff, 0x68, 0x3b, 0x9a, 0x95, 0x00] } => construct_fra_lower, compare_construct_fra_lower;
    Prologue { address: 0x5b9460, bytes: &[0x6a, 0xff, 0x68, 0xc8, 0x98, 0x95, 0x00] } => destroy_fra_cfa, compare_destroy_fra_cfa;
    Prologue { address: 0x5bd870, bytes: &[0x6a, 0xff, 0x68, 0x58, 0x9a, 0x95, 0x00] } => destroy_fra_lower, compare_destroy_fra_lower;
    Prologue { address: 0x642a70, bytes: &[0x6a, 0xff, 0x68, 0xde, 0xb9, 0x95, 0x00] } => create_fixed_fixtures, compare_create_fixed_fixtures;
    Prologue { address: 0x647230, bytes: &[0x6a, 0xff, 0x68, 0x6e, 0xba, 0x95, 0x00] } => ita_ser_b_set_up_fixed_fixtures, compare_ita_ser_b_set_up_fixed_fixtures;
    Prologue { address: 0x64be90, bytes: &[0x6a, 0xff, 0x68, 0x0e, 0xbb, 0x95, 0x00] } => ita_ser_c1a_set_up_fixed_fixtures, compare_ita_ser_c1a_set_up_fixed_fixtures;
    Prologue { address: 0x650100, bytes: &[0x6a, 0xff, 0x68, 0xae, 0xbb, 0x95, 0x00] } => ita_ser_c1b_set_up_fixed_fixtures, compare_ita_ser_c1b_set_up_fixed_fixtures;
    Prologue { address: 0x5bb540, bytes: &[0xb8, 0x01, 0x00, 0x00, 0x00] } => stub_return_one_ret8, compare_stub_return_one_ret8;
    Prologue { address: 0x5bb550, bytes: &[0xb8, 0x01, 0x00, 0x00, 0x00] } => stub_return_one_ret4, compare_stub_return_one_ret4;
    Prologue { address: 0x85c260, bytes: &[0x6a, 0xff, 0x68, 0xd1, 0x29, 0x96, 0x00] } => set_fixture_venue, compare_set_fixture_venue;
    Prologue { address: 0x85c800, bytes: &[0x83, 0xec, 0x18, 0x53, 0x55] } => find_available_venue, compare_find_available_venue;
    Prologue { address: 0x85cba0, bytes: &[0x55, 0x8b, 0xec, 0x83, 0xe4, 0xf8] } => calculate_match_attendance, compare_calculate_match_attendance;
    Prologue { address: 0x5bfe30, bytes: &[0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => fra_third_create_schedule, compare_fra_third_create_schedule;
    Prologue { address: 0x5be620, bytes: &[0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => fra_second_create_schedule, compare_fra_second_create_schedule;
    Prologue { address: 0x5bd920, bytes: &[0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => fra_lower_create_schedule, compare_fra_lower_create_schedule;
    Prologue { address: 0x5b9510, bytes: &[0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => fra_cfa_create_schedule, compare_fra_cfa_create_schedule;
    Prologue { address: 0x64ad80, bytes: &[0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => ita_ser_c1a_create_schedule, compare_ita_ser_c1a_create_schedule;
    Prologue { address: 0x64ef30, bytes: &[0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => ita_ser_c1b_create_schedule, compare_ita_ser_c1b_create_schedule;
    Prologue { address: 0x6409b0, bytes: &[0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => ita_ser_a_create_schedule, compare_ita_ser_a_create_schedule;
    Prologue { address: 0x645af0, bytes: &[0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => ita_ser_b_create_schedule, compare_ita_ser_b_create_schedule;
    Prologue { address: 0x5bb690, bytes: &[0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => fra_first_create_schedule, compare_fra_first_create_schedule;
    Prologue { address: 0x434ae0, bytes: &[0x8b, 0x51, 0x2c, 0x8b, 0x41, 0x30] } => bra_reg_minas_gerais_advance_stage, compare_bra_reg_minas_gerais_advance_stage;
    Prologue { address: 0x434b70, bytes: &[0x6a, 0xff, 0x68, 0xee, 0x47, 0x95, 0x00] } => bra_reg_minas_gerais_create_final_stage, compare_bra_reg_minas_gerais_create_final_stage;
    Prologue { address: 0x434ed0, bytes: &[0x56, 0x8b, 0xf1, 0x83, 0x7e, 0x30, 0x02] } => bra_reg_minas_gerais_record_season_honours, compare_bra_reg_minas_gerais_record_season_honours;
    Prologue { address: 0x438230, bytes: &[0x8b, 0x51, 0x2c, 0x8b, 0x41, 0x30] } => bra_reg_parana_advance_stage, compare_bra_reg_parana_advance_stage;
    Prologue { address: 0x4382c0, bytes: &[0x6a, 0xff, 0x68, 0x5e, 0x49, 0x95, 0x00] } => bra_reg_parana_create_final_stage, compare_bra_reg_parana_create_final_stage;
    Prologue { address: 0x4384e0, bytes: &[0x56, 0x8b, 0xf1, 0x83, 0x7e, 0x30, 0x02] } => bra_reg_parana_record_season_honours, compare_bra_reg_parana_record_season_honours;
    Prologue { address: 0x43a870, bytes: &[0x8b, 0x51, 0x2c, 0x8b, 0x41, 0x30] } => bra_reg_rio_advance_stage, compare_bra_reg_rio_advance_stage;
    Prologue { address: 0x43a900, bytes: &[0x6a, 0xff, 0x68, 0x5e, 0x4a, 0x95, 0x00] } => bra_reg_rio_create_final_stage, compare_bra_reg_rio_create_final_stage;
    Prologue { address: 0x43ad20, bytes: &[0x56, 0x8b, 0xf1, 0x83, 0x7e, 0x30, 0x02] } => bra_reg_rio_record_season_honours, compare_bra_reg_rio_record_season_honours;
    Prologue { address: 0x63ec50, bytes: &[0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => ita_cup_create_schedule, compare_ita_cup_create_schedule;
    Prologue { address: 0x63cf90, bytes: &[0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => ita_c_cup_create_schedule, compare_ita_c_cup_create_schedule;
    Prologue { address: 0x65f920, bytes: &[0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => ita_super_create_round_table, compare_ita_super_create_round_table;
    Prologue { address: 0x63c890, bytes: &[0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => ita_c1_super_create_round_table, compare_ita_c1_super_create_round_table;
    Prologue { address: 0x63faf0, bytes: &[0x81, 0xec, 0x8c, 0x02, 0x00, 0x00] } => ita_cup_select_entrants, compare_ita_cup_select_entrants;
    Prologue { address: 0x63dfe0, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => ita_c_cup_select_entrants, compare_ita_c_cup_select_entrants;
    Prologue { address: 0x65fa60, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => ita_super_init_team_list, compare_ita_super_init_team_list;
    Prologue { address: 0x427700, bytes: &[0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => bra_cup_create_round_table, compare_bra_cup_create_round_table;
    Prologue { address: 0x427c20, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => bra_cup_build_team_table, compare_bra_cup_build_team_table;
    Prologue { address: 0x427ec0, bytes: &[0x56, 0x8b, 0xf1, 0x6a, 0x00] } => bra_cup_set_up_rounds, compare_bra_cup_set_up_rounds;
    Prologue { address: 0x4c31c0, bytes: &[0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => conmebol_liber_create_schedule, compare_conmebol_liber_create_schedule;
    Prologue { address: 0x4c4930, bytes: &[0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => conmebol_merc_create_schedule, compare_conmebol_merc_create_schedule;
    Prologue { address: 0x4c5850, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => conmebol_merc_select_invited_clubs, compare_conmebol_merc_select_invited_clubs;
    Prologue { address: 0x5757a0, bytes: &[0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x08, 0x02, 0x00, 0x00] } => eng_second_create_schedule, compare_eng_second_create_schedule;
    Prologue { address: 0x5770e0, bytes: &[0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x08, 0x02, 0x00, 0x00] } => eng_third_create_schedule, compare_eng_third_create_schedule;
    Prologue { address: 0x571980, bytes: &[0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x08, 0x02, 0x00, 0x00] } => eng_first_create_schedule, compare_eng_first_create_schedule;
    Prologue { address: 0x573660, bytes: &[0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x08, 0x02, 0x00, 0x00] } => eng_prm_create_schedule, compare_eng_prm_create_schedule;
    Prologue { address: 0x84e450, bytes: &[0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x08, 0x02, 0x00, 0x00] } => spa_first_create_schedule, compare_spa_first_create_schedule;
    Prologue { address: 0x42fe10, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => bra_reg_bahia_start_new_season, compare_bra_reg_bahia_start_new_season;
    Prologue { address: 0x42ffc0, bytes: &[0x8b, 0x51, 0x2c, 0x8b, 0x41, 0x30] } => bra_reg_bahia_advance_stage, compare_bra_reg_bahia_advance_stage;
    Prologue { address: 0x431090, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => bra_reg_central_start_new_season, compare_bra_reg_central_start_new_season;
    Prologue { address: 0x431240, bytes: &[0x8b, 0x51, 0x2c, 0x8b, 0x41, 0x30] } => bra_reg_central_advance_stage, compare_bra_reg_central_advance_stage;
    Prologue { address: 0x432370, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => bra_reg_gaucho_start_new_season, compare_bra_reg_gaucho_start_new_season;
    Prologue { address: 0x432520, bytes: &[0x8b, 0x51, 0x2c, 0x8b, 0x41, 0x30] } => bra_reg_gaucho_advance_stage, compare_bra_reg_gaucho_advance_stage;
    Prologue { address: 0x4336f0, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => bra_reg_goias_start_new_season, compare_bra_reg_goias_start_new_season;
    Prologue { address: 0x4338a0, bytes: &[0x8b, 0x51, 0x2c, 0x8b, 0x41, 0x30] } => bra_reg_goias_advance_stage, compare_bra_reg_goias_advance_stage;
    Prologue { address: 0x435e80, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => bra_reg_north_start_new_season, compare_bra_reg_north_start_new_season;
    Prologue { address: 0x436030, bytes: &[0x8b, 0x51, 0x2c, 0x8b, 0x41, 0x30] } => bra_reg_north_advance_stage, compare_bra_reg_north_advance_stage;
    Prologue { address: 0x436e90, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => bra_reg_northeast_start_new_season, compare_bra_reg_northeast_start_new_season;
    Prologue { address: 0x437040, bytes: &[0x8b, 0x51, 0x2c, 0x8b, 0x41, 0x30] } => bra_reg_northeast_advance_stage, compare_bra_reg_northeast_advance_stage;
    Prologue { address: 0x4394d0, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => bra_reg_pern_start_new_season, compare_bra_reg_pern_start_new_season;
    Prologue { address: 0x439680, bytes: &[0x8b, 0x51, 0x2c, 0x8b, 0x41, 0x30] } => bra_reg_pern_advance_stage, compare_bra_reg_pern_advance_stage;
    Prologue { address: 0x43bd10, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => bra_reg_santa_start_new_season, compare_bra_reg_santa_start_new_season;
    Prologue { address: 0x43bec0, bytes: &[0x8b, 0x51, 0x2c, 0x8b, 0x41, 0x30] } => bra_reg_santa_advance_stage, compare_bra_reg_santa_advance_stage;
    Prologue { address: 0x43cd40, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => bra_reg_sp_start_new_season, compare_bra_reg_sp_start_new_season;
    Prologue { address: 0x43cef0, bytes: &[0x8b, 0x51, 0x2c, 0x8b, 0x41, 0x30] } => bra_reg_sp_advance_stage, compare_bra_reg_sp_advance_stage;
    Prologue { address: 0x853c90, bytes: &[0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x08, 0x02, 0x00, 0x00] } => spa_second_create_schedule, compare_spa_second_create_schedule;
    Prologue { address: 0x851260, bytes: &[0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x08, 0x02, 0x00, 0x00] } => spa_lower_create_schedule, compare_spa_lower_create_schedule;
    Prologue { address: 0x5bb160, bytes: &[0x6a, 0xff, 0x68, 0x81, 0x99, 0x95, 0x00] } => construct_fra_first, compare_construct_fra_first;
    Prologue { address: 0x42a950, bytes: &[0x8b, 0x51, 0x2c, 0x8b, 0x41, 0x30] } => bra_nat_first_advance_stage, compare_bra_nat_first_advance_stage;
    Prologue { address: 0x42a9e0, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] } => bra_nat_first_create_play_off_stage, compare_bra_nat_first_create_play_off_stage;
    Prologue { address: 0x42cdd0, bytes: &[0x8b, 0x51, 0x2c, 0x8b, 0x41, 0x30] } => bra_nat_second_advance_stage, compare_bra_nat_second_advance_stage;
    Prologue { address: 0x42ce60, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] } => bra_nat_second_create_play_off_stage, compare_bra_nat_second_create_play_off_stage;
    Prologue { address: 0x42a640, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => bra_nat_first_start_new_season, compare_bra_nat_first_start_new_season;
    Prologue { address: 0x42ae90, bytes: &[0xa1, 0xfc, 0xad, 0xad, 0x00] } => bra_nat_first_process_promotion_relegation_chain, compare_bra_nat_first_process_promotion_relegation_chain;
    Prologue { address: 0x42cc10, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => bra_nat_second_start_new_season, compare_bra_nat_second_start_new_season;
    Prologue { address: 0x6639c0, bytes: &[0x8b, 0x51, 0x2c, 0x8b, 0x41, 0x30] } => jap_j1_advance_stage, compare_jap_j1_advance_stage;
    Prologue { address: 0x663a50, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] } => create_championship_final_stage, compare_create_championship_final_stage;
    Prologue { address: 0x6636d0, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => jap_j1_start_new_season, compare_jap_j1_start_new_season;
    Prologue { address: 0x5f9970, bytes: &[0x6a, 0xff, 0x68, 0x5b, 0xaf, 0x95, 0x00] } => init_default_table, compare_init_default_table;
    Prologue { address: 0x5faec0, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] } => post_host_news, compare_post_host_news;
    Prologue { address: 0x5fb160, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => load_temp_file, compare_load_temp_file;
    Prologue { address: 0x689c20, bytes: &[0x53, 0x8b, 0xd9, 0x57, 0x8b, 0x43, 0x1c] } => league_process_promotion_relegation_chain, compare_league_process_promotion_relegation_chain;
    Prologue { address: 0x48e3e0, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] } => send_prize_money_news, compare_send_prize_money_news;
    Prologue { address: 0x48ea40, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] } => send_tv_rights_news, compare_send_tv_rights_news;
    Prologue { address: 0x425c10, bytes: &[0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => bra_champ_cup_create_schedule, compare_bra_champ_cup_create_schedule;
    Prologue { address: 0x4c1960, bytes: &[0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => con_merc_cup_create_schedule, compare_con_merc_cup_create_schedule;
    Prologue { address: 0x57a3c0, bytes: &[0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => eur_super_cup_create_round_table, compare_eur_super_cup_create_round_table;
    Prologue { address: 0x410910, bytes: &[0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => asia_super_cup_create_round_table, compare_asia_super_cup_create_round_table;
    Prologue { address: 0x40ae90, bytes: &[0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => asia_club_champ_create_round_table, compare_asia_club_champ_create_round_table;
    Prologue { address: 0x40cf90, bytes: &[0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => asia_cup_winner_create_round_table, compare_asia_cup_winner_create_round_table;
    Prologue { address: 0x56ade0, bytes: &[0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => eng_auto_cup_create_round_table, compare_eng_auto_cup_create_round_table;
    Prologue { address: 0x5e0900, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => goldcup_create_schedule, compare_goldcup_create_schedule;
    Prologue { address: 0x4012e0, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => african_nations_create_schedule, compare_african_nations_create_schedule;
    Prologue { address: 0x58b560, bytes: &[0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => fifa_confed_create_schedule, compare_fifa_confed_create_schedule;
    Prologue { address: 0x633300, bytes: &[0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => intertoto_cup_create_round_table, compare_intertoto_cup_create_round_table;
    Prologue { address: 0x92b770, bytes: &[0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => world_club_cup_create_round_table, compare_world_club_cup_create_round_table;
    Prologue { address: 0x632340, bytes: &[0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => inter_amer_cup_create_round_table, compare_inter_amer_cup_create_round_table;
    Prologue { address: 0x4c0f00, bytes: &[0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => con_champ_create_round_table, compare_con_champ_create_round_table;
    Prologue { address: 0x9025c0, bytes: &[0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => uefa_cup_create_round_table, compare_uefa_cup_create_round_table;
    Prologue { address: 0x6336c0, bytes: &[0x81, 0xec, 0x04, 0x02, 0x00, 0x00] } => intertoto_cup_collect_entrants, compare_intertoto_cup_collect_entrants;
    Prologue { address: 0x633f70, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => intertoto_cup_collect_club_list, compare_intertoto_cup_collect_club_list;
    Prologue { address: 0x57a500, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => eur_super_cup_select_teams, compare_eur_super_cup_select_teams;
    Prologue { address: 0x410a50, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => asia_super_cup_select_teams, compare_asia_super_cup_select_teams;
    Prologue { address: 0x63c9d0, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => ita_c1_super_init_team_list, compare_ita_c1_super_init_team_list;
    Prologue { address: 0x797230, bytes: &[0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => oceania_club_champ_create_schedule, compare_oceania_club_champ_create_schedule;
    Prologue { address: 0x40b620, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => asia_club_champ_build_team_table, compare_asia_club_champ_build_team_table;
    Prologue { address: 0x40d4d0, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => create_team_entries, compare_create_team_entries;
    Prologue { address: 0x43afa0, bytes: &[0x6a, 0xff, 0x68, 0x7e, 0x4a, 0x95, 0x00] } => bra_reg_rio_create_half_season_stage, compare_bra_reg_rio_create_half_season_stage;
    Prologue { address: 0x4351b0, bytes: &[0x6a, 0xff, 0x68, 0x0e, 0x48, 0x95, 0x00] } => bra_reg_minas_gerais_create_half_season_stage, compare_bra_reg_minas_gerais_create_half_season_stage;
    Prologue { address: 0x438760, bytes: &[0x6a, 0xff, 0x68, 0x7e, 0x49, 0x95, 0x00] } => create_half_stage, compare_create_half_stage;
    Prologue { address: 0x663c20, bytes: &[0x6a, 0xff, 0x68, 0x5e, 0xc1, 0x95, 0x00] } => jap_j1_create_half_season_stage, compare_jap_j1_create_half_season_stage;
    Prologue { address: 0x43a6a0, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => bra_reg_rio_start_new_season, compare_bra_reg_rio_start_new_season;
    Prologue { address: 0x434910, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => bra_reg_minas_gerais_start_new_season, compare_bra_reg_minas_gerais_start_new_season;
    Prologue { address: 0x438060, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => bra_reg_parana_start_new_season, compare_bra_reg_parana_start_new_season;
    Prologue { address: 0x411f70, bytes: &[0x8b, 0x51, 0x2c, 0x8b, 0x41, 0x30] } => aus_nsl_advance_stage, compare_aus_nsl_advance_stage;
    Prologue { address: 0x412010, bytes: &[0x6a, 0xff, 0x68, 0x1e, 0x3a, 0x95, 0x00] } => create_finals_series_stage, compare_create_finals_series_stage;
    Prologue { address: 0x4121c0, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] } => create_grand_final_stage, compare_create_grand_final_stage;
    Prologue { address: 0x909400, bytes: &[0x8b, 0x51, 0x2c, 0x8b, 0x41, 0x30] } => usa_mls_advance_stage, compare_usa_mls_advance_stage;
    Prologue { address: 0x9094c0, bytes: &[0x6a, 0xff, 0x68, 0x1e, 0x54, 0x96, 0x00] } => usa_mls_create_playoff_stage, compare_usa_mls_create_playoff_stage;
    Prologue { address: 0x9097d0, bytes: &[0x6a, 0xff, 0x68, 0x3e, 0x54, 0x96, 0x00] } => usa_mls_create_final_stage, compare_usa_mls_create_final_stage;
    Prologue { address: 0x427d10, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => bra_cup_start_new_season, compare_bra_cup_start_new_season;
    Prologue { address: 0x662950, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => jap_emp_cup_start_new_season, compare_jap_emp_cup_start_new_season;
    Prologue { address: 0x665bc0, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => jap_j_cup_start_new_season, compare_jap_j_cup_start_new_season;
    Prologue { address: 0x6664e0, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => jap_super_start_new_season, compare_jap_super_start_new_season;
    Prologue { address: 0x90b790, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => usa_open_cup_start_new_season, compare_usa_open_cup_start_new_season;
    Prologue { address: 0x90a760, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => usa_mls_all_stars_start_new_season_90a760, compare_usa_mls_all_stars_start_new_season_90a760;
    Prologue { address: 0x90a540, bytes: &[0x81, 0xec, 0x04, 0x02, 0x00, 0x00] } => set_up_teams, compare_set_up_teams;
    Prologue { address: 0x4c4410, bytes: &[0x81, 0xec, 0x10, 0x02, 0x00, 0x00] } => conmebol_liber_update_for_date, compare_conmebol_liber_update_for_date;
    Prologue { address: 0x4c37a0, bytes: &[0x81, 0xec, 0x04, 0x02, 0x00, 0x00] } => build_first_group_table, compare_build_first_group_table;
    Prologue { address: 0x4c3b10, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] } => conmebol_liber_create_group_stage, compare_conmebol_liber_create_group_stage;
    Prologue { address: 0x4c5c90, bytes: &[0x81, 0xec, 0x10, 0x02, 0x00, 0x00] } => conmebol_merc_update_for_date, compare_conmebol_merc_update_for_date;
    Prologue { address: 0x4c4e60, bytes: &[0x81, 0xec, 0x04, 0x02, 0x00, 0x00] } => conmebol_merc_create_first_group_table, compare_conmebol_merc_create_first_group_table;
    Prologue { address: 0x4c51d0, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] } => conmebol_merc_create_group_stage, compare_conmebol_merc_create_group_stage;
    Prologue { address: 0x4c2c90, bytes: &[0x81, 0xec, 0x10, 0x02, 0x00, 0x00] } => con_merc_cup_update_for_date, compare_con_merc_cup_update_for_date;
    Prologue { address: 0x4c1de0, bytes: &[0x81, 0xec, 0x04, 0x02, 0x00, 0x00] } => con_merc_cup_create_first_group_table, compare_con_merc_cup_create_first_group_table;
    Prologue { address: 0x4c2140, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] } => con_merc_cup_create_group_stage, compare_con_merc_cup_create_group_stage;
    Prologue { address: 0x5e2120, bytes: &[0x6a, 0xff, 0x68, 0xe6, 0xa8, 0x95, 0x00] } => goldcup_update_for_date, compare_goldcup_update_for_date;
    Prologue { address: 0x5e11e0, bytes: &[0x81, 0xec, 0x04, 0x02, 0x00, 0x00] } => goldcup_create_group_table, compare_goldcup_create_group_table;
    Prologue { address: 0x5e16b0, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] } => goldcup_create_group_stage, compare_goldcup_create_group_stage;
    Prologue { address: 0x908560, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => usa_mls_create_table, compare_usa_mls_create_table;
    Prologue { address: 0x908fc0, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] } => create_second_division_stage, compare_create_second_division_stage;
    Prologue { address: 0x9091e0, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] } => create_third_division_stage, compare_create_third_division_stage;
    Prologue { address: 0x908e00, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => usa_mls_start_new_season, compare_usa_mls_start_new_season;
    Prologue { address: 0x909bb0, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => usa_mls_set_predicted_positions, compare_usa_mls_set_predicted_positions;
    Prologue { address: 0x909a40, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => usa_mls_get_club_array, compare_usa_mls_get_club_array;
    Prologue { address: 0x42ee80, bytes: &[0x6a, 0xff, 0x68, 0xbe, 0x45, 0x95, 0x00] } => bra_nat_third_create_group_stage, compare_bra_nat_third_create_group_stage;
    Prologue { address: 0x42e430, bytes: &[0x6a, 0xff, 0x68, 0x4e, 0x45, 0x95, 0x00] } => create_second_round_stage, compare_create_second_round_stage;
    Prologue { address: 0x42e700, bytes: &[0x6a, 0xff, 0x68, 0x7c, 0x45, 0x95, 0x00] } => create_third_round_groups, compare_create_third_round_groups;
    Prologue { address: 0x42e980, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] } => bra_nat_third_create_final_stage, compare_bra_nat_third_create_final_stage;
    Prologue { address: 0x42def0, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => bra_nat_third_start_new_season, compare_bra_nat_third_start_new_season;
    Prologue { address: 0x42f0a0, bytes: &[0x81, 0xec, 0x04, 0x02, 0x00, 0x00] } => bra_nat_third_set_predicted_positions, compare_bra_nat_third_set_predicted_positions;
    Prologue { address: 0x56b730, bytes: &[0x81, 0xec, 0x80, 0x02, 0x00, 0x00] } => select_section_clubs, compare_select_section_clubs;
    Prologue { address: 0x56bab0, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] } => create_section_stage, compare_create_section_stage;
    Prologue { address: 0x56bc50, bytes: &[0x8b, 0x51, 0x2c, 0x8b, 0x41, 0x30] } => eng_auto_cup_advance_stage, compare_eng_auto_cup_advance_stage;
    Prologue { address: 0x56bce0, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] } => eng_auto_cup_create_final_stage, compare_eng_auto_cup_create_final_stage;
    Prologue { address: 0x56c010, bytes: &[0x81, 0xec, 0x10, 0x02, 0x00, 0x00] } => eng_auto_cup_rank_teams, compare_eng_auto_cup_rank_teams;
    Prologue { address: 0x522e00, bytes: &[0x6a, 0xff, 0x68, 0x3b, 0x78, 0x95, 0x00] } => construct_cup_stage, compare_construct_cup_stage;
    Prologue { address: 0x523300, bytes: &[0x56, 0x8b, 0xf1, 0x8b, 0x86, 0xa7, 0x00, 0x00, 0x00] } => destroy_cup_stage, compare_destroy_cup_stage;
    Prologue { address: 0x4c3d10, bytes: &[0x6a, 0xff, 0x68, 0xbe, 0x6a, 0x95, 0x00] } => conmebol_liber_create_knockout_stage, compare_conmebol_liber_create_knockout_stage;
    Prologue { address: 0x4c2330, bytes: &[0x6a, 0xff, 0x68, 0x0e, 0x6a, 0x95, 0x00] } => con_merc_cup_create_semi_final_stage, compare_con_merc_cup_create_semi_final_stage;
    Prologue { address: 0x4c20b0, bytes: &[0x8b, 0x51, 0x2c, 0x8b, 0x41, 0x30] } => con_merc_cup_advance_stage, compare_con_merc_cup_advance_stage;
    Prologue { address: 0x4c2560, bytes: &[0x81, 0xec, 0x24, 0x04, 0x00, 0x00] } => con_merc_cup_select_invited_clubs, compare_con_merc_cup_select_invited_clubs;
    Prologue { address: 0x4c5140, bytes: &[0x8b, 0x51, 0x2c, 0x8b, 0x41, 0x30] } => conmebol_merc_advance_stage, compare_conmebol_merc_advance_stage;
    Prologue { address: 0x5e1620, bytes: &[0x8b, 0x51, 0x2c, 0x8b, 0x41, 0x30] } => goldcup_advance_stage, compare_goldcup_advance_stage;
    Prologue { address: 0x5e1cd0, bytes: &[0x81, 0xec, 0x10, 0x02, 0x00, 0x00] } => set_up_team_list, compare_set_up_team_list;
    Prologue { address: 0x5e2340, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => goldcup_set_predicted_positions, compare_goldcup_set_predicted_positions;
    Prologue { address: 0x5e1530, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => goldcup_start_new_season, compare_goldcup_start_new_season;
    Prologue { address: 0x42f2d0, bytes: &[0x6a, 0xff, 0x68, 0xe9, 0x45, 0x95, 0x00] } => construct_bra_reg_bahia, compare_construct_bra_reg_bahia;
    Prologue { address: 0x4304b0, bytes: &[0x6a, 0xff, 0x68, 0x59, 0x46, 0x95, 0x00] } => construct_bra_reg_central, compare_construct_bra_reg_central;
    Prologue { address: 0x4317c0, bytes: &[0x6a, 0xff, 0x68, 0xc9, 0x46, 0x95, 0x00] } => construct_bra_reg_gaucho, compare_construct_bra_reg_gaucho;
    Prologue { address: 0x432b50, bytes: &[0x6a, 0xff, 0x68, 0x39, 0x47, 0x95, 0x00] } => construct_bra_reg_goias, compare_construct_bra_reg_goias;
    Prologue { address: 0x433d50, bytes: &[0x6a, 0xff, 0x68, 0xa9, 0x47, 0x95, 0x00] } => construct_bra_reg_minas_gerais, compare_construct_bra_reg_minas_gerais;
    Prologue { address: 0x435340, bytes: &[0x6a, 0xff, 0x68, 0x39, 0x48, 0x95, 0x00] } => construct_bra_reg_north, compare_construct_bra_reg_north;
    Prologue { address: 0x4363f0, bytes: &[0x6a, 0xff, 0x68, 0xa9, 0x48, 0x95, 0x00] } => construct_bra_reg_northeast, compare_construct_bra_reg_northeast;
    Prologue { address: 0x437400, bytes: &[0x6a, 0xff, 0x68, 0x19, 0x49, 0x95, 0x00] } => construct_bra_reg_parana, compare_construct_bra_reg_parana;
    Prologue { address: 0x4388f0, bytes: &[0x6a, 0xff, 0x68, 0xa9, 0x49, 0x95, 0x00] } => construct_bra_reg_pern, compare_construct_bra_reg_pern;
    Prologue { address: 0x439a40, bytes: &[0x6a, 0xff, 0x68, 0x19, 0x4a, 0x95, 0x00] } => construct_bra_reg_rio, compare_construct_bra_reg_rio;
    Prologue { address: 0x43b130, bytes: &[0x6a, 0xff, 0x68, 0xa9, 0x4a, 0x95, 0x00] } => construct_bra_reg_santa, compare_construct_bra_reg_santa;
    Prologue { address: 0x43c280, bytes: &[0x6a, 0xff, 0x68, 0x19, 0x4b, 0x95, 0x00] } => construct_bra_reg_sp, compare_construct_bra_reg_sp;
    Prologue { address: 0x5d6280, bytes: &[0x55, 0x8b, 0xec, 0x83, 0xe4, 0xf8] } => cqt4_update_average_attendance, compare_cqt4_update_average_attendance;
    Prologue { address: 0x797b40, bytes: &[0x6a, 0xff, 0x68, 0x16, 0xeb, 0x95, 0x00] } => oceania_club_champ_update_for_date, compare_oceania_club_champ_update_for_date;
    Prologue { address: 0x7978c0, bytes: &[0x81, 0xec, 0x04, 0x02, 0x00, 0x00] } => create_group_a_table, compare_create_group_a_table;
    Prologue { address: 0x797e80, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] } => create_group_b_stage, compare_create_group_b_stage;
    Prologue { address: 0x798220, bytes: &[0x81, 0xec, 0xfc, 0x03, 0x00, 0x00] } => oceania_club_champ_select_qualified_clubs, compare_oceania_club_champ_select_qualified_clubs;
    Prologue { address: 0x797df0, bytes: &[0x8b, 0x51, 0x2c, 0x8b, 0x41, 0x30] } => oceania_club_champ_advance_stage, compare_oceania_club_champ_advance_stage;
    Prologue { address: 0x798050, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] } => oceania_club_champ_create_knockout_stage, compare_oceania_club_champ_create_knockout_stage;
    Prologue { address: 0x40c070, bytes: &[0x8b, 0x51, 0x2c, 0x8b, 0x41, 0x30] } => asia_club_champ_advance_stage, compare_asia_club_champ_advance_stage;
    Prologue { address: 0x40c110, bytes: &[0x6a, 0xff, 0x68, 0xcc, 0x37, 0x95, 0x00] } => create_group_stages, compare_create_group_stages;
    Prologue { address: 0x40c3d0, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] } => asia_club_champ_create_semi_final_stage, compare_asia_club_champ_create_semi_final_stage;
    Prologue { address: 0x4025a0, bytes: &[0x6a, 0xff, 0x68, 0x46, 0x35, 0x95, 0x00] } => african_nations_update_for_date, compare_african_nations_update_for_date;
    Prologue { address: 0x4022c0, bytes: &[0x81, 0xec, 0x18, 0x02, 0x00, 0x00] } => draw_finalists, compare_draw_finalists;
    Prologue { address: 0x401ea0, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] } => african_nations_create_group_stage, compare_african_nations_create_group_stage;
    Prologue { address: 0x402090, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] } => asia_cup_winner_create_knockout_stage, compare_asia_cup_winner_create_knockout_stage;
    Prologue { address: 0x40dfa0, bytes: &[0x8b, 0x51, 0x2c, 0x8b, 0x41, 0x30] } => asia_cup_winner_advance_stage, compare_asia_cup_winner_advance_stage;
    Prologue { address: 0x40e030, bytes: &[0x64, 0xa1, 0x00, 0x00, 0x00, 0x00] } => create_finals_stage, compare_create_finals_stage;
    Prologue { address: 0x63f250, bytes: &[0x81, 0xec, 0x04, 0x02, 0x00, 0x00] } => ita_cup_create_first_group_table, compare_ita_cup_create_first_group_table;
    Prologue { address: 0x63d5e0, bytes: &[0x81, 0xec, 0x04, 0x02, 0x00, 0x00] } => init_first_group, compare_init_first_group;
    Prologue { address: 0x63f5b0, bytes: &[0x8b, 0x51, 0x2c, 0x8b, 0x41, 0x30] } => ita_cup_advance_stage, compare_ita_cup_advance_stage;
    Prologue { address: 0x63d950, bytes: &[0x8b, 0x51, 0x2c, 0x8b, 0x41, 0x30] } => ita_cup_advance_stage_63d950, compare_ita_cup_advance_stage_63d950;
    Prologue { address: 0x63f640, bytes: &[0x6a, 0xff, 0x68, 0x0e, 0xb9, 0x95, 0x00] } => ita_cup_create_group_stage, compare_ita_cup_create_group_stage;
    Prologue { address: 0x63d9e0, bytes: &[0x6a, 0xff, 0x68, 0x6e, 0xb8, 0x95, 0x00] } => ita_cup_create_group_stage_63d9e0, compare_ita_cup_create_group_stage_63d9e0;
    Prologue { address: 0x63f870, bytes: &[0x6a, 0xff, 0x68, 0x2e, 0xb9, 0x95, 0x00] } => ita_cup_create_knockout_stage, compare_ita_cup_create_knockout_stage;
    Prologue { address: 0x63dc10, bytes: &[0x6a, 0xff, 0x68, 0x9c, 0xb8, 0x95, 0x00] } => ita_cup_create_knockout_stage_63dc10, compare_ita_cup_create_knockout_stage_63dc10;
    Prologue { address: 0x57e8f0, bytes: &[0x6a, 0xff, 0x68, 0xa8, 0x88, 0x95, 0x00] } => euro_champ_qual_set_up_rules, compare_euro_champ_qual_set_up_rules;
    Prologue { address: 0x57d9d0, bytes: &[0x0f, 0xbe, 0x81, 0xf6, 0x00, 0x00, 0x00] } => get_group_team_count, compare_get_group_team_count;
    Prologue { address: 0x580b50, bytes: &[0x8b, 0x44, 0x24, 0x04, 0x8b, 0x49, 0x08] } => get_qualification_status, compare_get_qualification_status;
    Prologue { address: 0x57e7e0, bytes: &[0x81, 0xec, 0x04, 0x02, 0x00, 0x00] } => init_group_one_table, compare_init_group_one_table;
    Prologue { address: 0x902b90, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => build_entrants, compare_build_entrants;
    Prologue { address: 0x84d740, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] } => spa_cup_update_for_date, compare_spa_cup_update_for_date;
    Prologue { address: 0x584ad0, bytes: &[0x8b, 0x41, 0x0c, 0x8b, 0x48, 0x30] } => spa_cup_record_season_honours, compare_spa_cup_record_season_honours;
    Prologue { address: 0x5e93e0, bytes: &[0x8b, 0x41, 0x0c, 0x8b, 0x48, 0x28] } => spa_cup_record_season_honours_5e93e0, compare_spa_cup_record_season_honours_5e93e0;
    Prologue { address: 0x637750, bytes: &[0x8b, 0x41, 0x0c, 0x8b, 0x48, 0x1c] } => spa_cup_record_season_honours_637750, compare_spa_cup_record_season_honours_637750;
    Prologue { address: 0x63d770, bytes: &[0x8b, 0x41, 0x0c, 0x8b, 0x48, 0x3c] } => spa_cup_record_season_honours_63d770, compare_spa_cup_record_season_honours_63d770;
}
const N: usize = 2092;
include!(concat!(env!("OUT_DIR"), "/indices.rs")); // IDX_<NAME>: the position of each routine (build.rs)

/// Size of the engine object (the game allocates it with new(0xf63b)): side 1's block (+0xf5f2, 0x36
/// bytes) and the fields up to +0xf63a lie beyond 0xf5f8.
const ENGINE_SIZE: usize = 0xf63b;

const GLOBAL_TABLE: usize = 0xb67c30;

/// The player's value in a table of 11 words (one per side), by the player's position (0..=10).
unsafe fn table_by_position(this: usize, jogador: MatchPlayer, campo: usize) -> u16 {
    let posicao = jogador.position();
    if !(0..=10).contains(&posicao) {
        return 0;
    }
    let lado = jogador.side() as isize;
    let tabela = *((this as isize + campo as isize + lado * 4) as *const usize) as *const u16;
    *tabela.add(posicao as usize)
}

/// { `player_side` } (match_pl.cpp, 4 bytes, not hookable): the player's side. Read directly.
unsafe fn side_of_player(player: MatchPlayer) -> i8 {
    player.side()
}

unsafe fn random(n: i32) -> i32 {
    exe::utils_random(n)
}

// The rest of the engine, one module per source file of the game (reorganisation, 07/11/2026).
include!("motor/macros.rs");
#[path = "motor/area.rs"]
pub(crate) mod area;
pub(crate) use self::area::*;
#[path = "motor/award/award_manager.rs"]
pub(crate) mod award_award_manager;
pub(crate) use self::award_award_manager::*;
#[path = "motor/award/month_award.rs"]
pub(crate) mod award_month_award;
pub(crate) use self::award_month_award::*;
#[path = "motor/award/month_ratings.rs"]
pub(crate) mod award_month_ratings;
pub(crate) use self::award_month_ratings::*;
#[path = "motor/awol.rs"]
pub(crate) mod awol;
pub(crate) use self::awol::*;
#[path = "motor/cash.rs"]
pub(crate) mod cash;
pub(crate) use self::cash::*;
#[path = "motor/club_history.rs"]
pub(crate) mod club_history;
pub(crate) use self::club_history::*;
#[path = "motor/club_records.rs"]
pub(crate) mod club_records;
pub(crate) use self::club_records::*;
#[path = "motor/comp/comp.rs"]
pub(crate) mod comp_comp;
pub(crate) use self::comp_comp::*;
#[path = "motor/comp/comp_util.rs"]
pub(crate) mod comp_comp_util;
pub(crate) use self::comp_comp_util::*;
#[path = "motor/comp/conmebol_seeding.rs"]
pub(crate) mod comp_conmebol_seeding;
pub(crate) use self::comp_conmebol_seeding::*;
#[path = "motor/comp/cup.rs"]
pub(crate) mod comp_cup;
pub(crate) use self::comp_cup::*;
#[path = "motor/comp/cups/bra_champ_cup.rs"]
pub(crate) mod comp_cups_bra_champ_cup;
pub(crate) use self::comp_cups_bra_champ_cup::*;
#[path = "motor/comp/cups/bra_cup.rs"]
pub(crate) mod comp_cups_bra_cup;
pub(crate) use self::comp_cups_bra_cup::*;
#[path = "motor/comp/cups/cup_stage.rs"]
pub(crate) mod comp_cups_cup_stage;
pub(crate) use self::comp_cups_cup_stage::*;
#[path = "motor/comp/cups/eng_auto_cup.rs"]
pub(crate) mod comp_cups_eng_auto_cup;
pub(crate) use self::comp_cups_eng_auto_cup::*;
#[path = "motor/comp/cups/eng_cc_cup.rs"]
pub(crate) mod comp_cups_eng_cc_cup;
pub(crate) use self::comp_cups_eng_cc_cup::*;
#[path = "motor/comp/cups/eng_charity.rs"]
pub(crate) mod comp_cups_eng_charity;
pub(crate) use self::comp_cups_eng_charity::*;
#[path = "motor/comp/cups/eng_fa_cup.rs"]
pub(crate) mod comp_cups_eng_fa_cup;
pub(crate) use self::comp_cups_eng_fa_cup::*;
#[path = "motor/comp/cups/fra_cup.rs"]
pub(crate) mod comp_cups_fra_cup;
pub(crate) use self::comp_cups_fra_cup::*;
#[path = "motor/comp/cups/fra_lge_cup.rs"]
pub(crate) mod comp_cups_fra_lge_cup;
pub(crate) use self::comp_cups_fra_lge_cup::*;
#[path = "motor/comp/cups/fra_super.rs"]
pub(crate) mod comp_cups_fra_super;
pub(crate) use self::comp_cups_fra_super::*;
#[path = "motor/comp/cups/ita_c1_super.rs"]
pub(crate) mod comp_cups_ita_c1_super;
pub(crate) use self::comp_cups_ita_c1_super::*;
#[path = "motor/comp/cups/ita_c_cup.rs"]
pub(crate) mod comp_cups_ita_c_cup;
pub(crate) use self::comp_cups_ita_c_cup::*;
#[path = "motor/comp/cups/ita_cup.rs"]
pub(crate) mod comp_cups_ita_cup;
pub(crate) use self::comp_cups_ita_cup::*;
#[path = "motor/comp/cups/ita_super.rs"]
pub(crate) mod comp_cups_ita_super;
pub(crate) use self::comp_cups_ita_super::*;
#[path = "motor/comp/cups/jap_emp_cup.rs"]
pub(crate) mod comp_cups_jap_emp_cup;
pub(crate) use self::comp_cups_jap_emp_cup::*;
#[path = "motor/comp/cups/jap_j_cup.rs"]
pub(crate) mod comp_cups_jap_j_cup;
pub(crate) use self::comp_cups_jap_j_cup::*;
#[path = "motor/comp/cups/jap_super.rs"]
pub(crate) mod comp_cups_jap_super;
pub(crate) use self::comp_cups_jap_super::*;
#[path = "motor/comp/cups/spa_cup.rs"]
pub(crate) mod comp_cups_spa_cup;
pub(crate) use self::comp_cups_spa_cup::*;
#[path = "motor/comp/cups/spa_super.rs"]
pub(crate) mod comp_cups_spa_super;
pub(crate) use self::comp_cups_spa_super::*;
#[path = "motor/comp/cups/usa_mls_all_stars.rs"]
pub(crate) mod comp_cups_usa_mls_all_stars;
pub(crate) use self::comp_cups_usa_mls_all_stars::*;
#[path = "motor/comp/cups/usa_open_cup.rs"]
pub(crate) mod comp_cups_usa_open_cup;
pub(crate) use self::comp_cups_usa_open_cup::*;
#[path = "motor/comp/eurocomp/asia_club_champ.rs"]
pub(crate) mod comp_eurocomp_asia_club_champ;
pub(crate) use self::comp_eurocomp_asia_club_champ::*;
#[path = "motor/comp/eurocomp/asia_cup_winner.rs"]
pub(crate) mod comp_eurocomp_asia_cup_winner;
pub(crate) use self::comp_eurocomp_asia_cup_winner::*;
#[path = "motor/comp/eurocomp/asia_super_cup.rs"]
pub(crate) mod comp_eurocomp_asia_super_cup;
pub(crate) use self::comp_eurocomp_asia_super_cup::*;
#[path = "motor/comp/eurocomp/con_champ.rs"]
pub(crate) mod comp_eurocomp_con_champ;
pub(crate) use self::comp_eurocomp_con_champ::*;
#[path = "motor/comp/eurocomp/con_merc_cup.rs"]
pub(crate) mod comp_eurocomp_con_merc_cup;
pub(crate) use self::comp_eurocomp_con_merc_cup::*;
#[path = "motor/comp/eurocomp/conmebol_liber.rs"]
pub(crate) mod comp_eurocomp_conmebol_liber;
pub(crate) use self::comp_eurocomp_conmebol_liber::*;
#[path = "motor/comp/eurocomp/conmebol_merc.rs"]
pub(crate) mod comp_eurocomp_conmebol_merc;
pub(crate) use self::comp_eurocomp_conmebol_merc::*;
#[path = "motor/comp/eurocomp/eur_super_cup.rs"]
pub(crate) mod comp_eurocomp_eur_super_cup;
pub(crate) use self::comp_eurocomp_eur_super_cup::*;
#[path = "motor/comp/eurocomp/european_cup.rs"]
pub(crate) mod comp_eurocomp_european_cup;
pub(crate) use self::comp_eurocomp_european_cup::*;
#[path = "motor/comp/eurocomp/inter_amer_cup.rs"]
pub(crate) mod comp_eurocomp_inter_amer_cup;
pub(crate) use self::comp_eurocomp_inter_amer_cup::*;
#[path = "motor/comp/eurocomp/intertoto_cup.rs"]
pub(crate) mod comp_eurocomp_intertoto_cup;
pub(crate) use self::comp_eurocomp_intertoto_cup::*;
#[path = "motor/comp/eurocomp/oceania_club_champ.rs"]
pub(crate) mod comp_eurocomp_oceania_club_champ;
pub(crate) use self::comp_eurocomp_oceania_club_champ::*;
#[path = "motor/comp/eurocomp/uefa_cup.rs"]
pub(crate) mod comp_eurocomp_uefa_cup;
pub(crate) use self::comp_eurocomp_uefa_cup::*;
#[path = "motor/comp/eurocomp/world_club_cup.rs"]
pub(crate) mod comp_eurocomp_world_club_cup;
pub(crate) use self::comp_eurocomp_world_club_cup::*;
#[path = "motor/comp/fifa_rankings.rs"]
pub(crate) mod comp_fifa_rankings;
pub(crate) use self::comp_fifa_rankings::*;
#[path = "motor/comp/fix_man.rs"]
pub(crate) mod comp_fix_man;
pub(crate) use self::comp_fix_man::*;
#[path = "motor/comp/friendly.rs"]
pub(crate) mod comp_friendly;
pub(crate) use self::comp_friendly::*;
#[path = "motor/comp/host_country.rs"]
pub(crate) mod comp_host_country;
pub(crate) use self::comp_host_country::*;
#[path = "motor/comp/intercomp/african_nations.rs"]
pub(crate) mod comp_intercomp_african_nations;
pub(crate) use self::comp_intercomp_african_nations::*;
#[path = "motor/comp/intercomp/euro_champ_qual.rs"]
pub(crate) mod comp_intercomp_euro_champ_qual;
pub(crate) use self::comp_intercomp_euro_champ_qual::*;
#[path = "motor/comp/intercomp/fifa_confed.rs"]
pub(crate) mod comp_intercomp_fifa_confed;
pub(crate) use self::comp_intercomp_fifa_confed::*;
#[path = "motor/comp/intercomp/goldcup.rs"]
pub(crate) mod comp_intercomp_goldcup;
pub(crate) use self::comp_intercomp_goldcup::*;
#[path = "motor/comp/league.rs"]
pub(crate) mod comp_league;
pub(crate) use self::comp_league::*;
#[path = "motor/comp/leagues/arg_second.rs"]
pub(crate) mod comp_leagues_arg_second;
pub(crate) use self::comp_leagues_arg_second::*;
#[path = "motor/comp/leagues/aus_nsl.rs"]
pub(crate) mod comp_leagues_aus_nsl;
pub(crate) use self::comp_leagues_aus_nsl::*;
#[path = "motor/comp/leagues/bra_nat_first.rs"]
pub(crate) mod comp_leagues_bra_nat_first;
pub(crate) use self::comp_leagues_bra_nat_first::*;
#[path = "motor/comp/leagues/bra_nat_second.rs"]
pub(crate) mod comp_leagues_bra_nat_second;
pub(crate) use self::comp_leagues_bra_nat_second::*;
#[path = "motor/comp/leagues/bra_nat_third.rs"]
pub(crate) mod comp_leagues_bra_nat_third;
pub(crate) use self::comp_leagues_bra_nat_third::*;
#[path = "motor/comp/leagues/bra_reg_bahia.rs"]
pub(crate) mod comp_leagues_bra_reg_bahia;
pub(crate) use self::comp_leagues_bra_reg_bahia::*;
#[path = "motor/comp/leagues/bra_reg_central.rs"]
pub(crate) mod comp_leagues_bra_reg_central;
pub(crate) use self::comp_leagues_bra_reg_central::*;
#[path = "motor/comp/leagues/bra_reg_gaucho.rs"]
pub(crate) mod comp_leagues_bra_reg_gaucho;
pub(crate) use self::comp_leagues_bra_reg_gaucho::*;
#[path = "motor/comp/leagues/bra_reg_goias.rs"]
pub(crate) mod comp_leagues_bra_reg_goias;
pub(crate) use self::comp_leagues_bra_reg_goias::*;
#[path = "motor/comp/leagues/bra_reg_minas_gerais.rs"]
pub(crate) mod comp_leagues_bra_reg_minas_gerais;
pub(crate) use self::comp_leagues_bra_reg_minas_gerais::*;
#[path = "motor/comp/leagues/bra_reg_north.rs"]
pub(crate) mod comp_leagues_bra_reg_north;
pub(crate) use self::comp_leagues_bra_reg_north::*;
#[path = "motor/comp/leagues/bra_reg_northeast.rs"]
pub(crate) mod comp_leagues_bra_reg_northeast;
pub(crate) use self::comp_leagues_bra_reg_northeast::*;
#[path = "motor/comp/leagues/bra_reg_parana.rs"]
pub(crate) mod comp_leagues_bra_reg_parana;
pub(crate) use self::comp_leagues_bra_reg_parana::*;
#[path = "motor/comp/leagues/bra_reg_pern.rs"]
pub(crate) mod comp_leagues_bra_reg_pern;
pub(crate) use self::comp_leagues_bra_reg_pern::*;
#[path = "motor/comp/leagues/bra_reg_rio.rs"]
pub(crate) mod comp_leagues_bra_reg_rio;
pub(crate) use self::comp_leagues_bra_reg_rio::*;
#[path = "motor/comp/leagues/bra_reg_santa.rs"]
pub(crate) mod comp_leagues_bra_reg_santa;
pub(crate) use self::comp_leagues_bra_reg_santa::*;
#[path = "motor/comp/leagues/bra_reg_sp.rs"]
pub(crate) mod comp_leagues_bra_reg_sp;
pub(crate) use self::comp_leagues_bra_reg_sp::*;
#[path = "motor/comp/leagues/eng_first.rs"]
pub(crate) mod comp_leagues_eng_first;
pub(crate) use self::comp_leagues_eng_first::*;
#[path = "motor/comp/leagues/eng_prm.rs"]
pub(crate) mod comp_leagues_eng_prm;
pub(crate) use self::comp_leagues_eng_prm::*;
#[path = "motor/comp/leagues/eng_second.rs"]
pub(crate) mod comp_leagues_eng_second;
pub(crate) use self::comp_leagues_eng_second::*;
#[path = "motor/comp/leagues/eng_third.rs"]
pub(crate) mod comp_leagues_eng_third;
pub(crate) use self::comp_leagues_eng_third::*;
#[path = "motor/comp/leagues/fin_prm.rs"]
pub(crate) mod comp_leagues_fin_prm;
pub(crate) use self::comp_leagues_fin_prm::*;
#[path = "motor/comp/leagues/fra_cfa.rs"]
pub(crate) mod comp_leagues_fra_cfa;
pub(crate) use self::comp_leagues_fra_cfa::*;
#[path = "motor/comp/leagues/fra_first.rs"]
pub(crate) mod comp_leagues_fra_first;
pub(crate) use self::comp_leagues_fra_first::*;
#[path = "motor/comp/leagues/fra_lower.rs"]
pub(crate) mod comp_leagues_fra_lower;
pub(crate) use self::comp_leagues_fra_lower::*;
#[path = "motor/comp/leagues/fra_second.rs"]
pub(crate) mod comp_leagues_fra_second;
pub(crate) use self::comp_leagues_fra_second::*;
#[path = "motor/comp/leagues/fra_third.rs"]
pub(crate) mod comp_leagues_fra_third;
pub(crate) use self::comp_leagues_fra_third::*;
#[path = "motor/comp/leagues/ita_ser_a.rs"]
pub(crate) mod comp_leagues_ita_ser_a;
pub(crate) use self::comp_leagues_ita_ser_a::*;
#[path = "motor/comp/leagues/ita_ser_b.rs"]
pub(crate) mod comp_leagues_ita_ser_b;
pub(crate) use self::comp_leagues_ita_ser_b::*;
#[path = "motor/comp/leagues/ita_ser_c1a.rs"]
pub(crate) mod comp_leagues_ita_ser_c1a;
pub(crate) use self::comp_leagues_ita_ser_c1a::*;
#[path = "motor/comp/leagues/ita_ser_c1b.rs"]
pub(crate) mod comp_leagues_ita_ser_c1b;
pub(crate) use self::comp_leagues_ita_ser_c1b::*;
#[path = "motor/comp/leagues/ita_ser_c2a.rs"]
pub(crate) mod comp_leagues_ita_ser_c2a;
pub(crate) use self::comp_leagues_ita_ser_c2a::*;
#[path = "motor/comp/leagues/jap_j1.rs"]
pub(crate) mod comp_leagues_jap_j1;
pub(crate) use self::comp_leagues_jap_j1::*;
#[path = "motor/comp/leagues/jap_j2.rs"]
pub(crate) mod comp_leagues_jap_j2;
pub(crate) use self::comp_leagues_jap_j2::*;
#[path = "motor/comp/leagues/spa_first.rs"]
pub(crate) mod comp_leagues_spa_first;
pub(crate) use self::comp_leagues_spa_first::*;
#[path = "motor/comp/leagues/spa_lower.rs"]
pub(crate) mod comp_leagues_spa_lower;
pub(crate) use self::comp_leagues_spa_lower::*;
#[path = "motor/comp/leagues/spa_second.rs"]
pub(crate) mod comp_leagues_spa_second;
pub(crate) use self::comp_leagues_spa_second::*;
#[path = "motor/comp/leagues/usa_mls.rs"]
pub(crate) mod comp_leagues_usa_mls;
pub(crate) use self::comp_leagues_usa_mls::*;
#[path = "motor/comp/squad_manager.rs"]
pub(crate) mod comp_squad_manager;
pub(crate) use self::comp_squad_manager::*;
#[path = "motor/comp/stadium.rs"]
pub(crate) mod comp_stadium;
pub(crate) use self::comp_stadium::*;
#[path = "motor/comp/uefa_seeding.rs"]
pub(crate) mod comp_uefa_seeding;
pub(crate) use self::comp_uefa_seeding::*;
#[path = "motor/comp/worldcup/wc_asia_league.rs"]
pub(crate) mod comp_worldcup_wc_asia_league;
pub(crate) use self::comp_worldcup_wc_asia_league::*;
#[path = "motor/comp/worldcup/wc_europe_league.rs"]
pub(crate) mod comp_worldcup_wc_europe_league;
pub(crate) use self::comp_worldcup_wc_europe_league::*;
#[path = "motor/comp/worldcup/wc_oceania_league.rs"]
pub(crate) mod comp_worldcup_wc_oceania_league;
pub(crate) use self::comp_worldcup_wc_oceania_league::*;
#[path = "motor/comp/worldcup/wc_south_american_league.rs"]
pub(crate) mod comp_worldcup_wc_south_american_league;
pub(crate) use self::comp_worldcup_wc_south_american_league::*;
#[path = "motor/comp/worldcup/world_cup.rs"]
pub(crate) mod comp_worldcup_world_cup;
pub(crate) use self::comp_worldcup_world_cup::*;
#[path = "motor/compare/area.rs"]
pub(crate) mod compare_area;
pub(crate) use self::compare_area::*;
#[path = "motor/compare/award/award_manager.rs"]
pub(crate) mod compare_award_award_manager;
pub(crate) use self::compare_award_award_manager::*;
#[path = "motor/compare/award/month_award.rs"]
pub(crate) mod compare_award_month_award;
pub(crate) use self::compare_award_month_award::*;
#[path = "motor/compare/award/month_ratings.rs"]
pub(crate) mod compare_award_month_ratings;
pub(crate) use self::compare_award_month_ratings::*;
#[path = "motor/compare/awol.rs"]
pub(crate) mod compare_awol;
pub(crate) use self::compare_awol::*;
#[path = "motor/compare/cash.rs"]
pub(crate) mod compare_cash;
pub(crate) use self::compare_cash::*;
#[path = "motor/compare/club_history.rs"]
pub(crate) mod compare_club_history;
pub(crate) use self::compare_club_history::*;
#[path = "motor/compare/club_records.rs"]
pub(crate) mod compare_club_records;
pub(crate) use self::compare_club_records::*;
#[path = "motor/compare/comp/comp.rs"]
pub(crate) mod compare_comp_comp;
pub(crate) use self::compare_comp_comp::*;
#[path = "motor/compare/comp/comp_util.rs"]
pub(crate) mod compare_comp_comp_util;
pub(crate) use self::compare_comp_comp_util::*;
#[path = "motor/compare/comp/conmebol_seeding.rs"]
pub(crate) mod compare_comp_conmebol_seeding;
pub(crate) use self::compare_comp_conmebol_seeding::*;
#[path = "motor/compare/comp/cup.rs"]
pub(crate) mod compare_comp_cup;
pub(crate) use self::compare_comp_cup::*;
#[path = "motor/compare/comp/cups/bra_champ_cup.rs"]
pub(crate) mod compare_comp_cups_bra_champ_cup;
pub(crate) use self::compare_comp_cups_bra_champ_cup::*;
#[path = "motor/compare/comp/cups/bra_cup.rs"]
pub(crate) mod compare_comp_cups_bra_cup;
pub(crate) use self::compare_comp_cups_bra_cup::*;
#[path = "motor/compare/comp/cups/cup_stage.rs"]
pub(crate) mod compare_comp_cups_cup_stage;
pub(crate) use self::compare_comp_cups_cup_stage::*;
#[path = "motor/compare/comp/cups/eng_auto_cup.rs"]
pub(crate) mod compare_comp_cups_eng_auto_cup;
pub(crate) use self::compare_comp_cups_eng_auto_cup::*;
#[path = "motor/compare/comp/cups/eng_cc_cup.rs"]
pub(crate) mod compare_comp_cups_eng_cc_cup;
pub(crate) use self::compare_comp_cups_eng_cc_cup::*;
#[path = "motor/compare/comp/cups/eng_charity.rs"]
pub(crate) mod compare_comp_cups_eng_charity;
pub(crate) use self::compare_comp_cups_eng_charity::*;
#[path = "motor/compare/comp/cups/eng_fa_cup.rs"]
pub(crate) mod compare_comp_cups_eng_fa_cup;
pub(crate) use self::compare_comp_cups_eng_fa_cup::*;
#[path = "motor/compare/comp/cups/fra_cup.rs"]
pub(crate) mod compare_comp_cups_fra_cup;
pub(crate) use self::compare_comp_cups_fra_cup::*;
#[path = "motor/compare/comp/cups/fra_lge_cup.rs"]
pub(crate) mod compare_comp_cups_fra_lge_cup;
pub(crate) use self::compare_comp_cups_fra_lge_cup::*;
#[path = "motor/compare/comp/cups/fra_super.rs"]
pub(crate) mod compare_comp_cups_fra_super;
pub(crate) use self::compare_comp_cups_fra_super::*;
#[path = "motor/compare/comp/cups/ita_c1_super.rs"]
pub(crate) mod compare_comp_cups_ita_c1_super;
pub(crate) use self::compare_comp_cups_ita_c1_super::*;
#[path = "motor/compare/comp/cups/ita_c_cup.rs"]
pub(crate) mod compare_comp_cups_ita_c_cup;
pub(crate) use self::compare_comp_cups_ita_c_cup::*;
#[path = "motor/compare/comp/cups/ita_cup.rs"]
pub(crate) mod compare_comp_cups_ita_cup;
pub(crate) use self::compare_comp_cups_ita_cup::*;
#[path = "motor/compare/comp/cups/ita_super.rs"]
pub(crate) mod compare_comp_cups_ita_super;
pub(crate) use self::compare_comp_cups_ita_super::*;
#[path = "motor/compare/comp/cups/jap_emp_cup.rs"]
pub(crate) mod compare_comp_cups_jap_emp_cup;
pub(crate) use self::compare_comp_cups_jap_emp_cup::*;
#[path = "motor/compare/comp/cups/jap_j_cup.rs"]
pub(crate) mod compare_comp_cups_jap_j_cup;
pub(crate) use self::compare_comp_cups_jap_j_cup::*;
#[path = "motor/compare/comp/cups/jap_super.rs"]
pub(crate) mod compare_comp_cups_jap_super;
pub(crate) use self::compare_comp_cups_jap_super::*;
#[path = "motor/compare/comp/cups/spa_cup.rs"]
pub(crate) mod compare_comp_cups_spa_cup;
pub(crate) use self::compare_comp_cups_spa_cup::*;
#[path = "motor/compare/comp/cups/spa_super.rs"]
pub(crate) mod compare_comp_cups_spa_super;
pub(crate) use self::compare_comp_cups_spa_super::*;
#[path = "motor/compare/comp/cups/usa_mls_all_stars.rs"]
pub(crate) mod compare_comp_cups_usa_mls_all_stars;
pub(crate) use self::compare_comp_cups_usa_mls_all_stars::*;
#[path = "motor/compare/comp/cups/usa_open_cup.rs"]
pub(crate) mod compare_comp_cups_usa_open_cup;
pub(crate) use self::compare_comp_cups_usa_open_cup::*;
#[path = "motor/compare/comp/eurocomp/asia_club_champ.rs"]
pub(crate) mod compare_comp_eurocomp_asia_club_champ;
pub(crate) use self::compare_comp_eurocomp_asia_club_champ::*;
#[path = "motor/compare/comp/eurocomp/asia_cup_winner.rs"]
pub(crate) mod compare_comp_eurocomp_asia_cup_winner;
pub(crate) use self::compare_comp_eurocomp_asia_cup_winner::*;
#[path = "motor/compare/comp/eurocomp/asia_super_cup.rs"]
pub(crate) mod compare_comp_eurocomp_asia_super_cup;
pub(crate) use self::compare_comp_eurocomp_asia_super_cup::*;
#[path = "motor/compare/comp/eurocomp/con_champ.rs"]
pub(crate) mod compare_comp_eurocomp_con_champ;
pub(crate) use self::compare_comp_eurocomp_con_champ::*;
#[path = "motor/compare/comp/eurocomp/con_merc_cup.rs"]
pub(crate) mod compare_comp_eurocomp_con_merc_cup;
pub(crate) use self::compare_comp_eurocomp_con_merc_cup::*;
#[path = "motor/compare/comp/eurocomp/conmebol_liber.rs"]
pub(crate) mod compare_comp_eurocomp_conmebol_liber;
pub(crate) use self::compare_comp_eurocomp_conmebol_liber::*;
#[path = "motor/compare/comp/eurocomp/conmebol_merc.rs"]
pub(crate) mod compare_comp_eurocomp_conmebol_merc;
pub(crate) use self::compare_comp_eurocomp_conmebol_merc::*;
#[path = "motor/compare/comp/eurocomp/eur_super_cup.rs"]
pub(crate) mod compare_comp_eurocomp_eur_super_cup;
pub(crate) use self::compare_comp_eurocomp_eur_super_cup::*;
#[path = "motor/compare/comp/eurocomp/european_cup.rs"]
pub(crate) mod compare_comp_eurocomp_european_cup;
pub(crate) use self::compare_comp_eurocomp_european_cup::*;
#[path = "motor/compare/comp/eurocomp/inter_amer_cup.rs"]
pub(crate) mod compare_comp_eurocomp_inter_amer_cup;
pub(crate) use self::compare_comp_eurocomp_inter_amer_cup::*;
#[path = "motor/compare/comp/eurocomp/intertoto_cup.rs"]
pub(crate) mod compare_comp_eurocomp_intertoto_cup;
pub(crate) use self::compare_comp_eurocomp_intertoto_cup::*;
#[path = "motor/compare/comp/eurocomp/oceania_club_champ.rs"]
pub(crate) mod compare_comp_eurocomp_oceania_club_champ;
pub(crate) use self::compare_comp_eurocomp_oceania_club_champ::*;
#[path = "motor/compare/comp/eurocomp/uefa_cup.rs"]
pub(crate) mod compare_comp_eurocomp_uefa_cup;
pub(crate) use self::compare_comp_eurocomp_uefa_cup::*;
#[path = "motor/compare/comp/eurocomp/world_club_cup.rs"]
pub(crate) mod compare_comp_eurocomp_world_club_cup;
pub(crate) use self::compare_comp_eurocomp_world_club_cup::*;
#[path = "motor/compare/comp/fifa_rankings.rs"]
pub(crate) mod compare_comp_fifa_rankings;
pub(crate) use self::compare_comp_fifa_rankings::*;
#[path = "motor/compare/comp/fix_man.rs"]
pub(crate) mod compare_comp_fix_man;
pub(crate) use self::compare_comp_fix_man::*;
#[path = "motor/compare/comp/friendly.rs"]
pub(crate) mod compare_comp_friendly;
pub(crate) use self::compare_comp_friendly::*;
#[path = "motor/compare/comp/host_country.rs"]
pub(crate) mod compare_comp_host_country;
pub(crate) use self::compare_comp_host_country::*;
#[path = "motor/compare/comp/intercomp/african_nations.rs"]
pub(crate) mod compare_comp_intercomp_african_nations;
pub(crate) use self::compare_comp_intercomp_african_nations::*;
#[path = "motor/compare/comp/intercomp/euro_champ_qual.rs"]
pub(crate) mod compare_comp_intercomp_euro_champ_qual;
pub(crate) use self::compare_comp_intercomp_euro_champ_qual::*;
#[path = "motor/compare/comp/intercomp/fifa_confed.rs"]
pub(crate) mod compare_comp_intercomp_fifa_confed;
pub(crate) use self::compare_comp_intercomp_fifa_confed::*;
#[path = "motor/compare/comp/intercomp/goldcup.rs"]
pub(crate) mod compare_comp_intercomp_goldcup;
pub(crate) use self::compare_comp_intercomp_goldcup::*;
#[path = "motor/compare/comp/league.rs"]
pub(crate) mod compare_comp_league;
pub(crate) use self::compare_comp_league::*;
#[path = "motor/compare/comp/leagues/arg_second.rs"]
pub(crate) mod compare_comp_leagues_arg_second;
pub(crate) use self::compare_comp_leagues_arg_second::*;
#[path = "motor/compare/comp/leagues/aus_nsl.rs"]
pub(crate) mod compare_comp_leagues_aus_nsl;
pub(crate) use self::compare_comp_leagues_aus_nsl::*;
#[path = "motor/compare/comp/leagues/bra_nat_first.rs"]
pub(crate) mod compare_comp_leagues_bra_nat_first;
pub(crate) use self::compare_comp_leagues_bra_nat_first::*;
#[path = "motor/compare/comp/leagues/bra_nat_second.rs"]
pub(crate) mod compare_comp_leagues_bra_nat_second;
pub(crate) use self::compare_comp_leagues_bra_nat_second::*;
#[path = "motor/compare/comp/leagues/bra_nat_third.rs"]
pub(crate) mod compare_comp_leagues_bra_nat_third;
pub(crate) use self::compare_comp_leagues_bra_nat_third::*;
#[path = "motor/compare/comp/leagues/bra_reg_bahia.rs"]
pub(crate) mod compare_comp_leagues_bra_reg_bahia;
pub(crate) use self::compare_comp_leagues_bra_reg_bahia::*;
#[path = "motor/compare/comp/leagues/bra_reg_central.rs"]
pub(crate) mod compare_comp_leagues_bra_reg_central;
pub(crate) use self::compare_comp_leagues_bra_reg_central::*;
#[path = "motor/compare/comp/leagues/bra_reg_gaucho.rs"]
pub(crate) mod compare_comp_leagues_bra_reg_gaucho;
pub(crate) use self::compare_comp_leagues_bra_reg_gaucho::*;
#[path = "motor/compare/comp/leagues/bra_reg_goias.rs"]
pub(crate) mod compare_comp_leagues_bra_reg_goias;
pub(crate) use self::compare_comp_leagues_bra_reg_goias::*;
#[path = "motor/compare/comp/leagues/bra_reg_minas_gerais.rs"]
pub(crate) mod compare_comp_leagues_bra_reg_minas_gerais;
pub(crate) use self::compare_comp_leagues_bra_reg_minas_gerais::*;
#[path = "motor/compare/comp/leagues/bra_reg_north.rs"]
pub(crate) mod compare_comp_leagues_bra_reg_north;
pub(crate) use self::compare_comp_leagues_bra_reg_north::*;
#[path = "motor/compare/comp/leagues/bra_reg_northeast.rs"]
pub(crate) mod compare_comp_leagues_bra_reg_northeast;
pub(crate) use self::compare_comp_leagues_bra_reg_northeast::*;
#[path = "motor/compare/comp/leagues/bra_reg_parana.rs"]
pub(crate) mod compare_comp_leagues_bra_reg_parana;
pub(crate) use self::compare_comp_leagues_bra_reg_parana::*;
#[path = "motor/compare/comp/leagues/bra_reg_pern.rs"]
pub(crate) mod compare_comp_leagues_bra_reg_pern;
pub(crate) use self::compare_comp_leagues_bra_reg_pern::*;
#[path = "motor/compare/comp/leagues/bra_reg_rio.rs"]
pub(crate) mod compare_comp_leagues_bra_reg_rio;
pub(crate) use self::compare_comp_leagues_bra_reg_rio::*;
#[path = "motor/compare/comp/leagues/bra_reg_santa.rs"]
pub(crate) mod compare_comp_leagues_bra_reg_santa;
pub(crate) use self::compare_comp_leagues_bra_reg_santa::*;
#[path = "motor/compare/comp/leagues/bra_reg_sp.rs"]
pub(crate) mod compare_comp_leagues_bra_reg_sp;
pub(crate) use self::compare_comp_leagues_bra_reg_sp::*;
#[path = "motor/compare/comp/leagues/eng_first.rs"]
pub(crate) mod compare_comp_leagues_eng_first;
pub(crate) use self::compare_comp_leagues_eng_first::*;
#[path = "motor/compare/comp/leagues/eng_prm.rs"]
pub(crate) mod compare_comp_leagues_eng_prm;
pub(crate) use self::compare_comp_leagues_eng_prm::*;
#[path = "motor/compare/comp/leagues/eng_second.rs"]
pub(crate) mod compare_comp_leagues_eng_second;
pub(crate) use self::compare_comp_leagues_eng_second::*;
#[path = "motor/compare/comp/leagues/eng_third.rs"]
pub(crate) mod compare_comp_leagues_eng_third;
pub(crate) use self::compare_comp_leagues_eng_third::*;
#[path = "motor/compare/comp/leagues/fin_prm.rs"]
pub(crate) mod compare_comp_leagues_fin_prm;
pub(crate) use self::compare_comp_leagues_fin_prm::*;
#[path = "motor/compare/comp/leagues/fra_cfa.rs"]
pub(crate) mod compare_comp_leagues_fra_cfa;
pub(crate) use self::compare_comp_leagues_fra_cfa::*;
#[path = "motor/compare/comp/leagues/fra_first.rs"]
pub(crate) mod compare_comp_leagues_fra_first;
pub(crate) use self::compare_comp_leagues_fra_first::*;
#[path = "motor/compare/comp/leagues/fra_lower.rs"]
pub(crate) mod compare_comp_leagues_fra_lower;
pub(crate) use self::compare_comp_leagues_fra_lower::*;
#[path = "motor/compare/comp/leagues/fra_second.rs"]
pub(crate) mod compare_comp_leagues_fra_second;
pub(crate) use self::compare_comp_leagues_fra_second::*;
#[path = "motor/compare/comp/leagues/fra_third.rs"]
pub(crate) mod compare_comp_leagues_fra_third;
pub(crate) use self::compare_comp_leagues_fra_third::*;
#[path = "motor/compare/comp/leagues/ita_ser_a.rs"]
pub(crate) mod compare_comp_leagues_ita_ser_a;
pub(crate) use self::compare_comp_leagues_ita_ser_a::*;
#[path = "motor/compare/comp/leagues/ita_ser_b.rs"]
pub(crate) mod compare_comp_leagues_ita_ser_b;
pub(crate) use self::compare_comp_leagues_ita_ser_b::*;
#[path = "motor/compare/comp/leagues/ita_ser_c1a.rs"]
pub(crate) mod compare_comp_leagues_ita_ser_c1a;
pub(crate) use self::compare_comp_leagues_ita_ser_c1a::*;
#[path = "motor/compare/comp/leagues/ita_ser_c1b.rs"]
pub(crate) mod compare_comp_leagues_ita_ser_c1b;
pub(crate) use self::compare_comp_leagues_ita_ser_c1b::*;
#[path = "motor/compare/comp/leagues/ita_ser_c2a.rs"]
pub(crate) mod compare_comp_leagues_ita_ser_c2a;
pub(crate) use self::compare_comp_leagues_ita_ser_c2a::*;
#[path = "motor/compare/comp/leagues/jap_j1.rs"]
pub(crate) mod compare_comp_leagues_jap_j1;
pub(crate) use self::compare_comp_leagues_jap_j1::*;
#[path = "motor/compare/comp/leagues/jap_j2.rs"]
pub(crate) mod compare_comp_leagues_jap_j2;
pub(crate) use self::compare_comp_leagues_jap_j2::*;
#[path = "motor/compare/comp/leagues/spa_first.rs"]
pub(crate) mod compare_comp_leagues_spa_first;
pub(crate) use self::compare_comp_leagues_spa_first::*;
#[path = "motor/compare/comp/leagues/spa_lower.rs"]
pub(crate) mod compare_comp_leagues_spa_lower;
pub(crate) use self::compare_comp_leagues_spa_lower::*;
#[path = "motor/compare/comp/leagues/spa_second.rs"]
pub(crate) mod compare_comp_leagues_spa_second;
pub(crate) use self::compare_comp_leagues_spa_second::*;
#[path = "motor/compare/comp/leagues/usa_mls.rs"]
pub(crate) mod compare_comp_leagues_usa_mls;
pub(crate) use self::compare_comp_leagues_usa_mls::*;
#[path = "motor/compare/comp/squad_manager.rs"]
pub(crate) mod compare_comp_squad_manager;
pub(crate) use self::compare_comp_squad_manager::*;
#[path = "motor/compare/comp/stadium.rs"]
pub(crate) mod compare_comp_stadium;
pub(crate) use self::compare_comp_stadium::*;
#[path = "motor/compare/comp/uefa_seeding.rs"]
pub(crate) mod compare_comp_uefa_seeding;
pub(crate) use self::compare_comp_uefa_seeding::*;
#[path = "motor/compare/comp/worldcup/wc_asia_league.rs"]
pub(crate) mod compare_comp_worldcup_wc_asia_league;
pub(crate) use self::compare_comp_worldcup_wc_asia_league::*;
#[path = "motor/compare/comp/worldcup/wc_europe_league.rs"]
pub(crate) mod compare_comp_worldcup_wc_europe_league;
pub(crate) use self::compare_comp_worldcup_wc_europe_league::*;
#[path = "motor/compare/comp/worldcup/wc_oceania_league.rs"]
pub(crate) mod compare_comp_worldcup_wc_oceania_league;
pub(crate) use self::compare_comp_worldcup_wc_oceania_league::*;
#[path = "motor/compare/comp/worldcup/wc_south_american_league.rs"]
pub(crate) mod compare_comp_worldcup_wc_south_american_league;
pub(crate) use self::compare_comp_worldcup_wc_south_american_league::*;
#[path = "motor/compare/comp/worldcup/world_cup.rs"]
pub(crate) mod compare_comp_worldcup_world_cup;
pub(crate) use self::compare_comp_worldcup_world_cup::*;
#[path = "motor/compare/contract_manager.rs"]
pub(crate) mod compare_contract_manager;
pub(crate) use self::compare_contract_manager::*;
#[path = "motor/compare/database.rs"]
pub(crate) mod compare_database;
pub(crate) use self::compare_database::*;
#[path = "motor/compare/db_files.rs"]
pub(crate) mod compare_db_files;
pub(crate) use self::compare_db_files::*;
#[path = "motor/compare/discipline.rs"]
pub(crate) mod compare_discipline;
pub(crate) use self::compare_discipline::*;
#[path = "motor/compare/dispute.rs"]
pub(crate) mod compare_dispute;
pub(crate) use self::compare_dispute::*;
#[path = "motor/compare/file_screens.rs"]
pub(crate) mod compare_file_screens;
pub(crate) use self::compare_file_screens::*;
#[path = "motor/compare/fine.rs"]
pub(crate) mod compare_fine;
pub(crate) use self::compare_fine::*;
#[path = "motor/compare/fog_of_war.rs"]
pub(crate) mod compare_fog_of_war;
pub(crate) use self::compare_fog_of_war::*;
#[path = "motor/compare/formation.rs"]
pub(crate) mod compare_formation;
pub(crate) use self::compare_formation::*;
#[path = "motor/compare/game.rs"]
pub(crate) mod compare_game;
pub(crate) use self::compare_game::*;
#[path = "motor/compare/human_manager.rs"]
pub(crate) mod compare_human_manager;
pub(crate) use self::compare_human_manager::*;
#[path = "motor/compare/injury.rs"]
pub(crate) mod compare_injury;
pub(crate) use self::compare_injury::*;
#[path = "motor/compare/key_nation.rs"]
pub(crate) mod compare_key_nation;
pub(crate) use self::compare_key_nation::*;
#[path = "motor/compare/manager_manager.rs"]
pub(crate) mod compare_manager_manager;
pub(crate) use self::compare_manager_manager::*;
#[path = "motor/compare/match_day.rs"]
pub(crate) mod compare_match_day;
pub(crate) use self::compare_match_day::*;
#[path = "motor/compare/match_eng.rs"]
pub(crate) mod compare_match_eng;
pub(crate) use self::compare_match_eng::*;
#[path = "motor/compare/match_events.rs"]
pub(crate) mod compare_match_events;
pub(crate) use self::compare_match_events::*;
#[path = "motor/compare/match_man.rs"]
pub(crate) mod compare_match_man;
pub(crate) use self::compare_match_man::*;
#[path = "motor/compare/match_official.rs"]
pub(crate) mod compare_match_official;
pub(crate) use self::compare_match_official::*;
#[path = "motor/compare/match_pl.rs"]
pub(crate) mod compare_match_pl;
pub(crate) use self::compare_match_pl::*;
#[path = "motor/compare/match_screens.rs"]
pub(crate) mod compare_match_screens;
pub(crate) use self::compare_match_screens::*;
#[path = "motor/compare/media.rs"]
pub(crate) mod compare_media;
pub(crate) use self::compare_media::*;
#[path = "motor/compare/national_teams.rs"]
pub(crate) mod compare_national_teams;
pub(crate) use self::compare_national_teams::*;
#[path = "motor/compare/news.rs"]
pub(crate) mod compare_news;
pub(crate) use self::compare_news::*;
#[path = "motor/compare/officials_manager.rs"]
pub(crate) mod compare_officials_manager;
pub(crate) use self::compare_officials_manager::*;
#[path = "motor/compare/player_regen.rs"]
pub(crate) mod compare_player_regen;
pub(crate) use self::compare_player_regen::*;
#[path = "motor/compare/player_search.rs"]
pub(crate) mod compare_player_search;
pub(crate) use self::compare_player_search::*;
#[path = "motor/compare/player_stats.rs"]
pub(crate) mod compare_player_stats;
pub(crate) use self::compare_player_stats::*;
#[path = "motor/compare/printouts.rs"]
pub(crate) mod compare_printouts;
pub(crate) use self::compare_printouts::*;
#[path = "motor/compare/record_utils.rs"]
pub(crate) mod compare_record_utils;
pub(crate) use self::compare_record_utils::*;
#[path = "motor/compare/ruling_body/ruling_body.rs"]
pub(crate) mod compare_ruling_body_ruling_body;
pub(crate) use self::compare_ruling_body_ruling_body::*;
#[path = "motor/compare/scout_manager.rs"]
pub(crate) mod compare_scout_manager;
pub(crate) use self::compare_scout_manager::*;
#[path = "motor/compare/scrman.rs"]
pub(crate) mod compare_scrman;
pub(crate) use self::compare_scrman::*;
#[path = "motor/compare/setup.rs"]
pub(crate) mod compare_setup;
pub(crate) use self::compare_setup::*;
#[path = "motor/compare/shared.rs"]
pub(crate) mod compare_shared;
pub(crate) use self::compare_shared::*;
#[path = "motor/compare/shortlist_manager.rs"]
pub(crate) mod compare_shortlist_manager;
pub(crate) use self::compare_shortlist_manager::*;
#[path = "motor/compare/simulated_stats.rs"]
pub(crate) mod compare_simulated_stats;
pub(crate) use self::compare_simulated_stats::*;
#[path = "motor/compare/staff_contracts.rs"]
pub(crate) mod compare_staff_contracts;
pub(crate) use self::compare_staff_contracts::*;
#[path = "motor/compare/staff_records.rs"]
pub(crate) mod compare_staff_records;
pub(crate) use self::compare_staff_records::*;
#[path = "motor/compare/tactics.rs"]
pub(crate) mod compare_tactics;
pub(crate) use self::compare_tactics::*;
#[path = "motor/compare/training_manager.rs"]
pub(crate) mod compare_training_manager;
pub(crate) use self::compare_training_manager::*;
#[path = "motor/compare/transfer/argentina_rules.rs"]
pub(crate) mod compare_transfer_argentina_rules;
pub(crate) use self::compare_transfer_argentina_rules::*;
#[path = "motor/compare/transfer/australia_rules.rs"]
pub(crate) mod compare_transfer_australia_rules;
pub(crate) use self::compare_transfer_australia_rules::*;
#[path = "motor/compare/transfer/belgium_rules.rs"]
pub(crate) mod compare_transfer_belgium_rules;
pub(crate) use self::compare_transfer_belgium_rules::*;
#[path = "motor/compare/transfer/brazil_rules.rs"]
pub(crate) mod compare_transfer_brazil_rules;
pub(crate) use self::compare_transfer_brazil_rules::*;
#[path = "motor/compare/transfer/croatia_rules.rs"]
pub(crate) mod compare_transfer_croatia_rules;
pub(crate) use self::compare_transfer_croatia_rules::*;
#[path = "motor/compare/transfer/england_rules.rs"]
pub(crate) mod compare_transfer_england_rules;
pub(crate) use self::compare_transfer_england_rules::*;
#[path = "motor/compare/transfer/finland_rules.rs"]
pub(crate) mod compare_transfer_finland_rules;
pub(crate) use self::compare_transfer_finland_rules::*;
#[path = "motor/compare/transfer/france_rules.rs"]
pub(crate) mod compare_transfer_france_rules;
pub(crate) use self::compare_transfer_france_rules::*;
#[path = "motor/compare/transfer/germany_rules.rs"]
pub(crate) mod compare_transfer_germany_rules;
pub(crate) use self::compare_transfer_germany_rules::*;
#[path = "motor/compare/transfer/greece_rules.rs"]
pub(crate) mod compare_transfer_greece_rules;
pub(crate) use self::compare_transfer_greece_rules::*;
#[path = "motor/compare/transfer/holland_rules.rs"]
pub(crate) mod compare_transfer_holland_rules;
pub(crate) use self::compare_transfer_holland_rules::*;
#[path = "motor/compare/transfer/ireland_rules.rs"]
pub(crate) mod compare_transfer_ireland_rules;
pub(crate) use self::compare_transfer_ireland_rules::*;
#[path = "motor/compare/transfer/italy_rules.rs"]
pub(crate) mod compare_transfer_italy_rules;
pub(crate) use self::compare_transfer_italy_rules::*;
#[path = "motor/compare/transfer/korea_rules.rs"]
pub(crate) mod compare_transfer_korea_rules;
pub(crate) use self::compare_transfer_korea_rules::*;
#[path = "motor/compare/transfer/northern_ireland_rules.rs"]
pub(crate) mod compare_transfer_northern_ireland_rules;
pub(crate) use self::compare_transfer_northern_ireland_rules::*;
#[path = "motor/compare/transfer/norway_rules.rs"]
pub(crate) mod compare_transfer_norway_rules;
pub(crate) use self::compare_transfer_norway_rules::*;
#[path = "motor/compare/transfer/poland_rules.rs"]
pub(crate) mod compare_transfer_poland_rules;
pub(crate) use self::compare_transfer_poland_rules::*;
#[path = "motor/compare/transfer/portugal_rules.rs"]
pub(crate) mod compare_transfer_portugal_rules;
pub(crate) use self::compare_transfer_portugal_rules::*;
#[path = "motor/compare/transfer/russia_rules.rs"]
pub(crate) mod compare_transfer_russia_rules;
pub(crate) use self::compare_transfer_russia_rules::*;
#[path = "motor/compare/transfer/scotland_rules.rs"]
pub(crate) mod compare_transfer_scotland_rules;
pub(crate) use self::compare_transfer_scotland_rules::*;
#[path = "motor/compare/transfer/spain_rules.rs"]
pub(crate) mod compare_transfer_spain_rules;
pub(crate) use self::compare_transfer_spain_rules::*;
#[path = "motor/compare/transfer/turkey_rules.rs"]
pub(crate) mod compare_transfer_turkey_rules;
pub(crate) use self::compare_transfer_turkey_rules::*;
#[path = "motor/compare/transfer/usa_rules.rs"]
pub(crate) mod compare_transfer_usa_rules;
pub(crate) use self::compare_transfer_usa_rules::*;
#[path = "motor/compare/transfer/wales_rules.rs"]
pub(crate) mod compare_transfer_wales_rules;
pub(crate) use self::compare_transfer_wales_rules::*;
#[path = "motor/compare/transfer_manager.rs"]
pub(crate) mod compare_transfer_manager;
pub(crate) use self::compare_transfer_manager::*;
#[path = "motor/compare/transfer_offer.rs"]
pub(crate) mod compare_transfer_offer;
pub(crate) use self::compare_transfer_offer::*;
#[path = "motor/compare/utils.rs"]
pub(crate) mod compare_utils;
pub(crate) use self::compare_utils::*;
#[path = "motor/compare/weather.rs"]
pub(crate) mod compare_weather;
pub(crate) use self::compare_weather::*;
#[path = "motor/compare/zipdir.rs"]
pub(crate) mod compare_zipdir;
pub(crate) use self::compare_zipdir::*;
#[path = "motor/contract_manager.rs"]
pub(crate) mod contract_manager;
pub(crate) use self::contract_manager::*;
#[path = "motor/database.rs"]
pub(crate) mod database;
pub(crate) use self::database::*;
#[path = "motor/db_files.rs"]
pub(crate) mod db_files;
pub(crate) use self::db_files::*;
#[path = "motor/discipline.rs"]
pub(crate) mod discipline;
pub(crate) use self::discipline::*;
#[path = "motor/dispute.rs"]
pub(crate) mod dispute;
pub(crate) use self::dispute::*;
#[path = "motor/events.rs"]
pub(crate) mod events;
pub(crate) use self::events::*;
#[path = "motor/exe_calls.rs"]
pub(crate) mod exe_calls;
pub(crate) use self::exe_calls::*;
#[path = "motor/exe_fns.rs"]
pub(crate) mod exe_fns;
pub(crate) use self::exe_fns::*;
#[path = "motor/fields.rs"]
pub(crate) mod fields;
pub(crate) use self::fields::*;
#[path = "motor/file_screens.rs"]
pub(crate) mod file_screens;
pub(crate) use self::file_screens::*;
#[path = "motor/fine.rs"]
pub(crate) mod fine;
pub(crate) use self::fine::*;
#[path = "motor/fog_of_war.rs"]
pub(crate) mod fog_of_war;
pub(crate) use self::fog_of_war::*;
#[path = "motor/formation.rs"]
pub(crate) mod formation;
pub(crate) use self::formation::*;
#[path = "motor/game.rs"]
pub(crate) mod game;
pub(crate) use self::game::*;
#[path = "motor/globals.rs"]
pub(crate) mod globals;
pub(crate) use self::globals::*;
#[path = "motor/human_manager.rs"]
pub(crate) mod human_manager;
pub(crate) use self::human_manager::*;
#[path = "motor/index.rs"]
pub(crate) mod index;
pub(crate) use self::index::*;
#[path = "motor/injury.rs"]
pub(crate) mod injury;
pub(crate) use self::injury::*;
#[path = "motor/key_nation.rs"]
pub(crate) mod key_nation;
pub(crate) use self::key_nation::*;
#[path = "motor/keys.rs"]
pub(crate) mod keys;
pub(crate) use self::keys::*;
#[path = "motor/main.rs"]
pub(crate) mod main;
pub(crate) use self::main::*;
#[path = "motor/manager_manager.rs"]
pub(crate) mod manager_manager;
pub(crate) use self::manager_manager::*;
#[path = "motor/match_codes.rs"]
pub(crate) mod match_codes;
pub(crate) use self::match_codes::*;
#[path = "motor/bits.rs"]
pub(crate) mod bits;
pub(crate) use self::bits::*;
#[path = "motor/enums.rs"]
pub(crate) mod enums;
pub(crate) use self::enums::*;
#[path = "motor/vtables.rs"]
pub(crate) mod vtables;
pub(crate) use self::vtables::*;
#[path = "motor/match_day.rs"]
pub(crate) mod match_day;
pub(crate) use self::match_day::*;
#[path = "motor/match_eng.rs"]
pub(crate) mod match_eng;
pub(crate) use self::match_eng::*;
#[path = "motor/match_events.rs"]
pub(crate) mod match_events;
pub(crate) use self::match_events::*;
#[path = "motor/match_man.rs"]
pub(crate) mod match_man;
pub(crate) use self::match_man::*;
#[path = "motor/match_official.rs"]
pub(crate) mod match_official;
pub(crate) use self::match_official::*;
#[path = "motor/match_pl.rs"]
pub(crate) mod match_pl;
pub(crate) use self::match_pl::*;
#[path = "motor/match_screens.rs"]
pub(crate) mod match_screens;
pub(crate) use self::match_screens::*;
#[path = "motor/media.rs"]
pub(crate) mod media;
pub(crate) use self::media::*;
#[path = "motor/mem.rs"]
pub(crate) mod mem;
pub(crate) use self::mem::*;
#[path = "motor/national_teams.rs"]
pub(crate) mod national_teams;
pub(crate) use self::national_teams::*;
#[path = "motor/news.rs"]
pub(crate) mod news;
pub(crate) use self::news::*;
#[path = "motor/officials_manager.rs"]
pub(crate) mod officials_manager;
pub(crate) use self::officials_manager::*;
#[path = "motor/player_regen.rs"]
pub(crate) mod player_regen;
pub(crate) use self::player_regen::*;
#[path = "motor/player_search.rs"]
pub(crate) mod player_search;
pub(crate) use self::player_search::*;
#[path = "motor/player_stats.rs"]
pub(crate) mod player_stats;
pub(crate) use self::player_stats::*;
#[path = "motor/printouts.rs"]
pub(crate) mod printouts;
pub(crate) use self::printouts::*;
#[path = "motor/rdata_consts.rs"]
pub(crate) mod rdata_consts;
pub(crate) use self::rdata_consts::*;
#[path = "motor/record_utils.rs"]
pub(crate) mod record_utils;
pub(crate) use self::record_utils::*;
#[path = "motor/records.rs"]
pub(crate) mod records;
pub(crate) use self::records::*;
#[path = "motor/ruling_body/ruling_body.rs"]
pub(crate) mod ruling_body_ruling_body;
pub(crate) use self::ruling_body_ruling_body::*;
#[path = "motor/scout_manager.rs"]
pub(crate) mod scout_manager;
pub(crate) use self::scout_manager::*;
#[path = "motor/scrman.rs"]
pub(crate) mod scrman;
pub(crate) use self::scrman::*;
#[path = "motor/setup.rs"]
pub(crate) mod setup;
pub(crate) use self::setup::*;
#[path = "motor/shared.rs"]
pub(crate) mod shared;
pub(crate) use self::shared::*;
#[path = "motor/shortlist_manager.rs"]
pub(crate) mod shortlist_manager;
pub(crate) use self::shortlist_manager::*;
#[path = "motor/simulated_stats.rs"]
pub(crate) mod simulated_stats;
pub(crate) use self::simulated_stats::*;
#[path = "motor/staff_contracts.rs"]
pub(crate) mod staff_contracts;
pub(crate) use self::staff_contracts::*;
#[path = "motor/staff_records.rs"]
pub(crate) mod staff_records;
pub(crate) use self::staff_records::*;
#[path = "motor/tactics.rs"]
pub(crate) mod tactics;
pub(crate) use self::tactics::*;
#[path = "motor/training_manager.rs"]
pub(crate) mod training_manager;
pub(crate) use self::training_manager::*;
#[path = "motor/transfer/argentina_rules.rs"]
pub(crate) mod transfer_argentina_rules;
pub(crate) use self::transfer_argentina_rules::*;
#[path = "motor/transfer/australia_rules.rs"]
pub(crate) mod transfer_australia_rules;
pub(crate) use self::transfer_australia_rules::*;
#[path = "motor/transfer/belgium_rules.rs"]
pub(crate) mod transfer_belgium_rules;
pub(crate) use self::transfer_belgium_rules::*;
#[path = "motor/transfer/brazil_rules.rs"]
pub(crate) mod transfer_brazil_rules;
pub(crate) use self::transfer_brazil_rules::*;
#[path = "motor/transfer/croatia_rules.rs"]
pub(crate) mod transfer_croatia_rules;
pub(crate) use self::transfer_croatia_rules::*;
#[path = "motor/transfer/england_rules.rs"]
pub(crate) mod transfer_england_rules;
pub(crate) use self::transfer_england_rules::*;
#[path = "motor/transfer/finland_rules.rs"]
pub(crate) mod transfer_finland_rules;
pub(crate) use self::transfer_finland_rules::*;
#[path = "motor/transfer/france_rules.rs"]
pub(crate) mod transfer_france_rules;
pub(crate) use self::transfer_france_rules::*;
#[path = "motor/transfer/germany_rules.rs"]
pub(crate) mod transfer_germany_rules;
pub(crate) use self::transfer_germany_rules::*;
#[path = "motor/transfer/greece_rules.rs"]
pub(crate) mod transfer_greece_rules;
pub(crate) use self::transfer_greece_rules::*;
#[path = "motor/transfer/holland_rules.rs"]
pub(crate) mod transfer_holland_rules;
pub(crate) use self::transfer_holland_rules::*;
#[path = "motor/transfer/ireland_rules.rs"]
pub(crate) mod transfer_ireland_rules;
pub(crate) use self::transfer_ireland_rules::*;
#[path = "motor/transfer/italy_rules.rs"]
pub(crate) mod transfer_italy_rules;
pub(crate) use self::transfer_italy_rules::*;
#[path = "motor/transfer/korea_rules.rs"]
pub(crate) mod transfer_korea_rules;
pub(crate) use self::transfer_korea_rules::*;
#[path = "motor/transfer/northern_ireland_rules.rs"]
pub(crate) mod transfer_northern_ireland_rules;
pub(crate) use self::transfer_northern_ireland_rules::*;
#[path = "motor/transfer/norway_rules.rs"]
pub(crate) mod transfer_norway_rules;
pub(crate) use self::transfer_norway_rules::*;
#[path = "motor/transfer/poland_rules.rs"]
pub(crate) mod transfer_poland_rules;
pub(crate) use self::transfer_poland_rules::*;
#[path = "motor/transfer/portugal_rules.rs"]
pub(crate) mod transfer_portugal_rules;
pub(crate) use self::transfer_portugal_rules::*;
#[path = "motor/transfer/russia_rules.rs"]
pub(crate) mod transfer_russia_rules;
pub(crate) use self::transfer_russia_rules::*;
#[path = "motor/transfer/scotland_rules.rs"]
pub(crate) mod transfer_scotland_rules;
pub(crate) use self::transfer_scotland_rules::*;
#[path = "motor/transfer/spain_rules.rs"]
pub(crate) mod transfer_spain_rules;
pub(crate) use self::transfer_spain_rules::*;
#[path = "motor/transfer/turkey_rules.rs"]
pub(crate) mod transfer_turkey_rules;
pub(crate) use self::transfer_turkey_rules::*;
#[path = "motor/transfer/usa_rules.rs"]
pub(crate) mod transfer_usa_rules;
pub(crate) use self::transfer_usa_rules::*;
#[path = "motor/transfer/wales_rules.rs"]
pub(crate) mod transfer_wales_rules;
pub(crate) use self::transfer_wales_rules::*;
#[path = "motor/transfer_manager.rs"]
pub(crate) mod transfer_manager;
pub(crate) use self::transfer_manager::*;
#[path = "motor/transfer_offer.rs"]
pub(crate) mod transfer_offer;
pub(crate) use self::transfer_offer::*;
#[path = "motor/utils.rs"]
pub(crate) mod utils;
pub(crate) use self::utils::*;
#[path = "motor/weather.rs"]
pub(crate) mod weather;
pub(crate) use self::weather::*;
#[path = "motor/zipdir.rs"]
pub(crate) mod zipdir;
pub(crate) use self::zipdir::*;

// ---------- 161st batch: watch list, position code and staff criterion ----------

compare_value!(compare_staff_criterion, staff_criterion, IDX_STAFF_CRITERION, () -> u32);

// ---------- 162nd batch: the slots of the tactics table (tactics.cpp) ----------

/// Row `linha` of the tactics table of object `t` ([t+0x601], 0x91 bytes per row: the formation
/// word and 20 slots of 4 bytes from +2).
unsafe fn table_row(t: usize, linha: i32) -> usize {
    read_usize(t + tactics_fields::ROWS).wrapping_add((linha as usize).wrapping_mul(0x91))
}

/// The regions the three can touch: the club's row in the table (if any) and the pending
/// instructions with the counter (+0x30..+0x5f1).
unsafe fn table_regions(t: usize, clube: usize, humano: u32) -> Vec<(usize, usize)> {
    let mut r = vec![(t + tactics_fields::UNDO_SLOTS, 0x5c1)];
    if clube != 0 {
        let tactics_row: unsafe extern "stdcall" fn(usize, u32) -> i32 = core::mem::transmute(FN_TACTICS_GET_CLUB_TACTICS_INDEX);
        let l = tactics_row(clube, humano);
        if l != -1 {
            r.push((table_row(t, l), 0x91));
        }
    }
    r
}

unsafe extern "stdcall" fn compare_set_piece_code(c: i16) -> u8 {
    let r = original_fn::<unsafe extern "stdcall" fn(i16) -> u8>(IDX_SET_PIECE_CODE)(c);
    let n = set_piece_code(c);
    SCOREBOARD.record(r == n, || format!("set_piece_code({}) original={} rust={}", c, r, n));
    r
}
compare_split!(compare_add_disposition, add_disposition, IDX_ADD_DISPOSITION);

compare_split!(compare_split_d, split_d, IDX_SPLIT_D);
compare_split!(compare_split_e, split_e, IDX_SPLIT_E);
compare_split!(compare_split_f, split_f, IDX_SPLIT_F);
unsafe extern "thiscall" fn compare_remove_from_list(this: usize, alvo: usize) -> u8 {
    let this = MatchPlayer(this);
    let f = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u8>(IDX_REMOVE_FROM_LIST);
    let n = this.marker_count().max(0) as usize;
    let reg = [(this.addr() + match_player::MARKER_COUNT, 1), (this.addr() + match_player::MARKERS, 4 * n)];
    let (r, x, igual, mo, mr) = crate::comparison::side_by_side(&reg, || f(this.addr(), alvo), || remove_from_list(this.addr(), alvo));
    SCOREBOARD.record(igual && r == x, || format!("remove_from_list(n {}) original={} {:x?} rust={} {:x?}", n, r, mo, x, mr));
    r
}
unsafe extern "thiscall" fn compare_take_the_field(this: usize, j: usize) -> u8 {
    let j = MatchPlayer(j);
    let f = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u8>(IDX_TAKE_THE_FIELD);
    let l = side_of_player(j) as isize;
    let n = *((this as isize + 0x4592 + l) as *const i8) as isize;
    let reg = [(j.addr() + match_player::CELL_SLOT, 1), (j.addr() + match_player::CELL_X, 2), ((this as isize + 0x4592 + l) as usize, 1), ((this as isize + 0x453a + 4 * (n + 11 * l)) as usize, 4)];
    let (r, x, igual, mo, mr) = crate::comparison::side_by_side(&reg, || f(this, j.addr()), || take_the_field(this, j.addr()));
    SCOREBOARD.record(igual && r == x, || format!("take_the_field(side {}, n {}) original={} {:x?} rust={} {:x?}", l, n, r, mo, x, mr));
    r
}
compare_value!(compare_in_danger, in_danger, IDX_IN_DANGER, () -> u8);

compare_value!(compare_clear_path, clear_path_to, IDX_CLEAR_PATH_TO, (l: i8, x: i8, y: i8) -> u8);
compare_value!(compare_reaches_the_point, reaches_the_point, IDX_REACHES_THE_POINT, (x: i8, y: i8) -> u8);

unsafe extern "thiscall" fn compare_choose_standout(this: usize) {
    let this = Engine(this);
    let f = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_CHOOSE_STANDOUT);
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&[(this.addr() + engine::ADJUSTED_PLAYER_ID, 4)], || f(this.addr()), || choose_standout(this.addr()));
    SCOREBOARD.record(igual, || format!("choose_standout original={:x?} rust={:x?}", mo, mr));
}
unsafe extern "thiscall" fn compare_choose_by_role_b(this: usize, arg: u32) -> u8 {
    let f = original_fn::<unsafe extern "thiscall" fn(usize, u32) -> u8>(IDX_CHOOSE_BY_ROLE_B);
    let reg = crate::rng::STATE_REGIONS;
    let (r, n, igual, _, _) = crate::comparison::side_by_side(&reg, || f(this, arg), || crate::rng::without_counting(|| choose_by_role_b(this, arg)));
    SCOREBOARD.record(igual && r == n, || format!("choose_by_role_b({:#x}) original={} rust={}", arg, r as i8, n as i8));
    r
}
unsafe extern "thiscall" fn compare_formation_square(this: usize, j: usize, x: *mut u8, y: *mut u8) {
    let this = Engine(this);
    let j = MatchPlayer(j);
    let f = original_fn::<unsafe extern "thiscall" fn(usize, usize, *mut u8, *mut u8)>(IDX_FORMATION_SQUARE);
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&[(x as usize, 1), (y as usize, 1)], || f(this.addr(), j.addr(), x, y), || formation_square(this.addr(), j.addr(), x, y));
    SCOREBOARD.record(igual, || format!("formation_square(8eae {}, side {}, pos {}) original={:x?} rust={:x?}", this.possession_side(), j.side(), j.position(), mo, mr));
}

unsafe extern "thiscall" fn compare_compute_percentages(this: usize, i: i8) {
    let f = original_fn::<unsafe extern "thiscall" fn(usize, i8)>(IDX_COMPUTE_PERCENTAGES);
    let mut reg = Vec::new();
    for s in [i as isize, 3, 4] {
        for o in [0xf5dd, 0xf609, 0xf5d8, 0xf60e, 0xf613, 0xf5d3, 0xf5ce, 0xf604] {
            reg.push(((this as isize + o + s) as usize, 1));
        }
    }
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&reg, || f(this, i), || compute_percentages(this, i));
    SCOREBOARD.record(igual, || format!("compute_percentages({}) original={:x?} rust={:x?}", i, mo, mr));
}
unsafe extern "thiscall" fn compare_choose_by_role_c(this: usize, arg: u32, modo: u32) -> u8 {
    let f = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32) -> u8>(IDX_CHOOSE_BY_ROLE_C);
    let reg = crate::rng::STATE_REGIONS;
    let (r, n, igual, _, _) = crate::comparison::side_by_side(&reg, || f(this, arg, modo), || crate::rng::without_counting(|| choose_by_role_c(this, arg, modo)));
    SCOREBOARD.record(igual && r == n, || format!("choose_by_role_c({:#x}, {}) original={} rust={}", arg, modo as u8, r as i8, n as i8));
    r
}

compare_value!(compare_value_by_condition, value_by_condition, IDX_VALUE_BY_CONDITION, (alvo: usize) -> i32);
unsafe extern "thiscall" fn compare_apply_to_team(this: usize, j: usize, f: f32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, f32)>(IDX_APPLY_TO_TEAM);
    let reg: Vec<(usize, usize)> = (0..20).map(|k| (Engine(this).player(side_of_player(MatchPlayer(j)), k).addr() + 0x1a2, 4)).collect();
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(this, j, f), || apply_to_team(this, j, f));
    SCOREBOARD.record(igual, || format!("apply_to_team({}) original={:x?} rust={:x?}", f, mo, mr));
}
unsafe extern "thiscall" fn compare_apply_to_side(this: usize, l: i8, f: f32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i8, f32)>(IDX_APPLY_TO_SIDE);
    let reg: Vec<(usize, usize)> = (0..20).map(|k| (Engine(this).player(l, k).addr() + 0x1a2, 4)).collect();
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(this, l, f), || apply_to_side(this, l, f));
    SCOREBOARD.record(igual, || format!("apply_to_side({}, {}) original={:x?} rust={:x?}", l, f, mo, mr));
}
unsafe extern "thiscall" fn compare_choose_by_condition(this: usize, l: i8, modo: u32) -> u8 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i8, u32) -> u8>(IDX_CHOOSE_BY_CONDITION);
    let reg = crate::rng::STATE_REGIONS;
    let (r, n, igual, _, _) = crate::comparison::side_by_side(&reg, || o(this, l, modo), || crate::rng::without_counting(|| choose_by_condition(this, l, modo)));
    SCOREBOARD.record(igual && r == n, || format!("choose_by_condition({}, {}) original={} rust={}", l, modo as u8, r as i8, n as i8));
    r
}

compare_value!(compare_read_1bc, read_1bc, IDX_READ_1BC, () -> u16);
compare_value!(compare_player_table_a_1a, player_table_a_1a, IDX_PLAYER_TABLE_A_1A, () -> u16);
compare_value!(compare_player_table_b, player_table_b, IDX_PLAYER_TABLE_B, () -> u16);
compare_value!(compare_difference_of_player, difference_of_player, IDX_DIFFERENCE_OF_PLAYER, () -> u8);
compare_value!(compare_player_marks_rot, player_marks_rot, IDX_PLAYER_MARKS_ROT, () -> u64);
compare_value!(compare_mark_800000, mark_800000, IDX_MARK_800000, () -> u8);
compare_value!(compare_near_own_goal, near_own_goal, IDX_NEAR_OWN_GOAL, () -> u8);
compare_value!(compare_role_880, role_880, IDX_ROLE_880, () -> u8);
compare_value!(compare_player_by_record, player_by_record, IDX_PLAYER_BY_RECORD, (ficha: usize) -> usize);

unsafe fn card_regions(motor: usize, j: usize, tipo: *mut u8, peso: *mut f32) -> Vec<(usize, usize)> {
    let mut reg = vec![(j, 0x40), (tipo as usize, 1), (peso as usize, 4), ((motor as isize + 0xf5c9 + 54 * side_of_player(MatchPlayer(j)) as isize) as usize, 1)];
    reg.extend(crate::rng::STATE_REGIONS);
    reg
}

// The wrappers go through the inner (hooked) routine: they compare the same regions as it.
unsafe extern "thiscall" fn compare_engine_decide_foul(this: usize, j: usize, codigo: u16, tipo: *mut u8, valor: *mut u8) -> u8 {
    let this = Engine(this);
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u16, *mut u8, *mut u8) -> u8>(IDX_ENGINE_DECIDE_FOUL);
    let mut reg = vec![(this.addr() + engine::REFEREE + 8, 2), (tipo as usize, 1), (valor as usize, 1), (this.addr() + engine::PENDING_PLAY_KIND, 16)];
    reg.extend(crate::rng::STATE_REGIONS);
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(this.addr(), j, codigo, tipo, valor), || crate::rng::without_counting(|| engine_decide_foul(this.addr(), j, codigo, tipo, valor)));
    SCOREBOARD.record(igual && r == n, || format!("engine_decide_foul original={} {:x?} rust={} {:x?}", r, mo, n, mr));
    r
}

unsafe extern "thiscall" fn compare_look_for_offside(this: usize, l: i8, linha: i8, y: i8, passador: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i8, i8, i8, usize) -> usize>(IDX_LOOK_FOR_OFFSIDE);
    let motor = Engine(read_usize(this));
    let mut reg = vec![(this + 8, 2), (motor.addr() + engine::PENDING_PLAY_KIND, 16)];
    reg.extend(crate::rng::STATE_REGIONS);
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(this, l, linha, y, passador), || crate::rng::without_counting(|| look_for_offside(this, l, linha, y, passador)));
    SCOREBOARD.record(igual && r == n, || format!("look_for_offside({}, {}, {}, {}) original={:#x} {:x?} rust={:#x} {:x?}", l, linha, y, passador != 0, r, mo, n, mr));
    r
}
unsafe extern "thiscall" fn compare_engine_look_for_offside(this: usize, l: i8, linha: i8, y: i8, passador: usize) -> usize {
    let this = Engine(this);
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i8, i8, i8, usize) -> usize>(IDX_ENGINE_LOOK_FOR_OFFSIDE);
    let mut reg = vec![(this.addr() + engine::REFEREE + 8, 2), (this.addr() + engine::PENDING_PLAY_KIND, 16)];
    reg.extend(crate::rng::STATE_REGIONS);
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(this.addr(), l, linha, y, passador), || crate::rng::without_counting(|| engine_look_for_offside(this.addr(), l, linha, y, passador)));
    SCOREBOARD.record(igual && r == n, || format!("engine_look_for_offside original={:#x} {:x?} rust={:#x} {:x?}", r, mo, n, mr));
    r
}

unsafe extern "thiscall" fn compare_card_by_severity(this: usize, j: usize, tipo: *mut u8, g: u8, flag: u8) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, *mut u8, u8, u8)>(IDX_CARD_BY_SEVERITY);
    let mut reg = vec![(j, 0x40), (tipo as usize, 1)];
    reg.extend(crate::rng::STATE_REGIONS);
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(this, j, tipo, g, flag), || crate::rng::without_counting(|| card_by_severity(this, j, tipo, g, flag)));
    SCOREBOARD.record(igual, || format!("card_by_severity(g {}, flag {}) original={:x?} rust={:x?}", g, flag, mo, mr));
}
unsafe extern "thiscall" fn compare_engine_card_by_severity(this: usize, j: usize, tipo: *mut u8, g: u8, flag: u8) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, *mut u8, u8, u8)>(IDX_ENGINE_CARD_BY_SEVERITY);
    let mut reg = vec![(j, 0x40), (tipo as usize, 1)];
    reg.extend(crate::rng::STATE_REGIONS);
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(this, j, tipo, g, flag), || crate::rng::without_counting(|| engine_card_by_severity(this, j, tipo, g, flag)));
    SCOREBOARD.record(igual, || format!("engine_card_by_severity original={:x?} rust={:x?}", mo, mr));
}

unsafe extern "thiscall" fn compare_choose_by_role_e(this: usize, arg: u32, modo: u32) -> u8 {
    let f = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32) -> u8>(IDX_CHOOSE_BY_ROLE_E);
    let reg = crate::rng::STATE_REGIONS;
    let (r, n, igual, _, _) = crate::comparison::side_by_side(&reg, || f(this, arg, modo), || crate::rng::without_counting(|| choose_by_role_e(this, arg, modo)));
    SCOREBOARD.record(igual && r == n, || format!("choose_by_role_e({:#x}, {}) original={} rust={}", arg, modo as u8, r as i8, n as i8));
    r
}

unsafe extern "thiscall" fn compare_possession_statistic(this: usize, l: i8, janela: u8) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i8, u8)>(IDX_POSSESSION_STATISTIC);
    let reg = [(this, 0x230), (this + 0x8d40, 0x260)];
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(this, l, janela), || possession_statistic(this, l, janela));
    SCOREBOARD.record(igual, || format!("possession_statistic({}, {}) original={:x?} rust={:x?}", l, janela, mo, mr));
}

unsafe extern "thiscall" fn compare_match_summary(this: usize, momento: i16) {
    let this = Engine(this);
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i16)>(IDX_MATCH_SUMMARY);
    if is_recording_top_level() {
        call_recording(0x6ce460, this.addr(), [momento as u16 as u32, 0, 0, 0, 0], || {
            o(this.addr(), momento);
            0
        });
        return;
    }
    let mut reg = vec![(this.addr() + engine::PLAYERS, 40 * 0x1be), (this.addr() + 0xf5b0, 0x90)];
    reg.extend(crate::rng::STATE_REGIONS);
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(this.addr(), momento), || crate::rng::without_counting(|| match_summary(this.addr(), momento)));
    SCOREBOARD.record(igual, || format!("match_summary({}) original={:x?} rust={:x?}", momento, mo, mr));
}

compare_value!(compare_closest_on_side, closest_on_side, IDX_CLOSEST_ON_SIDE, (l: i8, a: i8, b: i8, todos: u8) -> usize);

#[allow(clippy::too_many_arguments)]
unsafe extern "thiscall" fn compare_goalkeeper_save(this: usize, a: i8, b: i8, a3: u8, codigo: i8, saida: *mut u8, rebote: *mut u8) -> u16 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i8, i8, u8, i8, *mut u8, *mut u8) -> u16>(IDX_GOALKEEPER_SAVE);
    let mut reg = vec![(saida as usize, 1), (rebote as usize, 1)];
    reg.extend(crate::rng::STATE_REGIONS);
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(this, a, b, a3, codigo, saida, rebote), || crate::rng::without_counting(|| goalkeeper_save(this, a, b, a3, codigo, saida, rebote)));
    SCOREBOARD.record(igual && r == n, || format!("goalkeeper_save({}, {}, {}, {}) original={} {:x?} rust={} {:x?}", a, b, a3, codigo, r as i16, mo, n as i16, mr));
    r
}

unsafe extern "thiscall" fn compare_control_of_move(this: usize, codigo: *mut i8, a: *mut u8, b: *mut u8) -> u16 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, *mut i8, *mut u8, *mut u8) -> u16>(IDX_CONTROL_OF_MOVE);
    let mut reg = vec![(codigo as usize, 1), (a as usize, 1), (b as usize, 1)];
    reg.extend(crate::rng::STATE_REGIONS);
    let c0 = *codigo;
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(this, codigo, a, b), || crate::rng::without_counting(|| control_of_move(this, codigo, a, b)));
    SCOREBOARD.record(igual && r == n, || format!("control_of_move({}) original={} {:x?} rust={} {:x?}", c0, r as i16, mo, n as i16, mr));
    r
}

// Recording of the calls to 0x6cd330 made by the original of `trace_move`: arguments, answer and
// the RNG state after each one. The Rust copy receives the recorded answers (without running the
// routine, which has effects on the players) and the RNG returns to the recorded state at each call.
static RECORDING_6CD330: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
#[allow(clippy::type_complexity)]
static TAPE_6CD330: std::sync::Mutex<Vec<((i8, usize, u32, u32, u32, u32, i8), u8, crate::comparison::MemorySnapshot)>> = std::sync::Mutex::new(Vec::new());

#[allow(clippy::too_many_arguments)]
unsafe extern "thiscall" fn compare_search_on_side_recording(this: usize, lado: i8, grupo: usize, a3: u32, a4: u32, a5: u32, a6: u32, modo: i8) -> u8 {
    let r = compare_procurar(this, lado, grupo, a3, a4, a5, a6, modo);
    if RECORDING_6CD330.load(std::sync::atomic::Ordering::Relaxed) {
        let depois = crate::comparison::MemorySnapshot::read(&crate::rng::STATE_REGIONS);
        TAPE_6CD330.lock().unwrap_or_else(|e| e.into_inner()).push(((lado, grupo, a3, a4, a5, a6, modo), r, depois));
    }
    r
}

#[allow(clippy::too_many_arguments)]
unsafe extern "thiscall" fn compare_trace_move(this: usize, x0: i8, y0: i8, x1: i8, y1: i8, codigo: i8, a6: u32, a7: u32) -> u8 {
    if is_recording_top_level() {
        // unit of the tape of the pass execution (the only one that calls it); inside, the comparison itself
        let xy = x0 as u8 as u32 | (y0 as u8 as u32) << 8 | (x1 as u8 as u32) << 16 | (y1 as u8 as u32) << 24;
        return call_recording(0x6ccb60, this, [xy, codigo as u8 as u32, a6, a7, 0], || compare_trace_move(this, x0, y0, x1, y1, codigo, a6, a7) as u32) as u8;
    }
    use std::sync::atomic::Ordering::Relaxed;
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i8, i8, i8, i8, i8, u32, u32) -> u8>(IDX_TRACE_MOVE);
    TAPE_6CD330.lock().unwrap_or_else(|e| e.into_inner()).clear();
    let tape_lock = core::cell::RefCell::new(Vec::new());
    let sequencia = core::cell::Cell::new(true);
    let reg = crate::rng::STATE_REGIONS;
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side(
        &reg,
        || {
            RECORDING_6CD330.store(true, Relaxed);
            let r = o(this, x0, y0, x1, y1, codigo, a6, a7);
            RECORDING_6CD330.store(false, Relaxed);
            *tape_lock.borrow_mut() = core::mem::take(&mut *TAPE_6CD330.lock().unwrap_or_else(|e| e.into_inner()));
            r
        },
        || {
            let gravadas = tape_lock.borrow();
            let mut prox = gravadas.iter();
            let n = crate::rng::without_counting(|| {
                trace_line_interception_with(this, x0, y0, x1, y1, codigo, a6, a7, &mut |l, c, dd, a, t, b| match prox.next() {
                    // the type and a6/a7 only have the low byte (the rest is stack garbage in the original)
                    Some(((gl, gc, gd, ga, gt, gb, 1), gr, depois)) if *gl == l && *gc == c && *gd == dd && *ga as u16 == a as u16 && *gt as u8 == t as u8 && *gb as u8 == b as u8 => {
                        depois.write_back();
                        *gr
                    }
                    _ => {
                        sequencia.set(false);
                        0
                    }
                })
            });
            if prox.next().is_some() {
                sequencia.set(false);
            }
            n
        },
    );
    SCOREBOARD.record(igual && r == n && sequencia.get(), || format!("trace_move({} {} -> {} {}, code {}) original={} rust={} seq={} {:x?} {:x?}", x0, y0, x1, y1, codigo, r, n, sequencia.get(), mo, mr));
    r
}

compare_value!(compare_has_match, has_match, IDX_HAS_MATCH, (clube: u32, so: u32) -> u32);
unsafe extern "C" fn compare_won_the_match(jogo: usize, clube: u32) -> u8 {
    if is_recording_post_match() {
        return call_recording(0x6b7cf0, 0, [jogo as u32, clube, 0, 0, 0], || original_fn::<unsafe extern "C" fn(usize, u32) -> u8>(IDX_WON_THE_MATCH)(jogo, clube) as u32) as u8;
    }
    let r = original_fn::<unsafe extern "C" fn(usize, u32) -> u8>(IDX_WON_THE_MATCH)(jogo, clube);
    let n = won_the_match(jogo, clube);
    SCOREBOARD.record(r == n, || format!("won_the_match original={} rust={}", r, n));
    r
}

// Port of 0x6d6f70 (commentary): in comparison, records the calls made by the original of a
// ported routine (with the RNG state afterwards); the Rust copy checks the sequence and does not run
// the commentary again (avoids duplicate events). Outside recording, it just passes through.
static PORT_6D6F70: AtomicUsize = AtomicUsize::new(0);
static RECORDING_6D6F70: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
/// In the comparison of `exe::match_player_try_block_shot` the tape keeps the seventh argument of the narration in the high part of the code.
static RECORD_A7: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
static TAPE_6D6F70: std::sync::Mutex<Vec<(usize, [u32; 7], crate::comparison::MemorySnapshot)>> = std::sync::Mutex::new(Vec::new());
const PROLOGUE_6D6F70: Prologue = Prologue { address: 0x6d6f70, bytes: &[0x8a, 0x44, 0x24, 0x10, 0x83, 0xec, 0x20] };

#[allow(clippy::too_many_arguments)]
unsafe extern "thiscall" fn port_6d6f70(this: usize, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32, a6: u32, a7: u32) {
    use std::sync::atomic::{AtomicU32, Ordering::Relaxed};
    // 0x6d6f70 may call itself (one comment triggers another): only the outer call is recorded
    static DEPTH: AtomicU32 = AtomicU32::new(0);
    let f: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, u32, u32) = core::mem::transmute(PORT_6D6F70.load(Relaxed));
    // inside a tape recording (the last two arguments are -1 and 0 in the ported callers)
    if is_recording() {
        let codigo = if RECORD_A7.load(Relaxed) { a1 & 0xffff | (a7 & 0xff) << 16 } else { a1 };
        call_recording(0x6d6f70, this, [codigo, a2, a3, a4, a5], || {
            f(this, a1, a2, a3, a4, a5, a6, a7);
            0
        });
        return;
    }
    let nivel = DEPTH.fetch_add(1, Relaxed);
    // in the autotest the narrator is fake: the call is only recorded
    if !SYNTHETIC.load(Relaxed) {
        f(this, a1, a2, a3, a4, a5, a6, a7);
    }
    DEPTH.fetch_sub(1, Relaxed);
    if nivel == 0 && RECORDING_6D6F70.load(Relaxed) {
        let depois = crate::comparison::MemorySnapshot::read(&crate::rng::STATE_REGIONS);
        TAPE_6D6F70.lock().unwrap_or_else(|e| e.into_inner()).push((this, [a1, a2, a3, a4, a5, a6, a7], depois));
    }
}

/// Runs the original recording the comments and the Rust copy replaying them; returns (equal, how many).
unsafe fn compare_with_commentary(original: impl FnOnce(), rust: impl FnOnce(AddEventCall)) -> (bool, usize) {
    compare_with_commentary_at(&[], original, rust)
}

/// 0x6d6f70 reads the 1st argument as a word and the 3rd to 6th as bytes: the rest is stack garbage.
const MASK_6D6F70: [u32; 7] = [0xffff, u32::MAX, 0xff, 0xff, 0xff, 0xff, u32::MAX];

fn same_arguments(a: &[u32; 7], b: &[u32; 7]) -> bool {
    (0..7).all(|i| a[i] & MASK_6D6F70[i] == b[i] & MASK_6D6F70[i])
}

/// Like `compare_com_comentarios`, also comparing the `extra` regions (besides the RNG state).
unsafe fn compare_with_commentary_at(extra: &[(usize, usize)], original: impl FnOnce(), rust: impl FnOnce(AddEventCall)) -> (bool, usize) {
    use std::sync::atomic::Ordering::Relaxed;
    // inside another comparison that is recording (a ported routine calls another that narrates, or a
    // tape recording is open): only the original runs, and its comments go into the outer recording
    if RECORDING_6D6F70.load(Relaxed) || is_recording() {
        original();
        return (true, 0);
    }
    TAPE_6D6F70.lock().unwrap_or_else(|e| e.into_inner()).clear();
    let tape_lock = core::cell::RefCell::new(Vec::new());
    let sequencia = core::cell::Cell::new(true);
    let mut reg = crate::rng::STATE_REGIONS.to_vec();
    reg.extend_from_slice(extra);
    let (_, _, igual, _, _) = crate::comparison::side_by_side(
        &reg,
        || {
            RECORDING_6D6F70.store(true, Relaxed);
            original();
            RECORDING_6D6F70.store(false, Relaxed);
            *tape_lock.borrow_mut() = core::mem::take(&mut *TAPE_6D6F70.lock().unwrap_or_else(|e| e.into_inner()));
        },
        || {
            let gravadas = tape_lock.borrow();
            let mut prox = gravadas.iter();
            crate::rng::without_counting(|| {
                rust(&mut |t, a| match prox.next() {
                    Some((gt, ga, depois)) if *gt == t && same_arguments(ga, &a) => depois.write_back(),
                    _ => sequencia.set(false),
                })
            });
            if prox.next().is_some() {
                sequencia.set(false);
            }
        },
    );
    let n = tape_lock.borrow().len();
    (igual && sequencia.get(), n)
}

unsafe extern "thiscall" fn compare_shot_value(this: usize, tipo: *mut i8, som: *mut u16) -> u16 {
    let this = MatchPlayer(this);
    let o = original_fn::<unsafe extern "thiscall" fn(usize, *mut i8, *mut u16) -> u16>(IDX_SHOT_VALUE);
    let mut reg = vec![(tipo as usize, 1), (som as usize, 2), (this.addr() + match_player::SHOT_QUALITY, 2)];
    reg.extend(crate::rng::STATE_REGIONS);
    let t0 = *tipo;
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(this.addr(), tipo, som), || crate::rng::without_counting(|| shot_value(this.addr(), tipo, som)));
    SCOREBOARD.record(igual && r == n, || format!("shot_value(kind {}) original={} {:x?} rust={} {:x?}", t0, r as i16, mo, n as i16, mr));
    r
}

compare_value!(compare_is_friendly, is_friendly, IDX_IS_FRIENDLY, (comp: usize) -> u32);
compare_value!(compare_contract_of, contract_of, IDX_CONTRACT_OF, (pessoa: usize) -> usize);

compare_value!(compare_is_on_holiday, is_on_holiday, IDX_IS_ON_HOLIDAY, (pessoa: usize) -> u32);
compare_value!(compare_manager_9a05, manager_9a05, IDX_MANAGER_9A05, (pessoa: usize) -> u32);

compare_value!(compare_manager_9ace, manager_9ace, IDX_MANAGER_9ACE, (pessoa: usize) -> u32);
compare_value!(compare_side_under_control, side_under_control, IDX_SIDE_UNDER_CONTROL, (lado: i8) -> u8);
compare_value!(compare_tactics_byte, tactics_byte, IDX_TACTICS_BYTE, (clube: usize, pos: i8, humano: u32) -> u8);
compare_value!(compare_row_person, row_person, IDX_ROW_PERSON, (clube: usize, qual: i8, humano: u32) -> usize);
compare_value!(compare_club_in_competition, club_in_competition, IDX_CLUB_IN_COMPETITION, (clube: usize) -> u8);
compare_value!(compare_manager_9b23, manager_9b23, IDX_MANAGER_9B23, (i: i8, pessoa: usize) -> u8);
compare_value!(compare_contract_flags, contract_flags, IDX_CONTRACT_FLAGS, () -> u32);
compare_value!(compare_word_0c, word_0c, IDX_WORD_0C, (id: i32) -> i32);
unsafe extern "C" fn compare_round_below_1000(v: i32) -> i32 {
    let r = original_fn::<unsafe extern "C" fn(i32) -> i32>(IDX_ROUND_BELOW_1000)(v);
    let n = round_below_1000(v);
    SCOREBOARD.record(r == n, || format!("round_below_1000({:#x}) original={} rust={}", v, r, n));
    r
}
compare_value!(compare_how_many_mark, how_many_mark, IDX_HOW_MANY_MARK, (alvo: usize) -> u8);
compare_value!(compare_who_marks, who_marks, IDX_WHO_MARKS, (alvo: usize) -> usize);
compare_order!(compare_order_by_5f, order_by_5f, IDX_ORDER_BY_5F);
compare_order!(compare_order_by_80, order_by_80, IDX_ORDER_BY_80);
compare_order!(compare_order_by_80_desc, order_by_80_desc, IDX_ORDER_BY_80_DESC);
compare_order!(compare_order_by_80_asc, order_by_80_asc, IDX_ORDER_BY_80_ASC);
compare_value!(compare_player_rating, player_rating, IDX_PLAYER_RATING, () -> i32);
compare_value!(compare_manager_table, manager_table, IDX_MANAGER_TABLE, (tecnico: usize) -> usize);
compare_value!(compare_sector_slot_taken, sector_slot_taken, IDX_SECTOR_SLOT_TAKEN, (mascara: u32, jogadores: usize) -> u32);
compare_value!(compare_slot_of_code, slot_of_code, IDX_SLOT_OF_CODE, (codigo: u16) -> u32);
compare_value!(compare_slot_code, slot_code, IDX_SLOT_CODE, (vaga: i8, segunda: u32) -> u16);
compare_value!(compare_read_person_mark, read_person_mark, IDX_READ_PERSON_MARK, (pessoa: usize, k: i8) -> u8);
compare_value!(compare_person_byte_44, person_byte_44, IDX_PERSON_BYTE_44, (pessoa: usize, k: i8) -> i32);
compare_value!(compare_person_field_3a, person_field_3a, IDX_PERSON_FIELD_3A, (pessoa: usize, k: i8) -> usize);
compare_value!(compare_person_competition, person_competition, IDX_PERSON_COMPETITION, (pessoa: usize) -> usize);
compare_value!(compare_national_side_byte_42, national_side_byte_42, IDX_NATIONAL_SIDE_BYTE_42, (clube: usize, flag: u32) -> u8);
compare_value!(compare_national_side_byte_43, national_side_byte_43, IDX_NATIONAL_SIDE_BYTE_43, (clube: usize, flag: u32) -> u8);
compare_value!(compare_ball_in_attack, ball_in_attack, IDX_BALL_IN_ATTACK, () -> u8);
compare_value!(compare_group_decided, group_decided, IDX_GROUP_DECIDED, (clube: usize, partida: usize) -> u32);
/// The rows of the staff's club in the tactics bank (0x8982b0 for human manager or not).
unsafe fn club_rows(clube: usize) -> Vec<(usize, usize)> {
    let taticas = read_usize(G_TACTICS);
    let linha: unsafe extern "stdcall" fn(usize, u32) -> i32 = core::mem::transmute(FN_TACTICS_GET_CLUB_TACTICS_INDEX);
    let mut v = Vec::new();
    for humano in 0..2u32 {
        let l = if clube == 0 || taticas == 0 { -1 } else { linha(clube, humano) };
        if l != -1 {
            v.push((read_usize(taticas + tactics_fields::ROWS).wrapping_add((l as usize).wrapping_mul(0x91)), 0x91));
        }
    }
    v
}
/// The first differing byte between the memories (the engine regions are large).
fn first_difference(mo: &crate::comparison::MemorySnapshot, mr: &crate::comparison::MemorySnapshot) -> String {
    mo.0.iter()
        .zip(mr.0.iter())
        .find_map(|(a, b)| {
            a.1.iter().zip(b.1.iter()).position(|(x, y)| x != y).map(|k| format!("{:#x}+{:#x}: {:x?} x {:x?}", a.0, k, &a.1[k..(k + 8).min(a.1.len())], &b.1[k..(k + 8).min(b.1.len())]))
        })
        .unwrap_or_default()
}
compare_value!(compare_players_on_path, players_on_path, IDX_PLAYERS_ON_PATH, (x1: i8, y1: i8, x2: i8, y2: i8) -> u8);

compare_value!(compare_direct_marker, direct_marker, IDX_DIRECT_MARKER, (l: i8, k: i8) -> u8);

/// Diagnosis: routines that stay original (to isolate a career difference without touching the
/// game environment, which shifts the heap). Empty in normal use.
const SEM_ROTINAS: &[&str] = &[];

/// Comparison of the set pieces of the 57th batch: whole engine and RNG.
unsafe fn compare_restart(nome: &str, this: usize, o: unsafe extern "thiscall" fn(usize), r: unsafe extern "thiscall" fn(usize)) {
    let mut regioes = vec![(this, ENGINE_SIZE), (G_MOUSE_LEFT_BUTTON_DOWN, 4)];
    regioes.extend(crate::rng::STATE_REGIONS);
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&regioes, || o(this), || crate::rng::without_counting(|| r(this)));
    SCOREBOARD.record(igual, || {
        let (a, b) = (&mo.0, &mr.0);
        let dif: Vec<usize> = (0..a.len()).flat_map(|q| (0..a[q].1.len()).filter(move |&i| a[q].1[i] != b[q].1[i]).map(move |i| a[q].0 + i)).take(16).collect();
        format!("{} differences em {:x?}", nome, dif)
    });
}

pub unsafe fn instalar(comparando: bool) {
    if comparando {
        for (k, pr) in POS_PROLOGUES.iter().enumerate() {
            if let Some(t) = detours::hook(&format!("gate_{:x} (after the match)", pr.address), pr, post_match_gates()[k]) {
                PORTAS_POS[k] = t;
            }
        }
        if let Some(t) = detours::hook("gate_6f5c30 (save)", &PROLOGUE_6F5C30, respond_hook as *const () as usize) {
            MATCH_PLAYER_RESPOND_TRAMPOLINE.store(t, Relaxed);
        }
        if let Some(t) = detours::hook("gate_6d6f70 (save)", &PROLOGUE_6D6F70, port_6d6f70 as *const () as usize) {
            PORT_6D6F70.store(t, Relaxed);
        }
        if let Some(t) = detours::hook("gate_6eaea0 (save)", &PROLOGUE_6EAEA0, init_player_hook as *const () as usize) {
            INIT_MATCH_PLAYER_TRAMPOLINE.store(t, Relaxed);
        }
        if let Some(t) = detours::hook("gate_716630 (save)", &PROLOGUE_716630, gate_start_fight as *const () as usize) {
            START_FIGHT_TRAMPOLINE.store(t, Relaxed);
        }
        if let Some(t) = detours::hook("gate_716e10 (save)", &PROLOGUE_716E10, gate_react_to_foul_decision as *const () as usize) {
            REACT_TO_FOUL_DECISION_TRAMPOLINE.store(t, Relaxed);
        }
        if let Some(t) = detours::hook("gate_716c90 (save)", &PROLOGUE_716C90, gate_handle_suspected_dive as *const () as usize) {
            HANDLE_SUSPECTED_DIVE_TRAMPOLINE.store(t, Relaxed);
        }
        if let Some(t) = detours::hook("gate_70dfd0 (save)", &PROLOGUE_70DFD0, gate_try_block_shot as *const () as usize) {
            TRY_BLOCK_SHOT_TRAMPOLINE.store(t, Relaxed);
        }
        if let Some(t) = detours::hook("gate_68d0d0 (save)", &PROLOGUE_68D0D0, gate_llist_add_tail as *const () as usize) {
            LLIST_ADD_TAIL_TRAMPOLINE.store(t, Relaxed);
        }
        if let Some(t) = detours::hook("gate_62ed70 (save)", &PROLOGUE_62ED70, gate_get_player_injury as *const () as usize) {
            GET_PLAYER_INJURY_TRAMPOLINE.store(t, Relaxed);
        }
    }
    let rust = rust_table();
    let comp = compare_table();
    let nomes = NAMES;
    for i in 0..N {
        if SEM_ROTINAS.contains(&nomes[i]) {
            // the original through the same pass-through detour as original mode (same private stack)
            linha(&format!("engine: {} stays with the original (SEM_ROTINAS)", nomes[i]));
            detours::hook(nomes[i], &PROLOGOS[i], detours::PASSTHROUGH);
            continue;
        }
        // in comparison 0x6f5c30 and 0x70dfd0 already have the port (installed earlier), which compares them too
        if comparando && (PROLOGOS[i].address == 0x6f5c30 || PROLOGOS[i].address == 0x70dfd0 || PROLOGOS[i].address == 0x867560) {
            continue;
        }
        let destino = if comparando { comp[i] } else { rust[i] };
        let feito = detours::hook(nomes[i], &PROLOGOS[i], destino);
        // refused (routine changed by another patch): orig(i) calls the exe itself, which runs without a detour
        ORIGINAIS[i] = feito.unwrap_or(PROLOGOS[i].address);
    }
    if comparando {
        linha("engine: comparison mode (original x Rust)");
    } else if let Some(p) = std::env::var("CM_DIAG_PESSOA").ok().and_then(|v| v.parse::<usize>().ok()) {
        // diagnosis (31/10/2026, test of CMRTE's "Make unhappy"): person P's new complaint and new mark,
        // with the contract fields before and after (the 0x4d8610 request of the human club has its own line)
        DIAG_PERSON.store(p, Relaxed);
        DIAG_CLUB.store(std::env::var("CM_DIAG_CLUBE").ok().and_then(|v| v.parse::<usize>().ok()).unwrap_or(usize::MAX), Relaxed);
        DIAG_COMPLAINT.store(detours::hook("diag complaint 0x867700", &Prologue { address: 0x867700, bytes: &[0x53, 0x56, 0x8b, 0xf1, 0x8b, 0x15, 0xb4, 0x23, 0xae, 0x00] }, diag_complaint as *const () as usize).unwrap_or(0), Relaxed);
    }
}

static DIAG_PERSON: AtomicUsize = AtomicUsize::new(usize::MAX);
static DIAG_COMPLAINT: AtomicUsize = AtomicUsize::new(0);
/// CM_DIAG_CLUBE=C: also every contract of club C (+4), to find who in the club receives a complaint or mark.
static DIAG_CLUB: AtomicUsize = AtomicUsize::new(usize::MAX);
static DIAG_MARK: AtomicUsize = AtomicUsize::new(0);

unsafe fn diag_contract(c: usize) -> String {
    format!("+0x44 {} +0x45 {:#x} +0x40 {:#x} +0x49 {} +0x4e {:#x} +0x4f {:#x}", read_u8(c + 0x44) as i8, read_u32(c + 0x45), read_u32(c + 0x40), read_u8(c + 0x49), read_u8(c + 0x4e), read_u8(c + 0x4f))
}

static DIAG_CALLS: AtomicU32 = AtomicU32::new(0);

/// At every power of 2 of calls (from any person): the person of the current contract and person P's contract
/// read directly from memory (`exe::contract_manager_get_contract` over [0xae1050], person = [0xae23bc] + id × 110), to check the save's writing.
unsafe fn diag_sample(c: usize, qual: &str) {
    DIAG_CALLS.fetch_add(1, Relaxed);
    if !DIAG_CALLS.load(Relaxed).is_power_of_two() {
        return;
    }
    let p = read_usize(G_STAFF_TABLE).wrapping_add(DIAG_PERSON.load(Relaxed).wrapping_mul(staff::SIZE));
    let k = exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), p);
    linha(&format!("diag {} call {}: current contract +0 {} +4 {}; the person's contract {}: {}", qual, DIAG_CALLS.load(Relaxed),
        read_i32(c), read_i32(c + 4), DIAG_PERSON.load(Relaxed),
        if k == 0 { "nenhum".to_string() } else { format!("+0 {} +4 {} {}", read_i32(k), read_i32(k + 4), diag_contract(k)) }));
}

unsafe extern "thiscall" fn diag_complaint(c: usize, f: u32, z: u32) -> u32 {
    if c != 0 {
        diag_sample(c, "queixa");
    }
    let alvo = c != 0 && (read_u32(c) as usize == DIAG_PERSON.load(Relaxed) || read_u32(c + 4) as usize == DIAG_CLUB.load(Relaxed));
    if alvo {
        linha(&format!("diag complaint person {} complaint {:#x} z {} before: {}", read_u32(c), f, z, diag_contract(c)));
    }
    let r = core::mem::transmute::<usize, unsafe extern "thiscall" fn(usize, u32, u32) -> u32>(DIAG_COMPLAINT.load(Relaxed))(c, f, z);
    if alvo {
        linha(&format!("diag complaint person {} returned {} after: {}", read_u32(c), r, diag_contract(c)));
    }
    // CM_DIAG_FORCAR=Q: a synthetic complaint Q on person P's contract, once, right after the game's first complaint
    // (the same event in the save variants: only the contract state decides whether the request goes out)
    static mut FORCED: bool = false;
    if !FORCED {
        if let Some(q) = std::env::var("CM_DIAG_FORCAR").ok().and_then(|v| u32::from_str_radix(v.trim_start_matches("0x"), 16).ok()) {
            FORCED = true;
            let k = exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), read_usize(G_STAFF_TABLE).wrapping_add(DIAG_PERSON.load(Relaxed).wrapping_mul(staff::SIZE)));
            if k != 0 {
                linha(&format!("diag forced complaint {:#x} person {} before: {}", q, DIAG_PERSON.load(Relaxed), diag_contract(k)));
                let r2 = core::mem::transmute::<usize, unsafe extern "thiscall" fn(usize, u32, u32) -> u32>(DIAG_COMPLAINT.load(Relaxed))(k, q, 0);
                linha(&format!("diag forced complaint returned {} after: {}", r2, diag_contract(k)));
            }
        }
    }
    r
}

unsafe extern "thiscall" fn diag_mark(c: usize, f: u32) -> u32 {
    if c != 0 {
        diag_sample(c, "marca");
    }
    let alvo = c != 0 && (read_u32(c) as usize == DIAG_PERSON.load(Relaxed) || read_u32(c + 4) as usize == DIAG_CLUB.load(Relaxed));
    if alvo {
        linha(&format!("diag mark person {} mark {:#x} before: {}", read_u32(c), f, diag_contract(c)));
    }
    let r = core::mem::transmute::<usize, unsafe extern "thiscall" fn(usize, u32) -> u32>(DIAG_MARK.load(Relaxed))(c, f);
    if alvo {
        linha(&format!("diag mark person {} returned {} after: {}", read_u32(c), r, diag_contract(c)));
    }
    r
}

/// Simulation in original mode: the same routines detoured to the original itself (private stack).
pub unsafe fn install_passthrough() {
    let (sonda, dentro, prologo) = crate::stack_probe::target();
    if sonda != 0 && prologo >= 5 && !PROLOGOS.iter().any(|p| p.address == sonda) {
        // routine outside the table: the prologue comes from the exe itself (the size, from the variable)
        let bytes: &'static [u8] = Box::leak(core::slice::from_raw_parts(sonda as *const u8, prologo).to_vec().into_boxed_slice());
        detours::hook_probe("stack probe", &Prologue { address: sonda, bytes }, dentro);
    }
    for (i, p) in PROLOGOS.iter().enumerate() {
        if p.address == sonda {
            detours::hook_probe(&format!("engine {} (stack probe)", i), p, dentro);
            continue;
        }
        detours::hook(&format!("engine {} (original, isolated)", i), p, detours::PASSTHROUGH);
    }
}

/// Autotest diagnosis: is the fake engine's grid consistent (counts 0..=11, each entry a player of
/// the side's block, in the right cell and with the right index at 0x28)?
unsafe fn grid_inconsistent(m: Engine) -> Option<String> {
    for x in 0..9i8 {
        for y in 0..12i8 {
            let casa = m.cell(x, y).addr();
            for l in 0..2usize {
                let n = read_i8(casa + (0x58 + l));
                if !(0..=11).contains(&n) {
                    return Some(format!("cell ({},{}) side {} count {}", x, y, l, n));
                }
                for i in 0..n as usize {
                    let p = MatchPlayer(read_usize(casa + 4 * (11 * l + i)));
                    let base = m.player(l as i8, 0).addr();
                    if p.addr() < base || p.addr() >= base + 20 * 0x1be || (p.addr() - base) % 0x1be != 0 {
                        return Some(format!("casa ({},{}) lado {} entrada {}/{} = {:#x}", x, y, l, i, n, p.addr()));
                    }
                    if p.cell_x() != x || p.cell_y() != y || p.cell_slot() != i as i8 {
                        return Some(format!("cell ({},{}) side {} entry {}: player em ({},{}) index {}", x, y, l, i, p.cell_x(), p.cell_y(), p.cell_slot()));
                    }
                }
            }
        }
    }
    None
}

/// Synthetic inputs: a fake engine object and player, with their own tables; positions, sides and
/// coordinates varying, including outside the valid ranges. Runs through the comparison functions,
/// which call the original and the Rust version with the same inputs.
pub unsafe fn autoteste(casos: u64) {
    let mut s = TestRng(0x4d4f544f52);
    let mut motor = vec![0u8; 0x10000]; // fields up to 0xf5f5 (side state)
    let mut tabelas = vec![vec![0u16; 11]; 4];
    let mut jogador = vec![0u8; 0x200];
    let outro = vec![0u8; 0x1a4];
    let mut pessoa = vec![0u8; 0x80]; // fields read up to 0x5d
    let mut formacoes = vec![vec![0u8; 4800]; 2];
    let mut grupo = vec![0u8; 0x60];
    let mut jogadores = vec![vec![0u8; 0x1a8]; 22];
    let mut fichas = vec![0u32; 40];
    let mut pessoas = vec![vec![0u8; 0x80]; 4];
    let mut jogos = vec![0u8; 0x69 * 12 + 0x80];
    let mut partida42 = vec![0u8; 0x40];
    let mut clube42 = vec![vec![0u8; 0x90]; 2];
    let mut jogo42 = vec![0u8; 0x70];
    let mut comp42 = vec![0u8; 0x10];
    let mut prefs46 = vec![0u8; 0x40];
    let mut pessoa46 = vec![0u8; 0x80];
    let mut tabela55 = vec![0u8; 79 * 80 + 8];
    let mut tecnicos56 = vec![0u8; 42735 * 16 + 16];
    let mut partida50 = vec![0u8; 0x60];
    SYNTHETIC.store(true, std::sync::atomic::Ordering::Relaxed);
    SCOREBOARD.self_test(|| {
        for _ in 0..casos {
            for t in tabelas.iter_mut() {
                for v in t.iter_mut() {
                    *v = s.next_u64() as u16;
                }
            }
            let m = motor.as_mut_ptr() as usize;
            for (lado, campo) in [(0usize, 0x8ebc), (1, 0x8ebc), (0, 0x8ec4), (1, 0x8ec4)] {
                let t = &tabelas[(campo - 0x8ebc) / 4 + lado];
                *((m + campo + lado * 4) as *mut usize) = t.as_ptr() as usize;
            }
            let x = s.next_u64();
            *((m + 0x8eaf) as *mut u8) = x as u8;
            *((m + 0x8eb0) as *mut u8) = (x >> 8) as u8;
            *((m + 0x8ed8) as *mut u16) = (x >> 16) as u16;
            let j = jogador.as_mut_ptr() as usize;
            let y = s.next_u64();
            *((j + 0x19) as *mut i8) = (y % 15) as i8 - 2; // -2..=12
            *((j + 0x27) as *mut i8) = ((y >> 8) % 2) as i8;
            *((j + 0x103) as *mut i8) = ((y >> 16) % 13) as i8 - 2;
            let lado = [0u8, 1, 2, 255][((y >> 24) % 4) as usize];
            compare_table_a(m, j);
            compare_table_b(m, j);
            compare_diferenca(m, lado, (y >> 32) as u8);
            compare_band(lado, ((y >> 40) % 10) as i8 - 1, ((y >> 44) % 14) as i8 - 1);
            compare_limit(lado, ((y >> 48) % 14) as i8 - 2);
            *((j + 0x27) as *mut i8) = [0i8, 1, 2, -1][((y >> 52) % 4) as usize];
            compare_player_condition(j);
            // coordinates with differences of -5..=5 (the global table is read from the game itself)
            let z = s.next_u64();
            let (a, b) = ((z % 20) as i8, ((z >> 8) % 20) as i8);
            let (c, d) = (a + ((z >> 16) % 11) as i8 - 5, b + ((z >> 24) % 11) as i8 - 5);
            compare_by_difference(m, a, b, c, d);
            // routines that take side and values from the player itself
            let w = s.next_u64();
            *((j + 0x27) as *mut i8) = [0i8, 1, 2, -1][(w % 4) as usize];
            *((j + 0x19) as *mut i8) = ((w >> 4) % 15) as i8 - 2;
            *((j + 0x102) as *mut i8) = ((w >> 8) % 10) as i8 - 1;
            *((j + 0x103) as *mut i8) = ((w >> 12) % 15) as i8 - 2;
            let o = outro.as_ptr() as usize;
            for (campo, bit) in [(0xf582usize, 20), (0xf586, 22), (0xf58a, 24)] {
                *((m + campo) as *mut usize) = if (w >> bit) & 1 == 1 { j } else { o };
            }
            *((j + 0x19e) as *mut usize) = if (w >> 26) % 4 == 0 { 0 } else { m };
            *((m + 0x8eac) as *mut u8) = (w >> 32) as u8;
            *((m + 0x8ead) as *mut u8) = (w >> 40) as u8;
            compare_player_difference(m, j);
            compare_player_limit(j);
            compare_player_band(j, lado);
            // the ones going through 0x6c38c0 only get side 0 or 1 (index of the side table)
            let lado_real = *((j + 0x27) as *const i8);
            *((j + 0x27) as *mut i8) = ((w >> 16) % 2) as i8;
            compare_player_table(j);
            compare_side_byte(m, j);
            // batch 3 (with side 0 or 1 still holding: 0x6f4600 indexes the side's block).
            // 0x6f4600 always has an engine (with 0x19e zeroed the original reads address 0 and crashes).
            *((j + 0x19e) as *mut usize) = m;
            let u = s.next_u64();
            for k in 0..16 {
                *((m + 0x91e2 + 6371 * ((j + 0x27) as *const u8).read() as usize + 0x5b4 + k) as *mut u8) = (u >> (k * 4)) as u8;
            }
            *((j + 0x108) as *mut u8) = (u >> 3) as u8;
            *((j + 0x10a) as *mut u8) = (u >> 11) as u8;
            *((j + 0x2b) as *mut i8) = ((u >> 19) % 5) as i8 - 1;
            *((m + 0x8eb2) as *mut u8) = ((u >> 22) % 3 == 0) as u8;
            *((m + 0xf57a) as *mut usize) = if (u >> 24) % 5 == 0 { j } else { o };
            *((j + 0x19) as *mut i8) = ((u >> 27) % 11) as i8;
            compare_position_byte(m + 0x91e2, ((u >> 31) % 16) as i8);
            compare_field_by_position(j);
            compare_can_choose(m, if (u >> 33) % 8 == 0 { 0 } else { j }, ((u >> 36) % 10) as i8 - 1, ((u >> 40) % 15) as i8 - 2);
            *((j + 0x27) as *mut i8) = lado_real;
            // direction: neighbouring coordinates (including the same point and the fixed cases), attributes
            let v = s.next_u64();
            let pp = pessoa.as_mut_ptr() as usize;
            *((j + 0x6d) as *mut usize) = pp;
            *((pp + 0x30) as *mut i8) = ((v >> 4) % 21) as i8;
            *((pp + 0x3b) as *mut i8) = ((v >> 9) % 21) as i8;
            *((j + 0x10c) as *mut i8) = ((v >> 14) % 21) as i8;
            *((j + 0x105) as *mut i8) = ((v >> 19) % 21) as i8;
            *((j + 0x19a) as *mut u16) = (v >> 24) as u16;
            let (a, b) = (((v >> 40) % 12) as i8, ((v >> 44) % 12) as i8);
            let (c, d) = if v % 3 == 0 {
                (a, b) // same point
            } else if v % 3 == 1 {
                ([4i8, 4, 3][((v >> 48) % 3) as usize], [0i8, 11, 5][((v >> 50) % 3) as usize])
            } else {
                (a + ((v >> 52) % 11) as i8 - 5, b + ((v >> 56) % 11) as i8 - 5)
            };
            let (a, b) = if v % 3 == 1 { (c, d) } else { (a, b) };
            compare_aim_set_facing_direction(j, a, b, c, d);
            // batch 4 (floating point): values around the ranges, negative and zero
            let q = s.next_u64();
            let faixas = [9500.0f32, 8750.0, 7750.0, 0.0, 10000.0, 100.0];
            let base = faixas[(q % 6) as usize];
            *((j + 0x4d) as *mut f32) = base + ((q >> 8) % 2001) as f32 / 10.0 - 100.0;
            *((j + 0xd5) as *mut f32) = ((q >> 20) % 400) as f32 / 10.0 - 5.0;
            let x = ((q >> 30) % 5000) as f32 / 7.0;
            let codigo = [-1i16, 19, 21, 33, 36, 46, 20, 47, 18, 0, -2, 1000][((q >> 44) % 12) as usize];
            compare_tire_fatigue(j, x, codigo);
            *((j + 0x10c) as *mut i8) = ((q >> 50) % 25) as i8 - 2;
            *((j + 0x19a) as *mut i16) = ((q >> 54) % 400) as i16 - 20;
            compare_facing(j, read_i8(j + 0x102).wrapping_add(((q >> 58) % 11) as i8 - 5), read_i8(j + 0x103).wrapping_add(((q >> 61) % 7) as i8 - 3));
            // batch 5
            let g = s.next_u64();
            let gp = grupo.as_mut_ptr() as usize;
            for (k, jog) in jogadores.iter_mut().enumerate() {
                let jp = jog.as_mut_ptr() as usize;
                *((jp + 0x2b) as *mut i8) = ((g >> (k % 32)) % 4) as i8 - 1;
                *((jp + 0x19) as *mut i8) = ((g >> ((k + 7) % 40)) % 13) as i8 - 2;
                *((gp + (k / 11) * 44 + (k % 11) * 4) as *mut usize) = jp;
            }
            *((gp + 0x58) as *mut i8) = (g % 13) as i8 - 1;
            *((gp + 0x59) as *mut i8) = ((g >> 5) % 13) as i8 - 1;
            let lado_g = ((g >> 10) % 2) as i8;
            let modo = [0i8, 1, 2, 2][((g >> 12) % 4) as usize];
            compare_procurar(m, lado_g, gp, (g >> 14) as u32, (g >> 20) as u32, (g >> 26) as u32, (g >> 32) as u32, modo);
            let h = s.next_u64();
            *((j + 0x27) as *mut i8) = [0i8, 1, 2][(h % 3) as usize];
            *((j + 0x102) as *mut i8) = ((h >> 2) % 9) as i8 - 1;
            *((j + 0x103) as *mut i8) = ((h >> 6) % 14) as i8 - 1;
            compare_reaches(j, ((h >> 10) % 9) as i8 - 1, ((h >> 14) % 14) as i8 - 1);
            *((j + 0x27) as *mut i8) = lado_real;
            // kick: angle by the global table (via fake engine), varied strength and attributes
            *((j + 0x19e) as *mut usize) = m;
            *((j + 0x19a) as *mut i16) = ((h >> 18) % 400) as i16 - 20;
            *((j + 0x2c) as *mut u8) = (h >> 27) as u8;
            *((j + 0xe5) as *mut f32) = ((h >> 30) % 300) as f32 / 7.0;
            let codigo = [0i16, 1, 3, 0x33, 0x5b, 0x40, 100, -1][((h >> 40) % 8) as usize];
            let (a, b) = (((h >> 43) % 12) as i8, ((h >> 47) % 12) as i8);
            let mut saida = 0xffffu16;
            compare_classificar(j, codigo, a, b, a + ((h >> 51) % 11) as i8 - 5, b + ((h >> 55) % 11) as i8 - 5, ((h >> 59) % 32) as f32 / 3.0, &mut saida);
            // batch 6: distances and the two blocks of 20 players in the engine
            let e = s.next_u64();
            for k in 0..40usize {
                let p = Engine(m).player(((k / 20)) as i8, k % 20).addr();
                let z = e.rotate_left(k as u32 * 3);
                *((p + 0x19) as *mut i8) = (z % 13) as i8 - 2;
                *((p + 0x27) as *mut i8) = (k / 20) as i8;
                *((p + 0x102) as *mut i8) = ((z >> 4) % 9) as i8;
                *((p + 0x103) as *mut i8) = ((z >> 8) % 12) as i8;
                *((p + 0x19e) as *mut usize) = m;
            }
            for k in 0..8usize {
                let casa = m + engine::GRID + grid_cell::OCCUPANT_COUNT + ((e >> (k * 7)) % 9700) as usize;
                *(casa as *mut u8) = ((e >> (k * 5)) % 3 == 0) as u8;
            }
            let jj = Engine(m).player((((e >> 50) % 2)) as i8, (((e >> 52) % 20)) as usize).addr();
            *((jj + 0x19) as *mut i8) = 3;
            compare_distance_to_point(jj, ((e >> 40) % 9) as i8, ((e >> 44) % 12) as i8);
            compare_distance_to_goal(jj);
            compare_mais_perto(m, jj);
            // batch 7: grid with valid lists, lines in all directions, pass with RNG
            let w2 = s.next_u64();
            for cel in 0..108usize {
                let g = m + engine::GRID + GRID_CELL_SIZE * cel;
                for l in 0..2usize {
                    *((g + 0x58 + l) as *mut i8) = ((w2.rotate_left((cel * 2 + l) as u32) >> 3) % 4) as i8;
                    for k in 0..11usize {
                        *((g + 44 * l + 4 * k) as *mut usize) = jogadores[(cel + k + l) % 22].as_ptr() as usize;
                    }
                }
            }
            let pt = |v: u64| (((v % 11) as i8) - 1, (((v >> 4) % 14) as i8) - 1);
            let (x1, y1) = pt(w2 >> 8);
            let (x2, y2) = match (w2 >> 16) % 4 {
                0 => (x1, ((w2 >> 20) % 14) as i8 - 1),
                1 => (((w2 >> 20) % 11) as i8 - 1, y1),
                _ => pt(w2 >> 24),
            };
            compare_players_in_line(m, x1, y1, x2, y2);
            let v2 = s.next_u64();
            *((j + 0x19e) as *mut usize) = m;
            *((j + 0x27) as *mut i8) = (v2 % 2) as i8;
            *((j + 0x102) as *mut i8) = ((v2 >> 2) % 9) as i8;
            *((j + 0x103) as *mut i8) = ((v2 >> 6) % 12) as i8;
            *((j + 0x10b) as *mut i8) = ((v2 >> 10) % 21) as i8;
            *((j + 0x19a) as *mut i16) = ((v2 >> 15) % 360) as i16;
            let (a, b) = (((v2 >> 24) % 9) as i8, ((v2 >> 28) % 12) as i8);
            let mut saida = 0u8;
            let ptr = if (v2 >> 32) % 3 == 0 { core::ptr::null_mut() } else { &mut saida as *mut u8 };
            compare_consegue(j, a, b, ((v2 >> 34) % 3 != 0) as i8, ((v2 >> 36) % 40) as f32 / 3.0, ((v2 >> 42) % 30) as f32 / 4.0, ptr);
            compare_in_the_box(j);
            // batch 8: positioning (formations with margin: the index goes negative with position < 0)
            let o2 = s.next_u64();
            for (l, f) in formacoes.iter_mut().enumerate() {
                for (k, b) in f.iter_mut().enumerate() {
                    *b = (o2.rotate_left((k % 64) as u32) as u8) ^ (k as u8).wrapping_mul(37) ^ (l as u8);
                }
                *((m + 0xf56e + 4 * l) as *mut usize) = f.as_ptr() as usize + 1200;
            }
            for lado_m in 0..2usize {
                *((m + 0x9766 + 6371 * lado_m) as *mut u32) = (o2 >> (lado_m * 16)) as u32 & 0x1d000;
                for pos in 0..14usize {
                    let marcas = m + 0x91e2 + 6371 * lado_m + 0x188b + 8 * pos;
                    *(marcas as *mut u64) = o2.rotate_left((pos * 5 + lado_m * 3) as u32) & 0xc00a_0000_c00a_0000;
                }
            }
            *((m + 6) as *mut i8) = ((o2 >> 3) % 2) as i8;
            *((m + 7) as *mut i8) = ((o2 >> 4) % 2) as i8;
            *((m + 0x8ea7) as *mut i8) = ((o2 >> 5) % 12) as i8 - 2;
            *((m + 0x8ea8) as *mut i8) = ((o2 >> 9) % 15) as i8 - 2;
            *((m + 0x8ea9) as *mut i8) = [-1i8, 0, 3][((o2 >> 13) % 3) as usize];
            *((m + 0x8eae) as *mut i8) = ((o2 >> 15) % 3) as i8;
            *((m + 0x8eb1) as *mut i8) = ((o2 >> 17) % 3) as i8 - 1;
            *((m + 0xf57a) as *mut usize) = if (o2 >> 19) % 3 == 0 { 0 } else { jogadores[((o2 >> 21) % 22) as usize].as_ptr() as usize };
            let p2 = s.next_u64();
            *((j + 0x27) as *mut i8) = (p2 % 2) as i8;
            *((j + 0x19) as *mut i8) = ((p2 >> 1) % 13) as i8 - 1;
            *((j + 0x102) as *mut i8) = ((p2 >> 5) % 12) as i8 - 2;
            *((j + 0x103) as *mut i8) = ((p2 >> 9) % 15) as i8 - 2;
            *((j + 0x19e) as *mut usize) = m;
            compare_marcas(m + 0x91e2, ((p2 >> 14) % 14) as i8);
            compare_attacking_role(j);
            compare_defending_role(j);
            let (mut sx, mut sy) = (0x55i8, 0x55i8);
            compare_position_place_players(m, j, &mut sx, &mut sy, 0);
            // batch 9: defensive lines over the two blocks of 20 (already filled above), varied y
            let y2 = s.next_u64();
            for k in 0..40usize {
                let p = Engine(m).player(((k / 20)) as i8, k % 20).addr();
                *((p + 0x103) as *mut i8) = (y2.rotate_left(k as u32 * 2) % 16) as i8 - 2;
                *((p + 0x19) as *mut i8) = (y2.rotate_left(k as u32 * 3 + 1) % 13) as i8 - 2;
            }
            compare_linhas(m);
            // batch 10: player update (all roles, varied attributes and ranges)
            let z = s.next_u64();
            let z2 = s.next_u64();
            // each field: zero with chance 1/2 ("all stopped" combinations), otherwise 1..=24; 1 and 2 up to 100
            for (k, o) in [1usize, 2, 3, 4, 5, 6, 7, 8, 9, 0xb, 0xc, 0xf, 0x10, 0x11].iter().enumerate() {
                let v = z.rotate_left(k as u32 * 5);
                let teto = if *o <= 2 { 100 } else { 24 };
                *((j + o) as *mut u8) = if v & 1 == 0 { 0 } else { (1 + (v >> 1) % teto) as u8 };
            }
            *((j + 0x1f) as *mut i8) = [-1i8, 10, 30, 31, 50][((z2 >> 2) % 5) as usize];
            *((j + 0x2b) as *mut i8) = ((z2 >> 5) % 5) as i8 - 2;
            *((j + 0x1a2) as *mut f32) = ((z2 >> 8) % 140) as f32 + 0.25;
            // word 0x35: half the time exactly at a threshold of the code (or a neighbour)
            const THRESHOLDS: [i16; 14] = [0x1cb5, 0x1cb6, 0x1b57, 0x1676, 0x1675, 0x1a5d, 0x186a, 0x1c52, 0x1c53, 0x1d4c, 0x1d4d, 0x1e46, 0x222e, 0x109a];
            *((j + 0x35) as *mut i16) = if (z2 >> 15) & 1 == 0 {
                THRESHOLDS[((z2 >> 16) % 14) as usize] + ((z2 >> 20) % 3) as i16 - 1
            } else {
                3500 + ((z2 >> 16) % 6500) as i16
            };
            *((pp + 0x5d) as *mut i8) = ((z2 >> 30) % 25) as i8;
            *((j + 0x69) as *mut usize) = pp;
            *((j + 0x27) as *mut i8) = ((z2 >> 35) % 2) as i8;
            *((j + 0x19) as *mut i8) = ((z2 >> 36) % 11) as i8;
            *((j + 0x19e) as *mut usize) = if (z2 >> 40) % 9 == 0 { 0 } else { m };
            *((m + 0xf5bd) as *mut i8) = ((z2 >> 42) % 5) as i8 - 1;
            *((m + 0xf5f3) as *mut i8) = ((z2 >> 45) % 5) as i8 - 1;
            *((m + 0xf5bf) as *mut u8) = ((z2 >> 48) % 4) as u8;
            *((m + 0xf5bf + 54) as *mut u8) = ((z2 >> 50) % 4) as u8;
            *((m + 0x8ed0) as *mut i16) = ((z2 >> 52) % 900) as i16 - 100;
            compare_atualizar(j);
            *((j + 0x19e) as *mut usize) = m;
            // batch 11
            let q1 = s.next_u64();
            let q2 = s.next_u64();
            *((j + 0x4d) as *mut f32) = ((q1 >> 2) % 12000) as f32 / 1.5;
            for o in 0x10eusize..=0x117 {
                *((j + o) as *mut i8) = (q1.rotate_left(o as u32) % 30) as i8 - 3;
            }
            compare_draw_moment(j);
            // tables with several 1s (position 1 and substitute), blocks with positions, substitutes and 0x20
            for t in tabelas.iter_mut() {
                for (k, v) in t.iter_mut().enumerate() {
                    if (q2 >> (k % 50)) & 3 == 0 {
                        *v = 1;
                    }
                }
            }
            for k in 0..40usize {
                let p = Engine(m).player(((k / 20)) as i8, k % 20).addr();
                let r = q2.rotate_left(k as u32 * 3);
                *((p + 0x19) as *mut i8) = [(r % 13) as i8 - 2, -2, -1, 0][((r >> 8) % 4) as usize];
                *((p + 0x1a) as *mut i8) = (r >> 12) as i8 % 12;
                *((p + 0x20) as *mut i8) = if (r >> 20) & 1 == 0 { -1 } else { 0 };
                *((p + 0x102) as *mut i8) = ((r >> 24) % 10) as i8 - 1;
                *((p + 0x103) as *mut i8) = ((r >> 28) % 13) as i8 - 1;
            }
            let algum = |sh: u32| Engine(m).player((((q2 >> sh) % 2)) as i8, (((q2 >> (sh + 1)) % 20)) as usize).addr();
            *((m + 0xf59e) as *mut usize) = if (q2 >> 40) % 3 == 0 { 0 } else { algum(41) };
            *((m + 0xf5a2) as *mut usize) = if (q2 >> 47) % 3 == 0 { 0 } else { algum(48) };
            compare_hold_position_one(m, ((q2 >> 55) % 3) as i8);
            let jj = algum(57);
            compare_caminho(m, jj, ((q2 >> 63) & 1) as i8);
            compare_espaco(m, jj);
            compare_off_centre(((q1 >> 40) % 12) as i8 - 2);
            compare_alem(m, ((q1 >> 44) % 14) as i8 - 1, ((q1 >> 48) % 3) as i8, ((q1 >> 50) & 1) as i8);
            // batch 12
            let g2 = s.next_u64();
            compare_ler_8ea7(m);
            compare_ler_8ea8(m);
            compare_ler_8ea9(m);
            compare_ler_8eaa(m);
            compare_ler_8eb2(m);
            compare_gravar_8ea9(m, g2 as u8);
            compare_gravar_8ea7(m, (g2 >> 8) as u8);
            compare_gravar_8ea8(m, (g2 >> 16) as u8);
            compare_gravar_8eaa(m, (g2 >> 24) as u8);
            compare_write_8ed8(m, (g2 >> 32) as u16);
            *((m + 0x8f7a) as *mut u32) = (g2 >> 3) as u32;
            *((m + 0x8f7e) as *mut u8) = (g2 >> 11) as u8;
            compare_read_8f7a(m);
            *((m + 0x8d46) as *mut i8) = ((g2 >> 40) % 30) as i8 - 5;
            *((m + 0x8d47) as *mut i8) = ((g2 >> 45) % 30) as i8 - 5;
            compare_posse(m, [-1i8, 0, 1, 2][((g2 >> 50) % 4) as usize]);
            *((j + 0x101) as *mut i8) = ((g2 >> 52) % 8) as i8 - 1;
            for k in 0..8usize {
                *((j + 0x1ae + 4 * k) as *mut usize) = (g2 >> (k * 3)) as usize % 5;
            }
            compare_in_list(j, ((g2 >> 56) % 6) as usize);
            for k in 0..4usize {
                *((m + 0xf58e + 4 * k) as *mut usize) = jogadores[k].as_ptr() as usize;
            }
            compare_extremos(m);
            // batch 13 (blocks already filled; some 0x2b zeroed; grid with counts)
            let h3 = s.next_u64();
            for k in 0..40usize {
                let p = Engine(m).player(((k / 20)) as i8, k % 20).addr();
                if (h3 >> (k % 60)) & 1 == 0 {
                    *((p + 0x2b) as *mut i8) = 0;
                }
            }
            for cel in 0..108usize {
                for l in 0..2usize {
                    // almost empty (1 in 8), so the pressure sum falls near the threshold (30..44)
                    let v = h3.rotate_left((cel * 2 + l) as u32);
                    *((m + engine::GRID + GRID_CELL_SIZE * cel + grid_cell::OCCUPANT_COUNT + l) as *mut i8) = if v % 8 == 0 { 1 + ((v >> 3) % 2) as i8 } else if v % 31 == 1 { -1 } else { 0 };
                }
            }
            compare_count_active(m, ((h3 >> 3) % 2) as i8);
            compare_special_code(((h3 >> 5) % 110) as i8 - 5);
            let jj3 = Engine(m).player((((h3 >> 12) % 2)) as i8, (((h3 >> 13) % 20)) as usize).addr();
            compare_atras(m, jj3);
            let mut alvo = -2i32;
            *((m + 0x4792) as *mut usize) = if (h3 >> 18) & 1 == 0 { &mut alvo as *mut i32 as usize } else { m + 0x10 };
            compare_no_match(m);
            compare_pressao_area(m, ((h3 >> 19) % 2) as i8);
            let (a3, b3) = (((h3 >> 20) % 10) as i8 - 1, ((h3 >> 24) % 13) as i8 - 1);
            compare_nearest_to_point(m, jj3, a3, b3);
            compare_nearest_to_point(m, jj3, read_i8(jj3 + 0x102), read_i8(jj3 + 0x103));
            compare_pressao_ponto(m, ((h3 >> 28) % 2) as i8, a3, b3, ((h3 >> 29) & 1) as i8);
            compare_smallest_distance(m, ((h3 >> 30) % 2) as i8, a3, b3, ((h3 >> 31) & 1) as i8);
            // batch 14 (tables with varied role bits, including none and 0x700)
            let h4 = s.next_u64();
            for t in tabelas.iter_mut() {
                for (k, v) in t.iter_mut().enumerate() {
                    let r = h4.rotate_left(k as u32 * 7) as u16;
                    *v = [0u16, 1, 2, 4, 8, 0x10, 0x20, 0x40, 0x140, 0x240, 0x440, 0x46, r][(r % 13) as usize];
                }
            }
            let delta = ((h4 >> 20) % 4000) as i16 - 2000;
            compare_split_a(m, ((h4 >> 32) % 2) as i8, delta);
            compare_split_b(m, ((h4 >> 33) % 2) as i8, delta);
            compare_split_c(m, ((h4 >> 34) % 2) as i8, delta);
            let alto = [0u32, 0x3f800000, 0x40000000, 0x12345600, 0xffffff00][((h4 >> 36) % 5) as usize];
            compare_escolher(m, (alto & !0xff) | ((h4 >> 40) % 2) as u32);
            // batch 15 (accessors; range extremes and sides outside 0/1)
            let h5 = s.next_u64();
            let h6 = s.next_u64();
            *((j + 0x19a) as *mut u16) = h5 as u16;
            compare_direction(j);
            *((m + 0xf59e) as *mut usize) = if h6 & 1 == 0 { j } else { o };
            *((m + 0xf5a2) as *mut usize) = if h6 & 2 == 0 { j } else { o };
            compare_clear_stored(m, [0u8, 1, 2, 255][((h5 >> 16) % 4) as usize]);
            *((j + 0x102) as *mut i8) = if (h5 >> 18) & 1 == 0 { ((h5 >> 19) % 11) as i8 - 1 } else { (h5 >> 19) as i8 };
            compare_x_fora(j);
            *((j + 0x19e) as *mut usize) = m;
            *((m + 0xf582) as *mut usize) = if (h5 >> 27) & 1 == 0 { j } else { o };
            compare_is_the_stored(j);
            *((m + 0x475a) as *mut u8) = if (h5 >> 28) % 3 == 0 { 0 } else { (h5 >> 30) as u8 | 1 };
            *((m + 0x4761) as *mut usize) = if (h5 >> 38) % 4 == 0 { 0 } else { j };
            *((j + 0x27) as *mut i8) = [0i8, 1, 2, -1][((h5 >> 40) % 4) as usize];
            compare_side_of_move(m);
            compare_side_state(j);
            compare_opponent_state(j);
            *((j + 0x103) as *mut i8) = if (h5 >> 42) & 1 == 0 { ((h5 >> 43) % 14) as i8 - 1 } else { (h5 >> 43) as i8 };
            compare_near_goal(j);
            compare_player_in_block(m, (h6 >> 3) as i8, (h6 >> 11) as i8);
            *((j + 0x27) as *mut i8) = ((h5 >> 51) % 2) as i8;
            *((j + 0x1a) as *mut i8) = if (h5 >> 52) & 1 == 0 { ((h5 >> 53) % 14) as i8 - 2 } else { (h5 >> 53) as i8 };
            compare_table_a_1a(m, j);
            *((m + 0x8ed4) as *mut u16) = [0x1efu16, 0x3de, 0, h6 as u16][((h6 >> 20) % 4) as usize];
            *((j + 0x38) as *mut u8) = (h6 >> 22) as u8;
            *((m + 0x4792) as *mut usize) = j;
            compare_marked_moment(m);
            let v19c = [200i16, 201, -200, -201, i16::MIN, i16::MAX, 0, (h6 >> 30) as i16 % 400][((h6 >> 46) % 8) as usize];
            compare_write_19c(j, v19c);
            compare_set_piece_code([12i16, 13, 21, 37, 46, 59, (h6 >> 49) as i16 % 70, (h6 >> 32) as i16][((h6 >> 61) % 8) as usize]);
            for o2 in 0..16 {
                *((m + 0x475a + o2) as *mut u8) = (h6 >> (o2 * 4)) as u8;
            }
            compare_clear_move(m);
            for k in 0..40usize {
                let p = Engine(m).player(((k / 20)) as i8, k % 20).addr();
                *((p + 0x19) as *mut i8) = (h6.rotate_left(k as u32) % 12) as i8 - 1;
            }
            compare_add_disposition(m, ((h5 >> 60) % 2) as i8, (h6 >> 7) as i16);
            // batch 16 (role tables of the 14th batch still holding)
            let h7 = s.next_u64();
            let h8 = s.next_u64();
            let delta7 = ((h7 >> 20) % 4000) as i16 - 2000;
            compare_split_d(m, (h7 % 2) as i8, delta7);
            compare_split_e(m, ((h7 >> 1) % 2) as i8, delta7);
            compare_split_f(m, ((h7 >> 2) % 2) as i8, delta7);
            // guard by the player (0x4761), not by the other (0x4765): the two vary independently
            *((m + 0x4761) as *mut usize) = if (h7 >> 3) % 3 == 0 { j } else { 0 };
            *((m + 0x4765) as *mut usize) = if (h7 >> 4) % 2 == 0 { o } else { 0 };
            compare_record_move(m, (h8 >> 1) as u16, if (h8 >> 17) % 4 == 0 { 0 } else { j as u32 }, (h8 >> 19) as u32, (h8 >> 2) as u8, (h8 >> 10) as u8, (h8 >> 18) as u8, (h8 >> 26) as u8, (h8 >> 34) as u8, (h8 >> 42) as u8);
            *((j + 0x101) as *mut i8) = ((h7 >> 5) % 6) as i8 - 1;
            for k in 0..4usize {
                *((j + 0x1ae + 4 * k) as *mut usize) = (h7 >> (8 + k * 3)) as usize % 5;
            }
            compare_remove_from_list(j, ((h7 >> 24) % 6) as usize);
            *((j + 0x27) as *mut i8) = [0i8, 1, 2, -1][((h7 >> 28) % 4) as usize];
            *((m as isize + 0x4592 + read_i8(j + 0x27) as isize) as *mut i8) = ((h7 >> 30) % 14) as i8 - 2;
            compare_take_the_field(m, j);
            *((j + 0x27) as *mut i8) = [0i8, 1, 2, -1][((h7 >> 34) % 4) as usize];
            *((j + 0x19e) as *mut usize) = m;
            *((j + 0x102) as *mut i8) = ((h7 >> 36) % 11) as i8 - 1;
            *((j + 0x103) as *mut i8) = ((h7 >> 40) % 14) as i8 - 1;
            compare_in_danger(j);
            // batch 17
            let h9 = s.next_u64();
            let h10 = s.next_u64();
            *((m + 0x475a) as *mut i8) = [0i8, 8, 1, 5][(h9 % 4) as usize];
            *((m + 0x4761) as *mut usize) = if (h9 >> 2) % 5 == 0 { 0 } else { j };
            *((j + 0x27) as *mut i8) = [0i8, 1, 2][((h9 >> 5) % 3) as usize];
            *((m + 0x8eae) as *mut i8) = [0i8, 1, 2][((h9 >> 7) % 3) as usize];
            *((m + 0x475b) as *mut i8) = ((h9 >> 9) % 10) as i8 - 1;
            *((m + 0x475c) as *mut i8) = ((h9 >> 13) % 14) as i8 - 1;
            compare_check_move(m);
            for k in 0..40usize {
                let p = Engine(m).player(((k / 20)) as i8, k % 20).addr();
                let r = h10.rotate_left(k as u32 * 5);
                *((p + 0x19) as *mut i8) = (r % 12) as i8 - 1;
                *((p + 0x102) as *mut i8) = ((r >> 4) % 9) as i8;
                *((p + 0x103) as *mut i8) = ((r >> 8) % 12) as i8;
            }
            let (x9, y9) = (((h9 >> 20) % 11) as i8 - 1, ((h9 >> 24) % 15) as i8 - 2);
            compare_clear_path(m, [0i8, 1, 2, -1][((h9 >> 28) % 4) as usize], x9, y9);
            let pp9 = Engine(m).player((((h9 >> 30) % 2)) as i8, (((h9 >> 31) % 20)) as usize).addr();
            compare_clear_path(m, ((h9 >> 36) % 2) as i8, read_i8(pp9 + 0x102), read_i8(pp9 + 0x103));
            *((j + 0x19e) as *mut usize) = m;
            *((j + 0x27) as *mut i8) = [0i8, 1, 2, -1][((h9 >> 37) % 4) as usize];
            *((j + 0x102) as *mut i8) = ((h9 >> 39) % 9) as i8;
            *((j + 0x103) as *mut i8) = ((h9 >> 43) % 12) as i8;
            *((j + 0x10c) as *mut i8) = ((h9 >> 47) % 25) as i8 - 2;
            let (xa, ya) = if (h9 >> 52) % 6 == 0 { (read_i8(j + 0x102), read_i8(j + 0x103)) } else { (((h9 >> 55) % 9) as i8, ((h9 >> 59) % 12) as i8) };
            if (h9 >> 63) == 1 {
                // at the exact threshold: 0x10c + 10 equal to the difference of the words
                let fundo9: i8 = if read_i8(j + 0x27) != 0 { 11 } else { 0 };
                let dif = (by_difference(m, xa, ya, 4, fundo9) as i16 as i32 - by_difference(m, xa, ya, read_i8(j + 0x102), read_i8(j + 0x103)) as i16 as i32).abs() - 10;
                if (-128..=127).contains(&dif) {
                    *((j + 0x10c) as *mut i8) = dif as i8;
                }
            }
            compare_reaches_the_point(j, xa, ya);
            // batch 18 (own records and persons; 0x69 and 0x6d always valid in the block)
            let h11 = s.next_u64();
            let h12 = s.next_u64();
            for (k, f) in fichas.iter_mut().enumerate() {
                *f = (h11.rotate_left(k as u32) as u32) ^ k as u32;
            }
            for (k, pe) in pessoas.iter_mut().enumerate() {
                for o in [0x39usize, 0x3e, 0x44] {
                    pe[o] = (h12.rotate_left((k * 3 + o) as u32) % 23) as u8;
                }
            }
            for k in 0..40usize {
                let p = Engine(m).player(((k / 20)) as i8, k % 20).addr();
                let r = h11.rotate_left(k as u32 * 3) ^ h12.rotate_right(k as u32);
                *((p + 0x19) as *mut i8) = (r % 13) as i8 - 2;
                *((p + 0x20) as *mut u8) = if (r >> 4) % 3 == 0 { 0xff } else { (r >> 6) as u8 };
                *((p + 0x35) as *mut i16) = if (r >> 9) % 4 == 0 { (r >> 12) as i16 } else { ((r >> 12) % 400) as i16 - 100 };
                for (o, sh) in [(0x03usize, 28), (0x06, 33), (0x0c, 38), (0x10, 43)] {
                    *((p + o) as *mut u8) = if (r >> 48) % 4 == 0 { (r >> sh) as u8 } else { ((r >> sh) % 21) as u8 };
                }
                *((p + 0x69) as *mut usize) = fichas.as_ptr().add(k) as usize;
                *((p + 0x6d) as *mut usize) = if (r >> 50) % 5 == 0 { 0 } else { pessoas[k % 4].as_ptr() as usize };
            }
            let partida11 = jogadores[0].as_mut_ptr() as usize;
            *((partida11 + 0x4d) as *mut u8) = (((h11 >> 40) % 10) as u8) | (((h11 >> 44) as u8) & 0xf0);
            *((m + 0x4792) as *mut usize) = partida11;
            compare_choose_standout(m);
            let alto11 = [0u32, 0x3f800000, 0x40000000, 0x12345600, 0xffffff00][((h11 >> 50) % 5) as usize];
            compare_choose_by_role_b(m, (alto11 & !0xff) | ((h11 >> 53) % 2) as u32);
            *((m + 0x8eae) as *mut i8) = [0i8, 1, 2, -1][((h12 >> 54) % 4) as usize];
            *((j + 0x27) as *mut i8) = ((h12 >> 56) % 2) as i8;
            *((j + 0x19) as *mut i8) = ((h12 >> 57) % 17) as i8 - 2;
            let (mut fx, mut fy) = (0u8, 0u8);
            compare_formation_square(m, j, &mut fx, &mut fy);
            // batch 19: counts (zeros, negatives and sums that overflow) and attributes
            let h13 = s.next_u64();
            let h14 = s.next_u64();
            for k in 0..30usize {
                let r = h13.rotate_left(k as u32 * 2) ^ h14.rotate_right(k as u32);
                let v = match r % 6 {
                    0 => 0,
                    1 => -(((r >> 8) % 50) as i32),
                    2 => (r >> 8) as i32,
                    _ => ((r >> 8) % 300) as i32,
                };
                *((m + 0x8f02 + 4 * k) as *mut i32) = v;
            }
            compare_compute_percentages(m, ((h13 >> 3) % 6) as i8 - 1);
            for k in 0..40usize {
                let p = Engine(m).player(((k / 20)) as i8, k % 20).addr();
                for (n, o) in [0xc9usize, 0xa1, 0x9d, 0x95, 0xb1, 0x7d, 0x99, 0xd9, 0xad, 0xa9].iter().enumerate() {
                    let r = h14.rotate_left((k * 10 + n) as u32 % 64);
                    *((p + o) as *mut f32) = (r % 3000) as f32 / 100.0 - if r % 17 == 0 { 40.0 } else { 0.0 };
                }
            }
            for (k, pe) in pessoas.iter_mut().enumerate() {
                let r = h13.rotate_left(k as u32 * 9);
                let w = [0x8c, 0x8d, 0x3c, 0x3b, (r >> 8) % 200, (r >> 16) % 400][(r % 6) as usize] as i16;
                pe[5..7].copy_from_slice(&w.to_le_bytes());
                pe[0x21] = ((r >> 24) % 25) as u8;
            }
            let alto13 = [0u32, 0x3f800000, 0x40000000, 0x12345600, 0xffffff00][((h13 >> 50) % 5) as usize];
            // batch 20: condition 0x1a2, record 0x69 (person with 0x5d) or none, byte 0xff, 0x1a6
            let h15 = s.next_u64();
            let h16 = s.next_u64();
            for (k, pe) in pessoas.iter_mut().enumerate() {
                pe[0x5d] = (h15.rotate_left(k as u32 * 11) % 34) as u8;
            }
            for k in 0..40usize {
                let p = Engine(m).player(((k / 20)) as i8, k % 20).addr();
                let r = h15.rotate_left(k as u32 * 3) ^ h16.rotate_right(k as u32 * 5);
                *((p + 0x1a2) as *mut f32) = [100.0f32, 0.0, 50.0, 99.5][(r % 4) as usize] + ((r >> 2) % 400) as f32 / 8.0 - 25.0;
                *((p + 0x69) as *mut usize) = if (r >> 12) % 5 == 0 { 0 } else { pessoas[k % 4].as_ptr() as usize };
                *((p + 0xff) as *mut i8) = ((r >> 15) % 40) as i8 - 8;
                *((p + 0x1a6) as *mut usize) = if (r >> 21) % 2 == 0 { j } else { 0 };
            }
            let pk = Engine(m).player((((h16 >> 30) % 2)) as i8, (((h16 >> 31) % 20)) as usize).addr();
            *((pk + 0x69) as *mut usize) = pessoas[0].as_ptr() as usize;
            let fat = [1.0f32, 0.0, 0.5, 1.5, 2.0, 0.95][((h15 >> 40) % 6) as usize] + ((h15 >> 43) % 9) as f32 / 64.0;
            compare_apply_factor(pk, fat);
            compare_value_by_condition(pk, [0usize, j, o][((h15 >> 47) % 3) as usize]);
            compare_value_by_condition(pk, *((pk + 0x1a6) as *const usize));
            let jt = if (h16 >> 36) % 2 == 0 { pk } else { j };
            *((j + 0x27) as *mut i8) = ((h16 >> 37) % 2) as i8;
            compare_apply_to_team(m, jt, fat);
            compare_apply_to_side(m, ((h16 >> 38) % 2) as i8, fat);
            compare_choose_by_condition(m, ((h16 >> 39) % 2) as i8, [2u32, 0, 1, 0x102, (h16 >> 40) as u32][((h16 >> 44) % 5) as usize]);
            // batch 21
            let h17 = s.next_u64();
            let h18 = s.next_u64();
            *((j + 0x1bc) as *mut u16) = h17 as u16;
            compare_read_1bc(j);
            *((j + 0x2b) as *mut u8) = (h17 >> 16) as u8;
            compare_add_2b(j, (h17 >> 24) as u8);
            compare_write_2b(j, (h17 >> 32) as u8);
            *((j + 0x27) as *mut i8) = ((h17 >> 40) % 2) as i8;
            *((j + 0x19) as *mut i8) = ((h17 >> 41) % 15) as i8 - 2;
            *((j + 0x1a) as *mut i8) = ((h17 >> 45) % 15) as i8 - 2;
            *((j + 0x103) as *mut i8) = ((h17 >> 49) % 13) as i8 - 1;
            *((m + 0xf582) as *mut usize) = if (h17 >> 53) & 1 == 0 { j } else { o };
            *((j + 0x19e) as *mut usize) = if (h17 >> 54) % 4 == 0 { 0 } else { m };
            compare_player_table_a_1a(j);
            compare_player_table_b(j);
            compare_difference_of_player(j);
            compare_role_880(j);
            *((j + 0x19e) as *mut usize) = m;
            for t in tabelas.iter_mut() {
                for (k, v) in t.iter_mut().enumerate() {
                    *v = [0x80u16, 0x800, 0x880, 0, (h18 >> k) as u16][((h18 >> (k * 3)) % 5) as usize];
                }
            }
            compare_role_880(j);
            *((j + 0x19) as *mut i8) = ((h17 >> 56) % 14) as i8 - 1;
            *((m + 0x9766 + 6371 * read_i8(j + 0x27) as usize) as *mut u32) = (h18 as u32) & 0x1801000;
            *((m + 6 + read_i8(j + 0x27) as usize) as *mut u8) = (h18 >> 32) as u8;
            compare_player_marks_rot(j);
            compare_mark_800000(j);
            *((j + 0x27) as *mut i8) = [0i8, 1, 2, -1][((h18 >> 40) % 4) as usize];
            *((m + 0x8ea8) as *mut i8) = ((h18 >> 42) % 14) as i8 - 1;
            compare_near_own_goal(j);
            *((m + 0x1a7) as *mut u8) = (h18 >> 8) as u8 | 1;
            compare_release_ball(j);
            let dono = jogadores[1].as_mut_ptr() as usize;
            *((dono + 0x23) as *mut usize) = Engine(m).player(((h18 >> 46) % 2) as i8, 0).addr();
            let alvo21 = if (h18 >> 47) % 4 == 0 { 0x1234 } else { *((Engine(m).player(((h18 >> 46) % 2) as i8, ((h18 >> 49) % 20) as usize).addr() + 0x69) as *const usize) };
            compare_player_by_record(dono, alvo21);
            let cod = [0x1f40u32 - 1, 0x1f40, 0x1f40 + 0x2a4, 0x1f40 + 0x2a5, 0x1f40 + ((h18 >> 52) % 0x2a5) as u32, (h18 >> 20) as u32][((h18 >> 61) % 6) as usize];
            // the table (0xb68788) is in the uninitialised area and is zeroed at start: temporary values
            let ic = ((cod & 0x7fff) as i32 - 0x1f40) as i16;
            let alvo_t = (0xb68788 + 18 * ic.clamp(0, 0x2a5) as usize) as *mut u8;
            let antes = *alvo_t;
            *alvo_t = (h17 >> 8) as u8 | 1;
            compare_code_byte(cod | (((h18 >> 15) as u32 & 1) << 15) | ((h17 as u32) & 0xffff0000));
            *alvo_t = antes;
            // batch 22
            let h19 = s.next_u64();
            let h20 = s.next_u64();
            for t in tabelas.iter_mut() {
                for (k, v) in t.iter_mut().enumerate() {
                    *v = [0u16, 1, 0x80, 0x100, 0x200, 0x400, 0x800, 0x880, (h20 >> k) as u16][((h20 >> (k * 4)) % 9) as usize];
                }
            }
            *((j + 0x19e) as *mut usize) = if (h19 >> 60) % 6 == 0 { 0 } else { m };
            *((j + 0x27) as *mut i8) = ((h19 >> 2) % 2) as i8;
            *((j + 0x19) as *mut i8) = ((h19 >> 3) % 11) as i8;
            *((j + 0x10a) as *mut i8) = ((h19 >> 7) % 14) as i8 - 2;
            let pe22 = pessoas[0].as_mut_ptr();
            *((j + 0x6d) as *mut usize) = pe22 as usize;
            for (o, sh) in [(0x17usize, 11), (0x18, 16), (0x19, 21)] {
                *pe22.add(o) = ((h19 >> sh) % 45) as u8;
            }
            compare_adjusted_rating(j, ((h19 >> 26) % 30) as i8 - 10);
            *((j + 0x19e) as *mut usize) = m;
            *((m + 0x477a) as *mut i32) = if (h19 >> 31) % 2 == 0 { -1 } else { (h19 >> 32) as i32 & 0xffff };
            *((j + 0x35) as *mut i16) = [7500i16, 7501, 7499, 9000, 3000, -5, (h20 >> 20) as i16][((h19 >> 34) % 7) as usize];
            *((j + 0x17) as *mut i8) = ((h19 >> 37) % 26) as i8 - 3;
            compare_adjust_17(j, if (h19 >> 42) % 4 == 0 { 0 } else { ((h19 >> 44) % 25) as i8 - 12 });
            // batch 23: possession (previous owner, pass from the same side or not, zones, role 1)
            let h21 = s.next_u64();
            let h22 = s.next_u64();
            for t in tabelas.iter_mut() {
                for (k, v) in t.iter_mut().enumerate() {
                    *v = if (h22 >> k) & 1 == 0 { 1 } else { (h22 >> (k + 11)) as u16 & 0xfe };
                }
            }
            let o23 = outro.as_ptr() as usize;
            *((o23 + 0x19e) as *mut usize) = m;
            *((j + 0x19e) as *mut usize) = m;
            // side only 0, 1 or 2: with -1 the original's role table reads before the engine and crashes
            *((j + 0x27) as *mut i8) = [0i8, 1, 0, 1, 2][((h21 >> 1) % 5) as usize];
            *((j + 0x19) as *mut i8) = ((h21 >> 4) % 11) as i8;
            *((j + 0x102) as *mut i8) = ((h21 >> 8) % 10) as i8 - 1;
            *((j + 0x103) as *mut i8) = ((h21 >> 12) % 14) as i8 - 1;
            *((j + 0x19a) as *mut u16) = (h21 >> 16) as u16;
            let pb = Engine(m).player(((h21 >> 32) % 2) as i8, ((h21 >> 33) % 20) as usize).addr();
            *((m + 0xf582) as *mut usize) = [0, o23, pb, j][((h21 >> 38) % 4) as usize];
            let ps = Engine(m).player(((h21 >> 40) % 2) as i8, ((h21 >> 41) % 20) as usize).addr();
            *((ps + 0x19e) as *mut usize) = m;
            *((pb + 0x19e) as *mut usize) = m;
            *((m + 0xf576) as *mut usize) = if (h21 >> 46) % 4 == 0 { 0 } else { ps };
            for (k, v) in [0x1a4usize, 0x1a5, 0x1a6, 0x1a7, 0x1cd, 0x8ea9, 0x8eab, 0x8eb1, 0x8eaa, 0x8d46, 0x8d47].iter().enumerate() {
                let r = (h22 >> (k * 5)) as u8;
                *((m + v) as *mut u8) = match r % 4 { 0 => 0, 1 => 1, 2 => 0xff, _ => r };
            }
            *((m + 0x4782) as *mut u32) = (h22 >> 13) as u32;
            compare_take_ball(j);
            // batch 24
            let h23 = s.next_u64();
            let h24 = s.next_u64();
            let arb = jogadores[2].as_mut_ptr() as usize;
            let pe24 = pessoas[1].as_mut_ptr();
            *(pe24.add(0x1e) as *mut i16) = [0i16, 151, 150, 349, 350, -3, ((h23 >> 8) % 420) as i16][(h23 % 7) as usize];
            *pe24.add(0x28) = [0u8, 0xf1, 0xf0, 2, (h23 >> 16) as u8 % 20][((h23 >> 24) % 5) as usize];
            compare_init_strictness(arb, m as u32, if (h23 >> 27) % 4 == 0 { 0 } else { pe24 as usize });
            *((arb + 4) as *mut usize) = if (h23 >> 29) % 4 == 0 { 0 } else { pe24 as usize };
            // kick: player of the move, ball owner and the line
            let jl = Engine(m).player(((h23 >> 31) % 2) as i8, ((h23 >> 32) % 20) as usize).addr();
            let dn = Engine(m).player(((h23 >> 37) % 2) as i8, ((h23 >> 38) % 20) as usize).addr();
            *((m + 0x475a) as *mut u8) = if (h23 >> 43) % 5 == 0 { 0 } else { 3 };
            *((m + 0x4761) as *mut usize) = if (h23 >> 45) % 6 == 0 { 0 } else { jl };
            *((m + 0xf582) as *mut usize) = if (h23 >> 47) % 6 == 0 { 0 } else { dn };
            *((m + 0x8ea8) as *mut i8) = ((h23 >> 49) % 13) as i8 - 1;
            *((m + 0x475c) as *mut i8) = ((h23 >> 53) % 13) as i8 - 1;
            compare_move_counts(m);
            *((m + 0x475a) as *mut u8) = if (h24 >> 3) % 3 == 0 { 3 } else { 0 };
            *((m + 0x8ea9) as *mut i8) = ((h24 >> 5) % 5) as i8 - 1;
            let jf = if (h24 >> 8) % 2 == 0 { jl } else { dn };
            *((jf + 0x14) as *mut u8) = [(h24 >> 9) as u8 % 25, 200, 255][((h24 >> 60) % 3) as usize];
            *((jf + 0x102) as *mut i8) = ((h24 >> 17) % 10) as i8 - 1;
            *((jf + 0x103) as *mut i8) = [0i8, 1, 10, 11, ((h24 >> 21) % 12) as i8][((h24 >> 25) % 5) as usize];
            *((arb + 8) as *mut u16) = (h24 >> 28) as u16;
            let (mut t24, mut v24) = (0u8, 0u8);
            compare_decide_foul(arb, jf, if (h24 >> 44) % 2 == 0 { 0x25 } else { (h24 >> 45) as u16 % 64 }, &mut t24, &mut v24);
            // batch 25: cards (type 1 mostly; 7 and others too)
            let h25 = s.next_u64();
            let h26 = s.next_u64();
            let jc = Engine(m).player(((h25 >> 1) % 2) as i8, ((h25 >> 2) % 20) as usize).addr();
            let oc = Engine(m).player(((h25 >> 7) % 2) as i8, ((h25 >> 8) % 20) as usize).addr();
            *((jc + 0x24) as *mut u8) = [0u8, 0, 1, 2][((h25 >> 13) % 4) as usize];
            *((jc + 0x102) as *mut i8) = ((h25 >> 15) % 10) as i8 - 1;
            *((jc + 0x103) as *mut i8) = [0i8, 1, 10, 11, ((h25 >> 19) % 12) as i8][((h25 >> 23) % 5) as usize];
            *((m + 0x8eb3) as *mut u8) = ((h25 >> 26) % 6 == 0) as u8;
            *((m + 0x8ed0) as *mut i16) = [0i16, 495, 496, 506, 1000, -600, (h26 >> 3) as i16][((h25 >> 29) % 7) as usize];
            let mut tc = [1u8, 1, 1, 7, 2, 0][((h25 >> 32) % 6) as usize];
            let mut pc = 0f32;
            let sev = ((h25 >> 35) % 30) as i8 - 5;
            let flag = ((h25 >> 40) % 3 == 0) as u8 | (((h25 >> 41) as u8) & 0xfe & if (h25 >> 49) % 5 == 0 { 0xff } else { 0 });
            compare_punish_foul(arb, jc, &mut tc, ((h25 >> 42) % 90) as i8, sev, flag, &mut pc);
            let mut tc2 = [1u8, 1, 1, 7, 2, 0][((h26 >> 32) % 6) as usize];
            *((jc + 0x19) as *mut i8) = if (h26 >> 36) % 7 == 0 { -1 } else { ((h26 >> 37) % 11) as i8 };
            compare_punish_move(arb, jc, oc, &mut tc2, ((h26 >> 41) % 90) as i8, ((h26 >> 48) % 30) as i8 - 5, ((h26 >> 54) % 3 == 0) as u8, &mut pc, ((h26 >> 56) % 2) as u8);
            // batch 26: the referee inside the engine (0xf5b2: engine, person, value)
            let h27 = s.next_u64();
            *((m + 0xf5b2) as *mut usize) = m;
            *((m + 0xf5b6) as *mut usize) = if h27 % 4 == 0 { 0 } else { pe24 as usize };
            *((m + 0xf5ba) as *mut u16) = (h27 >> 8) as u16;
            let (mut t26, mut v26) = (0u8, 0u8);
            compare_engine_decide_foul(m, jf, if (h27 >> 24) % 2 == 0 { 0x25 } else { (h27 >> 25) as u16 % 64 }, &mut t26, &mut v26);
            let mut tc3 = [1u8, 1, 7, 2][((h27 >> 32) % 4) as usize];
            let mut pc3 = 0f32;
            compare_engine_punish_foul(m, jc, &mut tc3, ((h27 >> 34) % 90) as i8, ((h27 >> 41) % 30) as i8 - 5, ((h27 >> 46) % 3 == 0) as u8, &mut pc3);
            let mut tc4 = [1u8, 1, 7, 2][((h27 >> 48) % 4) as usize];
            compare_engine_punish_move(m, jc, oc, &mut tc4, ((h27 >> 50) % 90) as i8, ((h27 >> 56) % 30) as i8 - 5, ((h27 >> 61) % 3 == 0) as u8, &mut pc3, ((h27 >> 63) & 1) as u8);
            // batch 27: offside (lines 0x8eaf/0x8eb0, attackers close to them)
            let h28 = s.next_u64();
            let h29 = s.next_u64();
            let l27 = [0i8, 1, 0, 1, 2][(h28 % 5) as usize];
            let (lin0, lin1) = (((h28 >> 3) % 8) as i8 + 2, ((h28 >> 6) % 8) as i8 + 2);
            *((m + 0x8eaf) as *mut i8) = lin0;
            *((m + 0x8eb0) as *mut i8) = lin1;
            *((m + 0x475a) as *mut u8) = if (h28 >> 9) % 4 == 0 { 3 } else { 0 };
            pe24.add(0x29).write(((h28 >> 11) % 30) as u8);
            for k in 0..40usize {
                let p = Engine(m).player(((k / 20)) as i8, k % 20).addr();
                let r = h29.rotate_left(k as u32 * 3);
                let base = if k < 20 { lin0 } else { lin1 };
                *((p + 0x103) as *mut i8) = base + (r % 6) as i8 - 2;
                *((p + 0x107) as *mut i8) = ((r >> 3) % 25) as i8 - 3;
            }
            let pas = if (h28 >> 16) % 3 == 0 { 0 } else { Engine(m).player(l27.max(0).min(1), ((h28 >> 18) % 20) as usize).addr() };
            let linha27 = ((h28 >> 24) % 12) as i8;
            let y27 = if (h28 >> 28) % 2 == 0 { linha27 + if l27 == 0 { 1 } else { -1 } } else { ((h28 >> 29) % 12) as i8 };
            compare_look_for_offside(m + 0xf5b2, l27, linha27, y27, pas);
            compare_engine_look_for_offside(m, l27, linha27, ((h29 >> 50) % 12) as i8, pas);
            // batch 28: card by severity (the referee's person always present here)
            let h30 = s.next_u64();
            let h31 = s.next_u64();
            pe24.add(0x25).write([0u8, 2, 3, 6, 7, 12, 13, 18, 19, 30][(h30 % 10) as usize]);
            pe24.add(0x24).write(((h30 >> 4) % 4) as u8);
            *((m + 0xf5b6) as *mut usize) = pe24 as usize;
            let jg = Engine(m).player(((h30 >> 6) % 2) as i8, ((h30 >> 7) % 20) as usize).addr();
            *((jg + 0x14) as *mut u8) = [((h30 >> 12) % 25) as u8, 200, 0][((h30 >> 17) % 3) as usize];
            *((jg + 0x19) as *mut i8) = if (h30 >> 19) % 6 == 0 { -1 } else { 3 };
            *((jg + 0x23) as *mut u8) = [0u8, 0, 1, 2][((h30 >> 22) % 4) as usize];
            *((jg + 0x24) as *mut u8) = [0u8, 0, 1, 2][((h30 >> 24) % 4) as usize];
            let mut tg = 0u8;
            let gv = [1u8, 2, 3, 4, 5, 6, 0, 7][((h30 >> 26) % 8) as usize];
            let fg = ((h30 >> 29) % 4 == 0) as u8;
            if (h30 >> 31) & 1 == 0 {
                compare_card_by_severity(m + 0xf5b2, jg, &mut tg, gv, fg);
            } else {
                compare_engine_card_by_severity(m, jg, &mut tg, gv, fg);
            }
            let _ = h31;
            // batch 29: person attributes at the thresholds (0x37, 0x43, sum of four)
            let h32 = s.next_u64();
            for (k, pe) in pessoas.iter_mut().enumerate() {
                let r = h32.rotate_left(k as u32 * 13);
                pe[0x37] = [15u8, 16, 5, 6, 9, 10, (r >> 8) as u8 % 21][(r % 7) as usize];
                pe[0x43] = [15u8, 16, 5, 6, 9, 10, (r >> 16) as u8 % 21][((r >> 3) % 7) as usize];
                for (n, o) in [0x40usize, 0x36, 0x26, 0x1b].iter().enumerate() {
                    pe[*o] = ((r >> (24 + n * 6)) % 26) as u8;
                }
            }
            for t in tabelas.iter_mut() {
                for (k, v) in t.iter_mut().enumerate() {
                    *v = [2u16, 4, 8, 0x88, 0x808, 0x10, 0x90, 0x20, 0x40, 0x140, 0, (h32 >> k) as u16][((h32 >> (k * 4)) % 12) as usize];
                }
            }
            let alto32 = [0u32, 0x3f800000, 0x40000000, 0x12345600, 0xffffff00][((h32 >> 50) % 5) as usize];
            compare_choose_by_role_e(m, (alto32 & !0xff) | ((h32 >> 53) % 2) as u32, if (h32 >> 54) % 3 == 0 { 0x100 } else { (h32 >> 56) as u32 & 0xff });
            // batch 30: window of 55 instants (index near the end), zones, periods
            let h33 = s.next_u64();
            *((m + 0x8ed4) as *mut u16) = [0x1efu16, 0x3de, 0x100, (h33 >> 4) as u16][(h33 % 4) as usize];
            *((m + 0x8ea8) as *mut i8) = [3i8, 4, 7, 8, -1, 11, ((h33 >> 20) % 14) as i8][((h33 >> 2) % 7) as usize];
            *((m + 0x8e9b) as *mut i8) = [0i8, 53, 54, ((h33 >> 24) % 55) as i8][((h33 >> 6) % 4) as usize];
            for k in 0..0x30usize {
                *((m + 0x1f0 + k) as *mut u8) = (h33.rotate_left(k as u32)) as u8;
            }
            for k in 0..55isize {
                for b in [0xa, 0x41, 0x78, 0x8d51, 0x8d88, 0x8dbf, 0x8df6, 0x8e2d, 0x8e64] {
                    *((m as isize + b + k) as *mut u8) = [0u8, 1, 0, 0xff][(h33.rotate_left((k as u32 + b as u32) % 64) % 4) as usize];
                }
            }
            compare_possession_statistic(m, [0i8, 1, 2, -1, -2][((h33 >> 30) % 5) as usize], [0u8, 1, 0x80][((h33 >> 32) % 3) as usize]);
            // batch 31: summary (ratings at the limits 1 and 10, sums that overflow)
            let h34 = s.next_u64();
            for k in 0..40usize {
                let p = Engine(m).player(((k / 20)) as i8, k % 20).addr();
                let r = h34.rotate_left(k as u32 * 7);
                *((p + 0x35) as *mut i16) = [-600i16, 499, 500, 1499, 1500, 10499, 10500, 12000, (r >> 8) as i16][(r % 9) as usize];
                for o in [1usize, 2, 4, 5, 7, 8, 9, 0xa, 0xb, 0xd, 0xe, 0xf, 0x12, 0x23, 0x24] {
                    *((p + o) as *mut u8) = (r >> (o % 50)) as u8;
                }
            }
            *((m + 0xf5bd) as *mut u8) = [0u8, 3, 200, (h34 >> 10) as u8][((h34 >> 2) % 4) as usize];
            *((m + 0xf5f3) as *mut u8) = [0u8, 2, 100, (h34 >> 18) as u8][((h34 >> 4) % 4) as usize];
            *((m + 0x8ed4) as *mut u16) = [0x1efu16, 0x3de, 7][((h34 >> 6) % 3) as usize];
            compare_match_summary(m, [0i16, 1, 0x3de, 2, -1][((h34 >> 8) % 5) as usize]);
            // batch 32: the closest on the side (including the own cell: distance zero)
            let h35 = s.next_u64();
            for k in 0..40usize {
                let p = Engine(m).player(((k / 20)) as i8, k % 20).addr();
                let r = h35.rotate_left(k as u32 * 3);
                *((p + 0x19) as *mut i8) = (r % 13) as i8 - 2;
                *((p + 0x2b) as *mut i8) = ((r >> 4) % 4) as i8 - 1;
                *((p + 0x102) as *mut i8) = ((r >> 8) % 9) as i8;
                *((p + 0x103) as *mut i8) = ((r >> 12) % 12) as i8;
            }
            compare_closest_on_side(m, ((h35 >> 40) % 2) as i8, ((h35 >> 41) % 12) as i8, ((h35 >> 45) % 9) as i8, [0u8, 1, 2][((h35 >> 49) % 3) as usize]);
            // batch 33: the goalkeeper (role 1 almost always), in the area or outside, close to the ball
            let h36 = s.next_u64();
            let h37 = s.next_u64();
            for t in tabelas.iter_mut() {
                for (k, v) in t.iter_mut().enumerate() {
                    *v = if (h37 >> k) % 5 == 0 { 2 } else { 1 };
                }
            }
            *((j + 0x19e) as *mut usize) = m;
            *((j + 0x27) as *mut i8) = ((h36 >> 1) % 2) as i8;
            *((j + 0x19) as *mut i8) = ((h36 >> 2) % 11) as i8;
            *((j + 0x2b) as *mut i8) = [0i8, 1, 2, -1][((h36 >> 6) % 4) as usize];
            *((j + 0x102) as *mut i8) = [1i8, 2, 4, 6, 7][((h36 >> 8) % 5) as usize];
            let yg = if read_i8(j + 0x27) == 1 { [9i8, 10, 11, 5][((h36 >> 11) % 4) as usize] } else { [0i8, 1, 2, 5][((h36 >> 11) % 4) as usize] };
            *((j + 0x103) as *mut i8) = yg;
            *((j + 0x168) as *mut i16) = ((h36 >> 13) % 1200) as i16 - 100;
            *((j + 0x16a) as *mut i16) = ((h36 >> 24) % 1200) as i16 - 100;
            *((j + 0x3b) as *mut i16) = ((h36 >> 35) % 300) as i16;
            *((j + 0xd1) as *mut f32) = ((h36 >> 44) % 200) as f32 / 4.0;
            *((j + 0xf) as *mut u8) = (h36 >> 52) as u8;
            *((m + 0x8ea7) as *mut i8) = ((h37 >> 40) % 9) as i8;
            *((m + 0x8ea8) as *mut i8) = yg + ((h37 >> 44) % 5) as i8 - 2;
            *((m + 0x8eaa) as *mut i8) = ((h37 >> 48) % 30) as i8 - 3;
            let (mut sd, mut rb) = (0u8, 0u8);
            let cod = [3i8, 4, 5, 26, 27, 0x33, 1, -2, ((h37 >> 52) % 60) as i8][((h37 >> 56) % 9) as usize];
            let alvo_y = yg + ((h37 >> 20) % 5) as i8 - 2;
            compare_goalkeeper_save(j, ((h37 >> 24) % 9) as i8, alvo_y, ((h37 >> 28) % 2) as u8, cod, &mut sd, &mut rb);
            // batch 34: consistent grid in both cells (lists with valid players)
            let h38 = s.next_u64();
            let lj = ((h38 >> 1) % 2) as i8;
            let jm = Engine(m).player(lj, ((h38 >> 2) % 20) as usize).addr();
            let (ox, oy) = (((h38 >> 7) % 9) as i8, ((h38 >> 11) % 12) as i8);
            let (nx, ny) = if (h38 >> 15) % 6 == 0 { (ox, oy) } else { (((h38 >> 18) % 9) as i8, ((h38 >> 22) % 12) as i8) };
            *((jm + 0x102) as *mut i8) = ox;
            *((jm + 0x103) as *mut i8) = oy;
            let ca = Engine(m).cell(ox, oy).addr();
            let cn = Engine(m).cell(nx, ny).addr();
            let n_ant = 1 + ((h38 >> 26) % 11) as i8;
            *((ca + 0x58 + lj as usize) as *mut i8) = n_ant;
            for i in 0..11usize {
                let outro_j = Engine(m).player(lj, ((h38 >> (30 + i % 20)) % 20) as usize).addr();
                *((ca as isize + 4 * (11 * lj as isize + i as isize)) as *mut usize) = if i % 5 == 4 { 0 } else { outro_j };
            }
            *((jm + 0x28) as *mut i8) = if (h38 >> 40) % 4 == 0 { -1 } else { ((h38 >> 42) % n_ant as u64) as i8 };
            if cn != ca {
                *((cn + 0x58 + lj as usize) as *mut i8) = ((h38 >> 46) % 11) as i8;
            } else if read_i8(jm + 0x28) < 0 {
                *((cn + 0x58 + lj as usize) as *mut i8) = ((h38 >> 46) % 11) as i8;
            }
            *((m + 0xf582) as *mut usize) = if (h38 >> 50) % 2 == 0 { jm } else { o };
            compare_move_to(jm, nx, ny);
            // batch 35: choice of cell (grid with small counts and valid lists)
            let h39 = s.next_u64();
            let h40 = s.next_u64();
            let lc = ((h39 >> 1) % 2) as i8;
            let jc5 = Engine(m).player(lc, ((h39 >> 2) % 20) as usize).addr();
            *((jc5 + 0x102) as *mut i8) = [0i8, 1, 2, 3, 4, 5, 6, 7, 8][((h39 >> 7) % 9) as usize];
            *((jc5 + 0x103) as *mut i8) = ((h39 >> 11) % 12) as i8;
            *((jc5 + 0x19) as *mut i8) = ((h39 >> 15) % 11) as i8;
            for (o, sh) in [(0x105usize, 19), (0x106, 21), (0x107, 23), (0x2c, 25), (0x2d, 27)] {
                *((jc5 + o) as *mut i8) = ((h39 >> sh) % 4) as i8 - (o == 0x2d) as i8 * 0;
            }
            *((jc5 + 0x6d) as *mut usize) = pessoas[2].as_ptr() as usize;
            pessoas[2][0x26] = ((h39 >> 29) % 20) as u8;
            *((jc5 + 0xe1) as *mut f32) = ((h39 >> 34) % 40) as f32 / 3.0;
            *((jc5 + 0x19a) as *mut u16) = ((h39 >> 40) % 360) as u16;
            *((m + 0x8eaf) as *mut i8) = ((h40 >> 1) % 8) as i8 + 2;
            *((m + 0x8eb0) as *mut i8) = ((h40 >> 4) % 8) as i8 + 2;
            for i in 0..2usize {
                *((m + 6371 * i + 0x9766) as *mut u32) = (h40 >> (8 + i)) as u32 & 0x400;
            }
            for cel in 0..108usize {
                for l in 0..2usize {
                    let r = h40.rotate_left((cel * 2 + l) as u32 % 64);
                    let casa = m + engine::GRID + GRID_CELL_SIZE * cel;
                    let n = if r % 5 == 0 { (1 + (r >> 3) % 2) as i8 } else { 0 };
                    *((casa + 0x58 + l) as *mut i8) = n;
                    for i in 0..2usize {
                        *((casa + 4 * (11 * l + i)) as *mut usize) = Engine(m).player(l as i8, ((r >> (8 + i * 5)) % 20) as usize).addr();
                    }
                }
            }
            for t in tabelas.iter_mut() {
                for (k, v) in t.iter_mut().enumerate() {
                    *v = [0x90u16, 0x810, 0xa0, 0x30, 0x10, 0x8b0, 1, 0, 0x880][((h40 >> (k * 5 % 60)) % 9) as usize];
                }
            }
            let (mut sxc, mut syc) = (((h39 >> 44) % 10) as i8 - 1, ((h39 >> 48) % 13) as i8 - 1);
            compare_choose_square(m, jc5, &mut sxc, &mut syc, ((h39 >> 52) % 16) as u32 | if (h39 >> 56) % 4 == 0 { 0xff00 } else { 0 });
            // batch 36: control (codes from the table, goalkeeper in the area, ball with or without owner)
            let h41 = s.next_u64();
            let h42 = s.next_u64();
            *((j + 0x19e) as *mut usize) = m;
            *((j + 0x27) as *mut i8) = (h41 % 2) as i8;
            *((j + 0x19) as *mut i8) = ((h41 >> 1) % 11) as i8;
            *((j + 0x102) as *mut i8) = ((h41 >> 5) % 9) as i8;
            *((j + 0x103) as *mut i8) = [0i8, 1, 10, 11, ((h41 >> 9) % 12) as i8][((h41 >> 13) % 5) as usize];
            for k in 0..0x40usize {
                *((j + 0x11a + 2 * k) as *mut u16) = (h42.rotate_left(k as u32) as u16) % 600;
            }
            *((j + 0x4d) as *mut f32) = ((h41 >> 16) % 10000) as f32 + 0.5;
            *((j + 0x75) as *mut f32) = ((h41 >> 30) % 50) as f32 / 2.0;
            *((j + 0x19a) as *mut i16) = ((h41 >> 36) % 360) as i16;
            *((j + 0x19c) as *mut i16) = ((h41 >> 45) % 100) as i16;
            *((m + 0x8ed8) as *mut i16) = ((h42 >> 3) % 360) as i16;
            *((m + 0x8ea9) as *mut i8) = [4i8, 3, 5, 6, -1, 9][((h42 >> 12) % 6) as usize];
            *((m + 0x8eaa) as *mut i8) = ((h42 >> 15) % 12) as i8 - 3;
            let dono36 = [j, o, 0][((h42 >> 20) % 3) as usize];
            *((m + 0xf582) as *mut usize) = dono36;
            *((o + 0x19c) as *mut i16) = ((h42 >> 40) % 90) as i16 + 1;
            let especiais = [37i8, 42, 43, 44, 45, 46, 47, 49, 50, 0x15, 0x25]; // 48 would hit the original's error (message box)
            let todos = [1i8, 12, 19, 21, 33, 34, 36, 37, 41, 42, 43, 44, 45, 46, 47, 49, 50, 0x15, 0x25];
            let mut cod36 = if dono36 == 0 { especiais[((h42 >> 24) % especiais.len() as u64) as usize] } else { todos[((h42 >> 24) % todos.len() as u64) as usize] };
            let (mut a36, mut b36) = (7u8, 7u8);
            compare_control_of_move(j, &mut cod36, &mut a36, &mut b36);
            // batch 37: trace (lines in the four directions, points, exits from the pitch)
            let h43 = s.next_u64();
            let (tx0, ty0) = (((h43 >> 1) % 11) as i8 - 1, ((h43 >> 5) % 14) as i8 - 1);
            let (tx1, ty1) = if (h43 >> 9) % 8 == 0 { (tx0, ty0) } else { (((h43 >> 12) % 11) as i8 - 1, ((h43 >> 16) % 14) as i8 - 1) };
            let cods = [6i8, 8, 9, 30, 31, 52, 54, 55, 60, 61, 62, 90, 91, 92, 94, 7, 0, 100];
            let cod37 = cods[((h43 >> 20) % cods.len() as u64) as usize];
            for cel in 0..108usize {
                for l in 0..2usize {
                    let r = h43.rotate_left((cel * 3 + l) as u32 % 64);
                    let casa = m + engine::GRID + GRID_CELL_SIZE * cel;
                    *((casa + 0x58 + l) as *mut i8) = if r % 4 == 0 { 1 } else { 0 };
                    *((casa + 44 * l) as *mut usize) = Engine(m).player(l as i8, ((r >> 5) % 20) as usize).addr();
                }
            }
            compare_trace_move(m, tx0, ty0, tx1, ty1, cod37, (h43 >> 26) as u32 & 0x1ff | 0xabcd0000, (h43 >> 40) as u32 & 0x7);
            // batch 38: list of the day's matches and scores (normal, extra time, penalties)
            let h44 = s.next_u64();
            let clubes = [0x1000u32, 0x2000, 0x3000, 0x4000];
            for k in 0..10usize {
                let jg = jogos.as_mut_ptr() as usize + 0x69 * k + 0x20;
                let r = h44.rotate_left(k as u32 * 6);
                *(jg as *mut u32) = clubes[(r % 4) as usize];
                *((jg + 4) as *mut u32) = clubes[((r >> 2) % 4) as usize];
                *((jg + 0x3b) as *mut u32) = if (r >> 4) % 3 == 0 { 0 } else { 7 };
            }
            let dia = jogadores[4].as_mut_ptr() as usize;
            *((dia + 0x19) as *mut usize) = jogos.as_ptr() as usize;
            *((dia + 0x21) as *mut i32) = ((h44 >> 10) % 12) as i32 - 1;
            compare_has_match(dia, [0x1000u32, 0x3000, 0x5000][((h44 >> 14) % 3) as usize], ((h44 >> 16) % 2) as u32);
            let jogo44 = jogadores[3].as_mut_ptr() as usize;
            *((jogo44 + 0x1c) as *mut u32) = 0x1000;
            *((jogo44 + 0x20) as *mut u32) = 0x2000;
            for (k, o) in [0x43usize, 0x44, 0x47, 0x48, 0x49, 0x4a].iter().enumerate() {
                *((jogo44 + o) as *mut i8) = [-1i8, 0, 1, 2, 3][((h44 >> (20 + k * 3)) % 5) as usize];
            }
            compare_won_the_match(jogo44, [0x1000u32, 0x2000][((h44 >> 40) % 2) as usize]);
            // batch 41: shot value (given and chosen types, zones, goalkeeper)
            let h45 = s.next_u64();
            let h46 = s.next_u64();
            let h47 = s.next_u64();
            *((j + 0x19e) as *mut usize) = m;
            *((j + 0x27) as *mut i8) = (h45 % 2) as i8;
            *((j + 0x19) as *mut i8) = ((h45 >> 1) % 11) as i8;
            *((j + 0x102) as *mut i8) = ((h45 >> 5) % 9) as i8;
            *((j + 0x103) as *mut i8) = [0i8, 1, 2, 9, 10, 11, ((h45 >> 9) % 12) as i8][((h45 >> 13) % 7) as usize];
            *((j + 0x10b) as *mut i8) = ((h45 >> 16) % 22) as i8;
            *((j + 0x109) as *mut i8) = ((h45 >> 21) % 26) as i8 - 5;
            // words by shot type (0x11a + 2 x type, up to 0x3a); then comes 0x19e (engine)
            for k in 0..0x3busize {
                *((j + 0x11a + 2 * k) as *mut i16) = (h46.rotate_left(k as u32 % 64) % 700) as i16;
            }
            *((j + 0x29) as *mut i16) = ((h45 >> 26) % 400) as i16;
            *((j + 0x19c) as *mut i16) = ((h45 >> 35) % 300) as i16 - 50;
            *((j + 0x198) as *mut i16) = if (h45 >> 44) % 9 == 0 { 0x3b } else { 7 };
            *((j + 0x4d) as *mut f32) = ((h47 >> 1) % 10001) as f32;
            for (o, sh) in [(0x79usize, 15), (0x81, 21), (0xe5, 27), (0xa1, 33), (0xb1, 39)] {
                *((j + o) as *mut f32) = ((h47 >> sh) % 120) as f32 / 4.0;
            }
            *((m + 0x8ea9) as *mut i8) = [0i8, 0, 1, 2, 3, 4, 5, -1][((h47 >> 45) % 8) as usize];
            *((m + 0x4756) as *mut i8) = ((h47 >> 48) % 6) as i8;
            *((m + 1) as *mut i8) = [2i8, 0x20, 0, 5][((h47 >> 51) % 4) as usize];
            for t in tabelas.iter_mut() {
                for (k, v) in t.iter_mut().enumerate() {
                    *v = if (h46 >> k) % 3 == 0 { 1 } else { 2 };
                }
            }
            let mut tc41 = [-1i8, -1, -1, -1, 0x33, 0x35, 3, 5, 0x16, 0x39, 0x3a, 0x1b][((h47 >> 53) % 12) as usize];
            let mut som41 = 0u16;
            compare_shot_value(j, &mut tc41, &mut som41);
            // batch 42: team strength (roles, attributes at the edges, competition, reputations)
            let h48 = s.next_u64();
            let h49 = s.next_u64();
            let l42 = (h48 % 2) as i8;
            for t in tabelas.iter_mut() {
                for (k, v) in t.iter_mut().enumerate() {
                    let r = h49.rotate_left(k as u32 * 5);
                    *v = [1u16, 2, 4, 8, 0x10, 0x20, 0x40, 0, 0x80, (r >> 8) as u16][(r % 10) as usize];
                }
            }
            for (k, pe) in pessoas.iter_mut().enumerate() {
                for o in 0x1busize..0x46 {
                    let r = h48.rotate_left((k * 37 + o) as u32 % 64) ^ h49.rotate_right(o as u32);
                    pe[o] = match r % 9 {
                        0 => (r >> 8) as u8,
                        1 => [5u8, 6, 9, 10, 15, 16][((r >> 8) % 6) as usize],
                        _ => ((r >> 8) % 21) as u8,
                    };
                }
                pe[0x21] = (h49.rotate_left(k as u32 * 7) % 25) as u8;
                let r = h48.rotate_left(k as u32 * 13);
                pe[5..7].copy_from_slice(&(((r >> 3) % 300) as i16 - 20).to_le_bytes());
            }
            for k in 0..20usize {
                let p = Engine(m).player(l42, k).addr();
                let r = h48.rotate_left(k as u32 * 3) ^ h49.rotate_right(k as u32 * 7);
                *((p + 0x19) as *mut i8) = if r % 7 == 0 { -1 } else { ((r >> 3) % 11) as i8 };
                *((p + 0x69) as *mut usize) = fichas.as_ptr().add(k) as usize;
                *((p + 0x6d) as *mut usize) = pessoas[(r >> 7) as usize % 4].as_ptr() as usize;
                *((p + 0x35) as *mut i16) = [7920i16, 7923, 5940, 5953, 5610, 5622, 6270, 6278, 6930, 6941, 7590, 7615, ((r >> 10) % 9000) as i16][((r >> 24) % 13) as usize];
                *((p + 0x4d) as *mut f32) = ((r >> 30) % 10001) as f32;
                for (n, o) in [0xddusize, 0xcd, 0xc1, 0x7d, 0xf1, 0xc5, 0x9d, 0xa9, 0x8d, 0x95, 0x71, 0xb9, 0x99, 0xd1, 0xa5, 0x75, 0xad].iter().enumerate() {
                    let z = h49.rotate_left((k * 17 + n) as u32 % 64) ^ r;
                    *((p + o) as *mut f32) = (z % 4000) as f32 / 100.0 - if z % 13 == 0 { 15.0 } else { 0.0 };
                }
            }
            let pa = partida42.as_mut_ptr() as usize;
            let (c42, f42, j42, x42) = (clube42[0].as_mut_ptr() as usize, clube42[1].as_mut_ptr() as usize, jogo42.as_mut_ptr() as usize, comp42.as_mut_ptr() as usize);
            *((pa + 0x14) as *mut usize) = if (h48 >> 40) % 6 == 0 { 0 } else { j42 };
            *((pa + 0x1c) as *mut usize) = if (h48 >> 43) % 8 == 0 { 0 } else { c42 };
            *((pa + 0x20) as *mut usize) = if (h48 >> 46) % 8 == 0 { 0 } else { f42 };
            *((c42 + 0x53) as *mut usize) = if (h48 >> 49) % 8 == 0 { 0 } else { x42 };
            *((j42 + 0x5d) as *mut usize) = if (h48 >> 52) % 4 == 0 { 0 } else { x42 };
            let ids = [KEY_NATION_GERMANY, KEY_NATION_SCOTLAND, KEY_NATION_BELGIUM, KEY_NATION_SWEDEN, KEY_NATION_NORWAY, KEY_NATION_ENGLAND, KEY_NATION_HOLLAND, KEY_NATION_DENMARK, KEY_NATION_PORTUGAL, KEY_NATION_FRANCE];
            // the game's competitions may be empty at the start: distinct values during the test
            let ids_antes: Vec<u32> = ids.iter().map(|&a| *(a as *const u32)).collect();
            for (i, &a) in ids.iter().enumerate() {
                *(a as *mut u32) = 0x7000 + i as u32;
            }
            *(x42 as *mut u32) = if (h48 >> 55) % 11 == 10 { 0x1234 } else { 0x7000 + ((h48 >> 55) % 11) as u32 };
            let reps = [1000i16, 1001, 1251, 751, 251, 0, -250, -249, -751, -750, 2500, 2499, 30000, -30000];
            *((c42 + 0x80) as *mut i16) = reps[((h49 >> 40) % 14) as usize];
            *((f42 + 0x80) as *mut i16) = if (h49 >> 44) % 3 == 0 { 0 } else { reps[((h49 >> 46) % 14) as usize] };
            *((m + 0x4792) as *mut usize) = pa;
            *((m + 0xf5bd) as *mut u8) = ((h49 >> 50) % 2) as u8;
            *((m + 0xf5f3) as *mut u8) = ((h49 >> 51) % 2) as u8;
            for o in 0..0x10usize {
                *((m + 0x1d8 + 2 * o) as *mut u16) = (h49 >> (o * 3)) as u16;
            }
            compare_team_strength(m, l42);
            // batch 43: kick-off (consistent grid, formations with distinct cells)
            let h50 = s.next_u64();
            let h51 = s.next_u64();
            core::ptr::write_bytes((m + engine::GRID) as *mut u8, 0, GRID_CELL_SIZE * 9 * GRID_ROWS);
            for (l, f) in formacoes.iter_mut().enumerate() {
                let base = f.as_mut_ptr() as usize + 1200 + if l == 1 { 0xa6 } else { 0x9e };
                for pos in 0..13usize {
                    let c = (h50.rotate_left((pos * 5 + l * 3) as u32) % 108) as u8;
                    // distinct cells per position in the own half (side 0: y 0..=5; side 1: 6..=11) and, now
                    // and then, outside it (exercises the y limit without exceeding 11 per cell)
                    // distinct cell per position (with the y limit, at most 4 per cell): a cell
                    // with more than 11 players corrupts the lists and the comparer of 0x6f46b0 reads garbage
                    let _ = c;
                    let x = ((h50 >> 8) % 9) as u8 + pos as u8;
                    let x = x % 9;
                    let mut y = (pos as u8 / 9) * 3 + ((h50 >> 12) % 3) as u8;
                    if (h50 >> ((pos + 13 * l) % 64)) % 4 == 0 {
                        y += 6;
                    }
                    if l == 1 {
                        y = (y + 6) % 12;
                    }
                    *((base + 216 * pos) as *mut u8) = y << 4 | x;
                }
            }
            let desloc = (h50 % 13) as usize;
            for k in 0..40usize {
                let l = k / 20;
                let p = Engine(m).player(l as i8, k % 20).addr();
                let r = h51.rotate_left(k as u32 * 3) ^ h50.rotate_right(k as u32);
                *((p + 0x27) as *mut i8) = l as i8;
                *((p + 0x19) as *mut i8) = if r % 6 == 0 { -1 } else { ((k + desloc) % 13) as i8 };
                *((p + 0x19e) as *mut usize) = m; // 0 would only crash in the original (the grid is read through the player's engine)
                *((p + 0x6d) as *mut usize) = pessoas[k % 4].as_ptr() as usize;
                *((p + 0x10c) as *mut i8) = ((r >> 8) % 21) as i8;
                *((p + 0x105) as *mut i8) = ((r >> 13) % 21) as i8;
                let (rx, ry) = (((r >> 18) % 9) as i8, ((r >> 22) % 12) as i8);
                *((p + 0x102) as *mut i8) = rx;
                *((p + 0x103) as *mut i8) = ry;
                let casa = Engine(m).cell(rx, ry).addr();
                let n = read_i8(casa + (0x58 + l));
                *((casa as isize + 4 * (11 * l as isize + n as isize)) as *mut usize) = p;
                *((casa + 0x58 + l) as *mut i8) = n + 1;
                *((p + 0x28) as *mut i8) = n;
            }
            // sometimes a side with nobody in positions 0..=10 (the kick-off without a player: byte 0 of the engine)
            if (h51 >> 58) % 4 == 0 {
                let lv = ((h51 >> 60) % 2) as usize;
                for k in 0..20usize {
                    *((Engine(m).player(lv as i8, k).addr() + 0x19) as *mut i8) = [-1i8, 11, 12][(k + (h51 >> 62) as usize) % 3];
                }
            }
            let bloco = |x: u64| Engine(m).player(((x % 2)) as i8, (((x >> 1) % 20)) as usize).addr();
            *((m + 0xf582) as *mut usize) = if (h51 >> 40) % 3 == 0 { 0 } else { bloco(h51 >> 42) };
            *((m + 0xf576) as *mut usize) = if (h51 >> 48) % 3 == 0 { 0 } else { bloco(h51 >> 50) };
            *((m + 0x8eae) as *mut i8) = [0i8, 1, -1, 2][((h51 >> 56) % 4) as usize];
            *((m + 0x8ed8) as *mut u16) = (h51 >> 20) as u16;
            *(m as *mut u8) = 0;
            static WARNINGS47: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
            let avisar = || WARNINGS47.fetch_add(1, std::sync::atomic::Ordering::Relaxed) < 6;
            if let Some(e) = grid_inconsistent(Engine(m)) {
                if avisar() {
                    linha(&format!("engine: inconsistent grid before the kick-off: {}", e));
                }
            }
            compare_kick_off(m);
            let grade47 = grid_inconsistent(Engine(m));
            if let Some(e) = &grade47 {
                if avisar() {
                    linha(&format!("engine: inconsistent grid after the kick-off: {}", e));
                }
            }
            // batch 47: fine positions (the kick-off grid stays consistent; positions 0..=10)
            for k in 0..40usize {
                let p = Engine(m).player(((k / 20)) as i8, k % 20).addr();
                let v = read_i8(p + 0x19);
                if !(0..=10).contains(&v) {
                    *((p + 0x19) as *mut i8) = (k % 11) as i8;
                }
                *(p as *mut u8) = (h51 >> (k % 50)) as u8;
            }
            let mut saida47 = [0xa5u8; 0x100];
            if grade47.is_none() {
                compare_fine_positions(m, saida47.as_mut_ptr());
            }
            // batch 52: take off the pitch (consistent kick-off grid, markings, modes,
            // goalkeeper kept, human manager with the globals swapped during the test)
            if grade47.is_none() {
                let h60 = s.next_u64();
                let h61 = s.next_u64();
                let lj = ((h60 >> 1) % 2) as usize;
                let jt = Engine(m).player(lj as i8, ((h60 >> 2) % 20) as usize).addr();
                for k in 0..40usize {
                    let p = Engine(m).player(((k / 20)) as i8, k % 20).addr();
                    *((p + 0x1aa) as *mut usize) = 0;
                    *((p + 0x101) as *mut i8) = 0;
                }
                // jt marks someone (who has it in the list) and is marked by up to 3
                let alvo = Engine(m).player((1 - lj) as i8, ((h60 >> 8) % 20) as usize).addr();
                if (h60 >> 13) % 3 != 0 {
                    *((jt + 0x1aa) as *mut usize) = alvo;
                    *((alvo + 0x101) as *mut i8) = 1;
                    *((alvo + 0x1ae) as *mut usize) = if (h60 >> 15) % 4 == 0 { m } else { jt };
                }
                let nm = ((h60 >> 17) % 4) as i8;
                *((jt + 0x101) as *mut i8) = nm;
                for i in 0..nm as usize {
                    let q = Engine(m).player((1 - lj) as i8, ((h60 >> (20 + 3 * i)) % 20) as usize).addr();
                    *((jt + 0x1ae + 4 * i) as *mut usize) = q;
                    *((q + 0x1aa) as *mut usize) = [jt, 0, alvo][((h60 >> (30 + 2 * i)) % 3) as usize];
                }
                *((m + 0xf59e) as *mut usize) = if (h60 >> 34) % 3 == 0 { jt } else { 0 };
                *((m + 0xf5a2) as *mut usize) = if (h60 >> 36) % 3 == 0 { jt } else { 0 };
                // human manager
                let globais60 = [0xae2364usize, G_HUMAN_MANAGER];
                let antes60: Vec<u32> = globais60.iter().map(|&a| *(a as *const u32)).collect();
                *(G_STAFF_COUNT as *mut i32) = 1000;
                let regs60 = tecnicos56.as_mut_ptr() as usize;
                for k in 0..16usize {
                    *((regs60 + 42735 * k + 0x9a01) as *mut u32) = if (h61 >> k) % 2 == 0 { 0 } else { 7 };
                    *((regs60 + 42735 * k + 0x9a05) as *mut u8) = ((h61 >> (k + 16)) % 2) as u8;
                }
                let gerente60 = [regs60, 16usize];
                *(G_HUMAN_MANAGER as *mut usize) = gerente60.as_ptr() as usize;
                let mut tec60 = [0u8; 0x60];
                *(tec60.as_mut_ptr() as *mut i32) = [1000 - 16, 999, 1000, 983, 5][((h61 >> 32) % 5) as usize];
                tec60[0x5f] = ((h61 >> 35) % 3) as u8;
                for l in 0..2usize {
                    *((m + 0x904d + 208 * l) as *mut usize) = if (h61 >> (37 + l)) % 3 == 0 { 0 } else { tec60.as_ptr() as usize };
                    *((m + 0x904b + 208 * l) as *mut u8) = ((h61 >> (40 + l)) % 2) as u8;
                }
                let globais_sel = [0xae235cusize, G_NATION_COUNT];
                let antes_sel: Vec<u32> = globais_sel.iter().map(|&a| *(a as *const u32)).collect();
                *(G_CLUB_COUNT as *mut i32) = 5000;
                *(G_NATION_COUNT as *mut i32) = 200;
                let clubes60 = [[4500i32 + ((h61 >> 42) % 2) as i32 * 200], [4500i32 + ((h61 >> 43) % 2) as i32 * 200]];
                let pa60 = partida50.as_mut_ptr() as usize;
                *((pa60 + 0x1c) as *mut usize) = clubes60[0].as_ptr() as usize;
                *((pa60 + 0x20) as *mut usize) = clubes60[1].as_ptr() as usize;
                *((m + 0x4792) as *mut usize) = pa60;
                compare_take_off_pitch(m, jt, [-1i8, -3, 0, 1, -2][((h61 >> 44) % 5) as usize]);
                // batch 56: choose the taker (types without the club's tactics: the object at
                // 0xde1d08 only exists with the database loaded; corners and free kicks near the goal are left to the real game)
                {
                    // contiguous records, like the game's person vector (0xae23bc, 110 bytes each): the human manager's
                    // penalty list keeps indices into it
                    let mut fichas56 = vec![0u8; 110 * 20];
                    let pessoas_antes = *(G_STAFF_TABLE as *const usize);
                    *(G_STAFF_TABLE as *mut usize) = fichas56.as_mut_ptr() as usize;
                    let mut pessoas56 = vec![[0u8; 0x30]; 20];
                    let mut excl56 = [0u8; 40];
                    let r0 = s.next_u64();
                    let lado56 = (r0 % 2) as i8;
                    // the side blocks return as they were at the end (records and persons here are local;
                    // the coordinates do not change, so as not to undo the grid)
                    let bloco56 = Engine(m).player(lado56, 0).addr();
                    let antes_bloco56 = core::slice::from_raw_parts(bloco56 as *const u8, 20 * 0x1be).to_vec();
                    let tab = *((m + 0x8ebc + 4 * lado56 as usize) as *const usize) as *mut u16;
                    for i in 0..11 {
                        let z = s.next_u64();
                        *tab.add(i) = [1u16, 0x40, 0x41, 0, 2, z as u16][(z % 6) as usize];
                    }
                    for k in 0..20usize {
                        let q = Engine(m).player(lado56, k).addr();
                        let z = s.next_u64();
                        let z2 = s.next_u64();
                        *((q + 0x19) as *mut i8) = [0i8, 3, 7, 10, 11, 12, -1, 5][(z % 8) as usize];
                        *((q + 0x27) as *mut i8) = lado56;
                        *((q + 0x69) as *mut usize) = fichas56.as_mut_ptr() as usize + 110 * k;
                        *((q + 0x6d) as *mut usize) = if (z >> 3) % 9 == 0 { 0 } else { pessoas56[k].as_mut_ptr() as usize };
                        for (n, o) in [0xddusize, 0xe9, 0x89, 0xad, 0xc9, 0xa1].iter().enumerate() {
                            let y = z2.rotate_left(n as u32 * 11) ^ z;
                            *((q + o) as *mut f32) = [0.0f32, 1.0, 10.5, 20.0, -3.0, ((y >> 8) % 4000) as f32 / 100.0][(y % 6) as usize];
                        }
                        *((q + 0x109) as *mut i8) = ((z >> 16) % 21) as i8 - 1;
                        fichas56[110 * k + 0x18] = [17u8, 19, 20, 23, 24, 30][((z >> 21) % 6) as usize];
                        fichas56[110 * k + 0x5a] = [0u8, 0, 1, 2, 5][((z >> 24) % 5) as usize];
                        pessoas56[k][0xf] = ((z >> 27) % 21) as u8;
                        *(pessoas56[k].as_mut_ptr().add(0xb) as *mut i16) = [0i16, 100, 10000, 20000, -50][((z >> 32) % 5) as usize];
                        pessoas56[k][0x29] = [0u8, 1, 10, 20, 0xf0][((z >> 36) % 5) as usize];
                        excl56[20 * lado56 as usize + k] = ((z >> 40) % 3 == 0) as u8;
                    }
                    // always a valid candidate (without tactics and without excluding: the original gives a fatal error with nobody)
                    let q0 = Engine(m).player(lado56, (r0 >> 1) as usize % 20).addr();
                    *((q0 + 0x19) as *mut i8) = 4;
                    *((q0 + 0x6d) as *mut usize) = pessoas56[0].as_mut_ptr() as usize;
                    let gq = Engine(m).player(lado56, (r0 >> 6) as usize % 20).addr();
                    *((m + 0xf59e) as *mut usize) = if (r0 >> 11) % 3 == 0 { gq } else { 0 };
                    *((m + 0xf5a2) as *mut usize) = if (r0 >> 13) % 3 == 0 { gq } else { 0 };
                    if gq == q0 {
                        *((m + 0xf59e) as *mut usize) = 0;
                        *((m + 0xf5a2) as *mut usize) = 0;
                    }
                    let a7 = [-1i8, 0, 1, 2, 4, 6, 7, 8, 9][((r0 >> 15) % 9) as usize];
                    let a8 = [-1i8, 0, 1, 2, 5, 9, 10, 11, 12][((r0 >> 19) % 9) as usize];
                    *((m + 0x8ea7) as *mut i8) = a7;
                    *((m + 0x8ea8) as *mut i8) = a8;
                    let mut tipo = [1i8, 2, 4, 5, 5, 5, 6, 7, 8][((r0 >> 23) % 9) as usize];
                    let ofs = if lado56 != 0 { 0 } else { 11 };
                    let (cx, ax) = ((a7 as i32 - 4).abs(), (a8 as i32 - ofs).abs());
                    if (tipo == 6 || tipo == 7) && (2..=6).contains(&a7) && cx <= 8 && ax <= 11 && (distance_table_cell(ax, cx) as f64) < read_f64(rdata::F64_4_1) {
                        tipo = 8; // near the goal the free kick consults the tactics
                    }
                    let excl = if tipo == 5 && (r0 >> 27) % 3 == 0 { excl56.as_ptr() as usize } else { 0 };
                    // human manager's penalty takers list (0x601070: record + 0x1e05 with 0x9a69 = 0):
                    // records of the players in some of the 50 slots
                    let i56 = *(tec60.as_ptr() as *const i32) - 1000 + 16;
                    if (0..16).contains(&i56) {
                        let reg = regs60 + 42735 * i56 as usize;
                        if tipo == 5 && (r0 >> 30) % 2 == 0 {
                            // half of the penalties through the list path (manager with mark 0x5f, without 0x9a01)
                            tec60[0x5f] = 1;
                            *((reg + 0x9a01) as *mut u32) = 0;
                            *((m + 0x904d + 208 * lado56 as usize) as *mut usize) = tec60.as_ptr() as usize;
                        }
                        for b in 0..50usize {
                            let z = s.next_u64();
                            *((reg + 0x1e05 + 4 * b) as *mut i32) = if z % 4 == 0 { ((z >> 8) % 20) as i32 } else { -1 };
                        }
                    }
                    compare_choose_taker(m, lado56, tipo, excl);
                    core::ptr::copy_nonoverlapping(antes_bloco56.as_ptr(), bloco56 as *mut u8, antes_bloco56.len());
                    *(G_STAFF_TABLE as *mut usize) = pessoas_antes;
                }
                for (i, &a) in globais60.iter().enumerate() {
                    *(a as *mut u32) = antes60[i];
                }
                for (i, &a) in globais_sel.iter().enumerate() {
                    *(a as *mut u32) = antes_sel[i];
                }
            }
            // batch 44: rolling ball (impulse, directions at the edges, exits from the pitch)
            let h52 = s.next_u64();
            *((m + 0xf582) as *mut usize) = if h52 % 9 == 0 { j } else { 0 };
            *((m + 0xf57a) as *mut usize) = if (h52 >> 4) % 9 == 0 { o } else { 0 };
            *((m + 0x8eaa) as *mut i8) = [0i8, 1, 1, 1, 2, 3, 5, -1][((h52 >> 8) % 8) as usize];
            *((m + 0x8eb2) as *mut u8) = if (h52 >> 11) % 6 == 0 { [1u8, 2, 3][((h52 >> 14) % 3) as usize] } else { 0 };
            *((m + 0x8ea9) as *mut i8) = [0i8, 1, 2, 7, -1][((h52 >> 16) % 5) as usize];
            let dirs = [0i16, 359, 40, 41, 49, 50, 130, 131, 139, 140, 220, 221, 229, 230, 310, 311, 319, 320, -10, 400];
            *((m + 0x8ed8) as *mut i16) = if (h52 >> 20) % 4 == 0 { ((h52 >> 24) % 420) as i16 - 30 } else { dirs[((h52 >> 33) % 20) as usize] };
            *((m + 0x8ea7) as *mut i8) = [0i8, 8, 4, 1, 7, -1, 9][((h52 >> 38) % 7) as usize];
            *((m + 0x8ea8) as *mut i8) = [0i8, 11, 5, 1, 10, -1, 12][((h52 >> 41) % 7) as usize];
            *((m + 0x8eab) as *mut i8) = [0i8, 1, 2][((h52 >> 44) % 3) as usize];
            *((m + 0x8eae) as *mut i8) = [0i8, 1, -1][((h52 >> 46) % 3) as usize];
            compare_roll_ball(m);
            // batch 67: order of the players (keeps the list, the count and 0xb67a64)
            {
                let mut antes67 = [0u8; 4 * 40];
                core::ptr::copy_nonoverlapping((m + 0x8f7e) as *const u8, antes67.as_mut_ptr(), antes67.len());
                let (n67, g67) = (*((m + 0x8eb8) as *const u8), *(G_ACTION_ORDER_MATCH_ENGINE as *const u32));
                compare_player_order(m);
                core::ptr::copy_nonoverlapping(antes67.as_ptr(), (m + 0x8f7e) as *mut u8, antes67.len());
                *((m + 0x8eb8) as *mut u8) = n67;
                *(G_ACTION_ORDER_MATCH_ENGINE as *mut u32) = g67;
            }
            // batch 45: condition test (condition 0..100, below 1 divides by zero)
            let h53 = s.next_u64();
            let cond = [0.0f32, 0.5, 0.999, 1.0, 1.5, 100.0, 99.99, 10.0, 31.62, ((h53 >> 8) % 100_001) as f32 / 1000.0][((h53 >> 4) % 10) as usize];
            *((j + 0x1a2) as *mut f32) = cond;
            *((j + 0x1a6) as *mut usize) = if (h53 >> 30) % 2 == 0 { o } else { 0 };
            let alvo53 = [0usize, o, j][((h53 >> 32) % 3) as usize];
            let cod53 = [1i8, 2, 3, 4, 5, 0, 6, -1, 1, 2, 5][((h53 >> 35) % 11) as usize];
            compare_condition_test(j, cod53, alvo53);
            // batch 46: preferences (nulls, each of the 3 slots, absent) and mark by position
            let h54 = s.next_u64();
            let pf = prefs46.as_mut_ptr() as usize;
            let pe46 = pessoa46.as_mut_ptr() as usize;
            let cand = [j, o, 0, 0x1234usize]; // 0: empty slot (the null target cannot match it)
            for k in 0..6usize {
                *((pf + 0x1c + 4 * k) as *mut usize) = cand[((h54 >> (k * 2)) % 4) as usize];
            }
            *((pe46 + 0x65) as *mut usize) = if (h54 >> 12) % 6 == 0 { 0 } else { pf };
            let alvo54 = [0usize, j, o, 0, 0x1234][((h54 >> 15) % 5) as usize];
            let quem54 = if (h54 >> 18) % 7 == 0 { 0 } else { pe46 };
            compare_likes(alvo54, quem54);
            compare_dislikes(alvo54, quem54);
            // batch 72: preferred clubs (+0x04..+0x0c) and avoided ones (+0x10..+0x18)
            for k in 0..6usize {
                *((pf + 4 + 4 * k) as *mut usize) = cand[((h54 >> (k * 2 + 22)) % 4) as usize];
            }
            compare_likes_club(quem54, alvo54);
            compare_dislikes_club(quem54, alvo54);
            let base54 = m + 0x91e2; // side 0: +0x187f still inside the fake engine
            *((base54 + 0x187f) as *mut u32) = (h54 >> 8) as u32 | 1;
            let v54 = (h54 >> 20) as u32;
            let ficha54: *const u32 = if (h54 >> 52) % 4 == 0 { core::ptr::null() } else { &v54 };
            compare_mark_position(base54, ((h54 >> 54) % 17) as i8 - 1, ficha54);
            // batch 48: leaves of other modules (game globals swapped during the test)
            let h55 = s.next_u64();
            let globais = [0xae2378usize, G_STAFF_COUNT, G_STAFF_STATE_TABLE, G_CLUB_COUNT, G_NATION_COUNT];
            let antes55: Vec<u32> = globais.iter().map(|&a| *(a as *const u32)).collect();
            compare_units_digit([0i32, 9, 10, 999, 1000, 1001, -1, -10, -999, i32::MIN, ((h55 >> 3) % 3000) as i32 - 1000][((h55 >> 60) % 11) as usize]);
            let ncomp = 200 + ((h55 >> 8) % 100) as i32;
            *(G_CLUB_COMP_COUNT as *mut i32) = ncomp;
            let mut comp55 = [0i32; 2];
            comp55[0] = [ncomp - 0x7f, ncomp - 0x80, ncomp, 0, -5, ((h55 >> 16) % 400) as i32][((h55 >> 24) % 6) as usize];
            let obj55 = [0usize, if (h55 >> 27) % 4 == 0 { comp55.as_ptr() as usize } else { 0x4321 }];
            compare_is_friendly(obj55.as_ptr() as usize, comp55.as_ptr() as usize);
            let npes = 50 + ((h55 >> 30) % 20) as i32;
            *(G_STAFF_COUNT as *mut i32) = npes;
            for k in 0..80usize {
                *((tabela55.as_mut_ptr() as usize + 79 * k) as *mut i32) = if (h55 >> (k % 40)) % 5 == 0 { -1 - (k as i32 % 3) } else { (k * 3) as i32 };
            }
            *(G_STAFF_STATE_TABLE as *mut usize) = tabela55.as_mut_ptr() as usize;
            let pessoa55 = [[0i32, 1, npes - 1, npes, npes + 3, 2, ((h55 >> 36) % 70) as i32][((h55 >> 40) % 7) as usize]];
            let contratos55 = [0x10000usize, 0];
            compare_contract_of(contratos55.as_ptr() as usize, if (h55 >> 43) % 6 == 0 { 0 } else { pessoa55.as_ptr() as usize });
            *(G_CLUB_COUNT as *mut i32) = 5000 + ((h55 >> 46) % 100) as i32;
            *(G_NATION_COUNT as *mut i32) = 200 + ((h55 >> 50) % 50) as i32;
            let limite55 = *(G_CLUB_COUNT as *const i32) - 2 * *(G_NATION_COUNT as *const i32);
            let clube55 = [[limite55, limite55 - 1, limite55 + 1, 0, 9999, -3][((h55 >> 54) % 6) as usize]];
            compare_is_national_team(clube55.as_ptr() as usize);
            for (i, &a) in globais.iter().enumerate() {
                *(a as *mut u32) = antes55[i];
            }
            // batch 49: human manager (indices at the edges, no person for the current manager),
            // cell codes (all valid ones and neighbours) and formation masks
            let h56 = s.next_u64();
            let globais56 = [0xae2364usize, G_SCREEN_MANAGER_CURRENT_SLOT];
            let antes56: Vec<u32> = globais56.iter().map(|&a| *(a as *const u32)).collect();
            let npes56 = 1000 + ((h56 >> 2) % 50) as i32;
            *(G_STAFF_COUNT as *mut i32) = npes56;
            let atual56 = ((h56 >> 8) % 4) as i16;
            *(G_SCREEN_MANAGER_CURRENT_SLOT as *mut i16) = atual56;
            let tab56 = G_SCREEN_RECORDS_MANAGER + 768 * atual56 as usize;
            let antes_tab56 = *(tab56 as *const u32);
            let ntec = 1 + ((h56 >> 12) % 16) as i32;
            let regs56 = tecnicos56.as_mut_ptr() as usize;
            for k in 0..16usize {
                let r = regs56 + 42735 * k;
                *((r + 0x9a01) as *mut u32) = (h56 as u32).rotate_left(k as u32);
                *((r + 0x9a05) as *mut u8) = if (h56 >> (k + 20)) % 3 == 0 { 0 } else { (k as u8) | 0x40 };
            }
            let obj56 = [regs56, ntec as usize];
            let id56 = [[npes56 - 16, npes56 - 17, npes56 - 16 + ntec - 1, npes56 - 16 + ntec, npes56 - 1, npes56 - 20 + ((h56 >> 40) % 30) as i32][((h56 >> 44) % 6) as usize]];
            *(tab56 as *mut usize) = id56.as_ptr() as usize;
            let quem56 = if (h56 >> 47) % 4 == 0 { 0 } else { id56.as_ptr() as usize };
            compare_is_on_holiday(obj56.as_ptr() as usize, quem56);
            let dentro56 = id56[0] - npes56 + 16;
            if (0..ntec).contains(&dentro56) {
                // batch 62: 0x9ace and 0x9a06 (person with club and nation of +0x88 at the edges)
                let mut nacao62 = [0u8; 0x90];
                *(nacao62.as_mut_ptr().add(0x88) as *mut i32) = [399, 400, 0, -1, 401, 5000][((h56 >> 50) % 6) as usize];
                let mut clube62 = [0u8; 0x60];
                *(clube62.as_mut_ptr().add(0x53) as *mut usize) = nacao62.as_ptr() as usize;
                let mut pessoa62 = [0u32; 16];
                pessoa62[0] = id56[0] as u32;
                pessoa62[9] = if (h56 >> 53) % 3 == 0 { 0 } else { clube62.as_ptr() as u32 };
                for k in 0..16usize {
                    let r = regs56 + 42735 * k;
                    *((r + 0x9ace) as *mut u32) = (h56 as u32).rotate_right(k as u32);
                    *((r + 0x9a06) as *mut u8) = if (h56 >> (k + 24)) % 3 == 0 { 0 } else { 0x80 | k as u8 };
                }
                *(tab56 as *mut usize) = pessoa62.as_ptr() as usize;
                // manager 0 with another valid person: picking the wrong slot shows up
                let mut outra62 = [0u32; 16];
                outra62[0] = (npes56 - 16 + (dentro56 + 1) % ntec) as u32;
                let antes_vaga0 = *(G_SCREEN_RECORDS_MANAGER as *const u32);
                if atual56 != 0 {
                    *(G_SCREEN_RECORDS_MANAGER as *mut usize) = outra62.as_ptr() as usize;
                }
                let quem62 = if (h56 >> 47) % 4 == 0 { 0 } else { pessoa62.as_ptr() as usize };
                compare_manager_9ace(obj56.as_ptr() as usize, quem62);
                for k in 0..16usize {
                    for b in 0..0x30usize {
                        *((regs56 + 42735 * k + 0x9b20 + b) as *mut u8) = (h56.rotate_left((k + b) as u32) as u8) ^ (b as u8);
                    }
                }
                compare_manager_9b23(obj56.as_ptr() as usize, ((h56 >> 57) % 40) as i8 - 3, quem62);
                let mut saida62 = 7u32;
                let sp = if (h56 >> 55) % 2 == 0 { core::ptr::null_mut() } else { &mut saida62 as *mut u32 };
                compare_manager_9a06(obj56.as_ptr() as usize, quem62, sp);
                // batch 63: club controlled by a human (club, national side and second national side
                // that falls back to the main one; manager of the club, another or none; with and without tactics)
                let h63 = s.next_u64();
                let globais63 = [0xb63c98usize, G_CLUB_COUNT, G_NATION_COUNT, G_CLUB_TABLE, G_NATION_TABLE];
                let antes63: Vec<u32> = globais63.iter().map(|&a| *(a as *const u32)).collect();
                *(G_HUMAN_MANAGER as *mut usize) = obj56.as_ptr() as usize;
                *(G_CLUB_COUNT as *mut i32) = 5000;
                *(G_NATION_COUNT as *mut i32) = 200;
                for k in 0..16usize {
                    let r = regs56 + 42735 * k;
                    if (h63 >> k) & 1 == 0 {
                        *((r + 0x9ace) as *mut u32) = 0;
                    }
                    if (h63 >> (k + 16)) & 1 == 0 {
                        *((r + 0x9a01) as *mut u32) = 0;
                    }
                }
                let tecnicos63 = [0usize, pessoa62.as_ptr() as usize, outra62.as_ptr() as usize];
                let mut clube_b63 = [0u8; 0x100];
                *(clube_b63.as_mut_ptr() as *mut i32) = 100;
                *(clube_b63.as_mut_ptr().add(0xcf) as *mut usize) = tecnicos63[((h63 >> 34) % 3) as usize];
                let nacao63 = [0i32; 4];
                *(G_CLUB_TABLE as *mut usize) = (clube_b63.as_ptr() as usize).wrapping_sub(club::SIZE * 4600);
                *(G_NATION_TABLE as *mut u32) = 1;
                let mut clube_a63 = [0u8; 0x100];
                *(clube_a63.as_mut_ptr() as *mut i32) = [100i32, 4650, 4850][((h63 >> 36) % 3) as usize];
                *(clube_a63.as_mut_ptr().add(0xcf) as *mut usize) = tecnicos63[((h63 >> 38) % 3) as usize];
                *(clube_a63.as_mut_ptr().add(0x53) as *mut usize) = nacao63.as_ptr() as usize;
                compare_human_club(
                    clube_a63.as_ptr() as usize,
                    ((h63 >> 40) & 1) as u32,
                    ((h63 >> 41) & 1) as u8,
                    tecnicos63[((h63 >> 42) % 3) as usize],
                    ((h63 >> 44) % 3 == 0) as u32,
                );
                // batch 82: class of the national side (clube_a63 goes through 0x5409b0 as in 63)
                compare_national_side_class(clube_a63.as_ptr() as usize, ((h63 >> 45) & 1) as u32);
                // batch 83: class by the person's national side (no national side or with clube_a63)
                let mut pessoa83 = [0u8; 0x60];
                *(pessoa83.as_mut_ptr().add(0x24) as *mut usize) = if (h63 >> 47) % 4 == 0 { 0 } else { clube_a63.as_ptr() as usize };
                pessoa83[0x5e] = (h63 >> 50) as u8;
                compare_class_by_national_side(pessoa83.as_ptr() as usize);
                // batch 84: bytes of the national side's block, special national sides (0x9d00a4/a8 with the
                // ids of clube_a63; 100 is not a national side, only it separates 0x9d00a8) and the person who dislikes the main national side (clube_b63)
                let mut blocos84 = vec![0u8; 110 * 2];
                for (k, b) in blocos84.iter_mut().enumerate() {
                    *b = (h63.rotate_left(k as u32 % 64) as u8) ^ (k as u8).wrapping_mul(7);
                }
                let obj84 = [0usize, blocos84.as_mut_ptr() as usize];
                compare_national_side_byte_42(obj84.as_ptr() as usize, clube_a63.as_ptr() as usize, ((h63 >> 48) & 1) as u32);
                compare_national_side_byte_43(obj84.as_ptr() as usize, clube_a63.as_ptr() as usize, ((h63 >> 49) & 1) as u32);
                let especiais84 = (*(G_KEY_CLUB_MLS_ALL_STARS_EAST as *const u32), *(G_KEY_CLUB_MLS_ALL_STARS_WEST as *const u32));
                *(G_KEY_CLUB_MLS_ALL_STARS_EAST as *mut u32) = [4650u32, 7, 9][((h63 >> 51) % 3) as usize];
                *(G_KEY_CLUB_MLS_ALL_STARS_WEST as *mut u32) = [100u32, 8][((h63 >> 53) % 2) as usize];
                compare_national_or_special(clube_a63.as_ptr() as usize);
                *(G_KEY_CLUB_MLS_ALL_STARS_EAST as *mut u32) = especiais84.0;
                *(G_KEY_CLUB_MLS_ALL_STARS_WEST as *mut u32) = especiais84.1;
                let mut pref84 = [0usize; 8];
                pref84[4 + ((h63 >> 54) % 3) as usize] = if (h63 >> 56) & 1 == 0 { clube_b63.as_ptr() as usize } else { 1 };
                let mut pessoa84 = [0u8; 0x70];
                *(pessoa84.as_mut_ptr().add(0x1a) as *mut usize) = nacao63.as_ptr() as usize;
                *(pessoa84.as_mut_ptr().add(0x65) as *mut usize) = pref84.as_ptr() as usize;
                compare_dislikes_national_side(pessoa84.as_ptr() as usize);
                // batch 75: fog of war (today's date, knowledge table,
                // synthetic persons; both sides with the synthetic human club)
                {
                    let h75 = s.next_u64();
                    let globais75 = [0xae2a60usize, G_STAFF_TABLE, G_CURRENT_DATE, G_CURRENT_DATE_LEAP];
                    let antes75: Vec<u32> = globais75.iter().map(|&a| *(a as *const u32)).collect();
                    *(G_CURRENT_DATE as *mut u32) = (h75 >> 3) as u32;
                    *(G_CURRENT_DATE_LEAP as *mut u32) = (h75 >> 29) as u32;
                    let mut conhec75 = vec![0u8; 9 * 1100];
                    for (k, b) in conhec75.iter_mut().enumerate() {
                        *b = if k % 9 == 0 { (h75.rotate_left(k as u32 % 61) % 4) as u8 } else { (k as u8).wrapping_mul(37) };
                    }
                    let obj75 = [conhec75.as_mut_ptr() as usize, 0];
                    let mut quadro75 = [0usize; 32];
                    for k in 0..32usize {
                        quadro75[k] = if (h75 >> (k % 40 + 5)) % 5 == 0 { 0 } else { obj75.as_ptr() as usize };
                    }
                    *(G_FOG_OF_WAR as *mut usize) = quadro75.as_ptr() as usize;
                    let mut pessoas75 = vec![0u8; 0x6e * 64];
                    for k in 0..64usize {
                        let z = h75.rotate_left(k as u32) ^ (k as u64 * 0x9e37);
                        *(pessoas75.as_mut_ptr().add(0x6e * k) as *mut i32) = if z % 9 == 0 { npes56 + ((z >> 8) % 3) as i32 } else { (k * 13) as i32 % npes56 };
                    }
                    *(G_STAFF_TABLE as *mut usize) = pessoas75.as_ptr() as usize;
                    let mut dados75 = vec![0u8; 0x1000];
                    for lado in 0..2usize {
                        for v in 0..20usize {
                            let z = h75.rotate_right((lado * 20 + v) as u32) ^ (v as u64 * 0x5bd1);
                            let e = dados75.as_mut_ptr().add(lado * 0x625 + 0x190 + v * 0x49);
                            *(e as *mut i32) = if z % 7 == 0 { -1 } else { ((z >> 4) % 64) as i32 };
                            *(e.add(4) as *mut i32) = if (z >> 12) % 7 == 0 { -1 } else { 5 };
                            *e.add(0x32) = if (z >> 16) % 3 == 0 { 0xff } else { 1 };
                        }
                    }
                    let mut partida75 = [0usize; 9];
                    partida75[7] = clube_a63.as_ptr() as usize;
                    // side 1 (+0x1c) with another manager (or none): swapping the sides shows up
                    let mut clube_1_75 = clube_a63;
                    *(clube_1_75.as_mut_ptr().add(0xcf) as *mut usize) = tecnicos63[((h75 >> 1) % 3) as usize];
                    partida75[8] = clube_a63.as_ptr() as usize;
                    partida75[7] = clube_1_75.as_ptr() as usize;
                    compare_fog_after_match(partida75.as_ptr() as usize, dados75.as_ptr() as usize);
                    // the person directly, with and without a scout (the RNG only comes in with one)
                    let mut obs75 = [0u8; 0x20];
                    *(obs75.as_mut_ptr().add(4) as *mut i16) = if (h75 >> 40) & 1 == 0 { (h75 >> 41) as i16 } else { ((h75 >> 41) % 1200) as i16 - 600 };
                    obs75[0x17] = (h75 >> 57) as u8;
                    let mut fonte75 = [0u8; 0x70];
                    *(fonte75.as_mut_ptr().add(0x69) as *mut usize) = if (h75 >> 58) % 4 == 0 { 0 } else { obs75.as_ptr() as usize };
                    let quem75 = [[((h75 >> 20) % npes56 as u64) as i32, npes56, npes56 - 1, 0][((h75 >> 60) % 4) as usize]];
                    compare_know_person(obj75.as_ptr() as usize, if (h75 >> 62) & 1 == 1 && (h75 >> 61) & 1 == 1 { 0 } else { quem75.as_ptr() as usize }, if (h75 >> 59) % 5 == 0 { 0 } else { fonte75.as_ptr() as usize });
                    compare_chance_to_know(obs75.as_ptr() as usize, (h75 >> 33) as i8);
                    // batch 76: watch list (repeated ids at +0xf and +0x13, types
                    // 11 and others) and contract flags
                    let h76 = s.next_u64();
                    let mut entradas76 = [0u8; 40 * 37];
                    for k in 0..40usize {
                        let z = h76.rotate_left(k as u32 * 3) ^ (k as u64 * 0x2545);
                        entradas76[37 * k] = if z % 3 == 0 { 11 } else { (z >> 2) as u8 % 14 };
                        *(entradas76.as_mut_ptr().add(37 * k + 0xf) as *mut u32) = 700 + ((z >> 8) % 6) as u32;
                        *(entradas76.as_mut_ptr().add(37 * k + 0x13) as *mut u32) = 700 + ((z >> 16) % 6) as u32;
                    }
                    let id76 = ((h76 >> 30) % 20) as usize;
                    let mut listas76 = [0u8; 9 * 20 + 4];
                    *(listas76.as_mut_ptr().add(9 * id76) as *mut usize) = entradas76.as_ptr() as usize;
                    let mut gerente76 = [0u8; 0xe0];
                    *(gerente76.as_mut_ptr().add(0xdc) as *mut usize) = listas76.as_ptr() as usize;
                    let pessoa76 = [id76 as u32];
                    let alvo76 = [700 + ((h76 >> 40) % 8) as u32];
                    compare_shortlist_entry(gerente76.as_ptr() as usize, pessoa76.as_ptr() as usize, alvo76.as_ptr() as usize, ((h76 >> 50) & 1) as u32, ((h76 >> 51) & 1) as u32);
                    let mut contrato76 = [0u32; 0x14];
                    for (k, c) in contrato76.iter_mut().enumerate() {
                        *c = (h76 as u32).rotate_left(k as u32) ^ k as u32;
                    }
                    compare_contract_flags(contrato76.as_ptr() as usize);
                    // batch 77: linked list (nodes of 10 bytes, unaligned), word
                    // +0xc in records of 31 bytes, rounding and nations of the group
                    let h77 = s.next_u64();
                    let mut nos77 = [0u8; 10 * 4 + 1];
                    let base77 = nos77.as_mut_ptr() as usize + 1;
                    for k in 0..4usize {
                        core::ptr::write_unaligned((base77 + 10 * k) as *mut u32, (h77 >> (8 * k)) as u32);
                        core::ptr::write_unaligned((base77 + 10 * k + 6) as *mut usize, if k == 3 || (h77 >> (k + 40)) % 4 == 0 { 0 } else { base77 + 10 * (k + 1) });
                    }
                    let mut cursor77 = [0u8; 0x10];
                    core::ptr::write_unaligned(cursor77.as_mut_ptr().add(0xa) as *mut usize, if (h77 >> 45) % 5 == 0 { 0 } else { base77 + 10 * ((h77 >> 46) % 4) as usize });
                    for _ in 0..3 {
                        compare_next_list_item(cursor77.as_mut_ptr() as usize);
                    }
                    let n77 = *(G_STAFF_COUNT as *const i32);
                    let mut regs77 = vec![0u8; 31 * 8];
                    for (k, b) in regs77.iter_mut().enumerate() {
                        *b = (h77.rotate_left(k as u32 % 64) as u8) ^ (k as u8).wrapping_mul(29);
                    }
                    let id77 = [n77 - 1, n77, n77 + 1, n77 - 8 + ((h77 >> 50) % 8) as i32][((h77 >> 48) % 4) as usize];
                    let tab77 = [(regs77.as_ptr() as usize).wrapping_sub(31usize.wrapping_mul((n77 - 8) as usize))];
                    compare_word_0c(tab77.as_ptr() as usize, id77);
                    let v77 = [(h77 >> 10) as i32, 990 + ((h77 >> 20) % 20) as i32, -(((h77 >> 30) % 30000) as i32), ((h77 >> 3) as i32 & !0xffff) | 999][((h77 >> 56) % 4) as usize];
                    compare_round_below_1000(v77);
                    let globais77 = [KEY_CLUB_COMP_B_INTERNATIONAL, KEY_CLUB_COMP_RESERVE, KEY_CLUB_COMP_UNDER_21_INTERNATIONAL, KEY_CLUB_COMP_SPANISH_LOWER_DIVISION, KEY_CLUB_COMP_FRENCH_CFA, KEY_CLUB_COMP_FRENCH_LOWER_DIVISION];
                    let antes77: Vec<u32> = globais77.iter().map(|&a| *(a as *const u32)).collect();
                    for (k, &a) in globais77.iter().enumerate() {
                        *(a as *mut u32) = 60 + k as u32;
                    }
                    let nacao77 = [58 + ((h77 >> 58) % 10) as u32];
                    compare_group_nation(if (h77 >> 62) == 0 { 0 } else { nacao77.as_ptr() as usize });
                    for (k, &a) in globais77.iter().enumerate() {
                        *(a as *mut u32) = antes77[k];
                    }
                    // batch 78: competition that counts (3 globals, table of 107 bytes,
                    // objects with stage, rounds and the first round) and the sorting comparisons
                    let h78 = s.next_u64();
                    let globais78 = [KEY_CLUB_COMP_SPANISH_LOWER_DIVISION, KEY_CLUB_COMP_FRENCH_CFA, KEY_CLUB_COMP_FRENCH_LOWER_DIVISION, G_CLUB_COMP_TABLE, G_COMP_OBJECT_TABLE];
                    let antes78: Vec<u32> = globais78.iter().map(|&a| *(a as *const u32)).collect();
                    for k in 0..3usize {
                        *(globais78[k] as *mut i32) = 10 + k as i32;
                    }
                    let mut tabela78 = [0u8; 107 * 16];
                    let mut rodada78 = [0u8; 0x70];
                    let primeira78 = [rodada78.as_mut_ptr() as usize];
                    let mut objs78 = [[0u8; 0xb8]; 16];
                    let mut ptrs78 = [0usize; 16];
                    for k in 0..16usize {
                        let z = h78.rotate_left(k as u32 * 4) ^ (k as u64 * 0x9e3);
                        *(tabela78.as_mut_ptr().add(107 * k + 0x5d) as *mut u32) = if z % 4 == 0 { 0 } else { z as u32 | 1 };
                        let o = &mut objs78[k];
                        o[0x43] = [2u8, 2, 1, 0][((z >> 3) % 4) as usize];
                        *(o.as_mut_ptr().add(0x3e) as *mut u16) = if (z >> 5) % 5 == 0 { 0 } else { 3 };
                        *(o.as_mut_ptr().add(0xb1) as *mut usize) = if (z >> 8) % 6 == 0 { 0 } else { primeira78.as_ptr() as usize };
                        *(o.as_mut_ptr().add(4) as *mut i32) = 500 + ((z >> 12) % 3) as i32;
                        ptrs78[k] = if (z >> 16) % 7 == 0 { 0 } else { o.as_ptr() as usize };
                    }
                    *(rodada78.as_mut_ptr().add(0x57) as *mut i32) = 500 + ((h78 >> 40) % 3) as i32;
                    *(rodada78.as_mut_ptr().add(0x60) as *mut i32) = 500 + ((h78 >> 42) % 3) as i32;
                    *(G_CLUB_COMP_TABLE as *mut usize) = tabela78.as_ptr() as usize;
                    *(G_COMP_OBJECT_TABLE as *mut usize) = ptrs78.as_ptr() as usize;
                    compare_competition_counts(((h78 >> 44) % 16) as i32);
                    for (k, &a) in globais78.iter().enumerate() {
                        *(a as *mut u32) = antes78[k];
                    }
                    let mut fichas78 = [[0u8; 0x84]; 2];
                    for (k, f) in fichas78.iter_mut().enumerate() {
                        f[0x5f] = (h78 >> (k * 8 + 1)) as u8;
                        *(f.as_mut_ptr().add(0x80) as *mut i16) = (h78 >> (k * 16 + 20)) as i16;
                    }
                    let pa78 = [if (h78 >> 60) % 4 == 0 { 0 } else { fichas78[0].as_ptr() as usize }];
                    let pb78 = [if (h78 >> 62) == 0 { 0 } else { fichas78[1].as_ptr() as usize }];
                    compare_order_by_5f(pa78.as_ptr(), pb78.as_ptr());
                    compare_order_by_80_desc(pa78.as_ptr(), pb78.as_ptr());
                    compare_order_by_80_asc(pa78.as_ptr(), pb78.as_ptr());
                    let qa78 = [fichas78[0].as_ptr() as usize];
                    let qb78 = [fichas78[1].as_ptr() as usize];
                    compare_order_by_80(qa78.as_ptr(), qb78.as_ptr());
                    // batch 80: fog tables (16, some null or inactive; fog on or
                    // off) and the formation (cells of the eleven masks swapped, or random cells)
                    let h80 = s.next_u64();
                    let antes_nevoa80 = *(G_ATTRIBUTE_MASKING_ENABLED as *const u8);
                    *(G_ATTRIBUTE_MASKING_ENABLED as *mut u8) = ((h80 >> 1) % 4 != 0) as u8;
                    let mut niveis80 = vec![0u8; 9 * 70];
                    for (k, b) in niveis80.iter_mut().enumerate() {
                        *b = 1 + (k % 3) as u8;
                    }
                    let base80 = (niveis80.as_mut_ptr() as usize).wrapping_sub(9usize.wrapping_mul((n77 - 64) as usize));
                    let mut quadros80 = [[0usize; 2]; 16];
                    let mut ptrs80 = [0usize; 16];
                    for k in 0..16usize {
                        quadros80[k] = [base80, ((h80 >> (k + 3)) % 3 != 0) as usize];
                        ptrs80[k] = if (h80 >> (k + 20)) % 4 == 0 { 0 } else { quadros80[k].as_ptr() as usize };
                    }
                    let pessoa80 = [n77 - 64 + ((h80 >> 40) % 66) as i32];
                    compare_forget_person(ptrs80.as_ptr() as usize, if (h80 >> 47) % 8 == 0 { 0 } else { pessoa80.as_ptr() as usize });
                    *(G_ATTRIBUTE_MASKING_ENABLED as *mut u8) = antes_nevoa80;
                    let tecnico80 = [n77 - 1 - ((h80 >> 50) % 16) as i32];
                    compare_manager_table(ptrs80.as_ptr() as usize, if (h80 >> 54) % 6 == 0 { 0 } else { tecnico80.as_ptr() as usize });
                    let codigos80: Vec<u16> = core::iter::once(1u16)
                        .chain([0x80u16, 0x100, 0x200, 0x400, 0x800].iter().flat_map(|&l| [2u16, 4, 8, 0x10, 0x20, 0x40].iter().map(move |&c| l | c)))
                        .collect();
                    let mapeadas80 = [0x101176c1u32, 0x101b26c1, 0x38115381, 0x3a242381, 0x280e07c1, 0x091a07c1, 0x2884ab81, 0x280aab81, 0x288c07c1, 0x281507c1, 0x288a8b81];
                    let mut casas80 = [0u16; 11];
                    if (h80 >> 56) % 2 == 0 {
                        let alvo = mapeadas80[((h80 >> 57) % 11) as usize];
                        let mut usados = 0u32;
                        let mut n = 0;
                        for &c in &codigos80 {
                            let b = square_bit(c as i16);
                            if b & alvo != 0 && b & usados == 0 && n < 11 {
                                casas80[n] = c;
                                usados |= b;
                                n += 1;
                            }
                        }
                    } else {
                        for k in 0..11usize {
                            let z = h80.rotate_left(k as u32 * 5);
                            casas80[k] = if z % 9 == 0 { 0x1234 } else { codigos80[(z >> 4) as usize % codigos80.len()] };
                        }
                    }
                    let mut form80 = vec![0u8; 0x1880];
                    for k in 0..11usize {
                        core::ptr::write_unaligned(form80.as_mut_ptr().add(0x12d + 2 * k) as *mut u16, casas80[k]);
                        form80[0x5b4 + k] = (h80 >> (k * 3)) as u8 & 0x1e;
                        core::ptr::write_unaligned(form80.as_mut_ptr().add(0x5bf + 4 * k) as *mut u32, (h80 >> k) as u32);
                    }
                    let f80 = form80.as_mut_ptr() as usize;
                    compare_formation_mask(f80);
                    let codigo80 = if (h80 >> 60) % 3 == 0 { 0x1234 } else { casas80[((h80 >> 33) % 11) as usize] };
                    compare_slot_of_code(f80, codigo80);
                    let mut jogadores80 = vec![0u8; 20 * 0x1be];
                    for i in 0..20usize {
                        let z = h80.rotate_right(i as u32 * 3);
                        jogadores80[i * 0x1be + 0x19] = [0xfeu8, 0xff, (z % 11) as u8, (z % 13) as u8][((z >> 8) % 4) as usize];
                        jogadores80[i * 0x1be + 0x1a] = ((z >> 12) % 12) as u8;
                    }
                    // always with a sector bit (1..0x40): without it the original shows formation.cpp's error box
                    let mascara80 = ((h80 >> 13) as u32 & 0xfff) | 0x200 | 1 << ((h80 >> 5) % 7);
                    compare_sector_slot_taken(f80, mascara80, if (h80 >> 62) == 0 { 0 } else { jogadores80.as_ptr() as usize });
                    compare_clear_assigned(f80);
                    // batch 81: cell of the slot (only slots 0..10: outside them the original shows the
                    // error box) and the criterion by the record's bytes (the words need the configuration
                    // of 0xae3138, which does not exist here yet; they are left to the real game)
                    let h81 = s.next_u64();
                    for k in 0..0x30usize {
                        form80[0x12d + k] = (h81 >> (k % 56)) as u8 ^ (k as u8).wrapping_mul(13);
                    }
                    compare_slot_code(f80, ((h81 >> 2) % 11) as i8, ((h81 >> 6) & 1) as u32);
                    let mut registro81 = [0u8; 0x60];
                    registro81[0x37] = (h81 >> 8) as u8;
                    registro81[0x52] = (h81 >> 16) as u8;
                    let a81 = ((h81 >> 24) as u32) & 0xffff_0000;
                    let b81 = ((h81 >> 40) as u32) & 0xffff_0000;
                    // batch 82: types, marks by person and the free record (synthetic table,
                    // index kept or not; never 0 records: 0x90cfc0(0) is not a game case)
                    let h82 = s.next_u64();
                    compare_type_not_11(((h82 >> 1) % 20) as i8 - 2);
                    compare_type_11_to_15(((h82 >> 6) % 20) as i8 - 2);
                    let mut marcas82 = vec![0u8; 110 * 12 + 0x100];
                    for (k, b) in marcas82.iter_mut().enumerate() {
                        *b = (h82.rotate_left(k as u32 % 64) as u8) | 1;
                    }
                    let obj82 = [0usize, marcas82.as_mut_ptr() as usize + 0x40];
                    let pessoa82 = [((h82 >> 11) % 10) as i32];
                    let k82 = ((h82 >> 15) % 9) as i8 - 2;
                    compare_read_person_mark(obj82.as_ptr() as usize, pessoa82.as_ptr() as usize, k82);
                    compare_clear_person_mark(obj82.as_ptr() as usize, pessoa82.as_ptr() as usize, ((h82 >> 19) % 9) as i8 - 2);
                    // batch 83: +0x44 (set and read, signed) and the address of +0x3a
                    let h83 = s.next_u64();
                    for (k, b) in marcas82.iter_mut().enumerate() {
                        *b = (h83.rotate_left(k as u32 % 64) as u8) ^ (k as u8);
                    }
                    let k83 = ((h83 >> 3) % 9) as i8 - 2;
                    compare_person_byte_44(obj82.as_ptr() as usize, pessoa82.as_ptr() as usize, k83);
                    compare_set_person_44(obj82.as_ptr() as usize, pessoa82.as_ptr() as usize, ((h83 >> 7) % 9) as i8 - 2);
                    compare_person_field_3a(obj82.as_ptr() as usize, pessoa82.as_ptr() as usize, ((h83 >> 11) % 9) as i8 - 2);
                    let antes83 = *(G_DATABASE_NAMES_LANGUAGE as *const u32);
                    *(G_DATABASE_NAMES_LANGUAGE as *mut u32) = (h83 >> 20) as u32;
                    compare_database_global_9b5984();
                    // batch 84: the person's competition (index +4 negative or not) and date with mark
                    let h84 = s.next_u64();
                    let antes84 = *(G_CLUB_COMP_TABLE as *const usize);
                    *(G_CLUB_COMP_TABLE as *mut usize) = 0x0200_0000;
                    for k in 0..10usize {
                        *(marcas82.as_mut_ptr().add(0x40 + 110 * k + 4) as *mut i32) = if (h84 >> k) % 4 == 0 { -1 - k as i32 } else { (h84 >> (k + 10)) as i32 & 0xff };
                    }
                    compare_person_competition(obj82.as_ptr() as usize, if (h84 >> 30) % 6 == 0 { 0 } else { pessoa82.as_ptr() as usize });
                    *(G_CLUB_COMP_TABLE as *mut usize) = antes84;
                    let origem84: [u8; 12] = core::array::from_fn(|k| (h84 >> (k * 5)) as u8);
                    let mut destino84 = [0u8; 12];
                    compare_copy_date_and_mark(destino84.as_mut_ptr() as usize, origem84.as_ptr() as usize);
                    *(G_DATABASE_NAMES_LANGUAGE as *mut u32) = antes83;
                    let globais82 = [0x9b59b4usize, G_COMMON_NAME_COUNT, G_COMMON_NAME_TABLE];
                    let antes82: Vec<u32> = globais82.iter().map(|&a| *(a as *const u32)).collect();
                    let n82 = 1 + ((h82 >> 23) % 8) as i32;
                    let mut tabela82 = vec![1u8; 60 * 9];
                    if (h82 >> 27) % 3 != 0 {
                        tabela82[60 * ((h82 >> 29) % 9) as usize] = 0;
                    }
                    *(G_COMMON_NAME_COUNT as *mut i32) = n82;
                    *(G_COMMON_NAME_TABLE as *mut usize) = tabela82.as_ptr() as usize;
                    *(G_EMPTY_COMMON_NAME_INDEX as *mut i32) = if (h82 >> 33) % 3 == 0 { ((h82 >> 35) % 9) as i32 } else { -1 };
                    compare_free_record();
                    for (k, &a) in globais82.iter().enumerate() {
                        *(a as *mut u32) = antes82[k];
                    }
                    compare_competition_criterion(if (h81 >> 60) % 5 == 0 { 0 } else { registro81.as_ptr() as usize }, a81, b81, [1u8, 0, 2, 0x81][((h81 >> 61) % 4) as usize]);
                    for (k, &a) in globais75.iter().enumerate() {
                        *(a as *mut u32) = antes75[k];
                    }
                }
                // batch 65: row and byte of the tactics table (single table, covers ids and humans)
                if TABELA65.is_empty() {
                    TABELA65 = vec![0u8; 0x91 * 5100];
                }
                for k in 0..0x91 * 40 {
                    TABELA65[0x91 * 4990 + k] = (h63 >> (k % 50)) as u8 ^ k as u8;
                }
                for &l in &[100usize, 4650, 4850] {
                    for k in 0..0x91 {
                        TABELA65[0x91 * l + k] = (h63 >> (k % 40)) as u8 ^ (l as u8);
                    }
                }
                let humano65 = ((h63 >> 54) & 1) as u32;
                compare_tactics_row(clube_a63.as_ptr() as usize, humano65);
                let mut obj65 = [0u8; 0x608];
                *(obj65.as_mut_ptr().add(0x601) as *mut usize) = TABELA65.as_ptr() as usize;
                compare_tactics_byte(obj65.as_mut_ptr() as usize, clube_a63.as_ptr() as usize, ((h63 >> 56) % 11) as i8, humano65);
                // batch 68: slots of the tactics row (indices at +2.., +0x52, +0x56) over
                // 30 synthetic persons of 110 bytes; the table is restored (mark_slot writes +0x52)
                {
                    let h68 = s.next_u64();
                    let mut pessoas68 = [0u8; 110 * 30];
                    for k in 0..30usize {
                        *(pessoas68.as_mut_ptr().add(110 * k) as *mut i32) = 7000 + (k as i32 % 25);
                    }
                    let antes68 = *(G_STAFF_TABLE as *const usize);
                    *(G_STAFF_TABLE as *mut usize) = pessoas68.as_ptr() as usize;
                    let linhas68: Vec<usize> = [100usize, 4650, 4850].into_iter().chain(4999..5033).collect();
                    let copia68: Vec<[u8; 0x91]> = linhas68.iter().map(|&l| {
                        let mut c = [0u8; 0x91];
                        c.copy_from_slice(&TABELA65[0x91 * l..0x91 * l + 0x91]);
                        c
                    }).collect();
                    for (n, &l) in linhas68.iter().enumerate() {
                        let r = TABELA65.as_mut_ptr().add(0x91 * l) as usize;
                        for v in 0..20usize {
                            let z = h68.rotate_left((v + n) as u32);
                            *((r + 2 + 4 * v) as *mut i32) = if z % 5 == 0 { -1 } else { ((z >> 8) % 30) as i32 };
                        }
                        *((r + 0x52) as *mut i32) = 7000 + ((h68 >> (n + 3)) % 30) as i32;
                        *((r + 0x56) as *mut i32) = [-1i32, 0, 5, 29, -7][((h68 >> (n + 9)) % 5) as usize];
                        *((r + 0x5a) as *mut i32) = [-1i32, 3, 12][((h68 >> (n + 17)) % 3) as usize];
                    }
                    let o68 = obj65.as_mut_ptr() as usize;
                    let c68 = clube_a63.as_ptr() as usize;
                    compare_marked_slot(o68, c68, humano65, ((h68 >> 40) & 1) as u32);
                    compare_mark_slot(o68, c68, ((h68 >> 41) % 20) as i8, humano65);
                    compare_row_person(o68, c68, ((h68 >> 46) % 2) as i8, humano65);
                    for (n, &l) in linhas68.iter().enumerate() {
                        TABELA65[0x91 * l..0x91 * l + 0x91].copy_from_slice(&copia68[n]);
                    }
                    *(G_STAFF_TABLE as *mut usize) = antes68;
                }
                // batch 69: club in the competition and special club
                {
                    let h69 = s.next_u64();
                    let mut cont69 = [[0u8; 0x80]; 2];
                    *(cont69[1].as_mut_ptr() as *mut i32) = 1;
                    let mut nac69 = [[0u8; 0x80]; 3];
                    for (k, n) in nac69.iter_mut().enumerate() {
                        *(n.as_mut_ptr().add(0x71) as *mut usize) = cont69[k % 2].as_ptr() as usize;
                    }
                    let mut base69 = [0u8; 0x80];
                    *(base69.as_mut_ptr().add(0x59) as *mut usize) = if (h69 >> 2) % 4 == 0 { 0 } else { cont69[0].as_ptr() as usize };
                    *(base69.as_mut_ptr().add(0x5d) as *mut usize) = nac69[(h69 % 3) as usize].as_ptr() as usize;
                    let mut clubes69 = [[0u8; 0x60]; 6];
                    for (k, c) in clubes69.iter_mut().enumerate() {
                        let z = h69.rotate_left(7 * k as u32);
                        *(c.as_mut_ptr() as *mut i32) = [100i32, 4799, 4800, 4900, 4650][(z % 5) as usize];
                        *(c.as_mut_ptr().add(0x53) as *mut usize) = if (z >> 3) % 5 == 0 { 0 } else { nac69[((z >> 5) % 3) as usize].as_ptr() as usize };
                        *(c.as_mut_ptr().add(0x57) as *mut usize) = [0usize, base69.as_ptr() as usize, 0x1234][((z >> 8) % 3) as usize];
                    }
                    let alvo69 = clubes69[((h69 >> 10) % 6) as usize].as_ptr() as usize;
                    // stages: participants (6 bytes) and tables (0x3b) with some of the clubs
                    let mut listas69 = [[0u8; 6 * 4]; 4];
                    let mut tabelas69 = [[0u8; 0x3b * 4]; 4];
                    let mut fases69 = [[0u8; 0xc0]; 4];
                    let mut comp69 = [0u8; 0xc0];
                    for f in 0..4usize {
                        let z = h69.rotate_right(9 * f as u32);
                        for k in 0..4usize {
                            let c = clubes69[((z >> (3 * k)) % 6) as usize].as_ptr() as usize;
                            *(listas69[f].as_mut_ptr().add(6 * k) as *mut usize) = c;
                            *(tabelas69[f].as_mut_ptr().add(0x3b * k) as *mut usize) = c;
                        }
                        fases69[f][0x43] = [0u8, 1, 2, 2][((z >> 20) % 4) as usize];
                        *(fases69[f].as_mut_ptr().add(0x3e) as *mut i16) = [0i16, 1, 2, 4, -1][((z >> 22) % 5) as usize];
                        *(fases69[f].as_mut_ptr().add(0xa7) as *mut usize) = listas69[f].as_ptr() as usize;
                        *(fases69[f].as_mut_ptr().add(0xb1) as *mut usize) = tabelas69[f].as_ptr() as usize;
                    }
                    let ponteiros69: [usize; 4] = [0, 1, 2, 3].map(|f| fases69[f].as_ptr() as usize);
                    *(comp69.as_mut_ptr().add(4) as *mut usize) = base69.as_ptr() as usize;
                    *(comp69.as_mut_ptr().add(0xc) as *mut usize) = ponteiros69.as_ptr() as usize;
                    *(comp69.as_mut_ptr().add(0x30) as *mut i32) = [-2i32, -1, 0, 1, 3][((h69 >> 30) % 5) as usize];
                    comp69[0x43] = [0u8, 2][((h69 >> 33) % 2) as usize];
                    *(comp69.as_mut_ptr().add(0x3e) as *mut i16) = 3;
                    *(comp69.as_mut_ptr().add(0xa7) as *mut usize) = listas69[3].as_ptr() as usize;
                    *(comp69.as_mut_ptr().add(0xb1) as *mut usize) = tabelas69[3].as_ptr() as usize;
                    compare_club_in_competition(comp69.as_mut_ptr() as usize, alvo69);
                    // special club: the id of one of the nine kept, or another
                    // (before the database loads the nine ids are 0: distinct values during the test)
                    let guardados = [KEY_CLUB_COMP_FIFA_WORLD_CUP, KEY_CLUB_COMP_FOOTBALL_TOURNAMENT_OF_THE_OLYMPIC_GAMES, KEY_CLUB_COMP_EUROPEAN_FOOTBALL_CHAMPIONSHIP, KEY_CLUB_COMP_COPA_AMERICA, KEY_CLUB_COMP_AFRICAN_CUP_OF_NATIONS, KEY_CLUB_COMP_ASIAN_CUP, KEY_CLUB_COMP_OCEANIA_NATIONS_CUP, KEY_CLUB_COMP_FOOTBALL_TOURNAMENT_OF_THE_ASIAN_GAMES, KEY_CLUB_COMP_CONFEDERATIONS_CUP];
                    let antes_g69: Vec<u32> = guardados.iter().map(|&a| *(a as *const u32)).collect();
                    for (k, &a) in guardados.iter().enumerate() {
                        *(a as *mut u32) = 3000 + 7 * k as u32;
                    }
                    let id69 = if (h69 >> 35) % 3 == 0 { 0x7fff_0000u32 } else { *(guardados[((h69 >> 37) % 9) as usize] as *const u32) };
                    let especial69 = [id69, 0];
                    compare_special_club(if (h69 >> 41) % 7 == 0 { 0 } else { especial69.as_ptr() as usize });
                    // batch 73: partner club (globals of the 82 pairs and of the 3 nations with
                    // distinct values; some partners -1 and -5 for the two acceptance tests)
                    {
                        let h73 = s.next_u64();
                        let nac73 = [KEY_NATION_SPAIN, KEY_NATION_GERMANY, KEY_NATION_PORTUGAL];
                        let mut todos73: Vec<usize> = nac73.to_vec();
                        for &(a, b) in A_B_CLUB_PAIRS.iter().chain(CLUB_PAIRS_B_A.iter()) {
                            todos73.push(a);
                            todos73.push(b);
                        }
                        let antes73: Vec<u32> = todos73.iter().map(|&a| *(a as *const u32)).collect();
                        for (k, &a) in nac73.iter().enumerate() {
                            *(a as *mut u32) = 900 + k as u32;
                        }
                        for (k, &(a, b)) in A_B_CLUB_PAIRS.iter().enumerate() {
                            *(a as *mut i32) = if (h73 >> (k % 50 + 3)) % 11 == 0 { -5 } else { 1000 + k as i32 };
                            *(b as *mut i32) = match (h73 >> (k % 60)) % 9 { 0 => -1, 1 => -5, _ => 2000 + k as i32 };
                        }
                        let ids73: Vec<u32> = A_B_CLUB_PAIRS.iter().map(|&(a, _)| *(a as *const u32))
                            .chain(A_B_CLUB_PAIRS.iter().map(|&(_, b)| *(b as *const u32))).collect();
                        let mut nacao73 = [0u32; 4];
                        nacao73[0] = [900u32, 901, 902, 903][((h73 >> 2) % 4) as usize];
                        let mut clube73 = [0u8; 0x250];
                        *(clube73.as_mut_ptr() as *mut u32) = if (h73 >> 5) % 5 == 0 { 7777 } else { ids73[((h73 >> 8) as usize) % ids73.len()] };
                        *(clube73.as_mut_ptr().add(0x53) as *mut usize) = if (h73 >> 16) % 9 == 0 { 0 } else { nacao73.as_ptr() as usize };
                        clube73[0x244] = ((h73 >> 20) % 3 != 0) as u8;
                        let mut eh73 = 5u32;
                        let ponteiro73 = if (h73 >> 23) % 3 == 0 { core::ptr::null_mut() } else { &mut eh73 as *mut u32 };
                        let antes_base73 = *(G_CLUB_TABLE as *const u32);
                        *(G_CLUB_TABLE as *mut u32) = 0x0100_0000;
                        compare_partner_club(clube73.as_mut_ptr() as usize, ponteiro73, ((h73 >> 25) & 1) as u32);
                        // batch 74: contract with the club and contract level
                        if CLUBES74.is_empty() {
                            CLUBES74 = vec![0u8; 581 * 2200];
                            for k in 0..2200usize {
                                *(CLUBES74.as_mut_ptr().add(581 * k) as *mut i32) = k as i32 - 16;
                            }
                        }
                        // from slot 16: partners with a negative index (-5) still fall inside the vector
                        *(G_CLUB_TABLE as *mut usize) = CLUBES74.as_ptr() as usize + club::SIZE * 16;
                        let h74 = s.next_u64();
                        let id_p74: i32 = [5i32, 40, 99999][((h74 >> 1) % 3) as usize];
                        let pessoa74 = [id_p74, 0, 0, 0];
                        let mut entrada74 = [0u8; 79];
                        *(entrada74.as_mut_ptr() as *mut i32) = [0i32, 1, 2, 3, -1][((h74 >> 3) % 5) as usize];
                        *(entrada74.as_mut_ptr().add(4) as *mut i32) = [0i32, 1, 2, 3, -1][((h74 >> 6) % 5) as usize];
                        let antes74 = (*(G_STAFF_STATE_TABLE as *const usize), *(G_CONTRACT_MANAGER as *const usize));
                        // limit of the national sides (clubs - 2 x nations) in the middle of the pair ids
                        let limites74 = (*(G_CLUB_COUNT as *const i32), *(G_NATION_COUNT as *const i32));
                        *(G_CLUB_COUNT as *mut i32) = 2100;
                        *(G_NATION_COUNT as *mut i32) = 400 + ((h74 >> 20) % 700) as i32;
                        *(G_STAFF_STATE_TABLE as *mut usize) = (entrada74.as_ptr() as usize).wrapping_sub((id_p74.max(0) as usize).wrapping_mul(staff_state::SIZE));
                        let id_c74 = *(clube73.as_ptr() as *const u32);
                        let mut regs74 = [0u8; 80 * 4];
                        for k in 0..4usize {
                            let z = h74.rotate_left(9 * k as u32);
                            *(regs74.as_mut_ptr().add(80 * k + 4) as *mut u32) = match z % 4 { 0 => id_c74, 1 => 2000 + (z >> 8) as u32 % 82, 2 => 1000 + (z >> 8) as u32 % 82, _ => 4321 };
                            regs74[80 * k + 0x4f] = (z >> 16) as u8;
                            regs74[80 * k + 0x44] = (z >> 24) as u8;
                        }
                        let obj74 = [regs74.as_ptr() as usize, 0];
                        *(G_CONTRACT_MANAGER as *mut usize) = obj74.as_ptr() as usize;
                        compare_contract_with_club(obj74.as_ptr() as usize, pessoa74.as_ptr() as usize, clube73.as_mut_ptr() as usize);
                        compare_contract_level(pessoa74.as_ptr() as usize, clube73.as_mut_ptr() as usize);
                        // batch 79: player rating (engine with clube73 on both sides,
                        // person with the same id, record, plausible today's and birth dates)
                        let h79 = s.next_u64();
                        let mut pessoa79 = [0u8; 0x50];
                        *(pessoa79.as_mut_ptr() as *mut i32) = id_p74;
                        pessoa79[0x22] = (h79 >> 3) as u8;
                        *(pessoa79.as_mut_ptr().add(0x3e) as *mut u16) = ((h79 >> 11) % 366) as u16;
                        *(pessoa79.as_mut_ptr().add(0x40) as *mut u16) = 1960 + ((h79 >> 20) % 45) as u16;
                        let mut ficha79 = [0u8; 0x30];
                        *(ficha79.as_mut_ptr().add(0xd) as *mut i16) = ((h79 >> 26) % 6000) as i16 - 1000;
                        ficha79[0x2f] = (h79 >> 40) as u8 % 30;
                        *(clube73.as_mut_ptr().add(0x80) as *mut i16) = ((h79 >> 48) % 4000) as i16;
                        // side 0 (+0x1c) with a copy of another shape (+0x80): swapping the sides shows up
                        let mut clube79 = clube73;
                        *(clube79.as_mut_ptr().add(0x80) as *mut i16) = ((h79 >> 30) % 4000) as i16 - 2000;
                        let mut tab79 = [0usize; 9];
                        tab79[7] = clube79.as_ptr() as usize;
                        tab79[8] = clube73.as_ptr() as usize;
                        let mut motor79 = vec![0u8; 0x4800];
                        *(motor79.as_mut_ptr().add(0x4792) as *mut usize) = tab79.as_ptr() as usize;
                        let mut jog79 = [0u8; 0x1a4];
                        jog79[0x27] = ((h79 >> 60) & 1) as u8;
                        *(jog79.as_mut_ptr().add(0x19e) as *mut usize) = motor79.as_ptr() as usize;
                        *(jog79.as_mut_ptr().add(0x69) as *mut usize) = pessoa79.as_ptr() as usize;
                        *(jog79.as_mut_ptr().add(0x6d) as *mut usize) = ficha79.as_ptr() as usize;
                        let hoje79 = (*(G_CURRENT_DATE as *const u32), *(G_CURRENT_DATE_LEAP as *const u32));
                        *(G_CURRENT_DATE as *mut u32) = (2001u32 << 16) | ((h79 >> 52) % 366) as u32;
                        *(G_CURRENT_DATE_LEAP as *mut u32) = 0;
                        compare_player_rating(jog79.as_ptr() as usize);
                        *(G_CURRENT_DATE as *mut u32) = hoje79.0;
                        *(G_CURRENT_DATE_LEAP as *mut u32) = hoje79.1;
                        *(G_STAFF_STATE_TABLE as *mut usize) = antes74.0;
                        *(G_CONTRACT_MANAGER as *mut usize) = antes74.1;
                        *(G_CLUB_COUNT as *mut i32) = limites74.0;
                        *(G_NATION_COUNT as *mut i32) = limites74.1;
                        *(G_CLUB_TABLE as *mut u32) = antes_base73;
                        for (k, &a) in todos73.iter().enumerate().rev() {
                            *(a as *mut u32) = antes73[k];
                        }
                    }
                    for (k, &a) in guardados.iter().enumerate() {
                        *(a as *mut u32) = antes_g69[k];
                    }
                }
                // batch 64: side under control (types 1 and 9, human club or not, modes 2..5)
                let mut partida64 = [0u8; 0x48];
                partida64[0x3a] = [0u8, 1, 9, 2][((h63 >> 46) % 4) as usize];
                partida64[0x40] = ((h63 >> 48) & 1) as u8;
                *(partida64.as_mut_ptr().add(0x1c) as *mut usize) = clube_a63.as_ptr() as usize;
                *(partida64.as_mut_ptr().add(0x20) as *mut usize) = clube_b63.as_ptr() as usize;
                let antes64 = (*((m + 0x4792) as *const usize), *((m + 0x1ab) as *const u16), *((m + 0x9766) as *const u32), *((m + 0x9766 + 6371) as *const u32));
                *((m + 0x4792) as *mut usize) = partida64.as_ptr() as usize;
                *((m + 0x1ab) as *mut u8) = 2 + ((h63 >> 49) % 4) as u8;
                *((m + 0x1ac) as *mut u8) = 2 + ((h63 >> 51) % 4) as u8;
                *((m + 0x9766) as *mut u32) = (h63 >> 20) as u32 & 0x4400;
                *((m + 0x9766 + 6371) as *mut u32) = (h63 >> 30) as u32 & 0x4400;
                compare_side_under_control(m, ((h63 >> 53) & 1) as i8);
                // batch 66: penalty order by the tactics table (both sides with a human
                // manager with mark 0x5f and without 0x9a01; +0x904b only with a national side, otherwise the choose_taker
                // of the autotest blocks could give a fatal error)
                if TABELA65.len() == 0x91 * 5100 {
                    let regs66 = |p: &[u32; 16]| regs56 + 42735 * (p[0] as i32 - npes56 + 16) as usize;
                    let mut tec66 = [pessoa62, outra62];
                    for t in tec66.iter_mut() {
                        *(t.as_mut_ptr() as *mut u8).add(0x5f) = 1;
                    }
                    for t in tec66.iter() {
                        *((regs66(t) + 0x9a01) as *mut u32) = 0;
                    }
                    partida64[0x41] = ((h63 >> 57) & 1) as u8;
                    let antes66 = (*(G_TACTICS as *const usize), *((m + 0x904d) as *const usize), *((m + 0x911d) as *const usize),
                        *((m + 0x904b) as *const u8), *((m + 0x911b) as *const u8));
                    // the mark goes to +0x8eda + 20 x side + table byte (signed): up to 128 bytes around
                    let mut marcas66 = [0u8; 0x140];
                    let mut ordem66 = [0u8; 22];
                    core::ptr::copy_nonoverlapping((m + 0x8eda - 0x80) as *const u8, marcas66.as_mut_ptr(), 0x140);
                    core::ptr::copy_nonoverlapping((m + 0x1af) as *const u8, ordem66.as_mut_ptr(), 22);
                    let mut obj66 = [0u8; 0x608];
                    *(obj66.as_mut_ptr().add(0x601) as *mut usize) = TABELA65.as_ptr() as usize;
                    *(G_TACTICS as *mut usize) = obj66.as_ptr() as usize;
                    let ta = ((h63 >> 58) & 1) as usize;
                    *((m + 0x904d) as *mut usize) = tec66[ta].as_ptr() as usize;
                    *((m + 0x911d) as *mut usize) = tec66[1 - ta].as_ptr() as usize;
                    let sel_a = *(clube_a63.as_ptr() as *const i32) >= 4600;
                    *((m + 0x904b) as *mut u8) = (sel_a && (h63 >> 59) & 1 == 1) as u8;
                    *((m + 0x911b) as *mut u8) = 0;
                    compare_penalty_order(m);
                    *(G_TACTICS as *mut usize) = antes66.0;
                    *((m + 0x904d) as *mut usize) = antes66.1;
                    *((m + 0x911d) as *mut usize) = antes66.2;
                    *((m + 0x904b) as *mut u8) = antes66.3;
                    *((m + 0x911b) as *mut u8) = antes66.4;
                    core::ptr::copy_nonoverlapping(marcas66.as_ptr(), (m + 0x8eda - 0x80) as *mut u8, 0x140);
                    core::ptr::copy_nonoverlapping(ordem66.as_ptr(), (m + 0x1af) as *mut u8, 22);
                }
                *((m + 0x4792) as *mut usize) = antes64.0;
                *((m + 0x1ab) as *mut u16) = antes64.1;
                *((m + 0x9766) as *mut u32) = antes64.2;
                *((m + 0x9766 + 6371) as *mut u32) = antes64.3;
                for (i, &a) in globais63.iter().enumerate() {
                    *(a as *mut u32) = antes63[i];
                }
                *(G_SCREEN_RECORDS_MANAGER as *mut u32) = antes_vaga0;
                *(tab56 as *mut usize) = id56.as_ptr() as usize;
                compare_manager_9a05(obj56.as_ptr() as usize, quem56);
            }
            *(tab56 as *mut u32) = antes_tab56;
            for (i, &a) in globais56.iter().enumerate() {
                *(a as *mut u32) = antes56[i];
            }
            let codigos56 = [1i16, 0x82, 0x84, 0x88, 0x90, 0xa0, 0xc0, 0x102, 0x104, 0x108, 0x110, 0x120, 0x140, 0x202, 0x204, 0x208, 0x210, 0x220, 0x240,
                0x402, 0x404, 0x408, 0x410, 0x420, 0x440, 0x802, 0x804, 0x808, 0x810, 0x820, 0x840];
            let c56 = codigos56[((h56 >> 50) % 31) as usize];
            // batch 50: second national side and aggregate score (draws, away goals, no first leg, bits of +0x38)
            let h57 = s.next_u64();
            let globais57 = [0xae235cusize, G_NATION_COUNT];
            let antes57: Vec<u32> = globais57.iter().map(|&a| *(a as *const u32)).collect();
            *(G_CLUB_COUNT as *mut i32) = 5000;
            *(G_NATION_COUNT as *mut i32) = 200;
            let clube57 = [[4800i32, 4799, 4801, 0, 4900][((h57 >> 2) % 5) as usize]];
            compare_is_second_national_team(clube57.as_ptr() as usize);
            // batch 60: club of the national side (arithmetic only; fake base and 0xae23a8)
            let globais60 = [0xae23b4usize, G_NATION_TABLE];
            let antes60: Vec<u32> = globais60.iter().map(|&a| *(a as *const u32)).collect();
            *(G_CLUB_TABLE as *mut u32) = 0x0100_0000 + ((h57 >> 53) as u32 & 0xfff) * 4;
            *(G_NATION_TABLE as *mut u32) = 0x0200_0000;
            let nacao60 = [[0i32, 1, 199, 200, -3, 7000][((h57 >> 30) % 6) as usize]];
            compare_main_national_team(nacao60.as_ptr() as usize);
            compare_b_national_team(nacao60.as_ptr() as usize);
            for (i, &a) in globais60.iter().enumerate() {
                *(a as *mut u32) = antes60[i];
            }
            for (i, &a) in globais57.iter().enumerate() {
                *(a as *mut u32) = antes57[i];
            }
            let pa57 = partida50.as_mut_ptr() as usize;
            *((pa57 + 0x30) as *mut i16) = [3i16, 13, 23, 4, 1003, 993, ((h57 >> 6) % 2000) as i16 - 500][((h57 >> 17) % 7) as usize];
            *((pa57 + 0x38) as *mut u16) = [0u16, 4, 0x4000, 0x4004, (h57 >> 20) as u16][((h57 >> 36) % 5) as usize];
            *((pa57 + 0x4b) as *mut i8) = [-1i8, 0, 1, 2, 3, 5][((h57 >> 39) % 6) as usize];
            *((pa57 + 0x4c) as *mut i8) = [0i8, 1, 2, 3, 4][((h57 >> 42) % 5) as usize];
            *((m + 0x4792) as *mut usize) = pa57;
            *((m + 0xf5bd) as *mut u8) = ((h57 >> 45) % 5) as u8;
            *((m + 0xf5f3) as *mut u8) = ((h57 >> 48) % 5) as u8;
            *((m + 0x8ed4) as *mut i16) = [0x3dei16, 0x3df, 0, 2000, -5][((h57 >> 51) % 5) as usize];
            let mut saidas57 = [0x77u8; 5];
            let sp = saidas57.as_mut_ptr();
            compare_aggregate_score(m, ((h57 >> 54) % 2) as i8, sp as *mut i8, sp.add(1) as *mut i8, sp.add(2), sp.add(3), sp.add(4), [0u8, 1, 0x80][((h57 >> 56) % 3) as usize]);
            // batch 51: direct marker (roles with sector and side, goalkeepers, outside, distances)
            let h58 = s.next_u64();
            let h59 = s.next_u64();
            for (ti, t) in tabelas.iter_mut().enumerate() {
                for (i, v) in t.iter_mut().enumerate() {
                    let _ = (h59, ti, i);
                    let r = s.next_u64(); // one draw per entry: rotations of the same number come out correlated
                    let setor = [1u16, 2, 4, 8, 0x10, 0x20, 0x40, 6, 0x48][(r % 9) as usize];
                    let lado_r = [0x80u16, 0x100, 0x200, 0x400, 0x800, 0x180, 0xa00][((r >> 4) % 7) as usize];
                    *v = setor | lado_r;
                }
            }
            for k in 0..40usize {
                let p = Engine(m).player(((k / 20)) as i8, k % 20).addr();
                let r = h58.rotate_left(k as u32 * 5);
                *((p + 0x19) as *mut i8) = if r % 5 == 0 { -1 } else { ((r >> 3) % 11) as i8 };
                *((p + 0x27) as *mut i8) = (k / 20) as i8;
                *((p + 0x19e) as *mut usize) = m;
            }
            for i in 0..10u32 {
                let k = ((h58 >> (i * 6)) % 40) as i8;
                compare_direct_marker(m, k / 20, k % 20);
            }
            // batch 59: designated marked player (marks 4 and 0x10, indices in the fake person vector,
            // opposing formation closed, 0x106, tables A/B of the marked player)
            {
                let mut pessoas59 = vec![0u8; 110 * 24];
                let pessoas_antes = *(G_STAFF_TABLE as *const usize);
                *(G_STAFF_TABLE as *mut usize) = pessoas59.as_mut_ptr() as usize;
                let r0 = s.next_u64();
                let l59 = (r0 % 2) as usize;
                let o59 = 1 - l59;
                let blocos = [Engine(m).player(0, 0).addr(), Engine(m).player(1, 0).addr()];
                let antes59: Vec<Vec<u8>> = blocos.iter().map(|&b| core::slice::from_raw_parts(b as *const u8, 20 * 0x1be).to_vec()).collect();
                let form = m + 0x91e2 + 6371 * l59;
                let antes_form = core::slice::from_raw_parts(form as *const u8, 6371).to_vec();
                let form_o = m + 0x91e2 + 6371 * o59;
                let antes_fo = *((form_o + 0x187b) as *const u32);
                let antes_af = *((m + 0xaf + o59) as *const u8);
                for k in 0..20usize {
                    let z = s.next_u64();
                    let q = Engine(m).player(o59 as i8, k).addr();
                    *((q + 0x19) as *mut i8) = if z % 5 == 0 { -1 } else { (k % 11) as i8 };
                    *((q + 0x69) as *mut usize) = pessoas59.as_ptr() as usize + 110 * ((z >> 4) % 24) as usize;
                    *((q + 0x27) as *mut i8) = o59 as i8;
                    *((q + 0x19e) as *mut usize) = if (z >> 9) % 7 == 0 { 0 } else { m };
                }
                for pos in 0..11usize {
                    let z = s.next_u64();
                    *((form + 0x5b4 + pos) as *mut u8) = [4u8, 0x14, 0x10, 0, 0x15, 5][(z % 6) as usize];
                    *((form + 0x5bf + 4 * pos) as *mut i32) = if z % 7 == 3 { -1 } else { ((z >> 4) % 24) as i32 };
                }
                *((form_o + 0x187b) as *mut u32) = if (r0 >> 2) % 3 == 0 { 0x1000_0000 } else { (r0 >> 8) as u32 & 0xefff_ffff };
                *((m + 0xaf + o59) as *mut u8) = [0u8, 1, 2, 5][((r0 >> 5) % 4) as usize];
                // tables A/B belong to the whole case: they are restored at the end (values from here lock later sections)
                let antes_tabelas: Vec<Vec<u16>> = tabelas.clone();
                for (ti, t) in tabelas.iter_mut().enumerate() {
                    for (i, v) in t.iter_mut().enumerate() {
                        let z = (r0 >> ((ti * 11 + i) % 50)) as u16;
                        *v = [0u16, 0x40, 0x41, 1, z][(z % 5) as usize];
                    }
                }
                for _ in 0..4 {
                    let z = s.next_u64();
                    let j = Engine(m).player(l59 as i8, (z % 20) as usize).addr();
                    *((j + 0x19e) as *mut usize) = m;
                    *((j + 0x27) as *mut i8) = l59 as i8;
                    *((j + 0x19) as *mut i8) = ((z >> 5) % 11) as i8;
                    *((j + 0x106) as *mut u8) = ((z >> 9) % 2) as u8;
                    let alvo77 = compare_designated_marked(j);
                    // batch 77: how many and who mark the target (or the player itself)
                    let alvo77 = if alvo77 != 0 && (z >> 20) % 4 != 0 { alvo77 } else { j };
                    // those of the other side with valid engine and side during both calls (0x7160f0 reads both)
                    let outro77 = (*((alvo77 + 0x27) as *const i8) == 0) as i8;
                    let guardados77: Vec<(usize, u8)> = (0..20).map(|k| {
                        let q = Engine(m).player(outro77, k).addr();
                        let g = (*((q + 0x19e) as *const usize), *((q + 0x27) as *const u8));
                        *((q + 0x19e) as *mut usize) = m;
                        *((q + 0x27) as *mut i8) = outro77;
                        g
                    }).collect();
                    compare_how_many_mark(m, alvo77);
                    compare_who_marks(m, alvo77);
                    for (k, &(g, l)) in guardados77.iter().enumerate() {
                        let q = Engine(m).player(outro77, k).addr();
                        *((q + 0x19e) as *mut usize) = g;
                        *((q + 0x27) as *mut u8) = l;
                    }
                    compare_designated_for_position(form, [-1i8, 0, 5, 10][((z >> 12) % 4) as usize]);
                }
                for (i, &b) in blocos.iter().enumerate() {
                    core::ptr::copy_nonoverlapping(antes59[i].as_ptr(), b as *mut u8, antes59[i].len());
                }
                core::ptr::copy_nonoverlapping(antes_form.as_ptr(), form as *mut u8, antes_form.len());
                *((form_o + 0x187b) as *mut u32) = antes_fo;
                *((m + 0xaf + o59) as *mut u8) = antes_af;
                *(G_STAFF_TABLE as *mut usize) = pessoas_antes;
                for (t, a) in tabelas.iter_mut().zip(antes_tabelas.iter()) {
                    t.copy_from_slice(a);
                }
            }
            // batch 58: formation cells (all valid bits, positions 0..=10; the
            // error case opens the game's box, outside the test) and stars (attributes in the ranges, roles)
            {
                let bits: [u64; 34] = [1, 2, 4, 8, 0x10, 0x20, 0x40, 0x80, 0x100, 0x200, 0x400, 0x800, 0x1000, 0x2000, 0x4000, 0x8000,
                    0x10000, 0x20000, 0x40000, 0x80000, 0x10_0000, 0x20_0000, 0x40_0000, 0x80_0000, 0x100_0000, 0x200_0000,
                    0x400_0000, 0x800_0000, 0x1000_0000, 0x2000_0000, 0x4000_0000, 0x8000_0000, 1 << 32, 2 << 32];
                let mais: [u64; 2] = [4 << 32, 8 << 32];
                let lado58 = (s.next_u64() % 2) as usize;
                let tabela58 = m + 0x91e2 + 6371 * lado58;
                for _ in 0..3 {
                    let z = s.next_u64();
                    let b = if z % 18 == 0 { mais[((z >> 8) % 2) as usize] } else { bits[((z >> 8) % 34) as usize] };
                    *((tabela58 + 0x188b + 8 * ((z >> 16) % 11) as usize) as *mut u64) = s.next_u64();
                    *((tabela58 + 0x187f) as *mut u32) = 0x5555;
                    compare_set_square(tabela58, ((z >> 16) % 11) as i8, b);
                }
                let bloco58 = Engine(m).player(lado58 as i8, 0).addr();
                let antes58 = core::slice::from_raw_parts(bloco58 as *const u8, 20 * 0x1be).to_vec();
                for k in 0..20usize {
                    let q = bloco58 + k * 0x1be;
                    let z = s.next_u64();
                    *((q + 0x19) as *mut i8) = if z % 6 == 0 { -1 } else { ((z >> 3) % 11) as i8 };
                    let pd = pessoas[k % 4].as_ptr() as usize;
                    *((q + 0x6d) as *mut usize) = pd;
                    *((q + 0x4d) as *mut f32) = [0.0f32, 5000.0, 7000.5, 8000.0, 8500.0, 9999.9, ((z >> 8) % 10000) as f32][((z >> 24) % 7) as usize];
                }
                for pd in pessoas.iter_mut() {
                    let z = s.next_u64();
                    pd[0x43] = (z % 21) as u8;
                    pd[0x33] = ((z >> 8) % 21) as u8;
                    pd[0x27] = ((z >> 16) % 21) as u8;
                }
                compare_mark_stars(m, lado58 as i8);
                core::ptr::copy_nonoverlapping(antes58.as_ptr(), bloco58 as *mut u8, antes58.len());
            }
            // batch 53: individual marking (marks by position on both sides, records)
            for l in 0..2usize {
                for pos in 0..16usize {
                    let r = s.next_u64();
                    *((m + 0x91e2 + 6371 * l + 0x5b4 + pos) as *mut u8) = [0x14u8, 0x10, 4, 0x15, 0, (r >> 8) as u8][(r % 6) as usize];
                }
            }
            for k in 0..40usize {
                let p = Engine(m).player(((k / 20)) as i8, k % 20).addr();
                let r = s.next_u64();
                *((p + 0x69) as *mut usize) = if r % 5 == 0 { 0 } else { fichas.as_ptr().add(k) as usize };
            }
            for (k, f) in fichas.iter_mut().enumerate() {
                *f = 0x1000 + k as u32;
            }
            compare_individual_marking(m);
            // batch 54: new reputation (national side, type 8 with the codes, match -2, zero ratio)
            {
                let globais = [0xae235cusize, G_NATION_COUNT, G_PLAYER_STATS];
                let antes: Vec<u32> = globais.iter().map(|&a| *(a as *const u32)).collect();
                let mut est = [0f64; 6];
                let mut pa = vec![0u8; 0x60];
                let mut cl = vec![vec![0u8; 0x90]; 2];
                let mut cp = vec![0u8; 0x70];
                *(G_CLUB_COUNT as *mut i32) = 5000;
                *(G_NATION_COUNT as *mut i32) = 200;
                *(G_PLAYER_STATS as *mut usize) = est.as_mut_ptr() as usize;
                let antes_m = *((m + 0x4792) as *const usize);
                for _ in 0..6 {
                    let r: Vec<u64> = (0..12).map(|_| s.next_u64()).collect();
                    for (i, e) in est.iter_mut().enumerate() {
                        *e = if r[i] % 5 == 0 { 0.0 } else { (r[i] >> 8) as f64 % 4000.0 / 100.0 };
                    }
                    let pp = pa.as_mut_ptr() as usize;
                    *(pp as *mut i32) = if r[6] % 6 == 0 { -2 } else { 1 };
                    *((pp + 0x14) as *mut usize) = cp.as_mut_ptr() as usize;
                    *((pp + 0x1c) as *mut usize) = cl[0].as_mut_ptr() as usize;
                    *((pp + 0x20) as *mut usize) = cl[1].as_mut_ptr() as usize;
                    *((pp + 0x40) as *mut u8) = (r[6] >> 4) as u8 % 2;
                    *((pp + 0x3a) as *mut u8) = [8u8, 8, 8, 0, 7][((r[6] >> 8) % 5) as usize];
                    *((pp + 0x32) as *mut u16) = [0x41eu16, 0x41f, 0x423, 0x426, 0x427, (r[7] >> 16) as u16][(r[7] % 6) as usize];
                    *((pp + 0x30) as *mut u16) = [0x77u16, 0x78, 0x7c, 0x7d, 0x81, 0x82, 0x87, 0x88, 0x95, 0x96, 0x9b, 0x9c][((r[7] >> 8) % 12) as usize];
                    *((cp.as_mut_ptr() as usize + 0x69) as *mut i16) = [0i16, 5, 10, 19, 20, 35, 30, -3, ((r[8] >> 8) % 21) as i16][(r[8] % 9) as usize];
                    for (i, c) in cl.iter_mut().enumerate() {
                        let z = r[9 + i];
                        *(c.as_mut_ptr() as *mut i32) = [4600i32, 4599, 4800, 0][(z % 4) as usize];
                        *((c.as_mut_ptr() as usize + 0x80) as *mut i16) = [0i16, 499, 500, 999, 1000, 4999, 5000, 9999, 10000, ((z >> 8) % 12000) as i16][((z >> 4) % 10) as usize];
                    }
                    // level 21..=28 with codes 0x78.. would give weight 0 (division by zero in the original): outside the test
                    *((m + 0x4792) as *mut usize) = pp;
                    *((m + 0x1d6) as *mut u32) = 0x55555555;
                    let valor = [0f32, 6.8, 1.0, 3.5, 10.0, 40.0][(r[11] % 6) as usize] + ((r[11] >> 8) % 1000) as f32 / 300.0 - 1.5;
                    compare_new_reputation(m, ((r[11] >> 20) % 2) as i8, valor, ((r[11] >> 21) % 2) as u8, ((r[11] >> 22) % 2) as u8);
                    // batch 55: player's reputation (same match; record, the player's club)
                    let mut reg = [0u8; 0x20];
                    let mut jog = [0u8; 0x70];
                    let mut ficha = [0u8; 0x10];
                    let rr = s.next_u64();
                    let (rp, jp, fp) = (reg.as_mut_ptr() as usize, jog.as_mut_ptr() as usize, ficha.as_mut_ptr() as usize);
                    *((rp + 0x1b) as *mut usize) = if rr % 23 == 0 { 0 } else { jp };
                    *((jp + 0x69) as *mut usize) = if rr % 19 == 1 { 0 } else { fp };
                    for (i, o) in [8usize, 0xa, 0xc].iter().enumerate() {
                        let z = s.next_u64();
                        *((fp + o) as *mut i16) = [0i16, 1, 999, 1000, 3000, 4999, 5000, 9999, 10000, ((z >> 8) % 12000) as i16][((z >> (i * 4)) % 10) as usize];
                    }
                    let alvo = cl[1].as_mut_ptr() as usize;
                    *((jp + 0x1a) as *mut usize) = [alvo, 0x1234, 0][((rr >> 8) % 3) as usize];
                    *((cl[0].as_mut_ptr() as usize + 0x53) as *mut usize) = [alvo, 0x4321, 0][((rr >> 10) % 3) as usize];
                    *((cl[1].as_mut_ptr() as usize + 0x53) as *mut usize) = [alvo, 0x4321, 0][((rr >> 12) % 3) as usize];
                    *((cp.as_mut_ptr() as usize + 0x5d) as *mut usize) = [alvo, 0x5678, 0][((rr >> 14) % 3) as usize];
                    let lado55 = ((rr >> 16) % 2) as usize;
                    compare_player_reputation(rp, cl[lado55].as_mut_ptr() as usize, cl[1 - lado55].as_mut_ptr() as usize, pp, valor, ((rr >> 17) % 2) as u8, ((rr >> 18) % 2) as u8);
                }
                *((m + 0x4792) as *mut usize) = antes_m;
                for (i, &a) in globais.iter().enumerate() {
                    *(a as *mut u32) = antes[i];
                }
            }
            compare_square_bit(match (h56 >> 56) % 4 { 0 => c56 + 1, 1 => c56 - 1, 2 => (h56 >> 30) as i16, _ => c56 });
            let setor56 = [1u32, 2, 4, 8, 0x10, 0x20, 0x40, 0x41, 0x7f][((h56 >> 33) % 9) as usize];
            let lado56 = [0x80u32, 0x100, 0x200, 0x400, 0x800, 0xf80, 0xc00, 0x0][((h56 >> 37) % 8) as usize];
            let m56 = setor56 | lado56 | (((h56 >> 18) as u32) & 0xfffff000);
            if m56 & 0x7f != 0 && (m56 & 0xf80 != 0 || m56 & 1 != 0) {
                let (mut st56, mut ld56) = (0x5555u16, 0xaaaau16);
                compare_split_mask(m56, &mut st56, &mut ld56);
            }
            for (i, &a) in ids.iter().enumerate() {
                *(a as *mut u32) = ids_antes[i];
            }
            compare_choose_by_role_c(m, (alto13 & !0xff) | ((h13 >> 53) % 2) as u32, if (h13 >> 54) % 3 == 0 { 0x100 | ((h13 >> 56) as u32 & 0xfe00) } else { (h13 >> 57) as u32 & 0xff });
            compare_distance_between(((q1 >> 51) % 12) as i8 - 1, ((q1 >> 55) % 14) as i8 - 1, ((q1 >> 59) % 11) as i8 - 1, ((q1 >> 33) % 14) as i8 - 1);
        }
    });
    SYNTHETIC.store(false, std::sync::atomic::Ordering::Relaxed);
}
