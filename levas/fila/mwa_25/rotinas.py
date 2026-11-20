# A Copa da Liga da Irlanda (ire_lge_cup.cpp, em cups\): o mata-mata, os inscritos e o calendário. Fecha o arquivo com o
# mwa_16, mwa_18 e mwa_23 (ficam fora só o build_fate_news_text, o load_extra_data e o set_club_progress, que não
# estão na lista). Depende do mwa_23 só para a ordem (nenhum código dele). Leitor r4_mwab (20/11/2026).
# - 0x637c00 (IreLgeCup::create_knockout_stage): os 8 vencedores (a 1a linha da tabela da copa e a de cada fase, por
#   League::get_table_row 0x6826d0, push 0; mov ecx -> thiscall, ret 4) e create_knockout_stage (cqr_9, o quadro do
#   ItaCup: a fase 7, Cup_Stage 0x522e00, o primeiro sorteio 0x51c410).
# - 0x637dd0 (IreLgeCup::collect_entrants): calloc (0x945501, cdecl) de 26; as faixas pelos grupos A a E da divisão de
#   reservas; a lista local (até 100) ordenada e embaralhada por 0x4afbd0 (cdecl, 3 argumentos); as faixas pela liga
#   regional; 0x4afb40 nas três primeiras faixas de 6; o preenchimento das vagas que sobram.
# - 0x637120 (IreLgeCup::create_schedule): create_schedule (cqo_10) com a fase 7, rótulo 0x3e9, 3 rodadas de grupo e 3
#   de mata-mata (same.py contra 0x63cf90 e rounds.py: as tabelas conferem).
# Convenções conferidas: o mata-mata e os inscritos thiscall, ret 0; o calendário thiscall, ret 0x10.
# Comparadores: o mata-mata só a original (a fase na vaga 7); os inscritos lado a lado com o gerador restaurado
# (compare_select_entrants; 26 entradas); o calendário lado a lado (ita_cup_compare_schedule).
LIGAS = 'ENG IRL NIR'
ATE = '2002-08-01'
SAVE = False
ROTINAS = [('ire_lge_cup_create_knockout_stage', 0x637c00, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('ire_lge_cup_collect_entrants', 0x637dd0, [0x81, 0xec, 0xa0, 0x03, 0x00, 0x00]),
           ('ire_lge_cup_create_schedule', 0x637120, [0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('a Copa da Liga da Irlanda: o mata-mata, os inscritos e o calendário',
        "the Irish League Cup: the knockout, the entrants and the schedule",
        [('ire_lge_cup_create_knockout_stage', 'IreLgeCup::create_knockout_stage', '0x637c00', 'ire_lge_cup.cpp', 'thiscall, ret 0',
          'regra: o mata-mata (fase 7) tem os 8 vencedores de grupo: o 1o da tabela da própria copa e o 1o de cada uma das 7 fases de grupo, nessa ordem; depois o primeiro sorteio',
          "rule: the knockout (stage 7) takes the 8 group winners: the top club of the cup's own table and of each of the 7 group stages, in that order; then the first draw"),
         ('ire_lge_cup_collect_entrants', 'IreLgeCup::collect_entrants', '0x637dd0', 'ire_lge_cup.cpp', 'thiscall, ret 0',
          'regra: 26 clubes da Irlanda: os que o banco de dados põe nos grupos A a E da copa (pela divisão de reservas: 6 em A, B e C, 4 em D e E) e, nas vagas livres, os outros clubes irlandeses pela reputação (com sorteio entre os 20 melhores), cada um na faixa da sua liga regional (Leinster com A, Connacht com C, Munster com D, Ulster com E, os outros com B); as três primeiras faixas embaralhadas; vagas que sobrem, os primeiros irlandeses da tabela que ainda não entraram',
          "rule: 26 clubs of Ireland: those the database puts in the cup's groups A to E (through the reserve division: 6 in A, B and C, 4 in D and E) and, in the free places, the other Irish clubs by reputation (with a draw among the best 20), each in the band of its regional league (Leinster with A, Connacht with C, Munster with D, Ulster with E, the rest with B); the first three bands shuffled; places still free, the first Irish clubs of the table not yet in"),
         ('ire_lge_cup_create_schedule', 'IreLgeCup::create_schedule', '0x637120', 'ire_lge_cup.cpp', 'thiscall, ret 0x10',
          'regra: os grupos (fases até 6) jogam 3 rodadas no fim de setembro (20, 24 e 27), cada grupo uma fase de liga (rótulo 0x3e9 + fase); o mata-mata (fase 7) de 8 clubes: quartas em 8 de novembro e semifinais em 29 de novembro, jogo único (desempate 3), e a final em ida e volta com 21 dias, a partir de 24 de janeiro do ano seguinte; categoria 6; o prêmio do vencedor é uma parte do fundo',
          "rule: the groups (stages up to 6) play 3 matchdays in late September (20, 24 and 27), each group a league stage (label 0x3e9 + stage); the 8-club knockout (stage 7): quarter-finals on 8 November and semi-finals on 29 November, single matches (tie rule 3), and a two-legged final 21 days apart from 24 January of the next year; category 6; the tie winner's prize is a share of the fund")])
CAMPOS = []
INTERESSA = []
