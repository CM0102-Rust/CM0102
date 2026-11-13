# A virada espanhola, o acesso e o rebaixamento (spa_first.cpp). Leitor mwf (12/11/2026), rodada 2 (o mundo até
# 2002-08-01; docs/cobertura_mundo.txt: cada uma 1 vez, na virada espanhola de meados de 2002; a troca sem a Segunda B
# rodou, então a carreira do mundo não tem a Segunda B carregada).
# - 0x850700 (SpaFirst::process_promotion_relegation_chain, a virtual +0xb0): as posições finais da Primeira e da
#   Segunda e a troca entre elas; com a Segunda B, as dela e dos grupos 0..2 (o original só percorre três das quatro
#   fases) e as trocas da Segunda com a Segunda B e com cada grupo. Sem a Segunda B devolve o endereço da tabela de
#   competições (o eax que o original deixa).
# - 0x8500c0 (SpaFirst::exchange_second_division_clubs_unplayed): sem a Segunda B, os rebaixados da Segunda descem
#   para o grupo B1 (e o time B cujo clube principal está agora na Segunda é rebaixado à força, com a notícia), e sobem
#   tantos quantos desceram, da tabela da divisão inferior e depois dos grupos B1..B4, nunca um time B para a divisão
#   do seu clube principal.
# - 0x84feb0 (SpaFirst::start_new_season): a cadeia e as trocas, a Primeira refeita (sem mexer nas fases), a virada
#   da Segunda e a da Segunda B (ou, sem ela, a da divisão inferior), previstas, expectativas, árbitros.
# Convenções: virtuais (mov ecx, esi; call [eax+n]; a +0xb0 com push 1, ret 4) -> thiscall; dentro: 0x540a50 (3
# pushes; add esp, 0xc -> cdecl), 0x6826d0 (push i -> thiscall, ret 4), 0x6830b0 e 0x6831a0 (3 pushes -> thiscall,
# ret 0xc), 0x48ece0 (4 pushes -> thiscall, ret 0x10), 0x689c80 (6 pushes -> thiscall, ret 0x18); 0x850300 e 0x850530
# chamadas com ecx = this e sem argumentos.
# Comparadores: a cadeia e a troca só a original (movem clubes e mandam notícias; a troca confere depois que os
# rebaixados da Segunda estão no B1); a virada só a original, com o ano +1.
LIGAS = 'ENG KOR ARG ITA FRA ESP AUS'
ATE = '2002-08-01'
SAVE = False
ROTINAS = [('spa_first_process_promotion_relegation_chain', 0x850700, [0xa1, 0xfc, 0xad, 0xad, 0x00]),
           ('spa_first_exchange_second_division_clubs_unplayed', 0x8500c0, [0x83, 0xec, 0x0c, 0xa1, 0xfc, 0xad, 0xad, 0x00]),
           ('spa_first_start_new_season', 0x84feb0, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('a virada espanhola, o acesso e o rebaixamento',
        'the Spanish season turnover, promotion and relegation',
        [('spa_first_process_promotion_relegation_chain', 'SpaFirst::process_promotion_relegation_chain', '0x850700', 'spa_first.cpp', 'thiscall, ret 4',
          'regra: o acesso e o rebaixamento na Espanha: grava as posições finais da Primeira e da Segunda e troca os clubes marcados entre elas; com a Segunda B, grava as dela e dos três primeiros grupos e troca a Segunda com a Segunda B e com cada um desses grupos',
          'rule: promotion and relegation in Spain: records the final positions of the First and Second Divisions and swaps the marked clubs between them; with Segunda B, records its own and its first three groups\' positions and swaps the Second Division with Segunda B and with each of those groups'),
         ('spa_first_exchange_second_division_clubs_unplayed', 'SpaFirst::exchange_second_division_clubs_unplayed', '0x8500c0', 'spa_first.cpp', 'thiscall, ret 0',
          'regra: sem a Segunda B, os rebaixados da Segunda descem para o grupo B1 e sobem tantos quantos desceram, primeiro da tabela da divisão inferior e depois dos clubes dos grupos B1 a B4; um time B nunca sobe para a divisão do seu clube principal, e o time B cujo clube principal está agora na Segunda é rebaixado à força, com notícia',
          'rule: without Segunda B, the Second Division\'s relegated clubs go down to group B1 and as many come up, first from the lower division table and then from the clubs of groups B1 to B4; a B team never goes up into its senior club\'s division, and a B team whose senior club is now in the Second Division is forced down, with news'),
         ('spa_first_start_new_season', 'SpaFirst::start_new_season', '0x84feb0', 'spa_first.cpp', 'thiscall, ret 0',
          'regra: a virada espanhola, puxada pela Primeira: os conflitos dos times B (com a Segunda B), o acesso e o rebaixamento e a troca com a Segunda B (ou, sem ela, com a divisão inferior); a Primeira refeita (ano seguinte, regras, 20 clubes, jogos, ordem) e as viradas da Segunda e da Segunda B (ou da divisão inferior); depois as posições previstas, as expectativas da diretoria e a renovação dos árbitros',
          'rule: the Spanish turnover, led by the First Division: the B team clashes (with Segunda B), promotion and relegation and the exchange with Segunda B (or, without it, with the lower division); the First Division rebuilt (next year, rules, 20 clubs, fixtures, order) and the turnovers of the Second Division and Segunda B (or the lower division); then the predicted positions, the board expectations and the referee renewal')])
CAMPOS = [('linha da tabela +0x37 (status 3 = rebaixado; a troca marca o time B rebaixado à força); clube +0x57 e +0x5b (movidos); valor; grava',
           'table row +0x37 (status 3 = relegated; the exchange marks the forced-down B team); club +0x57 and +0x5b (moved); value; writes')]
