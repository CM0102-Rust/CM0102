# Mutacoes da leva135 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva135.py --carreira 2002-06-01 --confirmar 2001-10-01
MUTACOES = [
    ('r_quinze', "[t - *w(0x1d2) as i32 > 0xa5, t - *w(0x1d4) as i32 > 0xa5]", "[t - *w(0x1d2) as i32 > 0x50, t - *w(0x1d4) as i32 > 0xa5]"),
    ('r_sem_folego', "if minute > 0x3c && sortear(6) == 0 {", "if minute > 0x3c && sortear(7) == 0 {", "reinicio sem folego"),
    ('r_folego', "read_f32_at(j + match_player::STAMINA) < 6750.0", "read_f32_at(j + match_player::STAMINA) < 6000.0", "reinicio sem folego"),
    ('r_postura', "if new != velha || *w(0x8ed0) <= 1 {", "if new != velha || *w(0x8ed0) <= 2 {"),
    ('r_acrescimo', "let r = sortear(10) as i8 as i32;", "let r = sortear(11) as i8 as i32;", "reinicio acrescimo"),
    ('r_cansar', "cansar(j, (r as f64 * read_f64_at(0x96d3d8) + 1.0) as f32);", "cansar(j, (r as f64 * read_f64_at(0x96d3d8) + 1.1) as f32);", "reinicio acrescimo"),
    ('r_sub', "(minute as i32) > sortear(0x32) + 10", "(minute as i32) > sortear(0x33) + 10", "reinicio substituicao da IA"),
    ('r_sub55', "*w(0x8ed0) as i32 - *w(last) as i32 > 0x37", "*w(0x8ed0) as i32 - *w(last) as i32 > 0x30", "reinicio substituicao da IA"),
    ('r_acresc_narr', "let v = if q <= 2 { 2 } else { q as i8 };", "let v = if q <= 2 { 3 } else { q as i8 };", "reinicio acrescimo narrado"),
    ('r_postura1', "let f = if (manager != 0 && f32m(0x476a + 4 * side) < 2.5 && tb(0xe) <= 6) || !attack { 0x20000 } else { 0x10000 };", "let f = if (manager != 0 && f32m(0x476a + 4 * side) < 2.5 && tb(0xe) <= 7) || !attack { 0x20000 } else { 0x10000 };", "reinicio stance 1"),
    ('r_postura2', "geral(tobj, if (tb(0x13) as i32) < lim(4) { 0x1000 } else { 0x800 });", "geral(tobj, if (tb(0x13) as i32) < lim(5) { 0x1000 } else { 0x800 });", "reinicio stance 2"),
    ('r_postura5', "geral(tobj, if e > 0x10 && attack { 0x10000 } else if e > 0xc { 0x20000 } else { 0x8000 });", "geral(tobj, if e > 0x10 && attack { 0x10000 } else if e > 0xd { 0x20000 } else { 0x8000 });", "reinicio stance 5"),
    ('r_canto', "if away || sortear(v1 - v2 + 5) == 0 {", "if away || sortear(v1 - v2 + 6) == 0 {", "reinicio canto da goal_crowd_reaction"),
    ('r_bola_parada', "                5 => 3,\n                6 | 7 => 4,", "                5 => 3,\n                6 | 7 => 5,"),
]
