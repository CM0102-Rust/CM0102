# A Copa da Liga da Irlanda do Norte (nir_lge_cup.cpp, em cups\): o mata-mata, os inscritos e o calendário. Fecha o
# arquivo com o mwa_17, mwa_19 e mwa_24 (ficam fora só o build_fate_news_text, o load_extra_data e o
# set_club_progress, que não estão na lista). Depende do mwa_25 (first_free_entrant_slot). Leitor r4_mwab (20/11/2026).
# - 0x78d600 (NirLgeCup::create_knockout_stage): os 2 primeiros de cada um dos 4 grupos (League::get_table_row
#   0x6826d0 com 0 e 1), vencedor e vice lado a lado, e create_knockout_stage (fase 3).
# - 0x78d800 (NirLgeCup::collect_entrants): calloc de 20; as faixas de 5 pelos grupos A a D da divisão de reservas; os
#   clubes norte-irlandeses sem divisão de reservas nas vagas livres.
# - 0x78cad0 (NirLgeCup::create_schedule): create_schedule com a fase 3, rótulo 0x3e9, 5 rodadas de grupo e 3 de
#   mata-mata (rounds.py: as tabelas conferem).
# Convenções conferidas: como as do mwa_25.
# Comparadores: como os do mwa_25 (os inscritos: 20 entradas).
LIGAS = 'ENG IRL NIR'
ATE = '2002-08-01'
SAVE = False
ROTINAS = [('nir_lge_cup_create_knockout_stage', 0x78d600, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('nir_lge_cup_collect_entrants', 0x78d800, [0x81, 0xec, 0x04, 0x02, 0x00, 0x00]),
           ('nir_lge_cup_create_schedule', 0x78cad0, [0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('a Copa da Liga da Irlanda do Norte: o mata-mata, os inscritos e o calendário',
        "the Northern Irish League Cup: the knockout, the entrants and the schedule",
        [('nir_lge_cup_create_knockout_stage', 'NirLgeCup::create_knockout_stage', '0x78d600', 'nir_lge_cup.cpp', 'thiscall, ret 0',
          'regra: o mata-mata (fase 3) tem os 2 primeiros de cada um dos 4 grupos (a tabela da própria copa e as 3 fases de grupo), vencedor e vice em seguida; depois o primeiro sorteio',
          "rule: the knockout (stage 3) takes the top two of each of the 4 groups (the cup's own table and the 3 group stages), winner and runner-up in turn; then the first draw"),
         ('nir_lge_cup_collect_entrants', 'NirLgeCup::collect_entrants', '0x78d800', 'nir_lge_cup.cpp', 'thiscall, ret 0',
          'regra: 20 clubes da Irlanda do Norte: os que o banco de dados põe nos grupos A a D da copa (pela divisão de reservas), 5 em cada, e nas vagas livres os outros norte-irlandeses sem grupo, na ordem da tabela',
          "rule: 20 clubs of Northern Ireland: those the database puts in the cup's groups A to D (through the reserve division), 5 in each, and in the free places the other Northern Irish clubs with no group, in table order"),
         ('nir_lge_cup_create_schedule', 'NirLgeCup::create_schedule', '0x78cad0', 'nir_lge_cup.cpp', 'thiscall, ret 0x10',
          'regra: os grupos (fases até 2) jogam 5 rodadas, quinzenais de 29 de agosto a 24 de outubro, cada grupo uma fase de liga (rótulo 0x3e9 + fase); o mata-mata (fase 3) de 8 clubes: quartas em 31 de outubro (jogo único), semifinais em ida e volta a partir de 14 de novembro e a final em 28 de novembro; categoria 6',
          "rule: the groups (stages up to 2) play 5 matchdays, every other week from 29 August to 24 October, each group a league stage (label 0x3e9 + stage); the 8-club knockout (stage 3): quarter-finals on 31 October (single matches), two-legged semi-finals from 14 November and the final on 28 November; category 6")])
CAMPOS = []
INTERESSA = []
