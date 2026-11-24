# Mutacoes (sx05b_4). O texto nao volta a simulacao: acusadas pelo comparador (compare_text_writer) nas noticias dos
# premios do ano ingleses no fim da temporada.
# - o modelo simples sem o periodo;
# - a manchete sem o caso 2 ("Major award for");
# - sem o vinculo do nome curto do vencedor.
MUTACOES = [
    ('sx05b_4_simples_sem_periodo',
     '        fmt(buf, 0xad5b68, cg, cg, n1, cn, period, n2);',
     '        fmt(buf, 0xad5b68, cg, cg, n1, cn, n2, period); // mutation'),
    ('sx05b_4_manchete_sem_dois',
     '        2 => fmt(buf, 0xad5aac, short),',
     '        2 => fmt(buf, 0xad5a7c, short), // mutation',
     'sx05b_4 year award headline'),
    ('sx05b_4_sem_nome_curto',
     '        exe::text_add_link(text, n.winner_short.as_ptr(), 0x7d7, p);',
     '        // mutation: no short name link'),
]
