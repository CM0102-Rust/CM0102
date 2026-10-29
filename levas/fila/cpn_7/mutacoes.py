# Mutacoes (com o ramo de cada uma). A busca sem missao deixa de cobrir a nacao da mesma regiao da nacao do clube e a
# nacao do proprio olheiro (o comparando acusa pelo valor; a carreira pelos jogadores que a busca 0x7b5be0 e os
# olheiros 0x7f8ff0 acham, e pelas transferencias que vem deles; a 247a mostrou que cobrir MAIS nacoes nao muda a
# carreira, por isso as duas cobrem MENOS). O amistoso nao sai da lista +0x34c8 (o jogo segue marcado e volta a ser
# tratado nos dias seguintes; o comparando acusa pela lista; a 0x5cc2d0 roda uma vez so e fica sem mutacao).
MUTACOES = [
    ('cpn_busca_mesma_regiao', 'return 1; // a mesma região da nação do clube',
     'return 0; // mutacao: a mesma regiao nao cobre',
     'cpn_7 mesma regiao'),
    ('cpn_busca_nacao_do_olheiro', 'return 1; // a nação não é estrangeira para o olheiro',
     'return 0; // mutacao: a nacao do olheiro nao cobre',
     'cpn_7 nacao do olheiro'),
    ('cpn_amistoso_lista', 'tirar(cpn7_d(this + 0x34c8), jogo) // tira o jogo da lista dos amistosos',
     '1 // mutacao: o jogo fica na lista',
     'cpn_7 tirar o amistoso'),
]
