# The rules of the Russian Premier League and the Turkish First and Second Divisions (rus_prm.cpp, tur_first.cpp,
# tur_second.cpp). Reader mwd (12/11/2026), round 2 (2 calls each: career creation and the turnover).
# - 0x7eb740 RusPrm::set_up_rules: the Croatian template with the Russian tie-breaks (3, 4, 1, 2) and the First Division
#   below; tie-break matches (+0xe6 = 1) and the rule bit 0x40.
# - 0x8fdf90 TurFirst::set_up_rules, 0x8ff280 TurSecond::set_up_rules: the same writes; only +0xbe (0 / 3) and the
#   leagues above and below change. TurFirst's lower league is the id global 0x9cf8d0 (unnamed in the community map).
# Conventions: thiscall, ret 0; the virtual +0x3c (push 0; push &+0x3a; push &+0xa9; push -1 -> thiscall, ret 0x10).
# Comparer: the rehearsal of eng_prm_compare_league_rules. Equal in cm0102-1080.exe.
# Depends on: apply_league_rules_with_flags (mwd_2).
LIGAS = 'ENG POL TUR GRE HRV RUS'
ATE = '2002-08-01'
SAVE = False
ROTINAS = [('rus_prm_set_up_rules', 0x7eb740, [0x53, 0x56, 0x8b, 0xf1, 0xb2, 0x03]),
           ('tur_first_set_up_rules', 0x8fdf90, [0x56, 0x8b, 0xf1, 0xba, 0x02, 0x00, 0x00, 0x00]),
           ('tur_second_set_up_rules', 0x8ff280, [0x56, 0x8b, 0xf1, 0xba, 0x02, 0x00, 0x00, 0x00])]
DOCS = ('as regras da liga principal russa e das divisões turcas', 'the rules of the Russian Premier League and the Turkish divisions',
        [('rus_prm_set_up_rules', 'RusPrm::set_up_rules', '0x7eb740', 'rus_prm.cpp', 'thiscall, ret 0',
          'regra: a liga principal russa: 2 turnos, 3 pontos por vitória e 1 por empate; desempate por vitórias, confronto direto, saldo e gols marcados; empate em pontos numa faixa decidido em jogo extra; ninguém sobe, 2 caem para a Primeira Divisão; 7 reservas e 3 substituições; bit 0x40 das regras',
          'rule: the Russian Premier League: 2 legs, 3 points a win and 1 a draw; tie-breaks wins, head to head, goal difference and goals for; a points tie across a band settled by a tie-break match; nobody goes up, 2 go down to the First Division; 7 substitutes and 3 substitutions; rule bit 0x40'),
         ('tur_first_set_up_rules', 'TurFirst::set_up_rules', '0x8fdf90', 'tur_first.cpp', 'thiscall, ret 0',
          'regra: a Primeira Divisão turca: 2 turnos, 3 pontos por vitória e 1 por empate; desempate por confronto direto, saldo, gols marcados e vitórias; ninguém sobe, 3 caem; 7 reservas e 3 substituições',
          'rule: the Turkish First Division: 2 legs, 3 points a win and 1 a draw; tie-breaks head to head, goal difference, goals for and wins; nobody goes up, 3 go down; 7 substitutes and 3 substitutions'),
         ('tur_second_set_up_rules', 'TurSecond::set_up_rules', '0x8ff280', 'tur_second.cpp', 'thiscall, ret 0',
          'regra: a Segunda Divisão turca: como a Primeira, com 3 subindo para a elite e 3 caindo para a segunda divisão categoria B',
          'rule: the Turkish Second Division: as the First Division, with 3 going up to the top flight and 3 down to the second division category B')])
CAMPOS = [('liga +0x3c, +0xc2..+0xc8, +0xab, +0xdb, +0x42, +0xbe..+0xc1, +0x1c, +0x20, +0xd9, +0x52, +0x49, +0x4a, +0xe6, +0xba', 'league +0x3c, +0xc2..+0xc8, +0xab, +0xdb, +0x42, +0xbe..+0xc1, +0x1c, +0x20, +0xd9, +0x52, +0x49, +0x4a, +0xe6, +0xba')]
