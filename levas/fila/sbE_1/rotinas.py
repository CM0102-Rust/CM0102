# Os passos da virada finlandesa: os clubes da Primeira Divisao (Ykkonen) refeitos, os seus dois grupos e a cadeia de
# acesso e rebaixamento (fin_prm.cpp; leitor sbE, rodada 5, 24/11/2026). Simulacao com a Finlandia (as ligas e a data
# dos itens mwe): os tres rodam na virada da Veikkausliiga (FinPrm::start_new_season 0x595d40, mwe_14), no fim de 2001.
# - 0x595f00 FinPrm::refresh_first_division_clubs thiscall() ret 0 (chamada direta, ecx = a Veikkausliiga): os times da
#   copa da repescagem da Ykkonen (fase 2, +0xa7 de 6 bytes, saida +5 = 1) que nao estao na Ykkonen vao para ela
#   (League::move_club_to_division 0x6830b0, thiscall(clube, registro, 1), ret 0xc); as linhas de status 3 da tabela da
#   Ykkonen e da fase 0 caem para a Kakkonen Norte (League::relegate_club_to_division 0x6831a0, ret 0xc); os quatro
#   clubes de indice em +0xee da Ykkonen (gravados pelo FinFirst::create_play_off_stages 0x594080) vao para a Ykkonen.
#   Prologo push ecx; mov eax, [0xadadfc] (6 bytes).
# - 0x596050 FinPrm::assign_first_division_groups thiscall() ret 0 (ecx nao lido): ate 20 clubes da Ykkonen pela ordem
#   do banco, qsort 0x945333 (cdecl, 4 pushes) com 0x53fd40, os dez primeiros com a divisao reserva (+0x60) no grupo
#   Sul e os outros no Norte. Prologo mov eax, [0x9cf7ec] (5 bytes).
# - 0x596110 FinPrm::process_promotion_relegation_chain thiscall(a) ret 4 (virtual): as posicoes finais (virtual +0xa4)
#   da Veikkausliiga, da Ykkonen e da fase 0 dela; League::promote_and_relegate 0x689c80 (6 pushes, ret 0x18)
#   Veikkausliiga/Ykkonen e Veikkausliiga/fase 0; o eax da ultima. Prologo mov eax, [0xadadfc] (5 bytes).
# Iguais no cm0102-1080.exe (inteiras). Comparadores: o primeiro e a cadeia so a original (movem clubes); os grupos lado
# a lado na divisao reserva dos clubes da Ykkonen.
SAVE = False
LIGAS = 'ENG SWE FIN NOR POR'
ATE = '2002-08-01'
ROTINAS = [('fin_prm_refresh_first_division_clubs', 0x595f00, [0x51, 0xa1, 0xfc, 0xad, 0xad, 0x00]),
           ('fin_prm_assign_first_division_groups', 0x596050, [0xa1, 0xec, 0xf7, 0x9c, 0x00]),
           ('fin_prm_process_promotion_relegation_chain', 0x596110, [0xa1, 0xfc, 0xad, 0xad, 0x00])]
DOCS = ('os passos da virada finlandesa: os clubes da Ykkönen, os seus grupos e a cadeia',
        'the Finnish turnover steps: the Ykkönen clubs, its groups and the chain',
        [('fin_prm_refresh_first_division_clubs', 'FinPrm::refresh_first_division_clubs', '0x595f00', 'fin_prm.cpp', 'thiscall, ret 0',
          'na virada, os vencedores da repescagem e os quatro clubes guardados pela Ykkönen entram na Ykkönen, e os rebaixados da Ykkönen (os dois grupos) caem para a Kakkonen Norte',
          'at the turnover, the play-off winners and the four clubs kept by the Ykkönen go into the Ykkönen, and the Ykkönen\'s relegated clubs (both groups) go down to the Kakkonen North'),
         ('fin_prm_assign_first_division_groups', 'FinPrm::assign_first_division_groups', '0x596050', 'fin_prm.cpp', 'thiscall, ret 0',
          'os clubes da Ykkönen (até 20) ordenados do sul para o norte: os dez primeiros no grupo Sul, os outros no grupo Norte',
          'the Ykkönen clubs (up to 20) sorted from south to north: the first ten in the South group, the others in the North group'),
         ('fin_prm_process_promotion_relegation_chain', 'FinPrm::process_promotion_relegation_chain', '0x596110', 'fin_prm.cpp', 'thiscall, ret 4',
          'grava as posições finais da Veikkausliiga e dos dois grupos da Ykkönen e troca os clubes marcados entre a Veikkausliiga e cada grupo',
          'records the final positions of the Veikkausliiga and of both Ykkönen groups and swaps the marked clubs between the Veikkausliiga and each group')])
CAMPOS = []
INTERESSA = []
