# Mutações (rx07_8). Os textos só rodam quando a caixa de um técnico humano mostra a notícia: o comparador de texto
# (lado a lado) acusa numa sessão do Lucas com a comparação; a carreira automática não os roda.
# - a manchete da proposta cancelada usa o nome no modo 1, não 5;
# - o corpo do teste cancelado liga o nome curto do clube, não o completo.
MUTACOES = [
    ('rx07_8_manchete_modo_1',
     '        news_text_person_name(s1, s2, s3, &mut name, 5, person); // rx07_8: the headline name',
     '        news_text_person_name(s1, s2, s3, &mut name, 1, person); // rx07_8: the headline name (mutation)',
     'rx07_8 offer cancelled text'),
    ('rx07_8_link_nome_curto',
     '        exe::text_add_link(out, club_name as *const u8, 0x7d5, c);',
     '        exe::text_add_link(out, (c.addr() + club::SHORT_NAME) as *const u8, 0x7d5, c); // mutation',
     'rx07_8 trial cancelled text'),
]
