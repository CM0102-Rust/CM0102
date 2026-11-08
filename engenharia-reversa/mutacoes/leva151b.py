# Mutacoes refeitas da leva151 (onde o ramo roda e a mutacao era fraca). Uso: python mutacoes.py mutacoes\leva151b.py --carreira 2002-06-01
MUTACOES = [
    ('a_fim_intervalo', "    let end = |e: u8| e == 2 || e == 8;", "    let end = |e: u8| e == 2 || e == 8 || e == 4;"),
    ('f_copia_nada', "    core::ptr::copy((p + day_match::FIXTURE_COPY) as *const u8, game as *mut u8, 0x4f);", "    core::ptr::copy((p + day_match::FIXTURE_COPY) as *const u8, game as *mut u8, 0);"),
]
