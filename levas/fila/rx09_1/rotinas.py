# A Segunda Divisao B espanhola na criacao do jogo, as regras e as tabelas de rodadas (spa_second_b.cpp; leitor rx09,
# rodada 6, 25/11/2026). So existe com a base NORMAL. Simulacao com a Espanha e a Suecia ate 2002-09-01 (a virada de
# 2002 e a primeira rodada da temporada seguinte).
# - 0x855f40 SpaSecondB::SpaSecondB thiscall(ano, registro) ret 8 (SEH so do operator new das estatisticas): League::League
#   0x682200, registro +4, vtable 0x96fc84, ano (palavra), disciplina 0x1a; League::load_from_temp_file(1) 0x687b10 encerra.
#   Senao +0x44 = 0xff, +0x30 = -1, +0x2c = 9, malloc 0x24 em +0xc (sem conferir); regras 0x856ef0 (caixa 0x48), tabela
#   0x856cd0 (0x4f), sort_table(0) 0x6827d0 (0x56), Comp_Stats (new 0x5ce, 0x49ee70; 0x5e), create_fixtures 0x6835c0
#   (0x67); create_group_stage(k) 0x857210 para k = 0..2 (o dword empurrado tem restos da pilha acima do byte) e
#   set_predicted_positions 0x8582f0 (chamada direta).
# - 0x856ef0 SpaSecondB::set_up_rules thiscall() ret 0 -> 1: as escritas conferidas uma a uma e a virtual +0x3c(-1,
#   &+0xa9, &+0x3a, 0) em +0xba.
# - 0x856260 SpaSecondB::create_schedule thiscall(fase, &rodadas, &extra, &liga) ret 0x10: fase (byte com sinal) < 3:
#   38 rodadas, extra fase + 0x3e9, malloc 0x9a6 (caixa 0xb0); 3..6: 6 rodadas, extra fase + 0x44a, malloc 0x186 (0x137);
#   7: copa de uma rodada, liga 0, extra 0x118, malloc 0x68 (0x15d); 8: a final, extra 0x122 (0x185); depois 0 sem
#   escrever. As datas tiradas pela emulacao (unicorn, cx14\emula.py) e conferidas na desmontagem; as rodadas de copa
#   pelas chamadas 0x521e60/0x521eb0 e as escritas em +7..+0x64.
# Prologos: push -1; push imm32 (7 bytes); push ebx; push esi; mov esi, ecx; mov dl, 3 (6 bytes); mov al, [esp+4];
# sub esp, 0x200 (10 bytes). Iguais no cm0102-1080.exe.
# Comparadores: o construtor so a original (compare_league_with_stages_ctor); as regras pelo ensaio do
# eng_prm_compare_league_rules; as tabelas de rodadas lado a lado (compare_league_or_cup_schedule).
# Depende dos itens cx14 (o molde dos grupos) so pelo rx09_3 em diante; este usa o que ja esta no motor.
SAVE = False
LIGAS = 'ENG ESP SWE'
ATE = '2002-09-01'
BASE = 'NORMAL'
ROTINAS = [('construct_spa_second_b', 0x855f40, [0x6a, 0xff, 0x68, 0xa9, 0x27, 0x96, 0x00]),
           ('spa_second_b_set_up_rules', 0x856ef0, [0x53, 0x56, 0x8b, 0xf1, 0xb2, 0x03]),
           ('spa_second_b_create_schedule', 0x856260, [0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('a Segunda Divisão B espanhola na criação do jogo, as regras e as tabelas de rodadas',
        'the Spanish Second Division B at game creation, its rules and its round tables',
        [('construct_spa_second_b', 'SpaSecondB::SpaSecondB', '0x855f40', 'spa_second_b.cpp', 'thiscall, ret 8',
          'regra: a Segunda B é criada com nove fases além do grupo 1 (os grupos 2 a 4, os quatro grupos do acesso, a repescagem do rebaixamento e a final dela); sem estado salvo, as regras, a tabela do grupo 1, a ordem, as estatísticas, os jogos, os grupos 2 a 4 e as posições previstas',
          'rule: the Second Division B is built with nine stages besides group 1 (groups 2 to 4, the four promotion play-off groups, the relegation play-off and its final); with no saved state, the rules, the group 1 table, the order, the statistics, the fixtures, groups 2 to 4 and the predicted positions'),
         ('spa_second_b_set_up_rules', 'SpaSecondB::set_up_rules', '0x856ef0', 'spa_second_b.cpp', 'thiscall, ret 0',
          'regra: dois turnos, 3 pontos por vitória e 1 por empate; desempate pelo confronto direto, saldo e gols marcados; ninguém sobe direto, 4 de cada grupo vão ao acesso, o 16º à repescagem do rebaixamento e os 4 últimos caem; a Segunda acima; 3 substituições e 5 reservas',
          'rule: two legs, 3 points a win and 1 a draw; tie-breaks head to head, goal difference, goals for; nobody goes up directly, 4 of each group go to the promotion play-off, the sixteenth to the relegation play-off and the last four down; the Second Division above; 3 substitutions and 5 substitutes'),
         ('spa_second_b_create_schedule', 'SpaSecondB::create_schedule', '0x856260', 'spa_second_b.cpp', 'thiscall, ret 0x10',
          'regra: os grupos jogam 38 rodadas aos domingos de 22 de agosto a 14 de maio, com a pausa de 19 de dezembro a 9 de janeiro; os grupos do acesso, 6 rodadas de 21 de maio a 11 de junho; a repescagem do rebaixamento, ida e volta a partir de 21 de maio (sorteio em 15 de maio); a final dela, ida e volta a partir de 4 de junho (sorteio em 29 de maio)',
          'rule: the groups play 38 rounds on Sundays from 22 August to 14 May, with the break from 19 December to 9 January; the promotion play-off groups, 6 rounds from 21 May to 11 June; the relegation play-off, two legs from 21 May (drawn on 15 May); its final, two legs from 4 June (drawn on 29 May)')])
CAMPOS = []
INTERESSA = []
