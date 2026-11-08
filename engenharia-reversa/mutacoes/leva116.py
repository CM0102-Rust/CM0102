# Mutacoes da leva116 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva116.py --jogo 2001-08-20
MUTACOES = [
    ('v_qsort', "    qsort(list.as_mut_ptr() as usize, n as u32, 4, if n < 11 { 0x6e0df0 } else { 0x6e0fe0 });", "    qsort(list.as_mut_ptr() as usize, n as u32, 4, if n < 12 { 0x6e0df0 } else { 0x6e0fe0 });"),
    ('v_goleiro', "        pos[i] = if i == 0 && goalkeeper != 0 {\n            0", "        pos[i] = if i == 0 && goalkeeper != 0 {\n            1"),
    ('v_extras', "    let limit = (*((comp + 0x4d) as *const u8) & 0xf) as i32 + 0xb;", "    let limit = (*((comp + 0x4d) as *const u8) & 0xf) as i32 + 0xa;"),
    ('v_forca', "            let r = sortear(0xbb8) + 0xbb8;", "            let r = sortear(0xbb8) + 0xbb7;"),
    ('v_saida', "                *((j + match_player::POSITION) as *mut i8) = -4;", "                *((j + match_player::POSITION) as *mut i8) = -3;"),
    ('v_codigo', "        2 => 0x1f5c,", "        2 => 0x1f59,"),
    ('v_cansaco', "        let cansado = if comentarios && sortear(w(sai, 0x31) as i32) < -10 {", "        let cansado = if comentarios && sortear(w(sai, 0x31) as i32) < -9 {"),
    ('v_capitao', "            if melhor == -1 || v > nota {\n                nota = v;\n                melhor = k;\n            }\n        }\n        j += 0x1be;\n    }\n    call(FN_TACTICS_SET_CAPTAIN_FROM_SLOT", "            if melhor == -1 || v >= nota {\n                nota = v;\n                melhor = k;\n            }\n        }\n        j += 0x1be;\n    }\n    call(FN_TACTICS_SET_CAPTAIN_FROM_SLOT"),
]
