# The Croatian leagues: the rules (cro_a1.cpp, cro_a2a.cpp, cro_a2b.cpp). Reader mwd (12/11/2026), round 2 (2 calls
# each: career creation and the turnover).
# - 0x5148e0 CroA1::set_up_rules, 0x515de0 CroA2a::set_up_rules, 0x516f00 CroA2b::set_up_rules: the same template; only
#   the league above (+0x1c) and below (+0x20) change: A1 -1 / A2a; A2a A1 / A2b; A2b A2a / -1 (the south group's
#   upper league is the north group: the promotion chain runs A1 -> A2a -> A2b).
# Conventions: thiscall, ret 0; the virtual +0x3c (push 0; push &+0x3a; push &+0xa9; push -1 -> thiscall, ret 0x10).
# Comparer: the rehearsal of eng_prm_compare_league_rules (the original runs, the band restored, the copy writes without
# the virtual; the listed fields compared, +0xd9 included). Equal in cm0102-1080.exe.
# Depends on: eng_prm_apply_league_rules / eng_prm_compare_league_rules (eng_prm.rs).
LIGAS = 'ENG POL TUR GRE HRV RUS'
ATE = '2002-08-01'
SAVE = False
ROTINAS = [('cro_a1_set_up_rules', 0x5148e0, [0x53, 0x56, 0x8b, 0xf1, 0xb2, 0x03]),
           ('cro_a2a_set_up_rules', 0x515de0, [0x53, 0x56, 0x8b, 0xf1, 0xb2, 0x03]),
           ('cro_a2b_set_up_rules', 0x516f00, [0x53, 0x56, 0x8b, 0xf1, 0xb2, 0x03])]
_R = '16 clubes, 2 turnos, 3 pontos por vitória e 1 por empate; desempate por confronto direto, saldo e gols marcados; empate em pontos numa faixa decidido em jogo extra (+0xe6 = 1); ninguém sobe direto, 2 caem; 7 reservas e 3 substituições; bit 0x40 das regras'
_E = '16 clubs, 2 legs, 3 points a win and 1 a draw; tie-breaks head to head, goal difference and goals for; a points tie across a band settled by a tie-break match (+0xe6 = 1); nobody goes up directly, 2 go down; 7 substitutes and 3 substitutions; rule bit 0x40'
DOCS = ('as ligas croatas: as regras', 'the Croatian leagues: the rules',
        [('cro_a1_set_up_rules', 'CroA1::set_up_rules', '0x5148e0', 'cro_a1.cpp', 'thiscall, ret 0', 'regra: a Primeira Divisão croata: ' + _R + '; abaixo, a Segunda Divisão (grupo norte)', 'rule: the Croatian First Division: ' + _E + '; below, the Second Division (north group)'),
         ('cro_a2a_set_up_rules', 'CroA2a::set_up_rules', '0x515de0', 'cro_a2a.cpp', 'thiscall, ret 0', 'regra: a Segunda Divisão croata, grupo norte: ' + _R + '; acima a Primeira Divisão, abaixo o grupo sul', 'rule: the Croatian Second Division north group: ' + _E + '; above the First Division, below the south group'),
         ('cro_a2b_set_up_rules', 'CroA2b::set_up_rules', '0x516f00', 'cro_a2b.cpp', 'thiscall, ret 0', 'regra: a Segunda Divisão croata, grupo sul: ' + _R + '; acima o grupo norte (não a Primeira), abaixo nada', 'rule: the Croatian Second Division south group: ' + _E + '; above the north group (not the First Division), nothing below')])
CAMPOS = [('liga +0x3c, +0xc2..+0xc8, +0xab, +0xdb, +0x42, +0xbe..+0xc1, +0x1c, +0x20, +0xd9 (bit 0x40), +0x52, +0x49, +0x4a, +0xe6, +0xba', 'league +0x3c, +0xc2..+0xc8, +0xab, +0xdb, +0x42, +0xbe..+0xc1, +0x1c, +0x20, +0xd9 (bit 0x40), +0x52, +0x49, +0x4a, +0xe6, +0xba')]
