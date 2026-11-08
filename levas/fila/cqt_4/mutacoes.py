# Mutacoes (cqt_4). Os tres construtores rodam uma vez na criacao da carreira de 8 ligas (contadores "cqt_4 <liga> novo"):
# - o Carioca um ano adiante (sem as posicoes previstas o jogo caia em 2001-040 numa rotina do estadual que le as
#   previsoes: trocada, 06/11);
# - o Catarinense com outra regra de disciplina (0 no lugar de 7: outras suspensoes nos jogos do estadual; na Italia, acusada);
# - o Paulista um ano adiante (os jogos do estadual saem para o ano seguinte: outros jogos e outra contagem de sorteios).
# - a media de publico com a mistura do publico real pesando 0,25 em vez de 0,2 (pedido do Lucas: o peso da mistura):
#   outras medias de publico dos clubes do meio da tabela no fim das temporadas brasileiras (outras rendas e publicos).
MUTACOES = [
    ('cqt4_carioca_um_ano_adiante',
     'bra_reg_rio_construct_state_league(this, ano, registro, &CQT4_CARIOCA,',
     'bra_reg_rio_construct_state_league(this, ano + 1, registro, &CQT4_CARIOCA, /* mutacao: um ano adiante */',
     'cqt_4 carioca novo'),
    ('cqt4_catarinense_outra_disciplina',
     '&CQT4_CATARINENSE, 7, true, "cqt_4 catarinense novo")',
     '&CQT4_CATARINENSE, 0, true, "cqt_4 catarinense novo") /* mutacao: outra disciplina */',
     'cqt_4 catarinense novo'),
    ('cqt4_paulista_um_ano_adiante',
     'bra_reg_rio_construct_state_league(this, ano, registro, &CQT4_PAULISTA,',
     'bra_reg_rio_construct_state_league(this, ano + 1, registro, &CQT4_PAULISTA, /* mutacao: um ano adiante */',
     'cqt_4 paulista novo'),
    ('cqt4_publico_mistura_mais_pesada',
     'match_pl_ftol(val as f64 * 0.2)); // cqt4: a mistura com o público real',
     'match_pl_ftol(val as f64 * 0.25)); // mutacao: a mistura pesa mais',
     'cqt_4 publico do meio da tabela'),
]
# Prova do agente (06/11, carreira Rust de 8 ligas ate 2002-06-01; base com os 12 itens cqr/cqs/cqt_1..4, impressao b690295afd222f6a): as quatro ACUSADAS (2001-014, 2001-055, 2001-021; a media de publico em 2002-012, no fim da temporada brasileira).
