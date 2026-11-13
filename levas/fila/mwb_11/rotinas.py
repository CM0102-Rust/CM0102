# A Liga de Gales (wel_first.cpp): o construtor, as regras e a virada. Depende do mwb_5 (construct_british_division) e do
# mwb_6 (british_division_new_season): integrar junto ou depois deles. Leitor mwb (12/11/2026), lista do mundo (Escócia e
# País de Gales).
# - 0x924e30 (WelFirst::WelFirst): o molde das divisões (disciplina 0x28; a tabela 0x925b30 fica na original: não entrou
#   na lista da cobertura).
# - 0x925c50 (WelFirst::set_up_rules): 2 turnos, ninguém sobe nem desce.
# - 0x925cf0 (WelFirst::start_new_season): a virada (primeiro as posições finais, virtual +0xa4); roda uma vez, no meio
#   de 2002.
# Convenções conferidas: como as do mwb_5 e do mwb_6 (a virtual +0xa4 thiscall, ret 0).
# Comparadores: o construtor só a original, com o plano; as regras lado a lado nos bytes; a virada só a original (o ano
# subiu um).
LIGAS = 'ENG SCO WAL'
ATE = '2002-08-01'
SAVE = False
ROTINAS = [('construct_wel_first', 0x924e30, [0x6a, 0xff, 0x68, 0xe9, 0x5c, 0x96, 0x00]),
           ('wel_first_set_up_rules', 0x925c50, [0x56, 0x8b, 0xf1, 0x33, 0xc0]),
           ('wel_first_start_new_season', 0x925cf0, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('a Liga de Gales: o construtor, as regras e a virada',
        'the League of Wales: the constructor, the rules and the turnover',
        [('construct_wel_first', 'WelFirst::WelFirst', '0x924e30', 'wel_first.cpp', 'thiscall, ret 8',
          'regra: a Liga de Gales nasce com a disciplina 0x28, sem fase além da liga, e monta regras, tabela (18 clubes), jogos, ordem, estatísticas e posições previstas',
          'rule: the League of Wales is created with discipline rule 0x28, with no stage beyond the league, and builds the rules, the table (18 clubs), the fixtures, the order, the statistics and the predicted positions'),
         ('wel_first_set_up_rules', 'WelFirst::set_up_rules', '0x925c50', 'wel_first.cpp', 'thiscall, ret 0',
          'regra: 2 turnos, 3 pontos por vitória e 1 por empate; ninguém sobe nem desce (não há divisão acima nem abaixo no jogo); desempate saldo, gols pró e posição; 5 reservas e 3 substituições',
          'rule: 2 rounds, 3 points for a win and 1 for a draw; nobody goes up or down (there is no division above or below in the game); tie-breaks goal difference, goals for and position; 5 substitutes and 3 substitutions'),
         ('wel_first_start_new_season', 'WelFirst::start_new_season', '0x925cf0', 'wel_first.cpp', 'thiscall, ret 0',
          'regra: na virada a Liga de Gales grava as posições finais, arquiva a temporada, libera tabela e rodadas, passa ao ano seguinte e refaz regras, tabela, jogos e ordem; depois as posições previstas, as expectativas da diretoria e a renovação dos árbitros',
          'rule: at the turnover the League of Wales records the final positions, archives the season, frees the table and the rounds, moves to the next year and rebuilds the rules, the table, the fixtures and the order; then the predicted positions, the board expectations and the renewal of the referees')])
