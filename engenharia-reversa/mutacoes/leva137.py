# Mutacoes da leva137 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva137.py --carreira 2002-06-01 --confirmar 2001-10-01
MUTACOES = [
    ('a_minuto', "let mut rating = (*((m + engine::REFEREE_RATING) as *const i16) as i32 / 1000) as i8;", "let mut rating = (*((m + engine::REFEREE_RATING) as *const i16) as i32 / 1001) as i8;"),
    ('a_cartoes', "    if cards > 2 {\n        rating = rating.wrapping_sub(2);", "    if cards > 3 {\n        rating = rating.wrapping_sub(2);"),
    ('a_sem_cartao', "    } else if cards == 0 {\n        rating = rating.wrapping_add(1);", "    } else if cards == 0 {\n        rating = rating.wrapping_add(2);"),
    ('a_faltas', "    if faltas > 5 {\n        rating = rating.wrapping_sub(1);", "    if faltas > 6 {\n        rating = rating.wrapping_sub(1);"),
    ('a_quatro', "if rating < 4 && sortear(500) != 0 {", "if rating < 4 && sortear(499) != 0 {"),
    ('a_oito', "} else if rating > 8 && sortear(0x19) != 0 {", "} else if rating > 8 && sortear(0x18) != 0 {"),
    ('a_cartoes_ficha', "*b(0xf) = (*b(0xf)).wrapping_add(*r.add(4));", "*b(0xf) = (*b(0xf)).wrapping_add(*r.add(5));"),
    ('a_media', "*((f + 4) as *mut f32) = (*w(0xa) as f64 / *w(0xc) as f64) as f32;", "*((f + 4) as *mut f32) = (*w(0xa) as f64 / *w(0xc) as f64 * 1.0001) as f32;"),
    ('a_ultima', "*b(0x10) = *r.add(6);", "*b(0x10) = *r.add(5);"),
]
