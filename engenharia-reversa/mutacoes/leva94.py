# Mutacoes da leva94 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva94.py --jogo 2001-08-20
MUTACOES = [
    ('p_area', "    let in_the_box = (side_b == 0 && read_i8(this + engine::BALL_Y) < 3)", "    let in_the_box = (side_b == 0 && read_i8(this + engine::BALL_Y) < 4)"),
    ('p_marca', "                if x == read_i8(this + engine::BALL_X) && y == read_i8(this + engine::BALL_Y) && !sai {", "                if x == read_i8(this + engine::BALL_X) && y == read_i8(this + engine::BALL_Y) && sai {"),
    ('p_longe', "        let far = if side_b == 0 { mark > 6 } else { mark < 5 };", "        let far = if side_b == 0 { mark > 7 } else { mark < 5 };"),
    ('p_coluna', "        let column = 4i8.wrapping_sub(s1).wrapping_add(random(2) as i8);", "        let column = 5i8.wrapping_sub(s1).wrapping_add(random(2) as i8);"),
    ('p_meio', "if l == 1 { 6 } else { 5 })", "if l == 1 { 5 } else { 5 })"),
    ('p_fundo', "(x, if (l == 1) != sai { 11 - s3 } else { s3 })", "(x, if (l == 1) == sai { 11 - s3 } else { s3 })"),
    ('p_contador', "    *counter = (*counter).wrapping_add(1);", "    *counter = (*counter).wrapping_add(2);"),
    ('p_apontar', "4, if side_b != 0 { 1 } else { 10 });", "4, if side_b != 0 { 2 } else { 10 });"),
]
