# Mutacoes da leva141 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva141.py --carreira 2002-06-01 --confirmar 2001-10-01
MUTACOES = [
    ('l_limite', 'if n > 3 && random(n as i32) != 0 && random(0x1f4) != 0 {', 'if n > 4 && random(n as i32) != 0 && random(0x1f4) != 0 {'),
    ('l_teto', '    if *w(engine::INJURY_COUNT) > 6 {\n        return;', '    if *w(engine::INJURY_COUNT) > 7 {\n        return;'),
    ('l_causador', 'let q = (o + 0x31) as *mut i16;\n        *q = (*q).wrapping_sub(10);', 'let q = (o + 0x31) as *mut i16;\n        *q = (*q).wrapping_sub(9);'),
    ('l_grave', 'if g as i8 > 0x32 {\n        crate::comparison::branch("injury serious");', 'if g as i8 > 0x33 {\n        crate::comparison::branch("injury serious");'),
    ('l_folego', 'local = random(ftol((c * c) as f64 * read_f64(rdata::F64_0_5))) as f32;', 'local = random(ftol((c * c) as f64 * read_f64(rdata::F64_0_5)) + 1) as f32;'),
    ('l_59', 'p.set_stamina_boost(r as f32);', 'p.set_stamina_boost((r + 1) as f32);'),
    ('l_d5', '*d5 = (*d5 as f64 * read_f32(rdata::F32_0_5) as f64) as f32;', '*d5 = (*d5 as f64 * read_f32(rdata::F32_0_5) as f64 * 1.01) as f32;'),
    ('l_forca', 'let v = (read_i8(person + player_sheet::INJURY_PRONENESS) as i32 * 25) as f64 + read_f64(rdata::F64_7100);', 'let v = (read_i8(person + player_sheet::INJURY_PRONENESS) as i32 * 26) as f64 + read_f64(rdata::F64_7100);'),
    ('l_conta', '    if *((m + engine::SUBSTITUTION_PENDING) as *mut u8) != 0 {\n        *w(engine::INJURY_COUNT) = (*w(engine::INJURY_COUNT)).wrapping_add(1);', '    if *((m + engine::SUBSTITUTION_PENDING) as *mut u8) != 0 {\n        *w(engine::INJURY_COUNT) = (*w(engine::INJURY_COUNT)).wrapping_add(2);'),
    ('r_soma', '*g = if *chance as i8 as i32 + *g as i8 as i32 > 0x64 { 0x64 }', '*g = if *chance as i8 as i32 + *g as i8 as i32 > 0x63 { 0x63 }', 'injury worsened'),
    ('r_grave', '} else if *chance as i8 > *g as i8 {\n        crate::comparison::branch("injury more serious");', '} else if *chance as i8 >= *g as i8 + 3 {\n        crate::comparison::branch("injury more serious");', 'injury more serious'),
    ('r_nova', 'r[5] = *chance;', 'r[5] = (*chance).wrapping_add(1);', 'injury recorded'),
    ('l_para3', 'if local >= read_f32(rdata::F32_500) && (p.injury_forces_off() as u8) != 0 {', 'if local > read_f32(rdata::F32_500) * 1.5 && (p.injury_forces_off() as u8) != 0 {', 'injury to 3 and leaves'),
    ('d_21b5', '0x21b5 => atuacao > random(1000) + 0x1f40 && b(0x30) != 0,', '0x21b5 => atuacao > random(1000) + 0x1f40,'),
    ('d_sim', 'crate::comparison::branch(if r { "destaque sim" } else { "destaque nao" });', 'let r = r || c == 0x21b1;\n    crate::comparison::branch(if r { "destaque sim" } else { "destaque nao" });'),
]
