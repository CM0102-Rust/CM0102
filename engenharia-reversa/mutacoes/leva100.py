# Mutacoes da leva100 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva100.py --jogo 2001-08-20
MUTACOES = [
    ('b_lateral', "        tx = if bx >= 2 { 9 } else { -1 };", "        tx = if bx >= 2 { 8 } else { -1 };"),
    ('b_espalhar', "    let n = espalhar as i32 + 1;", "    let n = espalhar as i32 + 2;"),
    ('b_dentro', "            if inside == 0 || (0..=11).contains(&ty) {", "            if inside == 0 || (0..=10).contains(&ty) {"),
    ('b_altura', "    *b(engine::BALL_ROLL) = if espalhar == 0 { 0 } else { (sortear(n) + 1) as i8 };", "    *b(engine::BALL_ROLL) = if espalhar == 0 { 0 } else { (sortear(n) + 2) as i8 };"),
    ('b_efeito', "        let i = dy as isize + 24 * dx as isize;", "        let i = dx as isize + 24 * dy as isize;"),
    ('b_estado', "    *b(engine::BALL_HEIGHT) = sortear(4) as i8;", "    *b(engine::BALL_HEIGHT) = sortear(5) as i8;"),
    ('b_cansa', "                if byte(j, match_player::POSITION) >= 0 && byte(j, match_player::MOMENT_ANTICIPATION) == 0 && sortear(3) == 0 {", "                if byte(j, match_player::POSITION) >= 0 && byte(j, match_player::MOMENT_ANTICIPATION) == 0 && sortear(4) == 0 {"),
    ('b_fundo', "        let routine = if byte(this, engine::BALL_Y) == 12 { if d != 0 { 0x70c040 } else { 0x70a910 } }", "        let routine = if byte(this, engine::BALL_Y) == 12 { if d == 0 { 0x70c040 } else { 0x70a910 } }"),
]
