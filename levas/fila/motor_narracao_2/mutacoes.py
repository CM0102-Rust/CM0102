# Mutacoes do match_wrap_up do motor (5). Uso: python mutacoes.py mutacoes\levaN.py --carreira 2002-06-01
MUTACOES = [
    ('n_anterior', "    let k = if w(t + 8) == 0 { 0xb } else { b(t, 8).wrapping_sub(1) };", "    let k = if w(t + 8) == 0 { 0xa } else { b(t, 8).wrapping_sub(1) };"),
    ('n_copia', "    let copiar = || core::ptr::copy((q as *const u8).add(5), (e as *mut u8).add(5), 8);", "    let copiar = || core::ptr::copy((q as *const u8).add(5), (e as *mut u8).add(5), 4);"),
    ('n_grupo', "                if GRUPO.contains(&s3) && GRUPO.contains(&s2) && sorteio(2) != 0 {\n                    return -0x2169;", "                if GRUPO.contains(&s3) && GRUPO.contains(&s2) && sorteio(2) != 0 {\n                    return -0x2168;"),
    ('x_media', "    let ate_media = |limit: i16| media().is_some_and(|v| v >= limit) as u32;", "    let ate_media = |limit: i16| media().is_some_and(|v| v > limit) as u32;"),
    ('x_minuto', "            if !do_clube() || sortear(-3) <= agregado || minuto < 0x1f || !na_competicao() {", "            if !do_clube() || sortear(-3) <= agregado || minuto < 0x20 || !na_competicao() {"),
    ('x_tabela', "        if valor <= sortear(5) + 8 {", "        if valor <= sortear(5) + 9 {"),
    ('j_forca', "    w(0x35, 0x17d4);", "    w(0x35, 0x17d5);"),
]
