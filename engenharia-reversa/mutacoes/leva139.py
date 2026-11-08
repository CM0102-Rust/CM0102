# Mutacoes da leva139 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva139.py --carreira 2002-06-01 --confirmar 2001-10-01
MUTACOES = [
    ('f_favorito', "    [0x9b, 0x9f, 0xa3].iter().any(|&o| *((b + o) as *const usize) == a) as u32", "    [0x9b, 0x9f].iter().any(|&o| *((b + o) as *const usize) == a) as u32"),
    ('f_rejeitado', "    [0xa7, 0xab, 0xaf].iter().any(|&o| *((b + o) as *const usize) == a) as u32", "    [0xa7, 0xab].iter().any(|&o| *((b + o) as *const usize) == a) as u32"),
    ('f_insatisfacao', "let r1 = random((a * b as i32 * c as i32 + 0x14) * 100);", "let r1 = random((a * b as i32 * c as i32 + 0x15) * 100);", "match_wrap_up insatisfeito"),
    ('f_rival', "if [0xb3, 0xb7, 0xbb].iter().any(|&o| read_usize(meu + o) == outro) {\n                        x += 10;", "if [0xb3, 0xb7, 0xbb].iter().any(|&o| read_usize(meu + o) == outro) {\n                        x += 11;"),
    ('f_gosta', "    } else if exe::staff_favours_club(person, outro) != 0 {\n                        x -= 20;", "    } else if exe::staff_favours_club(person, outro) != 0 {\n                        x -= 19;"),
    ('f_media', "average = if read_usize(pt + fixture::STADIUM) == read_usize(meu + club::STADIUM) { q - read_f64_at(rdata::F64_0_1) } else { q + read_f64_at(rdata::F64_0_2) };", "average = if read_usize(pt + fixture::STADIUM) == read_usize(meu + club::STADIUM) { q - read_f64_at(rdata::F64_0_1) * 1.01 } else { q + read_f64_at(rdata::F64_0_2) };", "match_wrap_up media do lado"),
    ('f_noventa', "    } else if *(m as *const u8) != 0 {\n                0x5a", "    } else if *(m as *const u8) != 0 {\n                0x59", "match_wrap_up media do lado"),
    ('f_minuto_reserva', "                end.wrapping_sub(c)", "                end.wrapping_sub(c).wrapping_add(1)", "match_wrap_up media do lado"),
    ('f_tipo6', "byte(p, match_player::YELLOW_CARDS) == 2 || byte(p, match_player::RED_CARDS) != 0", "byte(p, match_player::YELLOW_CARDS) == 2 || byte(p, match_player::RED_CARDS) == 0", "match_wrap_up evento tipo 6"),
    ('f_evento_jogador', "byte(p, match_player::MATCH_GRADE) < 7 && random(byte(p, match_player::MATCH_FORM) as i32) == 0", "byte(p, match_player::MATCH_GRADE) < 8 && random(byte(p, match_player::MATCH_FORM) as i32) == 0", "match_wrap_up evento do dia"),
    ('f_proposta', "if r1 > random(0x14) && byte(x, 0x14) > 5 {", "if r1 > random(0x15) && byte(x, 0x14) > 5 {", "match_wrap_up evento do dia"),
    ('f_lixo_evento', "            ev[0xd] = side as u8;\n                            ev[0xf..0x13].copy_from_slice(&u32::MAX.to_le_bytes());\n                            ev[0x13..0x15].copy_from_slice(&11u16.to_le_bytes());", "            ev[0xd] = side as u8;\n                            ev[0xf..0x13].copy_from_slice(&u32::MAX.to_le_bytes());\n                            ev[0x13..0x15].copy_from_slice(&11u16.to_le_bytes());\n                            ev[0x16] = 1;", "match_wrap_up insatisfeito"),
]
