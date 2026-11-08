# Mutacoes das noticias e da copa. Uso: python mutacoes.py mutacoes\levaN.py --carreira 2002-06-01
MUTACOES = [
    ('m_resposta', "    any_news_for_player(midia, *id, |r| *((r + 0x10) as *const u8) == 2) as u32", "    any_news_for_player(midia, *id, |r| *((r + 0x10) as *const u8) >= 2) as u32"),
    ('m_alguma', "    any_news_for_player(midia, *id, |_| true) as u8", "    any_news_for_player(midia, *id, |r| *((r + 0x10) as *const u8) != 0) as u8"),
    ('c_fase', "    if (al as i8) < 1 {\n        al = 1;", "    if (al as i8) < 2 {\n        al = 1;"),
    ('c_margem', "        if (*margem as i8) < 1 {\n            *margem = 0xff;", "        if (*margem as i8) < 0 {\n            *margem = 0xff;"),
]
