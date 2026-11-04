# Mutacoes (cqj_12), nas datas de comeco das eliminatorias (a primeira palavra da Date em +0xb6, de onde parte a tabela
# das rodadas da virtual +0x3c): uma semana antes muda as datas dos jogos das selecoes e as convocacoes (os jogadores
# das 8 ligas). As regras de pontos e a categoria (vitoria valendo 2, categoria 9) ESCAPARAM na carreira de 8 ligas ate
# 2002-06-01 (04/11, cm0102_agD2): os grupos sao estagios proprios (League_Stage) com regras proprias, e os campos da
# liga-mae nao chegam aos jogos; por isso as mutacoes ficam nas datas.
MUTACOES = [
    ('cqj12_europa_uma_semana_antes',
     'let dia = 0x19; // cqj12: o dia do começo das eliminatórias europeias',
     'let dia = 0x12; // mutacao: uma semana antes',
     'cqj_12 regras da europa'),
    ('cqj12_america_do_sul_uma_semana_antes',
     'let dia = 0x19; // cqj12: o dia do começo da eliminatória sul-americana',
     'let dia = 0x12; // mutacao: uma semana antes',
     'cqj_12 regras da america do sul'),
    ('cqj12_asia_uma_semana_antes',
     'let dia = 0x19; // cqj12: o dia do começo das eliminatórias asiáticas',
     'let dia = 0x12; // mutacao: uma semana antes',
     'cqj_12 regras da asia'),
]
