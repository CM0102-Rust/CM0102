# The contract and board accessors of the staff profile's Transfer tab (staff_screens.cpp, part 3; reader tel09, the
# screens round, 21/11/2026). Out-of-line copies of inline accessors, called only by the Transfer tab 0x87dd80
# (tel08_3, in the original); each is a 4- to 7-byte routine, the hook covers the alignment nops after it.
# - 0x8857b0 contract +0x44 (morale) thiscall -> al; 0x8857c0 +0x3f (matches left out) -> al; 0x8857d0 +0x4f >> 4
#   (squad status) -> al; 0x8857e0 +0x49 (leaving on a Bosman) -> al; 0x8857f0 +0x4e & 8 (transfer requested) -> eax.
# - 0x885800 thiscall(MANAGER_MANAGER, club) ret 4 -> ax: 0 for a null club, else the word +6 (board confidence) of
#   the club's 0x49-byte board record ([this] + id x 0x49).
# Comparer: compare_value! (the value side by side). They run only while the Transfer tab is drawn.
# How to reach them: a person's profile (click a player of another club) > the Transfer tab.
TELAS = True
ROTINAS = [('staff_screens_contract_morale', 0x8857b0, [0x8a, 0x41, 0x44, 0xc3, 0x90]),
           ('staff_screens_contract_matches_left_out', 0x8857c0, [0x8a, 0x41, 0x3f, 0xc3, 0x90]),
           ('staff_screens_contract_squad_status', 0x8857d0, [0x8a, 0x41, 0x4f, 0xc0, 0xe8, 0x04]),
           ('staff_screens_contract_leaving_on_bosman', 0x8857e0, [0x8a, 0x41, 0x49, 0xc3, 0x90]),
           ('staff_screens_contract_transfer_requested', 0x8857f0, [0x8a, 0x41, 0x4e, 0x83, 0xe0, 0x08]),
           ('staff_screens_board_confidence', 0x885800, [0x8b, 0x44, 0x24, 0x04, 0x85, 0xc0])]
DOCS = ('os acessos ao contrato e à diretoria da aba Transferência do perfil',
        "the contract and board accessors of the profile's Transfer tab",
        [('staff_screens_contract_morale', 'staff_screens_contract_morale', '0x8857b0', 'staff_screens.cpp', 'thiscall',
          'O moral do contrato (+0x44).', "The contract's morale (+0x44)."),
         ('staff_screens_contract_matches_left_out', 'staff_screens_contract_matches_left_out', '0x8857c0', 'staff_screens.cpp', 'thiscall',
          'Os jogos em que a pessoa do contrato ficou de fora (+0x3f).', "The matches the contract's person was left out (+0x3f)."),
         ('staff_screens_contract_squad_status', 'staff_screens_contract_squad_status', '0x8857d0', 'staff_screens.cpp', 'thiscall',
          'A situação no elenco: o nibble alto de +0x4f.', 'The squad status: the high nibble of +0x4f.'),
         ('staff_screens_contract_leaving_on_bosman', 'staff_screens_contract_leaving_on_bosman', '0x8857e0', 'staff_screens.cpp', 'thiscall',
          'Se a pessoa sai pela lei Bosman (+0x49).', 'Whether the person leaves on a Bosman (+0x49).'),
         ('staff_screens_contract_transfer_requested', 'staff_screens_contract_transfer_requested', '0x8857f0', 'staff_screens.cpp', 'thiscall',
          'O bit 8 (pedido de transferência) de +0x4e.', 'Bit 8 (transfer requested) of +0x4e.'),
         ('staff_screens_board_confidence', 'staff_screens_board_confidence', '0x885800', 'staff_screens.cpp', 'thiscall, ret 4',
          'A confiança da diretoria no técnico do clube (palavra +6 do registro de 0x49 bytes do clube na tabela do MANAGER_MANAGER); 0 sem clube.',
          "The board's confidence in the club's manager (word +6 of the club's 0x49-byte record in the MANAGER_MANAGER table); 0 without a club.")])
