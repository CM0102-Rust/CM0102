# Mutacoes da leva100 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva100.py --jogo 2001-08-20
MUTACOES = [
    ('b_lateral', "        tx = if bx >= 2 { 9 } else { -1 };", "        tx = if bx >= 2 { 8 } else { -1 };"),
    ('b_espalhar', "    let n = spread as i32 + 1;", "    let n = spread as i32 + 2;"),
    ('b_dentro', "            if inside == 0 || (0..=11).contains(&ty) {", "            if inside == 0 || (0..=10).contains(&ty) {"),
    ('b_altura', "    this.set_ball_roll(if spread == 0 { 0 } else { (random(n) + 1) as i8 });", "    this.set_ball_roll(if spread == 0 { 0 } else { (random(n) + 2) as i8 });"),
    ('b_efeito', "        let i = dy as isize + 24 * dx as isize;", "        let i = dx as isize + 24 * dy as isize;"),
    ('b_estado', "    this.set_ball_height(random(4) as i8);", "    this.set_ball_height(random(5) as i8);"),
    ('b_cansa', "                if j.position() >= 0 && j.moment_anticipation() == 0 && random(3) == 0 {", "                if j.position() >= 0 && j.moment_anticipation() == 0 && random(4) == 0 {"),
    ('b_fundo', "        let routine = if this.ball_y() == 12 { if d != 0 { 0x70c040 } else { 0x70a910 } }", "        let routine = if this.ball_y() == 12 { if d == 0 { 0x70c040 } else { 0x70a910 } }"),
]
