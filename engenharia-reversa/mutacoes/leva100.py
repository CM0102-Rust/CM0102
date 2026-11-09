# Mutacoes da leva100 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva100.py --jogo 2001-08-20
MUTACOES = [
    ('b_lateral', "        tx = if bx >= 2 { 9 } else { -1 };", "        tx = if bx >= 2 { 8 } else { -1 };"),
    ('b_espalhar', "    let n = spread as i32 + 1;", "    let n = spread as i32 + 2;"),
    ('b_dentro', "            if inside == 0 || (0..=11).contains(&ty) {", "            if inside == 0 || (0..=10).contains(&ty) {"),
    ('b_altura', "    *b(engine::BALL_ROLL) = if spread == 0 { 0 } else { (random(n) + 1) as i8 };", "    *b(engine::BALL_ROLL) = if spread == 0 { 0 } else { (random(n) + 2) as i8 };"),
    ('b_efeito', "        let i = dy as isize + 24 * dx as isize;", "        let i = dx as isize + 24 * dy as isize;"),
    ('b_estado', "    *b(engine::BALL_HEIGHT) = random(4) as i8;", "    *b(engine::BALL_HEIGHT) = random(5) as i8;"),
    ('b_cansa', "                if read_i8(j + match_player::POSITION) >= 0 && read_i8(j + match_player::MOMENT_ANTICIPATION) == 0 && random(3) == 0 {", "                if read_i8(j + match_player::POSITION) >= 0 && read_i8(j + match_player::MOMENT_ANTICIPATION) == 0 && random(4) == 0 {"),
    ('b_fundo', "        let routine = if read_i8(this + engine::BALL_Y) == 12 { if d != 0 { 0x70c040 } else { 0x70a910 } }", "        let routine = if read_i8(this + engine::BALL_Y) == 12 { if d == 0 { 0x70c040 } else { 0x70a910 } }"),
]
