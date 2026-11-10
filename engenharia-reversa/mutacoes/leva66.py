# Mutacoes da leva66 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva66.py --jogo 2002-01-01
MUTACOES = [
    ('p_limpa', "    core::ptr::write_bytes((m.addr() + engine::PENALTY_TAKER_ORDER) as *mut u8, 0xff, 22);", "    core::ptr::write_bytes((m.addr() + engine::PENALTY_TAKER_ORDER) as *mut u8, 0xff, 21);"),
    ('p_marca', "                *((m.addr() as isize + (engine::SHOOTOUT_TAKEN as isize) + 20 * l as isize + e as isize) as *mut u8) = 1;", "                *((m.addr() as isize + (engine::SHOOTOUT_TAKEN as isize) + 20 * l as isize + e as isize) as *mut u8) = 2;"),
    ('p_tipo5', "                let c = exe::cm3_match_engine_pick_restart_taker(m, lado, 5, m.addr() + engine::SHOOTOUT_TAKEN);", "                let c = exe::cm3_match_engine_pick_restart_taker(m, lado, 5, 0);"),
    ('p_5f', "                && (manager.classification() as i8) != 0\n", "\n"),
    ('p_11', "                *((m.addr() as isize + (engine::PENALTY_TAKER_ORDER as isize) + 11 * l as isize + team as isize) as *mut i8) = e;", "                *((m.addr() as isize + (engine::PENALTY_TAKER_ORDER as isize) + 11 * l as isize + team as isize) as *mut i8) = e.wrapping_add(1);"),
]
