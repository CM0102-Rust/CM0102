# A copa pelo desfecho e no fim da temporada (Cup.cpp, o que restava fora da fila): Cup::update_staff_records_for_status
# (0x519890, a virtual +0x44 das copas inglesas e da base, vtables 0x969b28/0x969934/0x9693c8, e chamada pela
# 0x902b90; 780 chamadas na carreira até 2002-06-01: o desfecho de cada time em cada jogo de copa), Cup::record_season_honours
# (0x519a90, a virtual +0x30; 11) e Cup::register_semi_final_losers_for_uefa (0x519fc0, só pela 0x519a90; 2). Sem
# SEH, sem sorteio; a 0x905790 só tem caixa com a semeadura cheia. Iguais no cm0102-1080.exe (corpo inteiro). O item
# antigo cpb_1 (26/10, nunca integrado, com a escuta das chamadas) tinha as três: este as refaz sem a escuta, no
# critério de 31/10.
# Convenções conferidas nos pontos de chamada da original (lição da 258ª):
# - 0x86a800 / 0x86b0b0 / 0x86b710 (registros de campeão, vice e terceiro): push final; push jogo; push clube; ecx =
#   [0xdd7f00]; sem add esp -> thiscall(registros, clube, jogo, final), ret 0xc.
# - 0x86bdd0 (classificação): push peso; push rodada; push fase; push comp; push clube; ecx = [0xdd7f00]; sem add esp
#   -> thiscall, ret 0x14 (fase e rodada: palavras sobre os 16 bits altos dos registradores, reproduzidos).
# - 0x86c000 (eliminação): 5 pushes na mesma ordem; ecx = [0xdd7f00] -> thiscall, ret 0x14.
# - 0x5c6710 (competição amistosa): push comp; ecx = [0xae2c74] -> thiscall, ret 4.
# - 0x669470 (índice da nação da competição): push comp; add esp, 4 -> cdecl de 1.
# - 0x4a4850 (registro no Comp_Stats): push clube; ecx = as estatísticas -> thiscall, ret 4.
# - virtual +0x88 do objeto da competição (0x5dce00 nas copas inglesas: devolve 0): ecx = o objeto; sem push ->
#   thiscall, ret 0.
# - 0x5fa730 (HostCountry::find_hosts): push 0; push &b; push &a; push ano; push id; ecx = [0xb5e134]; sem add esp ->
#   thiscall, ret 0x14.
# - 0x53b3d0 (seleção principal): push nação; add esp, 4 -> cdecl de 1.
# - 0x4afce0 (linha do histórico): 5 pushes; add esp, 0x14 -> cdecl(copa, campeão, vice, terceiro, anfitriã).
# - 0x519fc0: ecx = a copa; sem push -> thiscall, ret 0.
# - 0x905790 (UefaSeeding::record_cup_semi_final_losers): push b; push a; push id; ecx = [0xde1f58]; sem add esp ->
#   thiscall, ret 0xc.
# Comparadores: a 0x519890 só a original (os registros mexem nas reputações e no moral), com o plano do estado do
# clube (a Copa da UEFA para o campeão com a vaga); a 0x519a90 só a original, com o plano da linha livre do histórico
# (competição, ano e os quatro clubes); a 0x519fc0 lado a lado na tabela da semeadura (50 vagas de 12 bytes em +0x440).
SAVE = False
ROTINAS = [('copa_registros_pelo_desfecho', 0x519890, [0x53, 0x56, 0x8b, 0xf1, 0x57]),
           ('copa_honras_da_temporada', 0x519a90, [0x83, 0xec, 0x14, 0x53, 0x55]),
           ('copa_semifinalistas_para_a_uefa', 0x519fc0, [0x83, 0xec, 0x10, 0x53, 0x55])]
DOCS = ('a copa pelo desfecho e no fim da temporada: os registros pelo desfecho, as honras da temporada e os semifinalistas para a UEFA',
        'the cup by outcome and at season end: the records by outcome, the season honours and the semi-final losers for UEFA',
        [('copa_registros_pelo_desfecho', 'Cup::update_staff_records_for_status', '0x519890', 'Cup.cpp', 'thiscall, ret 0x18',
          'pelo desfecho do clube: o registro de campeão (e a vaga na Copa da UEFA pela Copa da Liga inglesa ou pela copa principal de uma nação europeia), de vice, de terceiro, a classificação para a rodada seguinte ou a eliminação, com o peso pela rodada; devolve 0',
          'by the club outcome: the winners record (and the UEFA Cup place through the English League Cup or a European nation main cup), runners-up, third place, qualified for the next round or knocked out, weighted by round; returns 0'),
         ('copa_honras_da_temporada', 'Cup::record_season_honours', '0x519a90', 'Cup.cpp', 'thiscall, ret 0',
          'acha o campeão, o vice, o terceiro (na copa galesa, pela estatística) e a seleção anfitriã, grava a linha do histórico e, na copa principal da nação, passa os semifinalistas à UEFA',
          'finds the winner, runner-up, third (in the Welsh cup, through the statistics) and the host national team, writes the history row and, in the nation main cup, passes the semi-final losers to UEFA'),
         ('copa_semifinalistas_para_a_uefa', 'Cup::register_semi_final_losers_for_uefa', '0x519fc0', 'Cup.cpp', 'thiscall, ret 0',
          'acha as semifinais (0x82) e a final (0x96) e manda à semeadura da UEFA os dois semifinalistas que não estão na final',
          'finds the semi-finals (0x82) and the final (0x96) and sends the two semi-finalists not in the final to the UEFA seeding')])
CAMPOS = [('Cup +0x2c / +0x44 / +0x3c / +0x3e / +0x40: as fases, a competição, as rodadas, os times e o ano; valor; lê (as três)',
           'Cup +0x2c / +0x44 / +0x3c / +0x3e / +0x40: stages, competition, rounds, teams and year; value; reads (all three)'),
          ('Cup +0xa3 / +0xa7 / +0xad: as rodadas (+7 tipo, +0x1a confrontos), os times (+5 desfecho) e os confrontos; ponteiro; lê (as três)',
           'Cup +0xa3 / +0xa7 / +0xad: rounds (+7 type, +0x1a ties), teams (+5 outcome) and ties; pointer; reads (all three)'),
          ('estado do clube [0xae2d14] + id × 6: a competição europeia da temporada; id; lê e grava a chave da Copa da UEFA (copa_registros_pelo_desfecho)',
           'club state [0xae2d14] + id × 6: the season European competition; id; reads and writes the UEFA Cup key (copa_registros_pelo_desfecho)'),
          ('jogo +0x30 / +0x32 / +0x34 / +0x3a: as palavras da rodada e a marca; valor; lê (copa_registros_pelo_desfecho)',
           'fixture +0x30 / +0x32 / +0x34 / +0x3a: the round words and the flag; value; reads (copa_registros_pelo_desfecho)'),
          ('histórico de competições [0xae23d4] (26 bytes por linha): +4 competição, +8 ano, +0xa..+0x16 os quatro clubes; grava pela 0x4afce0 (copa_honras_da_temporada)',
           'competition history [0xae23d4] (26 bytes per row): +4 competition, +8 year, +0xa..+0x16 the four clubs; written through 0x4afce0 (copa_honras_da_temporada)'),
          ('semeadura da UEFA [0xde1f58] +0x440: 50 vagas de 12 bytes (competição, os dois clubes); grava pela 0x905790 (copa_semifinalistas_para_a_uefa)',
           'UEFA seeding [0xde1f58] +0x440: 50 slots of 12 bytes (competition, the two clubs); written through 0x905790 (copa_semifinalistas_para_a_uefa)')]
INTERESSA = ['SGE', 'CMRTE']
