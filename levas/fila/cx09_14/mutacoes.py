# Mutacoes (cx09_14) pela carreira com a Finlandia ate 2002-08-01 (a temporada finlandesa de 2001 acaba em outubro de
# 2001, com as repescagens; a virada sobe e desce quem elas decidem):
# - o clube da Primeira que vence a repescagem contra a Veikkausliiga fica com status 0xfe (e nao 0): nao sobe (o
#   comparador do plano tambem acusa);
# - as posicoes previstas 3k + 1 (e nao 2k + 1): o comparador das posicoes acusa.
# A record_season_honours (o repasse ao estagio 1) fica sem mutacao propria: so escreve o historico.
MUTACOES = [
    ('cx09_14_vencedor_nao_sobe',
     '                    plan(FinFirstRecord::Promotion, row, Some(0), id)',
     '                    plan(FinFirstRecord::Promotion, row, Some(0xfe), id) // mutation'),
    ('cx09_14_prevista_passo_tres',
     '    two_group_predicted_positions(l, list, true, 2, |club, position| {',
     '    two_group_predicted_positions(l, list, true, 3, |club, position| { // mutation'),
]
