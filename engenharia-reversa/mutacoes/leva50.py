# Mutacoes da leva50 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva50.py
MUTACOES = [
    ('g_limite', "let limite = (*(G_CLUB_COUNT as *const i32)).wrapping_sub(*(G_NATION_COUNT as *const i32));", "let limite = (*(G_CLUB_COUNT as *const i32)).wrapping_sub(*(G_NATION_COUNT as *const i32)) + 1;"),
    ('a_troca', "let (este, outro) = if lado_b == 0 { (casa, fora) } else { (fora, casa) };", "let (este, outro) = (casa, fora);"),
    ('a_bonus', "        (false, true) => (0, 1),", "        (false, true) => (1, 0),"),
    ('a_fase3', "as i32) == 3 {\n        *fase3 = 1;", "as i32) == 4 {\n        *fase3 = 1;"),
    ('a_ida', "    if ida_casa != -1 {", "    if ida_casa != 0 {"),
    ('a_4c', "*outro = (*outro).wrapping_add(byte(partida, 0x4c));", "*outro = (*outro).wrapping_add(byte(partida, 0x4b));"),
    ('a_bit4', "            if f & 4 == 0 && tempo_ok {", "            if tempo_ok {"),
    ('a_tempo', "*((this + 0x8ed4) as *const i16) > 0x3de;", "*((this + 0x8ed4) as *const i16) >= 0x3de;"),
    ('a_menor', "                    if gols_fora + (bonus_este as i32) < ida_casa as i32 {", "                    if gols_fora + (bonus_este as i32) <= ida_casa as i32 {"),
    ('a_maior', "                } else if gols_fora + (bonus_outro as i32) > ida_casa as i32 {", "                } else if gols_fora + (bonus_outro as i32) >= ida_casa as i32 {"),
    ('a_ret', "        (*casa as u8).wrapping_sub(*fora as u8)", "        (*fora as u8).wrapping_sub(*casa as u8)"),
]
