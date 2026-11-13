# The league rules of the Norwegian Premier and First Divisions and of the Swedish Premier Division (reader mwe,
# whole-world round, 12/11/2026). Each runs when its league is created (the constructor) and at each season turnover
# (virtual +0x8c of start_new_season; the Norwegian and Swedish seasons turn over at the end of 2001):
# - 0x792700 NorPrm::set_up_rules (160 bytes);
# - 0x790b00 NorFirst::set_up_rules (144 bytes);
# - 0x890550 SwePrm::set_up_rules (160 bytes).
# The body of eng_prm_apply_league_rules (the English rules, cqi_4): byte, word and dword writes and the virtual +0x3c
# (create_schedule) of the main stage into +0xba. Equal in cm0102-1080.exe (whole).
# Conventions checked: called from the constructors (mov ecx, esi; call) and through vtable +0x8c -> thiscall, ret 0
# (plain `ret` in the three); inside, the virtual +0x3c: push 0; push &+0x3a; push &+0xa9; push -1; mov ecx ->
# thiscall, ret 0x10.
# Comparer: eng_prm_compare_league_rules (the original runs, the band +0x1c..+0xec is restored, the copy writes the
# rules without the virtual; the written fields must be equal; the original's band is kept).
# Determinism: only constants and the key globals.
LIGAS = 'ENG SWE FIN NOR POR'
ATE = '2002-08-01'
SAVE = False
ROTINAS = [('nor_prm_set_up_rules', 0x792700, [0x53, 0x56, 0x8b, 0xf1, 0xb8, 0x02, 0x00, 0x00, 0x00]),
           ('nor_first_set_up_rules', 0x790b00, [0x53, 0x56, 0x8b, 0xf1, 0xb8, 0x02, 0x00, 0x00, 0x00]),
           ('swe_prm_set_up_rules', 0x890550, [0x53, 0x56, 0x8b, 0xf1, 0xb2, 0x03])]
DOCS = ('as regras das duas divisões norueguesas e da primeira divisão sueca',
        'the league rules of the Norwegian Premier and First Divisions and of the Swedish Premier Division',
        [('nor_prm_set_up_rules', 'NorPrm::set_up_rules', '0x792700', 'nor_prm.cpp', 'thiscall, ret 0',
          'a Tippeligaen (14 clubes): turno e returno, 3 pontos por vitória e 1 por empate; desempate pelo saldo, pelos gols marcados e pelo código 3; ninguém sobe, o 12º joga o play-off de rebaixamento e os 2 últimos caem para a primeira divisão; 7 reservas e 3 substituições; depois o calendário',
          'the Tippeligaen (14 clubs): two legs, 3 points for a win and 1 for a draw; tie-breaks goal difference, goals for and code 3; nobody goes up, the 12th plays the relegation play-off and the bottom 2 go down to the First Division; 7 substitutes and 3 substitutions; then the schedule'),
         ('nor_first_set_up_rules', 'NorFirst::set_up_rules', '0x790b00', 'nor_first.cpp', 'thiscall, ret 0',
          'a primeira divisão norueguesa (16 clubes): turno e returno, 3 pontos por vitória; os 2 primeiros sobem, o 3º joga o play-off de acesso contra o 12º da Tippeligaen e os 4 últimos caem (sem divisão abaixo); 7 reservas e 3 substituições',
          'the Norwegian First Division (16 clubs): two legs, 3 points for a win; the top 2 go up, the 3rd plays the promotion play-off against the Tippeligaen 12th and the bottom 4 go down (no division below); 7 substitutes and 3 substitutions'),
         ('swe_prm_set_up_rules', 'SwePrm::set_up_rules', '0x890550', 'swe_prm.cpp', 'thiscall, ret 0',
          'a Allsvenskan (14 clubes): turno e returno, 3 pontos por vitória; desempate pelo saldo e pelos gols marcados; ninguém sobe, o 12º joga o play-off de rebaixamento e os 2 últimos caem para a Superettan; 5 reservas e 3 substituições',
          'the Allsvenskan (14 clubs): two legs, 3 points for a win; tie-breaks goal difference and goals for; nobody goes up, the 12th plays the relegation play-off and the bottom 2 go down to the Superettan; 5 substitutes and 3 substitutions')])
CAMPOS = []
INTERESSA = []
