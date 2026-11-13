# The rules of the Polish First and Second Divisions and the Turkish second division category B (pol_first.cpp,
# pol_second.cpp, tur_second_b.cpp). Reader mwd (12/11/2026), round 2 (2 calls each: career creation and the turnover).
# - 0x7c9690 PolFirst::set_up_rules (no +0xc8; +0xd9 = 8 as a word), 0x7cbf40 PolSecond::set_up_rules (+0xc1 = 6 in
#   2001, 4 afterwards; +0xd9 = 3), 0x900460 TurSecondB::set_up_rules (no +0xab/+0xdb/+0xc8/+0x52; +0xd9 = 2): the field
#   lists read from the disassembly.
# Conventions: thiscall, ret 0; the virtual +0x3c (push 0; push &+0x3a; push &+0xa9; push -1 -> thiscall, ret 0x10).
# Comparer: the rehearsal of eng_prm_compare_league_rules. Equal in cm0102-1080.exe.
LIGAS = 'ENG POL TUR GRE HRV RUS'
ATE = '2002-08-01'
SAVE = False
ROTINAS = [('pol_first_set_up_rules', 0x7c9690, [0x56, 0x8b, 0xf1, 0xb9, 0x02, 0x00, 0x00, 0x00]),
           ('pol_second_set_up_rules', 0x7cbf40, [0x56, 0x8b, 0xf1, 0xb8, 0x02, 0x00, 0x00, 0x00]),
           ('tur_second_b_set_up_rules', 0x900460, [0x56, 0x8b, 0xf1, 0xb8, 0x02, 0x00, 0x00, 0x00])]
DOCS = ('as regras das divisões polonesas e da segunda divisão turca B', 'the rules of the Polish divisions and the Turkish second division B',
        [('pol_first_set_up_rules', 'PolFirst::set_up_rules', '0x7c9690', 'pol_first.cpp', 'thiscall, ret 0',
          'regra: a Primeira Divisão polonesa: 2 turnos, 3 pontos por vitória e 1 por empate; desempate por confronto direto, saldo e gols marcados; 4 na faixa de acesso, ninguém cai direto (o rebaixamento sai da fase final); abaixo a Segunda Divisão; 7 reservas e 3 substituições; bits das regras 8',
          'rule: the Polish First Division: 2 legs, 3 points a win and 1 a draw; tie-breaks head to head, goal difference and goals for; 4 in the promotion band, nobody goes down directly (relegation comes from the final stage); the Second Division below; 7 substitutes and 3 substitutions; rule bits 8'),
         ('pol_second_set_up_rules', 'PolSecond::set_up_rules', '0x7cbf40', 'pol_second.cpp', 'thiscall, ret 0',
          'regra: a Segunda Divisão polonesa: como a Primeira, com 2 subindo direto e 2 indo ao mata-mata de acesso, 6 caindo em 2001 (20 clubes) e 4 depois; acima a Primeira; bits das regras 3',
          'rule: the Polish Second Division: as the First Division, with 2 going up directly and 2 to the promotion play-off, 6 going down in 2001 (20 clubs) and 4 afterwards; the First Division above; rule bits 3'),
         ('tur_second_b_set_up_rules', 'TurSecondB::set_up_rules', '0x900460', 'tur_second_b.cpp', 'thiscall, ret 0',
          'regra: a segunda divisão turca categoria B: 3 pontos por vitória e 1 por empate; desempate por saldo e gols marcados; 2 ao mata-mata; acima a categoria A; 7 reservas e 3 substituições; bits das regras 2',
          'rule: the Turkish second division category B: 3 points a win and 1 a draw; tie-breaks goal difference and goals for; 2 to the play-offs; category A above; 7 substitutes and 3 substitutions; rule bits 2')])
CAMPOS = [('liga +0x3c, +0xc2..+0xc8, +0xab, +0xdb, +0x42, +0xbe..+0xc1, +0x1c, +0x20, +0xd9, +0x52, +0x49, +0x4a, +0xba', 'league +0x3c, +0xc2..+0xc8, +0xab, +0xdb, +0x42, +0xbe..+0xc1, +0x1c, +0x20, +0xd9, +0x52, +0x49, +0x4a, +0xba')]
