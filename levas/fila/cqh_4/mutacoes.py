# Mutacoes (cqh_4), na tabela das rodadas das fases de grupos da Liga dos Campeoes (0x583c50: as 8 fases da primeira
# fase no sorteio de agosto de 2001 e as 4 da segunda em novembro):
# - a primeira rodada dos grupos A a D uma semana depois (19 de setembro): os jogos mudam de data;
# - a primeira rodada dos grupos E a H uma semana depois;
# - a ultima rodada dos grupos C e D da segunda fase uma semana depois (21 de marco de 2002).
# O contador fica depois da alocacao da tabela (todas as fases de grupos passam por ele).
MUTACOES = [
    ('cqh4_primeira_rodada_a_d_depois',
     '[(0, 0xc, 8, 0, 1), (1, 0x14, 8, 0, 2), (2, 0x1b, 8, 0, 2), (3, 0x11, 9, 0, 1), (4, 0x19, 9, 0, 2), (5, 7, 0xa, 0, 1)], // cqh4: a primeira fase, grupos A a D',
     '[(0, 0x13, 8, 0, 1), (1, 0x14, 8, 0, 2), (2, 0x1b, 8, 0, 2), (3, 0x11, 9, 0, 1), (4, 0x19, 9, 0, 2), (5, 7, 0xa, 0, 1)], // mutacao: uma semana depois',
     'cqh_4 round fixtures'),
    ('cqh4_primeira_rodada_e_h_depois',
     '[(0, 0xd, 8, 0, 2), (1, 0x13, 8, 0, 1), (2, 0x1a, 8, 0, 1), (3, 0x12, 9, 0, 2), (4, 0x18, 9, 0, 1), (5, 8, 0xa, 0, 2)], // cqh4: a primeira fase, grupos E a H',
     '[(0, 0x14, 8, 0, 2), (1, 0x13, 8, 0, 1), (2, 0x1a, 8, 0, 1), (3, 0x12, 9, 0, 2), (4, 0x18, 9, 0, 1), (5, 8, 0xa, 0, 2)], // mutacao: uma semana depois',
     'cqh_4 round fixtures'),
    ('cqh4_ultima_rodada_segunda_fase_depois',
     '[(0, 0x16, 0xa, 0, 2), (1, 5, 0xb, 0, 1), (2, 0xd, 1, 1, 1), (3, 0x15, 1, 1, 2), (4, 6, 2, 1, 1), (5, 0xe, 2, 1, 2)], // cqh4: a segunda fase, grupos C e D',
     '[(0, 0x16, 0xa, 0, 2), (1, 5, 0xb, 0, 1), (2, 0xd, 1, 1, 1), (3, 0x15, 1, 1, 2), (4, 6, 2, 1, 1), (5, 0x15, 2, 1, 2)], // mutacao: uma semana depois',
     'cqh_4 round fixtures'),
]
