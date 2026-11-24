# A tabela de rodadas da Copa da Liga alema e o calendario da Copa da Holanda (ger_lge_cup.cpp, hol_cup.cpp; leitor
# sbF, rodada 5, 24/11/2026). Competicoes da Alemanha e da Holanda; ATE 2002-08-01 (a Copa da Liga alema e feita na
# criacao, pela GerLgeCup::set_up_rounds do cx02_8, e na virada de julho de 2002; a Copa da Holanda na criacao e no
# primeiro dia completo da temporada).
# - 0x5dc3a0 GerLgeCup::create_round_table thiscall(fase, &rodadas, &extra, &saida) ret 0x10 -> tabela: so a fase -1;
#   o molde eng_cc_cup_create_single_stage_round_table com as rodadas da paridade do ano (movsx do +0x40, resto com
#   sinal) e as vagas 0 e 1 das rodadas 0 e 1 (cup_set_round_match_day 0x521f10, cdecl). A caixa da linha 0x98.
#   Tirada por emulacao (unicorn) da original nas fases -128..127, ano impar e par: as saidas, o tamanho e cada byte.
# - 0x5f36e0 HolCup::create_schedule thiscall(fase, &rodadas, &extra, &e_liga) ret 0x10: o molde da ItaCup
#   (create_schedule) com o mata-mata na fase 19 (0x13), o rotulo 0x3e9, as caixas 0xa7 e 0xc6; conferido por emulacao
#   (as mesmas chamadas e os mesmos 16 campos por rodada).
# Prologos: mov al, [esp+4]; sub esp, 0x200 (10 bytes); iguais no cm0102-1080.exe.
# Comparadores: lado a lado (as saidas e os bytes escritos, com as vagas na alema).
LIGAS = 'ENG BEL HOL GER DEN'
ATE = '2002-08-01'
SAVE = False
ROTINAS = [('ger_lge_cup_create_round_table', 0x5dc3a0, [0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('hol_cup_create_schedule', 0x5f36e0, [0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('a tabela de rodadas da Copa da Liga alemã e o calendário da Copa da Holanda',
        'the German League Cup round table and the Dutch Cup schedule',
        [('ger_lge_cup_create_round_table', 'GerLgeCup::create_round_table', '0x5dc3a0', 'ger_lge_cup.cpp', 'thiscall, ret 0x10',
          'regra: a Copa da Liga alemã tem três rodadas de jogo único em julho: as quartas com quatro clubes (sorteio em 1º de julho, jogo em 9 de julho), as semifinais em que entram dois clubes (sorteio em 11, jogo em 12) e a final em campo neutro [inferência] (sorteio em 14, jogo em 16); em ano par (o verão da Copa do Mundo [inferência]) os jogos vão cerca de duas semanas depois (24/7, 29/7 e a final em 1º de agosto); categoria 6, sem prêmio por rodada; as quartas e as semifinais com o dia de jogo na vaga 0',
          'rule: the German League Cup has three single-match rounds in July: the quarter-finals with four clubs (drawn on 1 July, played on 9 July), the semi-finals where two clubs enter (drawn on 11, played on 12) and the final at a neutral ground [inference] (drawn on 14, played on 16); in an even year (the World Cup summer [inference]) the matches go about two weeks later (24/7, 29/7 and the final on 1 August); category 6, no prize per round; the quarter- and semi-finals with the match day in slot 0'),
         ('hol_cup_create_schedule', 'HolCup::create_schedule', '0x5f36e0', 'hol_cup.cpp', 'thiscall, ret 0x10',
          'regra: os grupos da Copa da Holanda (fases até 18) têm três rodadas em agosto (4, 11 e 18), cada grupo uma fase de liga (é_liga = 1, rótulo 0x3e9 + fase); o mata-mata (fase 19) tem seis rodadas de jogo único: 40 clubes na 1ª (25 de setembro), 20 na 2ª (31 de outubro), 16 na 3ª (29 de janeiro, seis novos), quartas em 12 de fevereiro, semifinais em 12 de abril e a final em 21 de maio em campo neutro [inferência]; categoria 4 até a 3ª rodada e 6 depois; sem prêmio por rodada',
          'rule: the Dutch Cup groups (stages up to 18) play three matchdays in August (4, 11 and 18), each group a league stage (is_league = 1, label 0x3e9 + stage); the knockout (stage 19) has six single-match rounds: 40 clubs in the 1st (25 September), 20 in the 2nd (31 October), 16 in the 3rd (29 January, six new), quarter-finals on 12 February, semi-finals on 12 April and the final on 21 May at a neutral ground [inference]; category 4 up to the 3rd round and 6 after; no prize per round')])
CAMPOS = []
INTERESSA = []
