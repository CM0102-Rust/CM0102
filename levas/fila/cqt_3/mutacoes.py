# Mutacoes (cqt_3). Os tres construtores rodam uma vez na criacao da carreira de 8 ligas (contadores "cqt_3 <liga> novo"):
# - o Nordeste um ano adiante (sem as posicoes previstas o jogo caia em 2001-040 numa rotina do estadual que le as
#   previsoes: trocada, 06/11);
# - o Paranaense com outra regra de disciplina (0 no lugar de 7: outras suspensoes nos jogos do estadual; na Italia, acusada);
# - o Pernambucano um ano adiante (os jogos do estadual saem para o ano seguinte: outros jogos e outra contagem de sorteios).
MUTACOES = [
    ('cqt3_nordeste_um_ano_adiante',
     'cqt3_construir(this, ano, registro, &CQT3_NORDESTE,',
     'cqt3_construir(this, ano + 1, registro, &CQT3_NORDESTE, /* mutacao: um ano adiante */',
     'cqt_3 nordeste novo'),
    ('cqt3_paranaense_outra_disciplina',
     '&CQT3_PARANAENSE, 7, true, "cqt_3 paranaense novo")',
     '&CQT3_PARANAENSE, 0, true, "cqt_3 paranaense novo") /* mutacao: outra disciplina */',
     'cqt_3 paranaense novo'),
    ('cqt3_pernambucano_um_ano_adiante',
     'cqt3_construir(this, ano, registro, &CQT3_PERNAMBUCANO,',
     'cqt3_construir(this, ano + 1, registro, &CQT3_PERNAMBUCANO, /* mutacao: um ano adiante */',
     'cqt_3 pernambucano novo'),
]
