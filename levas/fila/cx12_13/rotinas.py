# Os calendarios da Premier Division e da Primeira Divisao norueguesas (nor_prm.cpp, nor_first.cpp; leitor cx12, rodada 5,
# 24/11/2026). Simulacao com a Noruega (LIGAS): a cobertura do mundo (docs/cobertura_mundo.txt, ate 2002-08-01) da 2
# chamadas na Premier (criacao do jogo e a virada no fim de 2001, temporada 2002 de abril a outubro) e 3 na Primeira
# (as mesmas e a fase 0, a repescagem de outubro de 2001); as rodadas de 2002 de abril e maio caem antes de 2002-06-01.
# - 0x7917c0 NorPrm::create_schedule thiscall(fase, &rodadas, &extra, &e_liga) ret 0x10 (virtual 0x3c): so a fase -1;
#   26 rodadas (malloc 0x69a; sem memoria a caixa da linha 0xa0 e 0); um conjunto de datas para os anos pares e outro
#   para os impares (o resto com sinal do ano por 2: and 0x80000001; jns...); duas escolhas pelo calendario: Date::Date
#   0x549f70 (thiscall, ret 0x10) de 30/4 (dia da semana -1) e 3/5 (6) e Date::days_since 0x54cdb0 (thiscall, a data
#   por valor, ret 8): menos de 2 dias, 26/4 e 30/4; senao 30/4 e 3/5; depois 6/5 e 10/5; de novo 13/5 (2) e 16/5 (-1):
#   16/5 e 20/5 ou 13/5 e 16/5. league_round_set_date 0x68a160 e league_round_set_match_slot 0x68a1c0 (cdecl).
#   DEFEITO: nos anos impares a vaga 0 da rodada 0 e escrita duas vezes (o jogo fixo 5, 1, 1 e depois o terminador).
#   Prologo mov eax, fs:[0] (6 bytes).
# - 0x790040 NorFirst::create_schedule thiscall(fase, ...) ret 0x10: fase -1, 30 rodadas quando 16 clubes do banco
#   (G_CLUB_COUNT, [0xae235c]) tem a Primeira Divisao, senao 26 (malloc n x 0x41; sem memoria a caixa da linha 0xc1 e
#   0), as mesmas duas escolhas, de 19/4 a 4/10, e a caixa da linha 0x211 se o numero de rodadas escritas nao bater
#   (nunca com estas tabelas); fase 0, a repescagem (*e_liga = 0 quando dado, 1 rodada, extra 0, malloc 0x68; sem
#   memoria a caixa da linha 0x224): sorteio 19/10 (0x521e60), jogos 25/10 (0x521eb0) e os campos da rodada de copa,
#   o molde eng_cc_cup_write_cup_round com NORWEGIAN_PLAY_OFF. DEFEITO: com 16 clubes, 17/6 em duas rodadas seguidas.
#   Prologo mov eax, fs:[0] (6 bytes).
# As tabelas foram extraidas por um emulador de caminhos sobre a desmontagem (cada combinacao das escolhas e da paridade
# ou do numero de clubes) e conferidas de volta: as escolhas e o comeco de maio sao os mesmos nas duas ligas.
# Iguais no cm0102-1080.exe (inteiras). Comparadores: lado a lado sobre as saidas e a tabela (compare_c1_schedule).
SAVE = False
LIGAS = 'ENG SWE FIN NOR POR'
ATE = '2002-08-01'
ROTINAS = [('nor_prm_create_schedule', 0x7917c0, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('nor_first_create_schedule', 0x790040, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00])]
DOCS = ('os calendários da Premier Division e da Primeira Divisão norueguesas', 'the Norwegian Premier and First Division schedules',
        [('nor_prm_create_schedule', 'NorPrm::create_schedule', '0x7917c0', 'nor_prm.cpp', 'thiscall, ret 0x10',
          'o calendário da Premier norueguesa: 26 rodadas de abril a outubro, com datas diferentes nos anos pares e ímpares; as rodadas do fim de abril e de meados de maio mudam conforme o dia da semana: se 3 de maio (no dia 6) cai menos de 2 dias depois de 30 de abril, e se 16 de maio cai menos de 2 dias depois de 13 de maio (no dia 2)',
          'the Norwegian Premier schedule: 26 rounds from April to October, with different dates in even and odd years; the late April and mid-May rounds move with the weekday: whether 3 May (moved to weekday 6) falls less than 2 days after 30 April, and whether 16 May falls less than 2 days after 13 May (moved to weekday 2)'),
         ('nor_first_create_schedule', 'NorFirst::create_schedule', '0x790040', 'nor_first.cpp', 'thiscall, ret 0x10',
          'o calendário da Primeira Divisão norueguesa: 30 rodadas com 16 clubes, senão 26, de 19 de abril a 4 de outubro, com as mesmas escolhas da Premier; na fase 0, a repescagem contra um clube da Premier: sorteio em 19 de outubro e ida e volta a partir de 25 de outubro, 7 dias entre os jogos',
          'the Norwegian First Division schedule: 30 rounds with 16 clubs, otherwise 26, from 19 April to 4 October, with the same choices as the Premier; at stage 0, the play-off against a Premier club: drawn on 19 October, two legs from 25 October, 7 days apart')])
CAMPOS = []
INTERESSA = []
