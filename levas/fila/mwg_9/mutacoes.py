# Mutacoes (mwg_9) (os comparadores so rodam o original: pegam so pela carreira; as preliminares da Copa dos Campeoes
# sao sorteadas e jogadas em julho, dentro da janela ate 2002-08-01):
# - a semeadura da UEFA nao passa para a temporada nova (outros cabecas de chave nas preliminares de julho).
# A Supercopa (fim de agosto) e a Copa da UEFA (preliminares em agosto) ficam sem mutacao propria: o molde da virada
# das copas ja esta provado (mwf_1, mwf_5) e o efeito delas so aparece depois da data da cadeia. Mutacoes que pulam um
# passo da virada (a tabela das preliminares, a arvore) deixam a copa sem tabela e arriscam derrubar o jogo: fora.
MUTACOES = [
    ('mwg9_semeadura_parada',
     "exe::uefaseeding_update_season(seeding, 0); // mwg9: the seeding's new season",
     "let _ = seeding; // mutation: the seeding stays in the old season",
     'mwg_9 european cup rollover'),
]
