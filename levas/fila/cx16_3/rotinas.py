# As eliminatórias da CONCACAF para a Copa do Mundo: as inscritas, os dois grupos da América Central e o dia da seção
# (wc_concacaf_cup.cpp; leitor cx16, rodada 5, 24/11/2026). Simulação, LIGAS = 'ENG', ATE = '2002-08-01' (como o
# mwg_13): as inscritas rodam na criação (a campanha de 2000, a lista fixa) e na virada da seção entre 2002-06-01 e
# 2002-08-01 (a campanha de 2004, pela base de dados); os grupos só na virada (a campanha de 2000 fica dormente); o dia
# todo dia (cobertura_competicoes: 2064 chamadas até 2002-06-01), sem chegar ao dia de ocupar as seleções (10 de janeiro
# do ano da campanha: 2000 e 2004).
# - 0x916250 WcConcacafCup::select_entrants thiscall ret 0: zera +0xba/+0xbb/+0xbc; as sedes da Copa de daqui a dois
#   anos (HostCountry::find_hosts 0x5fa730, 5 pushes, ret 0x14; sem [0xb5e134], caixa 0x467) e a última campeã
#   (0x4afd80, cdecl) da América do Norte contam como isentas (região 6 em +0xbb, as outras em +0xbc); +0xb6 = 35, +0xb2 =
#   calloc(35, 4) (0x945501; null: caixa 0x4aa e 0); ano 2000: as 35 seleções fixas (0x53b3d0) e 1; senão as seleções
#   da América do Norte (e as sem continente) na ordem da base que não são sede nem campeã: região 6 nas 25 primeiras
#   vagas, região 7 nas 10 últimas, até 25 - (+0xbb) e 10 - (+0xbc); faltando, 0; senão as duas partes ordenadas (qsort
#   0x945333 com 0x4b0810) e 1.
# - 0x916cd0 WcConcacafCup::create_central_american_groups thiscall ret 0 (quadro SEH só para os new): vetor de 3
#   (0xc; null: caixa 0x572); o grupo 0 com as vagas 25, 27 (e 29 se +0xbc < 2), o grupo 1 com 26, 28 (e 30 se +0xbc <
#   1), as vagas zeradas; cada grupo: a tabela de rodadas pela virtual +0x3c (fase 0 ou 1), operator new 0xee e
#   League_Stage::League_Stage 0x68b100 (ret 0x58; 2, o registro, a tabela, as rodadas, 3, 1, 0xa, {1,2,3,0}, {1,1,0,0},
#   o ano, o índice do grupo, o rótulo, 0xf, 4, 0, 0x28, -1, 0, 2) em +0xc[0] e +0xc[1] (null: caixas 0x591 e 0x5b3),
#   a tabela liberada e a notícia do sorteio (0x684230); no fim o vetor liberado, +0x30 = 1 e a notícia do segundo
#   grupo DE NOVO (defeito do original: sai duas vezes).
# - 0x915e40 WcConcacafCup::update_for_date thiscall(hoje, completo) ret 8 (quadro SEH só para a data local): Date::Date
#   0x549f70 (10, 0, ano, -1); hoje = 10 de janeiro do ano da campanha e completo: libera as seleções da América do
#   Norte (0x775070) e ocupa (0x775220, ret 8) as inscritas não nulas (a nação sem conferir) e as seleções das fases de
#   -1 (a tabela de times da copa, linhas de 6) até a atual (as tabelas das fases, linhas de 0x3b); depois
#   Cup::update_for_date 0x51f890 (ret 8), cujo eax devolve.
# Convenções conferidas na desmontagem e nos pontos de chamada. Iguais no cm0102-1080.exe (inteiras).
# Comparadores: as inscritas lado a lado (a original, depois a cópia com uma lista própria, liberada, e o estado da
# original devolvido); os grupos só a original contra o plano (as vagas, as tabelas dos grupos e a fase atual); o dia
# só a original (no dia de ocupar, as inscritas ocupadas com a seção).
SAVE = False
LIGAS = 'ENG'
ATE = '2002-08-01'
ROTINAS = [('wc_concacaf_cup_select_entrants', 0x916250, [0x81, 0xec, 0x14, 0x02, 0x00, 0x00]),
           ('wc_concacaf_cup_create_central_american_groups', 0x916cd0, [0x6a, 0xff, 0x68, 0x9c, 0x58, 0x96, 0x00]),
           ('wc_concacaf_cup_update_for_date', 0x915e40, [0x6a, 0xff, 0x68, 0x68, 0x58, 0x96, 0x00])]
DOCS = ('as eliminatórias da CONCACAF: as inscritas, os grupos da América Central e o dia da seção',
        'the CONCACAF qualifying: the entrants, the Central American groups and the section day',
        [('wc_concacaf_cup_select_entrants', 'WcConcacafCup::select_entrants', '0x916250', 'wc_concacaf_cup.cpp', 'thiscall, ret 0',
          'regra: as eliminatórias da CONCACAF têm 25 vagas caribenhas e 10 da América Central e do Norte, menos as sedes e a campeã da Copa do Mundo que forem da América do Norte; a campanha de 2000 tem as suas 35 seleções fixas; as seguintes tomam as seleções norte-americanas na ordem da base, cada parte ordenada',
          'rule: the CONCACAF qualifying has 25 Caribbean places and 10 Central and North American ones, less the World Cup hosts and holder from North America; the 2000 campaign has its fixed 35 national teams; later ones take the North American national teams in database order, each part sorted'),
         ('wc_concacaf_cup_create_central_american_groups', 'WcConcacafCup::create_central_american_groups', '0x916cd0', 'wc_concacaf_cup.cpp', 'thiscall, ret 0',
          'regra: os dois grupos da América Central tomam as vagas cabeças de chave alternadas (o primeiro 25, 27 e 29, o segundo 26, 28 e 30; dois por grupo quando há isentas), 3 pontos por vitória e 1 por empate; a notícia do sorteio do segundo grupo sai duas vezes (defeito do original)',
          'rule: the two Central American groups take the seeded slots in turn (the first 25, 27 and 29, the second 26, 28 and 30; two per group with exempt nations), 3 points a win and 1 a draw; the second group draw news goes out twice (defect of the original)'),
         ('wc_concacaf_cup_update_for_date', 'WcConcacafCup::update_for_date', '0x915e40', 'wc_concacaf_cup.cpp', 'thiscall, ret 8',
          'regra: em 10 de janeiro do ano da campanha, na atualização completa, todas as seleções inscritas e as que já estão numa fase ficam ocupadas com as eliminatórias; depois o dia da copa',
          'rule: on 10 January of the campaign year, in the full update, every entrant national team and every team already in a stage becomes busy with the qualifying; then the cup day')])
CAMPOS = [('eliminatórias da CONCACAF +0xb2 lista (35 vagas), +0xb6 contagem, +0xba/+0xbb/+0xbc isentas; nação +0x75 região (6 Caribe, 7 América Central e do Norte [inferência]); valor/ponteiro; lê e grava',
           'CONCACAF qualifying +0xb2 list (35 slots), +0xb6 count, +0xba/+0xbb/+0xbc exempt; nation +0x75 region (6 Caribbean, 7 Central and North America [inference]); value/pointer; reads and writes')]
