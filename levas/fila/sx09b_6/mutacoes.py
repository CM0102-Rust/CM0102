# Mutacoes (sx09b_6). Acusadas pelo comparador quando uma disputa acontece e a noticia chega a um tecnico humano; as
# disputas provavelmente nao acontecem na carreira de ferias ENG ate 2002-06-01 (ver sx09b_5): se escaparem, e por isso.
# - o resultado com um a mais na vaga 9.
MUTACOES = [
    ('sx09b_disputa_resultado',
     'set_news_slot(n, 9, outcome as i8 as i32 as u32); // sx09b_6: the outcome',
     'set_news_slot(n, 9, (outcome as i8 as i32 + 1) as u32); // mutation',
     "sx09b_6 dispute news to the club's nation"),
]
