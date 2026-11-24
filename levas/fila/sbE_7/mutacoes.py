# Mutacoes (sbE_7), com a Polonia ate 2002-08-01 (os calendarios de 2001/02 na criacao do jogo, a fase final em
# novembro de 2001, a repescagem em junho de 2002):
# - a primeira rodada da fase final da Primeira 7 dias depois (acusada pelo comparador lado a lado);
# - uma rodada de meio de semana da Segunda em 2001 7 dias depois (acusada pelo comparador lado a lado);
# - a repescagem de acesso em jogo unico (quem sobe muda: a carreira; acusada tambem pelo comparador).
MUTACOES = [
    ('sbE_7_fase_final_depois',
     '(17, 10, 0, 5, 1, &[(-1, -1, -1)]), // 0: 17 Nov',
     '(24, 10, 0, 5, 1, &[(-1, -1, -1)]), // 0: 24 Nov (mutation)',
     'sbE_7 polish first division schedule'),
    ('sbE_7_segunda_meio_de_semana',
     '(20, 8, 0, 2, 2, &[(-1, -1, -1)]), // 8: 20 Sep',
     '(27, 8, 0, 2, 2, &[(-1, -1, -1)]), // 8: 27 Sep (mutation)',
     'sbE_7 polish second division schedule'),
    ('sbE_7_repescagem_jogo_unico',
     '(3, 5, 1, 5, 1, 0), 0xa0, (0, 0x200, 3), 5, (4, 2, 4, 0), (0, 2, 4), (0, 0, 0));',
     '(3, 5, 1, 5, 1, 0), 0xa0, (0, 0x200, 3), 5, (4, 2, 4, 0), (0, 1, 4), (0, 0, 0)); // mutation: one leg',
     'sbE_7 polish play-off round'),
]
