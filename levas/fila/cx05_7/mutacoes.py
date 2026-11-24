# Mutacoes (cx05_7), carreira de 8 ligas ate 2004-07-15 (a semeadura em 10 de janeiro de 2004, o sorteio em 10 de
# fevereiro, os jogos em junho de 2004).
# - o detentor do titulo vai para a quarta vaga e nao a terceira: os potes mudam (o comparador lado a lado da
#   semeadura acusa; a carreira tambem, pelos grupos);
# - o pote gravado na lista e k/2 e nao k/4: o comparador lado a lado acusa;
# - o sorteio enche cada grupo com um pote a mais: um grupo pode ficar com dois times do mesmo pote (a carreira acusa
#   pelos grupos).
MUTACOES = [
    ('cx05_7_detentor_quarta_vaga',
     '            swap_entrants(e, copy + 0xc); // cx05_7: the holder in the third seed',
     '            swap_entrants(e, copy + 0x12); // mutation'),
    ('cx05_7_pote_metade',
     '        write_u8(seeded + 4, (k / 4) as u8); // cx05_7: the pot of four',
     '        write_u8(seeded + 4, (k / 2) as u8); // mutation'),
    ('cx05_7_nivel_mais_um',
     '        let level = (k / 4) as i8 + 1; // cx05_7: the pot fills one team per group',
     '        let level = (k / 4) as i8 + 2; // mutation'),
]
