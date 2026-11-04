# Mutacoes (cqk_10), nas datas dos torneios de selecoes (a primeira palavra da Date em +0xb6, de onde parte a tabela
# das rodadas da virtual +0x3c): uma semana depois muda as datas dos jogos e das convocacoes (jogadores das 8 ligas).
# A vitoria valendo 2 na Copa Africana e na Copa America ESCAPOU na carreira de 8 ligas ate 2002-06-01 (04/11,
# cm0102_agD2): os grupos sao estagios proprios (League_Stage) com regras proprias; por isso as mutacoes ficam nas datas.
MUTACOES = [
    ('cqk10_copa_africana_uma_semana_depois',
     'let dia = 5; // cqk10: o dia da data da Copa Africana',
     'let dia = 12; // mutacao: uma semana depois',
     'cqk_10 regras da copa africana'),
    ('cqk10_copa_america_uma_semana_depois',
     'let dia = 0x14; // cqk10: o dia da data da Copa América',
     'let dia = 0x1b; // mutacao: uma semana depois',
     'cqk_10 regras da copa america'),
    ('cqk10_oceania_uma_semana_depois',
     'let dia = 0xf; // cqk10: o dia do começo na Oceania',
     'let dia = 0x16; // mutacao: uma semana depois',
     'cqk_10 regras da oceania'),
]
