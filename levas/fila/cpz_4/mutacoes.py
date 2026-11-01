# Mutacoes (cpz_4). A 0x584ae0 roda a cada desfecho de clube na Liga dos Campeoes (clubes de todas as nacoes): as duas
# primeiras rodadas da qualificacao em julho e agosto de 2001 (os eliminados saem; so os da terceira vao a Copa da
# UEFA), a fase 1 em novembro (os quartos saem) e a fase 2 ate marco de 2002. Duas deixam o clube eliminado com a
# competicao europeia (+0x1db do clube, no bloco dos clubes que a impressao do banco le todo dia; a lista dos
# inscritos 0x586660 e os sorteios seguintes o contam como vivo); a terceira muda o peso da classificacao na fase 2
# (os pontos do tecnico, importancia^2 x peso). Os contadores ficam colados em cada efeito, depois das condicoes.
MUTACOES = [
    ('cpz4_eliminado_na_qualificacao_continua',
     'cpz4_sair(clube); // cpz4: quem cai na qualificação sai da competição europeia',
     'let _ = clube; // mutacao: o eliminado na qualificacao continua na competicao',
     'cpz_4 caiu na qualificacao'),
    ('cpz4_eliminado_na_fase_1_continua',
     'cpz4_sair(clube); // cpz4: quem cai na fase 1 sai da competição europeia',
     'let _ = clube; // mutacao: o eliminado na fase 1 continua na competicao',
     'cpz_4 caiu na fase 1'),
    ('cpz4_fase_2_pesa_metade',
     'cpz4_classificado(clube, comp, 0, 0x78, 0x32); // cpz4: passar da fase 2 pesa 0x32',
     'cpz4_classificado(clube, comp, 0, 0x78, 0x19); // mutacao: passar da fase 2 pesa a metade',
     'cpz_4 passou da fase 2'),
]
