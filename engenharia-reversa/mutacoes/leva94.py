# Mutacoes da leva94 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva94.py --jogo 2001-08-20
MUTACOES = [
    ('p_area', "    let na_area = (lado_b == 0 && byte(this, 0x8ea8) < 3)", "    let na_area = (lado_b == 0 && byte(this, 0x8ea8) < 4)"),
    ('p_marca', "                if x == byte(this, 0x8ea7) && y == byte(this, 0x8ea8) && !sai {", "                if x == byte(this, 0x8ea7) && y == byte(this, 0x8ea8) && sai {"),
    ('p_longe', "        let longe = if lado_b == 0 { marca > 6 } else { marca < 5 };", "        let longe = if lado_b == 0 { marca > 7 } else { marca < 5 };"),
    ('p_coluna', "        let coluna = 4i8.wrapping_sub(s1).wrapping_add(sortear(2) as i8);", "        let coluna = 5i8.wrapping_sub(s1).wrapping_add(sortear(2) as i8);"),
    ('p_meio', "if l == 1 { 6 } else { 5 })", "if l == 1 { 5 } else { 5 })"),
    ('p_fundo', "(x, if (l == 1) != sai { 11 - s3 } else { s3 })", "(x, if (l == 1) == sai { 11 - s3 } else { s3 })"),
    ('p_contador', "    *contador = (*contador).wrapping_add(1);", "    *contador = (*contador).wrapping_add(2);"),
    ('p_apontar', "4, if lado_b != 0 { 1 } else { 10 });", "4, if lado_b != 0 { 2 } else { 10 });"),
]
