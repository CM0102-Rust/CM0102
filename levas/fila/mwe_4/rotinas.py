# The league rules of the Swedish First Division and of the Portuguese Premier and Second Divisions (reader mwe,
# whole-world round, 12/11/2026). Each runs when its league is created and at each season turnover (virtual +0x8c;
# the Swedish season turns over at the end of 2001, the Portuguese in the summer of 2002):
# - 0x88f090 SweFirst::set_up_rules (192 bytes);
# - 0x7cf930 PorPrm::set_up_rules (176 bytes);
# - 0x7d16a0 PorSecond::set_up_rules (192 bytes).
# The body of eng_prm_apply_league_rules (cqi_4); SweFirst and PorSecond pick the division below by bit 4 of the
# nation's league_selected (as EngThird). PorSecond writes 0x22 into +0xa9 before the virtual (which writes the round
# count there again). Equal in cm0102-1080.exe (whole).
# Conventions checked: called from the constructors (mov ecx, esi; call) and through vtable +0x8c -> thiscall, ret 0;
# the virtual +0x3c: push 0; push &+0x3a; push &+0xa9; push -1; mov ecx -> thiscall, ret 0x10.
# Comparer: eng_prm_compare_league_rules (dry run).
# Determinism: only constants, the key globals and the nation byte.
LIGAS = 'ENG SWE FIN NOR POR'
ATE = '2002-08-01'
SAVE = False
ROTINAS = [('swe_first_set_up_rules', 0x88f090, [0x53, 0x56, 0x8b, 0xf1, 0xb8, 0x02, 0x00, 0x00, 0x00]),
           ('por_prm_set_up_rules', 0x7cf930, [0x56, 0x8b, 0xf1, 0xba, 0x02, 0x00, 0x00, 0x00]),
           ('por_second_set_up_rules', 0x7d16a0, [0x56, 0x8b, 0xf1, 0xba, 0x02, 0x00, 0x00, 0x00])]
DOCS = ('as regras da Superettan sueca e das duas divisões portuguesas',
        'the league rules of the Swedish Superettan and of the Portuguese Premier and Second Divisions',
        [('swe_first_set_up_rules', 'SweFirst::set_up_rules', '0x88f090', 'swe_first.cpp', 'thiscall, ret 0',
          'a Superettan (16 clubes): turno e returno, 3 pontos por vitória; desempate pelo saldo e pelos gols marcados; os 2 primeiros sobem, o 3º joga o play-off de acesso contra o 12º da Allsvenskan e os 3 últimos caem (para a segunda divisão, quando ela é jogável); 5 reservas e 3 substituições',
          'the Superettan (16 clubs): two legs, 3 points for a win; tie-breaks goal difference and goals for; the top 2 go up, the 3rd plays the promotion play-off against the Allsvenskan 12th and the bottom 3 go down (to the Second Division, when it is playable); 5 substitutes and 3 substitutions'),
         ('por_prm_set_up_rules', 'PorPrm::set_up_rules', '0x7cf930', 'por_prm.cpp', 'thiscall, ret 0',
          'a primeira divisão portuguesa (18 clubes): turno e returno, 3 pontos por vitória; desempate pelo confronto direto, depois saldo, gols marcados e o código 3; ninguém sobe e os 3 últimos caem para a Liga de Honra; 7 reservas e 3 substituições',
          'the Portuguese Premier Division (18 clubs): two legs, 3 points for a win; tie-breaks head to head, then goal difference, goals for and code 3; nobody goes up and the bottom 3 go down to the Liga de Honra; 7 substitutes and 3 substitutions'),
         ('por_second_set_up_rules', 'PorSecond::set_up_rules', '0x7d16a0', 'por_second.cpp', 'thiscall, ret 0',
          'a Liga de Honra (18 clubes, 34 rodadas): turno e returno, 3 pontos por vitória, desempate pelo confronto direto; os 3 primeiros sobem e os 3 últimos caem (para a segunda divisão B, quando ela é jogável); 7 reservas e 3 substituições',
          'the Liga de Honra (18 clubs, 34 rounds): two legs, 3 points for a win, ties broken head to head first; the top 3 go up and the bottom 3 go down (to the Second Division B, when it is playable); 7 substitutes and 3 substitutions')])
CAMPOS = []
INTERESSA = []
