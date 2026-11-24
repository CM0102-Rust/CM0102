# As posicoes previstas, a ordem da tabela da fase final e a cadeia de acesso e rebaixamento da Primeira Divisao
# polonesa (pol_first.cpp; leitor cx13, rodada 5, 24/11/2026). Simulacao com a Polonia (as ligas e a data dos itens
# mwd): as posicoes previstas na criacao do jogo (e na virada), a ordem na fase final (de novembro de 2001 a maio de
# 2002), a cadeia no fim da temporada.
# - 0x7ca1e0 PolFirst::set_predicted_positions thiscall() ret 0: sem estatisticas, a caixa da linha 0x346; senao qsort
#   0x945333 (cdecl, 4 pushes) da lista dos 16 clubes (+0xee) com 0x4b07d0 e Comp_Stats::set_club_final_position
#   0x4a2540 (thiscall(clube, k), ret 8) de cada um. Prologo sub esp, 0x200 (6 bytes).
# - 0x7ca450 pol_first_compare_championship_group_first cdecl(a, b) -> i32: a ordem da tabela da fase final (posta em
#   +0xa3 da fase pelo create_final_stage 0x7c9b60): o clube cuja divisao reserva (+0x60) tem o id da chave 0x9cf814
#   (chamada KEY_CLUB_COMP_IRISH_GROUP_A; o create_final_stage marca o grupo do titulo com ela e o de rebaixamento com a
#   0x9cf818) vem antes. Prologo mov eax, [esp+4]; push esi (5 bytes).
# - 0x7ca490 PolFirst::process_promotion_relegation_chain thiscall(a) ret 4: as posicoes finais (virtual +0xa4) da fase
#   1 e da Segunda Divisao, depois League::promote_and_relegate 0x689c80 (this, fase 1, segunda, 1, 1, -1, -1; ret
#   0x18), cujo resultado devolve. Prologo push esi; mov esi, ecx; mov eax, [esi+0xc] (6 bytes).
# Iguais no cm0102-1080.exe (inteiras). Comparadores: as posicoes lado a lado (a lista e o byte de cada entrada); a
# ordem pelo compare_order!; a cadeia so a original (move clubes).
SAVE = False
LIGAS = 'ENG POL TUR GRE HRV RUS'
ATE = '2002-08-01'
ROTINAS = [('pol_first_set_predicted_positions', 0x7ca1e0, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('pol_first_compare_championship_group_first', 0x7ca450, [0x8b, 0x44, 0x24, 0x04, 0x56]),
           ('pol_first_process_promotion_relegation_chain', 0x7ca490, [0x56, 0x8b, 0xf1, 0x8b, 0x46, 0x0c])]
DOCS = ('as posições previstas, a ordem da fase final e a cadeia de acesso e rebaixamento da Primeira Divisão polonesa',
        'the Polish First Division\'s predicted positions, final stage order and promotion and relegation chain',
        [('pol_first_set_predicted_positions', 'PolFirst::set_predicted_positions', '0x7ca1e0', 'pol_first.cpp', 'thiscall, ret 0',
          'os 16 clubes ordenados pela força; o lugar de cada um (0 a 15) é a posição prevista nas estatísticas',
          'the 16 clubs sorted by strength; each one\'s place (0 to 15) is its predicted position in the statistics'),
         ('pol_first_compare_championship_group_first', 'pol_first_compare_championship_group_first', '0x7ca450', 'pol_first.cpp', 'cdecl',
          'na tabela da fase final, os clubes do grupo do título vêm antes dos do grupo de rebaixamento (o grupo marcado na divisão reserva do clube)',
          'in the final stage table, the clubs of the championship group come before those of the relegation group (the group marked in the club\'s reserve division)'),
         ('pol_first_process_promotion_relegation_chain', 'PolFirst::process_promotion_relegation_chain', '0x7ca490', 'pol_first.cpp', 'thiscall, ret 4',
          'grava as posições finais da fase final e da Segunda Divisão e troca os clubes marcados entre as duas',
          'records the final positions of the final stage and the Second Division and swaps the marked clubs between them')])
CAMPOS = []
INTERESSA = []
