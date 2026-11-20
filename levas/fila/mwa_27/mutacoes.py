# Mutacoes (mwa_27): a abertura da Liga da Irlanda 7 dias antes (8/8; as duas divisoes); a volta do play-off 9
# dias depois da ida em vez de 2 (outras condicoes no jogo que decide o acesso; o comparador lado a lado pega se a
# carreira nao).
MUTACOES = [
    ('mwa27_abertura_7_dias_antes', '[(15, 7, 0, 6, 1, &[(-1, -1, -1)])]; // mwa27: the opening matchday', '[(8, 7, 0, 6, 1, &[(-1, -1, -1)])]; // mutation: 8 August', 'mwa_27 irish premier schedule'),
    ('mwa27_volta_9_dias_depois', '(2, 1, 2, 0), (0, 2, 2), (0, 0, 0)); // mwa27: the play-off on 27 April', '(2, 1, 2, 0), (0, 2, 9), (0, 0, 0)); // mutation: the second leg 9 days later', 'mwa_27 irish play-off round'),
]
