# Mutacoes (cqt_1). Os tres construtores rodam uma vez na criacao da carreira de 8 ligas (contadores "cqt_1 <liga> novo"):
# - o Baiano um ano adiante (sem as posicoes previstas o jogo caia em 2001-040 numa rotina do estadual que le as
#   previsoes: trocada, 06/11);
# - o Centro-Oeste com outra regra de disciplina (0 no lugar de 7: outras suspensoes nos jogos do estadual; na Italia, acusada);
# - o Gaúcho um ano adiante (os jogos do estadual saem para o ano seguinte: outros jogos e outra contagem de sorteios).
MUTACOES = [
    ('cqt1_baiano_um_ano_adiante',
     'bra_reg_bahia_construct_state_league(this, ano, registro, &BAHIA_CHAMPIONSHIP,',
     'bra_reg_bahia_construct_state_league(this, ano + 1, registro, &BAHIA_CHAMPIONSHIP, /* mutacao: um ano adiante */',
     'cqt_1 baiano novo'),
    ('cqt1_centro_oeste_outra_disciplina',
     '&CENTRO_OESTE_STATE_CHAMPIONSHIP, 7, true, "cqt_1 centro_oeste novo")',
     '&CENTRO_OESTE_STATE_CHAMPIONSHIP, 0, true, "cqt_1 centro_oeste novo") /* mutacao: outra disciplina */',
     'cqt_1 centro_oeste novo'),
    ('cqt1_gaucho_um_ano_adiante',
     'bra_reg_bahia_construct_state_league(this, ano, registro, &GAUCHO_STATE_LEAGUE,',
     'bra_reg_bahia_construct_state_league(this, ano + 1, registro, &GAUCHO_STATE_LEAGUE, /* mutacao: um ano adiante */',
     'cqt_1 gaucho novo'),
]
# Prova do agente (06/11, carreira Rust de 8 ligas ate 2002-06-01; base com os 12 itens cqr/cqs/cqt_1..4, impressao b690295afd222f6a): as tres ACUSADAS (2001-014, 2001-076, 2001-014).
