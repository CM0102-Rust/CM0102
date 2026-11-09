# Mutacoes da leva104 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva104.py --jogo 2001-08-20
MUTACOES = [
    ('x_lado', "    let side = (j.side() == 0) as usize;", "    let side = (j.side() != 0) as usize;"),
    ('x_base', "    let base = || j.moment_movement() as i32 + j.ball_control() as i32 / 20;", "    let base = || j.moment_movement() as i32 + j.ball_control() as i32 / 19;"),
    ('x_marca2', "            count = count.wrapping_add(2);", "            count = count.wrapping_add(3);"),
    ('x_dobro', "            weight = r.wrapping_add(r);", "            weight = r;"),
    ('x_2b', "            weight = ftol(v as f64 * k / (p.busy() as i32 + 1) as f64);", "            weight = ftol(v as f64 * k / (p.busy() as i32 + 2) as f64);"),
    ('x_bit', "p.action() & 0x100 != 0)", "p.action() & 0x200 != 0)"),
    ('x_5', "        if list != 0 && weight as i16 > 5 {", "        if list != 0 && weight as i16 > 4 {"),
    ('x_segundo', "                if melhor != 0 {\n                    segundo = melhor;", "                if melhor == 0 {\n                    segundo = melhor;"),
    ('x_ordem', "        if melhor != 0 {\n            copy(list, melhor);\n        }\n        if segundo != 0 {\n            copy(list, segundo);\n        }", "        if segundo != 0 {\n            copy(list, segundo);\n        }\n        if melhor != 0 {\n            copy(list, melhor);\n        }"),
]
