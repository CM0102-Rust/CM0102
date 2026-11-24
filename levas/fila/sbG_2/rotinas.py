# As eliminatorias africanas da Copa do Mundo: o construtor, as tabelas de rodadas e as colocacoes previstas
# (wc_african_cup.cpp; leitor sbG, rodada 5, 24/11/2026). Simulacao, LIGAS = 'ENG' e ATE = '2002-08-01', como os
# irmaos das outras secoes (cx16_1 e cx16_2, de que depende, e o mwg_13): as eliminatorias existem em todo jogo; as tres
# rodam na criacao (a campanha de 2000, dormente: +0x45 = 0) e a tabela e as colocacoes de novo na virada da secao entre
# 2002-06-01 e 2002-08-01 (cobertura do mundo: 1, 2 e 2 chamadas).
# - 0x90f210 WcAfricanCup::WcAfricanCup thiscall(ano, registro) ret 8 (quadro SEH so para o new das estatisticas):
#   Cup::Cup 0x518640, +4 o registro, a vtable 0x970ad0, os campos (+0x1c a chave da secao sul-americana 0x9cf778, +0x20
#   a da europeia 0x9cf780), o ano da campanha (a conta da qualifying_campaign_year, antes da carga), Cup::load_from_disk
#   0x51fc00 (push 1); 0x9106a0 (0: caixa 0x55), as fases pelo byte +0xba (0 ou 3: 5, 1: 4, outro: 7), Comp_Stats (0x5ce;
#   null: 0x65), o vetor das fases, +0xb1 = 0 e +0x3a = 0, 0x9111d0, 0x910120 (so 0 e erro: caixa 0x7b), a tabela de
#   rodadas pela virtual +0x3c (-1, &+0x3c, &+0x3a, 0) em +0xa3; ano da campanha abaixo do do jogo: +0x45 = 0 (dword);
#   Cup::map_fixture_tree 0x518790 (0: 0x90).
# - 0x90f600 WcAfricanCup::create_round_table thiscall(fase, &rodadas, &extra, &e_liga) ret 0x10 (metodo +0x3c): fase -1
#   a primeira rodada (copa, 1 rodada, 0x68, linha 0xde; 50/48/46 selecoes pelo byte +0xba); os grupos (formato 0 abaixo
#   de 5, 1 abaixo de 4, 2 abaixo de 6, 3 abaixo de 4; liga de 10 rodadas, ou 6 no formato 2; rotulo 0x407 + fase, 0x472
#   na fase 5; linha 0x126); a repescagem final (formato 2 fase 6, formato 3 fase 4; copa, 1 rodada, 0x68, linha 0x171;
#   6 ou 4 selecoes); outra, 0 sem escrever. O indice da ultima rodada do grupo vai com o ebx do chamador nos bytes altos
#   (0x68a160 le a palavra).
# - 0x9111d0 WcAfricanCup::rank_teams thiscall() ret 0: sem estatisticas, caixa 0x556; copia da lista +0xb2 (contagem
#   +0xb6; sem memoria, caixa 0x55e) ordenada pela reputacao (qsort 0x945333 com 0x4b07d0), libera as selecoes da Africa
#   (0x775070, push 0; push continente), as colocacoes (0x4a2540, ret 8) por faixas do formato +0xba, e cada selecao
#   ocupada com esta competicao (0x775220, ret 8), sem conferir nulos. Defeito do original: no formato 3 os dois
#   primeiros recebem 1 e logo depois 3 (ficam com 3).
# Prologos: push -1; push 0x965679 (7 bytes); sub esp, 0x204 (6); sub esp, 0x204 (6). Iguais no cm0102-1080.exe
# (inteiras).
# Comparadores: o construtor so a original contra o plano (como o da CONCACAF, cx16_1); a tabela de rodadas lado a lado
# (compare_phase_table_with_fields, cx16_2); as colocacoes a original e a copia de novo sobre o que ela deixou
# (concacaf_entrant_marks, cx16_1).
SAVE = False
LIGAS = 'ENG'
ATE = '2002-08-01'
ROTINAS = [('construct_wc_african_cup', 0x90f210, [0x6a, 0xff, 0x68, 0x79, 0x56, 0x96, 0x00]),
           ('wc_african_cup_create_round_table', 0x90f600, [0x81, 0xec, 0x04, 0x02, 0x00, 0x00]),
           ('wc_african_cup_rank_teams', 0x9111d0, [0x81, 0xec, 0x04, 0x02, 0x00, 0x00])]
DOCS = ('as eliminatórias africanas da Copa do Mundo: o construtor, as tabelas de rodadas e as colocações previstas',
        'the African World Cup qualifiers: the constructor, the round tables and the predicted positions',
        [('construct_wc_african_cup', 'WcAfricanCup::WcAfricanCup', '0x90f210', 'wc_african_cup.cpp', 'thiscall, ret 8',
          'regra: na criação do jogo, as eliminatórias africanas da campanha do ciclo de quatro anos (2000 num jogo de 2001): os inscritos, as fases pelo número de isentas, as estatísticas, as colocações previstas, a primeira rodada e a sua tabela; uma campanha começada antes do ano do jogo fica dormente',
          "rule: at game creation, the African qualifiers of the four-year cycle's campaign (2000 in a 2001 game): the entrants, the stages by the number of exempt nations, the statistics, the predicted positions, the first round and its table; a campaign begun before the game's year stays dormant"),
         ('wc_african_cup_create_round_table', 'WcAfricanCup::create_round_table', '0x90f600', 'wc_african_cup.cpp', 'thiscall, ret 0x10',
          'regra: a primeira rodada em ida e volta (sorteio em 18 de janeiro, jogos a partir de 8 de abril; 50, 48 ou 46 seleções pelas isentas), os grupos de dez rodadas (de junho ao julho seguinte), ou seis no formato 2, e nos formatos 2 e 3 uma repescagem final em ida e volta (sorteio em 29 de setembro, jogos em 20 de outubro do ano seguinte)',
          'rule: the two-legged first round (drawn 18 January, played from 8 April; 50, 48 or 46 teams by the exempt nations), the group stages of ten rounds (June to the next July), or six with format 2, and with formats 2 and 3 a final two-legged play-off (drawn 29 September, played 20 October of the next year)'),
         ('wc_african_cup_rank_teams', 'WcAfricanCup::rank_teams', '0x9111d0', 'wc_african_cup.cpp', 'thiscall, ret 0',
          'regra: as colocações previstas das seleções africanas pela reputação, em faixas que dependem das isentas (de cinco em cinco sem isentas; no formato 3 os dois melhores ficam com 3, um defeito do original), e cada seleção fica ocupada com estas eliminatórias',
          'rule: the predicted positions of the African national teams by reputation, in bands that depend on the exempt nations (five by five with none; with format 3 the best two get 3, a defect of the original), and every national team becomes busy with these qualifiers')])
CAMPOS = []
INTERESSA = []
