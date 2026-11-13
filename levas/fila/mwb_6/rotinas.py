# A First Division escocesa (sco_first.cpp): o calendário, a virada e o progresso dos clubes; os moldes dos calendários
# escoceses (ScottishSchedule) e da virada das divisões (british_division_new_season), usados pelos itens mwb_9 a
# mwb_11. Depende do mwb_5 (BritishDivisionSpec, SCOTTISH_FIRST_DIVISION): integrar junto ou depois dele. Leitor mwb
# (12/11/2026), lista do mundo (Escócia e País de Gales).
# - 0x7ee2e0 (ScoFirst::create_schedule): 36 rodadas de 7/8 a 6/5; a do fim do ano pelo dia da semana (8 casos e o
#   padrão, com a caixa da linha 0x118); em 1999 o mata-mata do acesso (fase 0, 3 rodadas).
# - 0x7eef00 (ScoFirst::start_new_season): a virada; roda uma vez, no meio de 2002.
# - 0x7ef100 (ScoFirst::set_club_progress): fora de 1999 repassa à comum (0x48e380); roda 1865 vezes.
# Convenções conferidas: o calendário thiscall, ret 0x10 (0x68a160/0x68a1c0 cdecl; 0x549f70 push -1; push ano; push
# 0xb; push 1; lea ecx -> thiscall, ret 0x10); a virada thiscall, ret 0 (0x687970 push 0 -> thiscall, ret 4; 0x9452ca
# cdecl; 0x4a1c50 push 1 -> ret 4; as virtuais +0x8c e +0x5c thiscall, ret 0; 0x68aa80 thiscall, ret 0; 0x79cee0
# thiscall([0xb74340], +4), ret 4); o progresso thiscall, ret 0x14 (repassa os 5 argumentos com o mesmo ecx; 0x4a4850
# push clube; mov ecx -> thiscall, ret 4).
# Comparadores: o calendário lado a lado (compare_league_round_table); a virada só a original (o ano subiu um); o
# progresso lado a lado nos 3 códigos da entrada do clube (só a original no mata-mata de 1999).
# O caminho de 1999 (a fase 0 do calendário e do progresso) não roda numa carreira de 2001.
LIGAS = 'ENG SCO WAL'
ATE = '2002-08-01'
SAVE = False
ROTINAS = [('sco_first_create_schedule', 0x7ee2e0, [0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x08, 0x02, 0x00, 0x00]),
           ('sco_first_start_new_season', 0x7eef00, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('sco_first_set_club_progress', 0x7ef100, [0x66, 0x81, 0x79, 0x40, 0xcf, 0x07])]
DOCS = ('a First Division escocesa: o calendário, a virada e o progresso dos clubes',
        'the Scottish First Division: the schedule, the turnover and the club progress',
        [('sco_first_create_schedule', 'ScoFirst::create_schedule', '0x7ee2e0', 'sco_first.cpp', 'thiscall, ret 0x10',
          'regra: 36 rodadas (10 clubes, 4 turnos), aos sábados de 7/8 a 6/5 do ano seguinte: 19 antes do fim do ano (nenhuma em 9/10), a do fim do ano em 26, 27 ou 28/12 conforme o dia da semana e 16 depois (3/1 no próprio dia); em 1999 o mata-mata do acesso: 27/4, 31/5 e 4/6',
          'rule: 36 rounds (10 clubs, 4 rounds), Saturdays from 7/8 to 6/5 of the next year: 19 before the year end (none on 9/10), the year-end round on 26, 27 or 28/12 by the weekday and 16 after (3/1 on its own day); in 1999 the promotion play-off: 27/4, 31/5 and 4/6'),
         ('sco_first_start_new_season', 'ScoFirst::start_new_season', '0x7eef00', 'sco_first.cpp', 'thiscall, ret 0',
          'regra: na virada a First é refeita: arquiva a temporada, libera tabela, rodadas e fases, passa ao ano seguinte (a fase do mata-mata só em 1999), refaz regras, tabela, jogos e ordem; depois as posições previstas, as expectativas da diretoria e a renovação dos árbitros',
          'rule: at the turnover the First Division is rebuilt: the season is archived, the table, rounds and stages freed, the year moves on (the play-off stage only in 1999), the rules, the table, the fixtures and the order are rebuilt; then the predicted positions, the board expectations and the renewal of the referees'),
         ('sco_first_set_club_progress', 'ScoFirst::set_club_progress', '0x7ef100', 'sco_first.cpp', 'thiscall, ret 0x14',
          'regra: fora de 1999 os códigos de progresso do clube vão à rotina comum; em 1999 vão à entrada do clube nas estatísticas (no mata-mata, o clube da Premier na entrada da Premier, com 1 -> 8 e 2, 3 -> 10; o da First com 1 -> 1 e 2, 3 -> 3)',
          "rule: outside 1999 the club's progress codes go to the common routine; in 1999 they go into the club's statistics entry (in the play-off the Premier club into the Premier's entry, with 1 -> 8 and 2, 3 -> 10; the First Division club with 1 -> 1 and 2, 3 -> 3)")])
