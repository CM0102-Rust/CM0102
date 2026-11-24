# As eliminatórias da CONCACAF para a Copa do Mundo: o construtor, as colocações previstas e a tabela do mata-mata
# caribenho (wc_concacaf_cup.cpp; leitor cx16, rodada 5, 24/11/2026). Simulação: o construtor roda na criação do jogo (a
# campanha de 2000, a das eliminatórias de 2002, dormente: +0x45 = 0, com a lista fixa de 35 seleções) e as três rodam
# de novo na virada da seção entre 2002-06-01 e 2002-08-01 (a campanha de 2004; mwg_13, leva 314). Como o mwg_13:
# LIGAS = 'ENG', ATE = '2002-08-01' (as seções existem em qualquer carreira).
# - 0x9149f0 WcConcacafCup::WcConcacafCup thiscall(ano, registro) ret 8 (quadro SEH só para o new das estatísticas):
#   Cup::Cup 0x518640, a vtable 0x970c24, os campos, Cup::load_from_disk 0x51fc00 (push 1); o ano da campanha (a mesma
#   conta da qualifying_campaign_year), as fases (7), 0x916250 (0: caixa 0x5e), Comp_Stats (0x5ce; null: 0x66),
#   0x917db0, 0x915d40 (0: 0x70), a tabela de rodadas pela virtual +0x3c (-1, &+0x3c, &+0x3a, 0) em +0xa3; ano da
#   campanha abaixo do do jogo: +0x45 = 0 (dword); senão 0x916cd0; Cup::map_fixture_tree 0x518790 (0: 0x8a). O +0x20
#   (seção de baixo) recebe o valor da chave da seção asiática; o +0x1c, -1.
# - 0x917db0 WcConcacafCup::rank_teams thiscall ret 0: cópia da lista +0xb2 (contagem +0xb6) ordenada pela reputação
#   (qsort 0x945333 com 0x4b07d0), libera as seleções da América do Norte (0x775070, push 0; push continente), as
#   colocações (0x4a2540, ret 8) 1 x (3 - isentas), depois a própria posição + 1 até a sexta (o original empilha o byte
#   com lixo nos bytes altos; a chamada só guarda o byte), 7 x 3, 10 x 3 e 13 até contagem - isentas; marca as seleções
#   como ocupadas com esta competição (0x775220, ret 8). Isentas = byte +0xba.
# - 0x915d40 WcConcacafCup::build_team_table thiscall ret 0: +0x3e = 0x18 - byte +0xbb (word), +0xa7 = malloc(6 x n)
#   (null: caixa 0x350 e -1), as linhas com os n primeiros da lista e os bytes +4 e +5 zerados; 1.
# Convenções conferidas na desmontagem e nos pontos de chamada (o construtor pela tabela do setup.rs, ret 8; as outras
# chamadas diretas com mov ecx, esi). Iguais no cm0102-1080.exe (inteiras).
# Comparadores: o construtor só a original contra o plano; rank_teams e build_team_table lado a lado (a original, depois
# a cópia de novo sobre o que ela deixou: as colocações e as marcas de ocupada, ou a tabela, têm de bater).
SAVE = False
LIGAS = 'ENG'
ATE = '2002-08-01'
ROTINAS = [('construct_wc_concacaf_cup', 0x9149f0, [0x6a, 0xff, 0x68, 0x29, 0x58, 0x96, 0x00]),
           ('wc_concacaf_cup_rank_teams', 0x917db0, [0x81, 0xec, 0x08, 0x02, 0x00, 0x00]),
           ('wc_concacaf_cup_build_team_table', 0x915d40, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('as eliminatórias da CONCACAF: o construtor, as colocações previstas e a tabela do mata-mata caribenho',
        'the CONCACAF qualifying: the constructor, the predicted positions and the Caribbean knockout team table',
        [('construct_wc_concacaf_cup', 'WcConcacafCup::WcConcacafCup', '0x9149f0', 'wc_concacaf_cup.cpp', 'thiscall, ret 8',
          'regra: as eliminatórias da CONCACAF nascem com 7 fases, abaixo da seção asiática, no ano do ciclo de quatro anos; uma campanha que começou antes do ano do jogo (a de 2000, numa carreira de 2001) fica dormente; senão os dois grupos da América Central são criados logo',
          'rule: the CONCACAF qualifying is born with 7 stages, below the Asian section, in the year of the four-year cycle; a campaign begun before the game year (2000, in a 2001 career) stays dormant; otherwise the two Central American groups are created at once'),
         ('wc_concacaf_cup_rank_teams', 'WcConcacafCup::rank_teams', '0x917db0', 'wc_concacaf_cup.cpp', 'thiscall, ret 0',
          'regra: as colocações previstas pela reputação: as três melhores (menos as isentas) em 1, depois 4, 5 e 6, as três seguintes 7, as três seguintes 10 e o resto 13; todas as seleções inscritas ficam ocupadas com a competição',
          'rule: the predicted places by reputation: the three best (fewer by the exempt) 1, then 4, 5 and 6, the next three 7, the next three 10 and the rest 13; every entrant national team becomes busy with the competition'),
         ('wc_concacaf_cup_build_team_table', 'WcConcacafCup::build_team_table', '0x915d40', 'wc_concacaf_cup.cpp', 'thiscall, ret 0',
          'regra: o mata-mata caribenho tem as 24 primeiras caribenhas da lista, menos as caribenhas isentas',
          'rule: the Caribbean knockout has the first 24 Caribbean entrants of the list, fewer by the exempt Caribbean nations')])
CAMPOS = [('eliminatórias da CONCACAF +0xb2 lista (35 vagas: 25 caribenhas, 10 da América Central e do Norte), +0xb6 contagem (dword), +0xba isentas, +0xbb isentas caribenhas, +0xbc isentas das outras (bytes) [inferência pelos usos]; valor/ponteiro; lê e grava',
           'CONCACAF qualifying +0xb2 list (35 slots: 25 Caribbean, 10 Central and North American), +0xb6 count (dword), +0xba exempt, +0xbb exempt Caribbean, +0xbc exempt others (bytes) [inference from the uses]; value/pointer; reads and writes')]
