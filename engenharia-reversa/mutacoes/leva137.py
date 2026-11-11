# Mutacoes da leva137 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva137.py --carreira 2002-06-01 --confirmar 2001-10-01
MUTACOES = [
    ('a_minuto', "let mut rating = (m.referee_rating() as i32 / 1000) as i8;", "let mut rating = (m.referee_rating() as i32 / 1001) as i8;"),
    ('a_cartoes', "    if reds > 2 {\n        rating = rating.wrapping_sub(2);", "    if reds > 3 {\n        rating = rating.wrapping_sub(2);"),
    ('a_sem_cartao', "    } else if reds == 0 {\n        rating = rating.wrapping_add(1);", "    } else if reds == 0 {\n        rating = rating.wrapping_add(2);"),
    ('a_faltas', "    if yellows > 5 {\n        rating = rating.wrapping_sub(1);", "    if yellows > 6 {\n        rating = rating.wrapping_sub(1);"),
    ('a_quatro', "if rating < 4 && random(500) != 0 {", "if rating < 4 && random(499) != 0 {"),
    ('a_oito', "} else if rating > 8 && random(0x19) != 0 {", "} else if rating > 8 && random(0x18) != 0 {"),
    ('a_cartoes_ficha', "*((f + 0xf) as *mut u8) = (*((f + 0xf) as *mut u8)).wrapping_add(*r.add(4));", "*((f + 0xf) as *mut u8) = (*((f + 0xf) as *mut u8)).wrapping_add(*r.add(5));"),
    ('a_media', "*((f + 4) as *mut f32) = (*((f + 0xa) as *mut i16) as f64 / *((f + 0xc) as *mut i16) as f64) as f32;", "*((f + 4) as *mut f32) = (*((f + 0xa) as *mut i16) as f64 / *((f + 0xc) as *mut i16) as f64 * 1.0001) as f32;"),
    ('a_ultima', "*((f + 0x10) as *mut u8) = *r.add(6);", "*((f + 0x10) as *mut u8) = *r.add(5);"),
]
