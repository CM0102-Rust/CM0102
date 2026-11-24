# A Copa dos Campeoes do Brasil: os grupos, as semifinais e a virada da temporada (bra_champ_cup.cpp; leitor cx01,
# rodada 5, 24/11/2026). Usa BRA_CHAMP_CUP_SOURCE do cx01_10 (integrar depois dele), create_knockout_stage (ita_cup.rs)
# e compare_new_season (bra_reg_rio.rs). Carreira de 8 ligas: os grupos em 2001-183, as semifinais em 2001-200, a
# virada 1 vez ate 2002-06-01.
# - 0x426470 BraChampCup::create_group_stages thiscall() ret 0 -> 0/1 (SEH so para os dois new): o slot 0x3c (push 0;
#   push &rotulo; push &rodadas; push grupo -> thiscall, ret 0x10); operator new 0xee; League_Stage::League_Stage
#   0x68b100 (22 push; mov ecx, novo -> thiscall, ret 0x58; os lugares [2, 0, 0, 0] sao copiados para +0xbe); free;
#   League::send_group_draw_news 0x684230 (mov ecx, etapa -> thiscall, ret 0); as caixas 0x252 e 0x268.
# - 0x426720 BraChampCup::create_semi_final_stage thiscall() ret 0 (SEH so para o new): malloc 0x10; o slot 0x3c (push 0;
#   push &extra; push &rodadas; push 2); operator new 0xb2 e Cup_Stage::Cup_Stage 0x522e00 (15 push -> thiscall, ret
#   0x3c); free da lista e da tabela; Cup::draw_round 0x51c410 (push 0; mov ecx, etapa -> thiscall, ret 4); as caixas
#   0x286 e 0x29d. O mesmo molde do create_knockout_stage (ita_cup.rs), conferido instrucao por instrucao.
# - 0x4262c0 BraChampCup::start_new_season thiscall() ret 0 (slot 8): League::free_table 0x687970 (push 0; mov ecx ->
#   thiscall, ret 4); free; Comp_Stats::archive_season_and_reset 0x4a1c50 (push 1; mov ecx, [+8] -> thiscall, ret 4); o
#   slot 0 das etapas (push 1); o slot 0x8c; a caixa 0x1fc.
# Iguais no cm0102-1080.exe (as tres inteiras).
# Comparadores: so a original (alocam, mandam noticia e sorteiam), com o plano: os clubes de cada grupo e os quatro das
# semifinais (como conjuntos) contra as etapas construidas; a virada pelo compare_new_season (ano +1, 3 fases, fase -1).
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
SAVE = False
ROTINAS = [('bra_champ_cup_create_group_stages', 0x426470, [0x6a, 0xff, 0x68, 0x8c, 0x43, 0x95, 0x00]),
           ('bra_champ_cup_create_semi_final_stage', 0x426720, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('bra_champ_cup_start_new_season', 0x4262c0, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('a Copa dos Campeões do Brasil: os grupos, as semifinais e a virada da temporada',
        'the Brazilian Champions Cup: the groups, the semi-finals and the season turnover',
        [('bra_champ_cup_create_group_stages', 'BraChampCup::create_group_stages', '0x426470', 'bra_champ_cup.cpp', 'thiscall, ret 0',
          'monta os dois grupos de 4: o vencedor da preliminar com os inscritos 3, 4 e 5, o segundo da preliminar com os inscritos 6, 7 e 8; passam os 2 primeiros de cada; cada grupo manda a notícia do sorteio',
          "builds the two groups of 4: the preliminary's winner with entrants 3, 4 and 5, its runner-up with entrants 6, 7 and 8; the top 2 of each go through; each group sends its draw news"),
         ('bra_champ_cup_create_semi_final_stage', 'BraChampCup::create_semi_final_stage', '0x426720', 'bra_champ_cup.cpp', 'thiscall, ret 0',
          'monta o mata-mata de 4 com os dois primeiros de cada grupo, cruzados (1º do A com o 2º do B, 2º do A com o 1º do B), e faz o primeiro sorteio',
          "builds the 4-club knockout with the top two of each group, crossed (A's winner with B's runner-up, A's runner-up with B's winner), and makes the first draw"),
         ('bra_champ_cup_start_new_season', 'BraChampCup::start_new_season', '0x4262c0', 'bra_champ_cup.cpp', 'thiscall, ret 0',
          'vira a temporada: libera a tabela, as datas e as fases, arquiva as estatísticas, o ano sobe um e as regras são montadas de novo (sem tabela, sem clubes, inativa)',
          'turns the season over: frees the table, the dates and the stages, archives the statistics, the year goes up by one and the rules are set up again (no table, no clubs, inactive)')])
CAMPOS = []
INTERESSA = []
