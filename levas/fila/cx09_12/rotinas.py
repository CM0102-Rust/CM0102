# A tabela, as regras e as posicoes previstas da Terceira Divisao belga (bel_third.cpp; leitor cx09, rodada 5,
# 24/11/2026). Simulacao com a Belgica ('ENG BEL HOL GER DEN', ate 2002-08-01): na cobertura do mundo cada uma roda 2
# vezes (a criacao do jogo e a virada de julho de 2002, pelo bel_third_start_new_season do mwc_19).
# - 0x4224e0 BelThird::fill_league_table thiscall() ret 0 -> 0/1: +0x3e = 16; com +0x40 = 2001, os clubes ([0xae23b4],
#   0x245 bytes, indice dword ate [0xae235c] - 2 x [0xae2350], relido a cada passo) da divisao [0x9cf5a4] ou [0x9cf5a8]
#   (A ou B) guardam a divisao em +0x60 e passam a +4 da liga; malloc(16 x 0x3b) em +0xb1 (caixa 0x22e e 0); os clubes
#   com +0x60 = A e +0x57 = [0x9cf7c4], na ordem, por League::init_table_row 0x687430 (linha, clube, 0, 0; o original
#   empilha a linha num dword com lixo em cima, so o byte e lido); o 17o: a caixa (0x23f) e +0x57 do clube = 0 (defeito:
#   o clube perde a divisao). Devolve linhas = +0x3e. Prologo sub esp, 0x200 (6 bytes).
# - 0x4226f0 BelThird::set_up_rules thiscall() ret 0 -> 1: +0x3c = 2, +0xc2 3, +0xc3 1, +0xc4 2, +0x42 1, +0xc5..+0xc8
#   3,1,2,0, +0xbe..+0xc1 1,0,1,2, +0xfe..+0x103 = 0, +0x1c = [0x9cf5a0], +0x4a 3, +0x20 -1, +0x49 4, +0xd9 0x43; +0xba =
#   virtual 0x3c (-1, &+0xa9, &+0x3a, 0). Prologo push ebx; push esi; mov esi, ecx; mov dl, 3 (6 bytes).
# - 0x424030 BelThird::set_predicted_positions thiscall() ret 0 (virtual 0x5c): sem +8 a caixa (0x6bf); malloc(+0x3e x
#   4) (caixa 0x6c7); grupo A (a liga) e B (fase 0): os clubes da tabela (contagem da liga), qsort 0x945333 (lista, n, 4,
#   0x4b07d0), Comp_Stats::set_club_final_position 0x4a2540 (clube, 3k + 1; o original empilha AX do imul com lixo em
#   cima, so o byte e lido); free. Prologo sub esp, 0x204 (6 bytes).
# Comparadores: a tabela so a original, conferida contra a escolha da copia (os clubes em ordem e o retorno); as regras
# a seco (compare_nation_league_rules, faixa ate +0x104 como o comentario dele preve); as posicoes so a original,
# conferidas contra a ordem da copia numa lista local.
# Iguais no cm0102-1080.exe. Depende do cx09_8 (BEL_THIRD_SOURCE).
SAVE = False
LIGAS = 'ENG BEL HOL GER DEN'
ATE = '2002-08-01'
ROTINAS = [('bel_third_fill_league_table', 0x4224e0, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('bel_third_set_up_rules', 0x4226f0, [0x53, 0x56, 0x8b, 0xf1, 0xb2, 0x03]),
           ('bel_third_set_predicted_positions', 0x424030, [0x81, 0xec, 0x04, 0x02, 0x00, 0x00])]
DOCS = ('a tabela, as regras e as posições previstas da Terceira Divisão belga', 'the Belgian Third Division table, rules and predicted positions',
        [('bel_third_fill_league_table', 'BelThird::fill_league_table', '0x4224e0', 'bel_third.cpp', 'thiscall, ret 0',
          'o grupo A da Terceira Divisão belga tem 16 clubes; em 2001 as Terceiras A e B viram uma divisão só e cada clube guarda o grupo de origem; a tabela do grupo A leva os clubes vindos da Terceira A, na ordem do banco',
          'Belgian Third Division group A has 16 clubs; in 2001 Third Divisions A and B become one division and each club keeps its original group; the group A table takes the clubs that came from Third Division A, in database order'),
         ('bel_third_set_up_rules', 'BelThird::set_up_rules', '0x4226f0', 'bel_third.cpp', 'thiscall, ret 0',
          'as regras da Terceira Divisão belga: ida e volta, 3 pontos por vitória e 1 por empate, desempate 3, 1, 2; um sobe, um vai à repescagem de rebaixamento e dois descem; acima a Segunda Divisão, abaixo nenhuma; os períodos zerados',
          'the Belgian Third Division rules: home and away, 3 points for a win and 1 for a draw, tie-breakers 3, 1, 2; one goes up, one to the relegation play-off and two go down; above the Second Division, nothing below; the periods cleared'),
         ('bel_third_set_predicted_positions', 'BelThird::set_predicted_positions', '0x424030', 'bel_third.cpp', 'thiscall, ret 0',
          'as posições previstas: em cada grupo os clubes ordenados pela força recebem 1, 4, 7... (3k + 1)',
          'the predicted positions: in each group the clubs ordered by strength get 1, 4, 7... (3k + 1)')])
CAMPOS = []
INTERESSA = []
