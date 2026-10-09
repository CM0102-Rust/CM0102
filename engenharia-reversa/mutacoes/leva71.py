# Mutacoes da leva71 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva71.py --jogo 2001-08-15
MUTACOES = [
    ('e_base', "    let base = (*((c + 0x29) as *const i16) as i32 - 1000) * 3 - r1;", "    let base = (*((c + 0x29) as *const i16) as i32 - 1000) * 3 - r1 + 1;"),
    ('e_v34', "        let mut v34 = ftol(f32_em(0x968db0) as f64 - d58 as f64 * d58 as f64 * f32_em(0x969580) as f64);", "        let mut v34 = ftol(f32_em(0x968db0) as f64 - d58 as f64 * d58 as f64 * f32_em(0x969560) as f64);"),
    ('e_cerco', "        v1c = v1c.wrapping_add((5 - n_i) * 400);", "        v1c = v1c.wrapping_add((5 - n_i) * 401);"),
    ('e_pow', "            v1c = v1c.wrapping_add(ftol(pow_do_jogo((3 - n_i) as f64, f64_em(0x968930)) * f64_em(0x9688a8)));", "            v1c = v1c.wrapping_add(ftol(pow_do_jogo(f64_em(0x968930), (3 - n_i) as f64) * f64_em(0x9688a8)));"),
    ('e_k7c', "            k7c = k7c.wrapping_mul(k7c).wrapping_add(1);", "            k7c = k7c.wrapping_mul(k7c).wrapping_add(2);"),
    ('e_zera', "            if v < f64_em(0x9689a0) || v.is_nan() {\n                *v1c = v1c.wrapping_add(ftol(*d58 as f64 * *d58 as f64 * k as f64));", "            if (*d58 as f64) < f64_em(0x9689a0) || v.is_nan() {\n                *v1c = v1c.wrapping_add(ftol(*d58 as f64 * *d58 as f64 * k as f64));"),
    ('e_100', "        v1c = v1c.wrapping_add(ftol(d58 as f64 * d58 as f64 * f32_em(0x968960) as f64));", "        v1c = v1c.wrapping_add(ftol(d58 as f64 * d58 as f64 * f32_em(0x969560) as f64));"),
    ('e_4d', "                tipo = 0x3d;\n                nota = base;", "                tipo = 0x3e;\n                nota = base;"),
    ('e_tipo1e', "                    nota = v30b;\n                    tipo = 0x1e;", "                    nota = v30b + 1;\n                    tipo = 0x1e;"),
    ('e_movimento', "                    tx = x78;\n                    ty = y77;\n                    moveu = 1;", "                    tx = x78;\n                    ty = y77;\n                    moveu = 2;"),
    ('e_gosta', "                base = base.wrapping_add(1000);\n            }\n        }", "                base = base.wrapping_add(999);\n            }\n        }"),
    ('e_preferido', "        if ficha_p == preferido(tabela, clube_p, 0, (marca_p == 0) as u32) {\n            v1c = v1c.wrapping_add(1000);", "        if ficha_p == preferido(tabela, clube_p, 0, (marca_p == 0) as u32) {\n            v1c = v1c.wrapping_add(900);"),
]
