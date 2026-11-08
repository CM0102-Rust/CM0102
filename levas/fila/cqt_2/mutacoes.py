# Mutacoes (cqt_2). Os tres construtores rodam uma vez na criacao da carreira de 8 ligas (contadores "cqt_2 <liga> novo"):
# - o Goiano um ano adiante (sem as posicoes previstas o jogo caia em 2001-040 numa rotina do estadual que le as
#   previsoes: trocada, 06/11);
# - o Mineiro com outra regra de disciplina (0 no lugar de 7: outras suspensoes nos jogos do estadual; na Italia, acusada);
# - o Norte um ano adiante (os jogos do estadual saem para o ano seguinte: outros jogos e outra contagem de sorteios).
MUTACOES = [
    ('cqt2_goiano_um_ano_adiante',
     'bra_reg_goias_construct_state_league(this, ano, registro, &CQT2_GOIANO,',
     'bra_reg_goias_construct_state_league(this, ano + 1, registro, &CQT2_GOIANO, /* mutacao: um ano adiante */',
     'cqt_2 goiano novo'),
    ('cqt2_mineiro_outra_disciplina',
     '&CQT2_MINEIRO, 7, true, "cqt_2 mineiro novo")',
     '&CQT2_MINEIRO, 0, true, "cqt_2 mineiro novo") /* mutacao: outra disciplina */',
     'cqt_2 mineiro novo'),
    ('cqt2_norte_um_ano_adiante',
     'bra_reg_goias_construct_state_league(this, ano, registro, &CQT2_NORTE,',
     'bra_reg_goias_construct_state_league(this, ano + 1, registro, &CQT2_NORTE, /* mutacao: um ano adiante */',
     'cqt_2 norte novo'),
]
# Prova do agente (06/11, carreira Rust de 8 ligas ate 2002-06-01; base com os 12 itens cqr/cqs/cqt_1..4, impressao b690295afd222f6a): as tres ACUSADAS (2001-014, 2001-055, 2001-014).
