# Mutacoes da leva56 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva56.py
# As de corner e falta perto do gol (tatica do clube) so aparecem no jogo real: mutacoes\leva56_jogo.py
MUTACOES = [
    ('c_excl', "        if kind == 5 && excluidos != 0 && *((excluidos as isize + i) as *const u8) != 0 {", "        if kind == 4 && excluidos != 0 && *((excluidos as isize + i) as *const u8) != 0 {"),
    ('c_guard', "                1 | 2 | 3 | 8 => continue,", "                1 | 3 | 8 => continue,"),
    ('c_10', "        if j.position() > 10 || j.player().is_null() {", "        if j.position() > 11 || j.player().is_null() {"),
    ('c_e9', "1 | 2 => ftol(f(j.addr(), if kind == 1 { 0xdd } else { 0xe9 }) - 2.0 * exe::match_player_distance_to_cell(j, a7, a8) as f64) as i16,", "1 | 2 => ftol(f(j.addr(), if kind == 1 { 0xdd } else { 0xdd }) - 2.0 * exe::match_player_distance_to_cell(j, a7, a8) as f64) as i16,"),
    ('c_2x', "1 | 2 => ftol(f(j.addr(), if kind == 1 { 0xdd } else { 0xe9 }) - 2.0 *", "1 | 2 => ftol(f(j.addr(), if kind == 1 { 0xdd } else { 0xe9 }) - 1.0 *"),
    ('c_t4', "            4 => if table_a(this.addr(), j.addr()) == 1 { 100 } else { 0 },", "            4 => if table_a(this.addr(), j.addr()) & 1 != 0 { 100 } else { 0 },"),
    ('c_lista_sel', "                        || is_national_team(read_usize(partida + (if side == 0 { 0x1c } else { 0x20 }))) != 0", "                        || is_national_team(read_usize(partida + (if side == 0 { 0x20 } else { 0x1c }))) != 0"),
    ('c_5f', "                    && (manager.classification() as i8) != 0\n", "\n"),
    ('c_lista_ordem', "                            v = 10000 - b as i16;", "                            v = 10000 + b as i16;"),
    ('c_menos20', "(read_i8(j.player().addr() + 0xf) as i32 * 20 - 20) as f64) as i16;", "(read_i8(j.player().addr() + 0xf) as i32 * 20) as f64) as i16;"),
    ('c_goleiro5', "                    -10000\n", "                    10000\n"),
    ('c_idade', "                        if age < 20 {\n                            s = ftol(s as f64 * read_f64(rdata::F64_0_35)) as i16;", "                        if age < 21 {\n                            s = ftol(s as f64 * read_f64(rdata::F64_0_35)) as i16;"),
    ('c_5a', "                        if b5a != 0 && exe::utils_random(b5a as i32) == 0 {", "                        if b5a != 0 && exe::utils_random(b5a as i32) == 1 {"),
    ('c_0b', "                        s = ftol((read_i16(j.player().addr() + 0xb) as f64 * read_f64(rdata::F64_2_5ENEG_05)", "                        s = ftol((read_i16(j.player().addr() + 0xd) as f64 * read_f64(rdata::F64_2_5ENEG_05)"),
    ('c_longe', "            ftol(f(j.addr(), match_player::FREE_KICKS) - g2 as f64 * g1 as f64) as i16", "            ftol(f(j.addr(), match_player::FREE_KICKS) - g2 as f64) as i16"),
    ('c_z', "                } else if table_ball() < read_f64(rdata::F64_6_1) {", "                } else if table_ball() < read_f64(rdata::F64_4_1) {"),
    ('c_8', "            8 => ftol(read_f32(rdata::F32_100) as f64 - exe::match_player_distance_to_cell(j, a7, a8) as f64) as i16,", "            8 => ftol(read_f32(rdata::F32_100) as f64 + exe::match_player_distance_to_cell(j, a7, a8) as f64) as i16,"),
    ('c_maior', "        if melhor == 0 || v > melhor_v {", "        if melhor == 0 || v >= melhor_v {"),
]
