# Mutacoes da leva141 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva141.py --carreira 2002-06-01 --confirmar 2001-10-01
MUTACOES = [
    ('l_limite', 'if n > 3 && sortear(n as i32) != 0 && sortear(0x1f4) != 0 {', 'if n > 4 && sortear(n as i32) != 0 && sortear(0x1f4) != 0 {'),
    ('l_teto', '    if *w(0xf639) > 6 {\n        return;', '    if *w(0xf639) > 7 {\n        return;'),
    ('l_causador', 'let q = (o + 0x31) as *mut i16;\n        *q = (*q).wrapping_sub(10);', 'let q = (o + 0x31) as *mut i16;\n        *q = (*q).wrapping_sub(9);'),
    ('l_grave', 'if g as i8 > 0x32 {\n        crate::comparacao::ramo("lesao grave");', 'if g as i8 > 0x33 {\n        crate::comparacao::ramo("lesao grave");'),
    ('l_folego', 'local = sortear(ftol((c * c) as f64 * f64_em(0x968968))) as f32;', 'local = sortear(ftol((c * c) as f64 * f64_em(0x968968)) + 1) as f32;'),
    ('l_59', '*((p + 0x59) as *mut f32) = r as f32;', '*((p + 0x59) as *mut f32) = (r + 1) as f32;'),
    ('l_d5', '*d5 = (*d5 as f64 * f32_em(0x968f18) as f64) as f32;', '*d5 = (*d5 as f64 * f32_em(0x968f18) as f64 * 1.01) as f32;'),
    ('l_forca', 'let v = (byte(pessoa, 0x2d) as i32 * 25) as f64 + f64_em(0x96d458);', 'let v = (byte(pessoa, 0x2d) as i32 * 26) as f64 + f64_em(0x96d458);'),
    ('l_conta', '    if *b(m, 0x8eb5) != 0 {\n        *w(0xf639) = (*w(0xf639)).wrapping_add(1);', '    if *b(m, 0x8eb5) != 0 {\n        *w(0xf639) = (*w(0xf639)).wrapping_add(2);'),
    ('r_soma', '*g = if *chance as i8 as i32 + *g as i8 as i32 > 0x64 { 0x64 }', '*g = if *chance as i8 as i32 + *g as i8 as i32 > 0x63 { 0x63 }', 'lesao agravada'),
    ('r_grave', '} else if *chance as i8 > *g as i8 {\n        crate::comparacao::ramo("lesao mais grave");', '} else if *chance as i8 >= *g as i8 + 3 {\n        crate::comparacao::ramo("lesao mais grave");', 'lesao mais grave'),
    ('r_nova', 'r[5] = *chance;', 'r[5] = (*chance).wrapping_add(1);', 'lesao registrada'),
    ('l_para3', 'if local >= f32_em(0x96d1a8) && *b(p, 0x1c) != 0 {', 'if local > f32_em(0x96d1a8) * 1.5 && *b(p, 0x1c) != 0 {', 'lesao para 3 e sai'),
    ('d_21b5', '0x21b5 => atuacao > sortear(1000) + 0x1f40 && b(0x30) != 0,', '0x21b5 => atuacao > sortear(1000) + 0x1f40,'),
    ('d_sim', 'crate::comparacao::ramo(if r { "destaque sim" } else { "destaque nao" });', 'let r = r || c == 0x21b1;\n    crate::comparacao::ramo(if r { "destaque sim" } else { "destaque nao" });'),
]
