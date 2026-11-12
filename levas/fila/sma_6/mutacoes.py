# Mutacoes (sma_6). Os textos nao voltam para a simulacao: as mutacoes sao acusadas pela comparacao (compare_text_writer
# e o lado a lado do buffer), aceito pela sessao principal em 12/11/2026. Na carreira ate 2002-06-01: 0x6a4ae0 roda 7
# vezes, 0x6a4d40 84, 0x694440 1.
# - a frase do titulo com as colunas de 0x800 trocadas;
# - a manchete das expectativas sempre "announce expectations";
# - o corpo da confianca sem os jogadores citados.
MUTACOES = [
    ('sma6_frase_do_titulo',
     'say(t[col][late][outside_europe as usize]); // sma6: the title sentence',
     'say(t[col][1 - late][outside_europe as usize]); // mutation: the 0x800 columns swapped',
     'sma_6 title expectation'),
    ('sma6_manchete_fixa',
     'format(t, h); // sma6: the expectations headline',
     'format(t, 0xa0ab18 + 0 * h); // mutation: always "announce expectations"',
     'sma_6 expectations body'),
    ('sma6_sem_jogadores',
     'sprintf(t, 0x991948, first.as_ptr() as usize, players.as_ptr() as usize); // sma6: two sentences',
     'sprintf(t, 0x991944, first.as_ptr() as usize); // mutation: without the named players',
     'sma_6 confidence body'),
]
