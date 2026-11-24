# Os calendarios da Primeira Divisao turca e da segunda divisao turca categoria B (tur_first.cpp, tur_second_b.cpp;
# leitor cx14, rodada 5, 24/11/2026). Simulacao com a Turquia (as ligas e a data dos itens mwd): os calendarios na
# criacao do jogo e na virada (2 chamadas da Primeira, 17 da categoria B na cobertura do mundo).
# - 0x8fd220 TurFirst::create_schedule thiscall(fase, &rodadas, &extra, &liga) ret 0x10: so a fase -1, 34 rodadas em
#   linha reta (34 League::set_round_date 0x68a160 e 98 set_round_match_slot 0x68a1c0, cdecl); a tabela
#   TURKISH_FIRST_DIVISION_ROUNDS tirada dos pushes e conferida contra a execucao da rotina original no unicorn
#   (rascunho cx14/emula.py); o corpo e o fixed_league_schedule (caixa 0xa2 de tur_first.cpp).
# - 0x8ff7e0 TurSecondB::create_schedule (mesma convencao): os grupos da primeira fase (fase < 4) com as 18 datas de
#   domingo e 4 de quarta so quando o grupo tem 22 rodadas (fase 0), o numero de rodadas gravado no fim, e o laco das
#   faixas de TV ((fase + k + 1) % 5 == 0 por idiv; com 22 rodadas as rodadas 1, 5, 9 e 15 sem faixa: defeito do
#   original, as de quarta sao 3, 5, 11 e 15); o grupo de acesso (4), os de classificacao (5..9) e o mata-mata (10, 3
#   rodadas de copa: Cup::set_round_draw_date 0x521e60 / set_round_match_date 0x521eb0 e os campos da rodada,
#   conferidos campo a campo contra o eng_cc_cup_write_cup_round) em linha reta. As tabelas tiradas da execucao no
#   unicorn para cada fase (-1, 0..3, 4, 5..9, 10; os anos 2001 e 2002 dao as mesmas chamadas fora o ano). Caixas 0xb4,
#   0xfd, 0x134, 0x16c; fase > 10: 0 sem nada.
# Prologos: mov al, [esp+4]; sub esp, 0x200 / 0x208 (10 bytes). Iguais no cm0102-1080.exe (inteiras). Comparadores:
# lado a lado (as saidas e os bytes escritos; liga abaixo da fase 10, copa na 10).
SAVE = False
LIGAS = 'ENG POL TUR GRE HRV RUS'
ATE = '2002-08-01'
ROTINAS = [('tur_first_create_schedule', 0x8fd220, [0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('tur_second_b_create_schedule', 0x8ff7e0, [0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x08, 0x02, 0x00, 0x00])]
DOCS = ('os calendários da Primeira Divisão turca e da categoria B',
        'the schedules of the Turkish First Division and category B',
        [('tur_first_create_schedule', 'TurFirst::create_schedule', '0x8fd220', 'tur_first.cpp', 'thiscall, ret 0x10',
          'regra: o calendário da Primeira Divisão turca: 34 rodadas aos domingos de 13 de agosto a 20 de maio, com pausa de inverno, quase todas com faixas de TV',
          'rule: the Turkish First Division schedule: 34 rounds on Sundays from 13 August to 20 May, with a winter break, almost all with TV slots'),
         ('tur_second_b_create_schedule', 'TurSecondB::create_schedule', '0x8ff7e0', 'tur_second_b.cpp', 'thiscall, ret 0x10',
          'regra: o calendário da categoria B turca: os grupos da primeira fase com 18 domingos de 13 de agosto a 10 de dezembro (o grupo 2 com mais 4 quartas), a TV numa rodada a cada cinco; o grupo de acesso com 18 domingos de 14 de janeiro a 20 de maio, todos com TV; os grupos de classificação com 14 domingos de 11 de fevereiro a 20 de maio; o mata-mata em jogos únicos em 26, 28 e 30 de maio',
          'rule: the Turkish category B schedule: the first phase groups with 18 Sundays from 13 August to 10 December (group 2 with 4 more Wednesdays), TV on one round in five; the promotion group with 18 Sundays from 14 January to 20 May, all with TV; the classification groups with 14 Sundays from 11 February to 20 May; the play-off in single matches on 26, 28 and 30 May')])
CAMPOS = []
INTERESSA = []
