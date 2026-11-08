# Mutacoes. As copas sorteiam e montam as rodadas na data de cada uma (sorteios e jogos que mudam o calendario, o
# elenco usado e a sequencia do aleatorio): o sorteio da rodada um dia depois (a data igual a hoje deixa de bastar) e a
# montagem da rodada um dia depois (a data de ontem em vez da de hoje; a pendente segue igual). O risco de doenca da
# viagem com o teste do terceiro mundo invertido (6.064 chamadas ate 2002-06-01: os visitantes das viagens comuns ganham
# o risco e os elencos mudam); sem ramo, porque o ramo do efeito nao roda na base (as ligas da carreira sao europeias).
MUTACOES = [
    ('cps13_sorteio_atrasado', 'if ano > hano || (ano == hano && dia as i16 > hdia as i16) { // cps13: a data do sorteio',
     'if ano > hano || (ano == hano && dia as i16 >= hdia as i16) { // mutacao: um dia depois',
     'cps_13 round draw'),
    ('cps13_montagem_atrasada', 'if (ano, dia) != (hano, hdia) { // cps13: a data da montagem',
     'if (ano, dia) != (hano, hdia.wrapping_sub(1)) { // mutacao: um dia depois',
     'cps_13 round setup'),
    ('cps13_doenca_invertida', 'if cps13_b(nacao + 0x7e) != 3 { // cps13: só o terceiro mundo',
     'if cps13_b(nacao + 0x7e) == 3 { // mutacao: menos o terceiro mundo'),
]
