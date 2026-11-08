# Mutacoes da copa (cpq_11). O lado sorteado ao contrario quando as duas vagas estao livres (o mesmo numero de
# sorteios; os mandantes das copas inglesas mudam: renda e resultado). Qualquer par de clubes com estadio conta como
# estadio compartilhado (sem a igualdade: quase todo confronto seguinte da rodada se inverte; o comparando acusa na
# primeira rodada sorteada, a carreira pelos mandantes). A escala pela rodada tackle pelas rodadas em vez de
# rodadas - 1 (os registros do jogador da EngAutoCup, 0x86bdd0/0x86c000; PODE ESCAPAR da carreira se a EngAutoCup nao
# chamar ate 2002-06-01: o ramo diz).
MUTACOES = [
    ('cpq_sorteio_do_lado', '                    if casa_livre && sorteio(2) != 0 {',
     '                    if casa_livre && sorteio(2) == 0 { // mutacao: o lado ao contrario',
     'cpq_11 sorteio mandante'),
    ('cpq_estadio_qualquer', '                    if sa == sb && sa != 0 {',
     '                    if sa != 0 && sb != 0 { // mutacao: qualquer estadio',
     'cpq_11 estadio comparado'),
    ('cpq_escala_rodadas', '.wrapping_div(n - 1); // a média por rodada',
     '.wrapping_div(n); // mutacao: rodadas em vez de rodadas - 1',
     'cpq_11 escala pela rodada'),
]
