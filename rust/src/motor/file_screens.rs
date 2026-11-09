use super::*;

/// Builds the general.dat summary: the active nations (two groups), the human managers with club and nation, the game
/// date, the version, the play time and time().
///
/// `build_save_game_summary(summary)` (cdecl): the summary that `save_game_data` writes into `general.dat` (right after
/// the version):
///
/// - +0: the nations of the league table (0xb63d60, 0x23 of 0x48 bytes) with bit 2 set in +0x11c, the first through
/// sprintf with its own name as the format (0x944c9f), the others through the translated ", <%s>" (0x66f4e0); +1000
/// (0x3e8): the same with bit 1;
///
/// - 16 managers, from the last of the person table (0xae23bc, 0x6e bytes) backwards: a human one (+0x5f = 1) gives the
/// name at +0x7e4 + 0x78 × i (`database_format_staff_name`, 200) and at +0x820 + 0x78 × i "club / nation" (+0x39 and +0x24, name at +0x38
/// of each), only one of them, or nothing; a non-human one only zeroes the name (the club stays as it was in the
/// caller's buffer);
///
/// - +0xf64 the game date (`exe::date_operator_3d` of 0xae2c90), +0x7d0 the version (0x90d130), +0xf6c the play time ([0xdd7140] +=
/// time() − [0xdd7144]; [0xdd7144] = time()) and +0xf70 the time().
/// Original: `build_save_game_summary` (0x591f80, file_screens.cpp, cdecl).
///
/// [pt] { 0x591f80 } (cdecl, file_screens.cpp): `build_save_game_summary(resumo)`: o resumo que o
/// `save_game_data` grava no `general.dat` (logo depois da versão):
/// - +0: as nações da tabela de ligas (0xb63d60, 0x23 de 0x48 bytes) com o bit 2 em +0x11c, a
///   primeira pelo sprintf com o próprio nome como formato (0x944c9f), as outras por ", <%s>"
///   traduzido (0x66f4e0); +1000 (0x3e8): o mesmo com o bit 1;
/// - 16 técnicos, do último da tabela de pessoas (0xae23bc, 0x6e bytes) para trás: humano (+0x5f
///   = 1) dá o nome em +0x7e4 + 0x78 × i (0x53dba0, 200) e em +0x820 + 0x78 × i "clube / nação"
///   (+0x39 e +0x24, nome em +0x38 de cada), só um deles, ou nada; não humano só zera o nome (o
///   clube fica como estava no buffer do chamador);
/// - +0xf64 a data do jogo (0x417c10 de 0xae2c90), +0x7d0 a versão (0x90d130), +0xf6c o tempo de
///   jogo ([0xdd7140] += time() − [0xdd7144]; [0xdd7144] = time()) e +0xf70 o time().
pub(crate) unsafe extern "C" fn build_save_game_summary(summary: *mut u8) {
    let format: unsafe extern "C" fn(*mut u8, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let translate: unsafe extern "C" fn(*mut u8, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let version: unsafe extern "C" fn() -> *const u8 = core::mem::transmute(FN_UTILS_GET_VERSION_STRING);
    let mut nation = [0u8; 100];
    *summary = 0;
    *summary.add(0x3e8) = 0;
    for (bit, destino) in [(2u8, summary), (1u8, summary.add(0x3e8))] {
        let mut first = true;
        for i in 0..0x23usize {
            let n = *((G_LEAGUE_TABLE + i * 0x48) as *const usize);
            if n == 0 || *((n + nation::LEAGUE_SELECTED) as *const u8) & bit == 0 {
                continue;
            }
            if first {
                format(nation.as_mut_ptr(), n + nation::SHORT_NAME);
                first = false;
            } else {
                translate(nation.as_mut_ptr(), 0x9c8134, n + nation::SHORT_NAME);
            }
            append_text(destino, nation.as_ptr());
        }
        if bit == 1 && first {
            *summary.add(0x3e8) = 0;
        }
    }
    let how_many = *(G_STAFF_COUNT as *const i32);
    let table = *(G_STAFF_TABLE as *const usize);
    for i in 0..0x10i32 {
        let club_txt = summary.add(0x820 + 0x78 * i as usize);
        let p = Staff(table.wrapping_add((how_many - i - 1).wrapping_mul(0x6e) as usize));
        if p.classification() != 1 {
            *club_txt.sub(0x3c) = 0;
            continue;
        }
        exe::database_format_staff_name(p, (club_txt.sub(0x3c)) as *mut u8 as usize, 200, 1);
        let club = Club(core::ptr::read_unaligned((p.addr() + staff::CLUB_JOB) as *const usize));
        let nation_of = core::ptr::read_unaligned((p.addr() + staff::NATIONAL_JOB) as *const usize);
        if !club.is_null() && nation_of != 0 {
            translate(club_txt, 0x9c80f4, club.addr() + club::SHORT_NAME, nation_of + nation::SHORT_NAME);
        } else if !club.is_null() {
            copy_text(club_txt, (club.addr() + club::SHORT_NAME) as *const u8);
        } else if nation_of != 0 {
            copy_text(club_txt, (nation_of + nation::SHORT_NAME) as *const u8);
        } else {
            *club_txt = 0;
        }
    }
    let mut copy = [0u8; 8];
    exe::date_operator_3d(summary as usize + 0xf64, (copy.as_mut_ptr()) as usize, G_CURRENT_DATE);
    copy_text(summary.add(0x7d0), version());
    let mut now = 0i32;
    let t = exe::at_946ebb(&mut now);
    let total = (*(G_GAME_PLAY_TIME_TOTAL as *const i32)).wrapping_add(t.wrapping_sub(*(G_GAME_PLAY_TIME_LAST_STAMP as *const i32)));
    *(G_GAME_PLAY_TIME_TOTAL as *mut i32) = total;
    core::ptr::write_unaligned(summary.add(0xf6c) as *mut i32, total);
    *(G_GAME_PLAY_TIME_LAST_STAMP as *mut i32) = t;
    let t2 = exe::at_946ebb(core::ptr::null_mut());
    core::ptr::write_unaligned(summary.add(0xf70) as *mut i32, t2);
}
