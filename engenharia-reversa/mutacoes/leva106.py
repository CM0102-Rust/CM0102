# Mutacoes da leva106 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva106.py --jogo 2001-08-20
MUTACOES = [
    ('e_nivel20', "            Some(n) => n.min(20),", "            Some(n) => n.min(19),"),
    ('e_fator', "        if (jm.assists() as i8) == 0 {\n            factor = (factor as f64 * (read_f32(rdata::F32_0_9523810148) as f64)) as f32;", "        if (jm.assists() as i8) != 0 {\n            factor = (factor as f64 * (read_f32(rdata::F32_0_9523810148) as f64)) as f32;"),
    ('e_idade', "    let age = |o: usize, min: i8| ((read_i16(sheet().addr() + o) as i32 / 1000) as i8).wrapping_add(0x10).max(min);", "    let age = |o: usize, min: i8| ((read_i16(sheet().addr() + o) as i32 / 1000) as i8).wrapping_add(0x11).max(min);"),
    ('e_div9', "        v.max(4)", "        v.max(5)"),
    ('e_divd', "    } else if (comp.category() as i8) != 8 {\n        div_d = (0xca - nivel0 as i32 * 10) as i16;", "    } else if (comp.category() as i8) != 8 {\n        div_d = (0xcb - nivel0 as i32 * 10) as i16;"),
    ('e_desvio', "        desvio *= desvio.abs();", "        desvio *= 1.0;"),
    ('e_passo', "    let step_towards = (level as i32 as f64 * read_f64(rdata::F64_0_5) + read_f64(rdata::F64_5)) * desvio * read_f64(rdata::F64_50);", "    let step_towards = (level as i32 as f64 * read_f64(rdata::F64_0_5) + read_f64(rdata::F64_3)) * desvio * read_f64(rdata::F64_50);"),
    ('e_novo', "    let new_b = ((div_b as i32 - 1) * vb + nb as i16 as i32) / div_b as i32;", "    let new_b = ((div_b as i32 - 2) * vb + nb as i16 as i32) / div_b as i32;"),
    ('e_moral', "        (random(3) + (x as i32).min(dif)) as i16", "        (random(4) + (x as i32).min(dif)) as i16"),
    ('e_clamp', "    let delta = random(m.clamp(-20, 20) as i32);", "    let delta = random(m.clamp(-19, 20) as i32);"),
    ('e_lim', "    let lim = (level as i16 as i32 * 300 + 4000) as i16;", "    let lim = (level as i16 as i32 * 300 + 4100) as i16;"),
    ('e_pot', "    let lo = pot * 50 - 3750;", "    let lo = pot * 50 - 3700;"),
    ('e_teto9', "    let r9 = preso(c9, f_9, (read_f32(rdata::F32_250) as f64), (read_f32(rdata::F32_375) as f64), t9);", "    let r9 = preso(c9, f_9, (read_f32(rdata::F32_250) as f64), (read_f32(rdata::F32_300) as f64), t9);"),
    ('e_10000', "        let v = (v as i16).min(10000);", "        let v = (v as i16).min(9999);"),
    ('e_rep', "                rb = ftol(target as i16 as f64 * read_f64(rdata::F64_0_1) + rb as i16 as f64 * read_f64(rdata::F64_0_9));", "                rb = ftol(target as i16 as f64 * read_f64(rdata::F64_0_1) + rb as i16 as f64);"),
]
