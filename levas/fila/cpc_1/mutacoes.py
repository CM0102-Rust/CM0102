# Mutacoes das competicoes. A da proxima partida inverte o filtro da competicao (sorteios de copa, 259 chamadas): a
# carreira ate 2002-06-01 deve acusar; so escaparia se todas as chamadas passassem comp -1. A do estadio em uso inverte
# a comparacao do estadio (marcacao dos amistosos, 2.203 chamadas): deve acusar. A da data internacional passa o ano
# seguinte a 0x54a130 (220 chamadas): deve acusar.
MUTACOES = [
    ('cpc_proxima_competicao', '(comp == -1 || comp == d(0)) // a competição', '(comp == -1 || comp != d(0)) // mutacao: a competicao trocada'),
    ('cpc_estadio_em_uso', 'if e != 0 && e == estadio { // o estádio da partida', 'if e != 0 && e != estadio { // mutacao: outro estadio'),
    ('cpc_data_amistoso', 'acertar(this + 3, alto | b(1), mes, ano, semana) // a data com o ano dado', 'acertar(this + 3, alto | b(1), mes, ano.wrapping_add(1), semana) // mutacao: o ano seguinte'),
]
