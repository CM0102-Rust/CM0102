# The rules of the Greek First and Second Divisions and the Russian First Division (gre_prm.cpp, gre_second.cpp,
# rus_first.cpp). Reader mwd (12/11/2026), round 2 (2 calls each: career creation and the turnover; the Russian season
# runs inside the calendar year).
# - 0x5eae90 GrePrm::set_up_rules, 0x5ebde0 GreSecond::set_up_rules (with the rule bit 0x40: or byte [+0xd9], 0x40),
#   0x7ea810 RusFirst::set_up_rules (no rule bits): the field lists read from the disassembly.
# Conventions: thiscall, ret 0; the virtual +0x3c (push 0; push &+0x3a; push &+0xa9; push -1 -> thiscall, ret 0x10).
# Comparer: the rehearsal of eng_prm_compare_league_rules. Equal in cm0102-1080.exe.
# Depends on: apply_league_rules_with_flags (mwd_2), eng_prm_apply_league_rules / eng_prm_compare_league_rules.
LIGAS = 'ENG POL TUR GRE HRV RUS'
ATE = '2002-08-01'
SAVE = False
ROTINAS = [('gre_prm_set_up_rules', 0x5eae90, [0x55, 0x56, 0x8b, 0xf1, 0xb2, 0x03]),
           ('gre_second_set_up_rules', 0x5ebde0, [0x56, 0x8b, 0xf1, 0xb8, 0x02, 0x00, 0x00, 0x00]),
           ('rus_first_set_up_rules', 0x7ea810, [0x56, 0x8b, 0xf1, 0xb8, 0x02, 0x00, 0x00, 0x00])]
DOCS = ('as regras das divisões gregas e da Primeira Divisão russa', 'the rules of the Greek divisions and the Russian First Division',
        [('gre_prm_set_up_rules', 'GrePrm::set_up_rules', '0x5eae90', 'gre_prm.cpp', 'thiscall, ret 0',
          'regra: a Primeira Divisão grega: 2 turnos, 3 pontos por vitória e 1 por empate; desempate por saldo, gols marcados e vitórias; ninguém sobe, 2 caem para a Segunda; 7 reservas e 3 substituições; bit 0x40 das regras',
          'rule: the Greek First Division: 2 legs, 3 points a win and 1 a draw; tie-breaks goal difference, goals for, wins; nobody goes up, 2 go down to the Second Division; 7 substitutes and 3 substitutions; rule bit 0x40'),
         ('gre_second_set_up_rules', 'GreSecond::set_up_rules', '0x5ebde0', 'gre_second.cpp', 'thiscall, ret 0',
          'regra: a Segunda Divisão grega: como a Primeira, 2 sobem e 2 caem; acima a Primeira, abaixo nada',
          'rule: the Greek Second Division: as the First Division, 2 go up and 2 go down; above the First Division, nothing below'),
         ('rus_first_set_up_rules', 'RusFirst::set_up_rules', '0x7ea810', 'rus_first.cpp', 'thiscall, ret 0',
          'regra: a Primeira Divisão russa: 2 turnos, 3 pontos por vitória e 1 por empate; desempate por vitórias, confronto direto, saldo e gols marcados; 2 sobem para a liga principal e 3 caem; 7 reservas e 3 substituições',
          'rule: the Russian First Division: 2 legs, 3 points a win and 1 a draw; tie-breaks wins, head to head, goal difference and goals for; 2 go up to the Premier League and 3 go down; 7 substitutes and 3 substitutions')])
CAMPOS = [('liga +0x3c, +0xc2..+0xc8, +0xab, +0xdb, +0x42, +0xbe..+0xc1, +0x1c, +0x20, +0xd9, +0x52, +0x49, +0x4a, +0xa7, +0xba', 'league +0x3c, +0xc2..+0xc8, +0xab, +0xdb, +0x42, +0xbe..+0xc1, +0x1c, +0x20, +0xd9, +0x52, +0x49, +0x4a, +0xa7, +0xba')]
