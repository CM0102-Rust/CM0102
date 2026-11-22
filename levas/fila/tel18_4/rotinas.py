# The transfer list, release and trial dialogs (transfer_screens.cpp, part 2; reader tel18, the screens round,
# 21/11/2026). Equal in cm0102-1080.exe. Needs tel17_1 (TS_MINUS, TS_PLUS, TS_COLOUR_POPUP), tel17_2 (ts_* helpers),
# tel17_4 (TsTextAreaLinks) and tel26_1. Its helpers ts_no_yes_bar, ts_contract_club_any and
# ts_title_in_contract_club are used by tel18_5.
# Conventions checked in the disassembly and at the call sites:
# - 0x8f8120 open_transfer_list_dialog cdecl(player, loan) ret 0; 0x8f8630 open_release_dialog cdecl(player) ret 0;
#   0x8f8be0 open_offer_trial_dialog cdecl(player) ret 0.
# - 0x8f8170 / 0x8f8670 (SEH frame, a Cash) / 0x8f8c30 builds cdecl() ret 0; the trial build asserts at lines
#   0x21a6, 0x21b4, 0x21bb.
# - 0x8f8450 / 0x8f8b00 / 0x8f9080 handlers cdecl(item) -> int.
# Callees: STAFF_CONTRACT::wants_to_leave 0x866f60 (thiscall), apply_event_flags 0x867700 (thiscall, 2),
# days_to_end_date 0x85fa00 (thiscall, returns a short), TRANSFER_MANAGER::queue_free_transfer 0x8c16b0 (3), the
# shortlist manager list calls on [TM+0x856] (0x842320 with 2, 0x844280 / 0x8442b0 / 0x8442d0 with 1),
# transfer_screens_open_confirm_trial_offer 0x8fb530 (cdecl, 2: the weeks byte over the item argument's high bytes).
# Comparers: the builds side by side over the display list (+ the contract lookup global; the trial build only the
# original on its assert paths); No/Back side by side; the Yes/Offer answers and the week steps (widgets and redraw)
# only the original; the openers only the original.
TELAS = True
ROTINAS = [('transfer_screens_open_transfer_list_dialog', 0x8f8120, [0x6a, 0x00, 0x6a, 0x00, 0x6a, 0x01]),
           ('transfer_screens_build_transfer_list_dialog', 0x8f8170, [0x81, 0xec, 0x9c, 0x01, 0x00, 0x00]),
           ('transfer_screens_handle_transfer_list_dialog', 0x8f8450, [0x66, 0x8b, 0x15, 0x7a, 0x7c, 0xb7, 0x00]),
           ('transfer_screens_open_release_dialog', 0x8f8630, [0x6a, 0x00, 0x6a, 0x00, 0x6a, 0x01]),
           ('transfer_screens_build_release_dialog', 0x8f8670, [0x6a, 0xff, 0x68, 0xa6, 0x4e, 0x96, 0x00]),
           ('transfer_screens_handle_release_dialog', 0x8f8b00, [0x66, 0x8b, 0x15, 0x7a, 0x7c, 0xb7, 0x00]),
           ('transfer_screens_open_offer_trial_dialog', 0x8f8be0, [0x6a, 0x00, 0x6a, 0x00, 0x6a, 0x01]),
           ('transfer_screens_build_offer_trial_dialog', 0x8f8c30, [0x81, 0xec, 0xa8, 0x03, 0x00, 0x00]),
           ('transfer_screens_handle_offer_trial_dialog', 0x8f9080, [0x66, 0x8b, 0x15, 0x7a, 0x7c, 0xb7, 0x00])]
_F = 'transfer_screens.cpp'
DOCS = ('as janelas da lista de transferências, da dispensa e do período de teste', 'the transfer list, release and trial dialogs',
        [('transfer_screens_open_transfer_list_dialog', 'transfer_screens_open_transfer_list_dialog', '0x8f8120', _F, 'cdecl',
          'abre a janela de pôr ou tirar o jogador da lista de transferências ou de empréstimos (parâmetros: o jogador e se é a de empréstimos)',
          'opens the dialog that puts the player on or takes him off the transfer or loan list (params: the player and whether it is the loan list)'),
         ('transfer_screens_build_transfer_list_dialog', 'transfer_screens_build_transfer_list_dialog', '0x8f8170', _F, 'cdecl',
          'monta a janela da lista: pela situação do jogador (já na lista ou não, de transferências ou empréstimos), o subtítulo e a pergunta com o link dele, nas cores do clube do contrato; botões Não (1) e Sim (2)',
          "builds the list dialog: by the player's situation (already listed or not, transfer or loan), the subtitle and the question with his link, in his contract club's colours; No (1) and Yes (2) buttons"),
         ('transfer_screens_handle_transfer_list_dialog', 'transfer_screens_handle_transfer_list_dialog', '0x8f8450', _F, 'cdecl',
          'responde à janela da lista: Não -10; Sim põe ou tira o jogador da lista de empréstimos, ou põe na de transferências, ou tira dela e, se ele quer sair, sem a queixa 0x10000 e com lealdade abaixo de 15, o moral cai 50 e a queixa é registrada (-10)',
          'answers the list dialog: No -10; Yes puts the player on or takes him off the loan list, or puts him on the transfer list, or takes him off it and, when he wants to leave, without the grievance 0x10000 and with loyalty under 15, his morale drops by 50 and the grievance is raised (-10)'),
         ('transfer_screens_open_release_dialog', 'transfer_screens_open_release_dialog', '0x8f8630', _F, 'cdecl',
          'abre a janela de dispensar o jogador (parâmetro: o jogador)',
          'opens the release dialog (param: the player)'),
         ('transfer_screens_build_release_dialog', 'transfer_screens_build_release_dialog', '0x8f8670', _F, 'cdecl',
          'monta a janela de dispensa: a indenização (contrato do tipo 2: salário vezes os dias até o fim / 7, um quarto se ele quer sair) e a pergunta com o valor e o clube, ou "dispensar de graça"; botões Não (1) e Sim (2)',
          'builds the release dialog: the pay-off (contract of type 2: wage times the days to its end / 7, a quarter when he wants to leave) and the question with the money and the club, or "release on a free transfer"; No (1) and Yes (2) buttons'),
         ('transfer_screens_handle_release_dialog', 'transfer_screens_handle_release_dialog', '0x8f8b00', _F, 'cdecl',
          'responde à janela de dispensa: Não -10; Sim põe o jogador na fila de transferências livres com a indenização (-10)',
          "answers the release dialog: No -10; Yes queues the player's free transfer with the pay-off (-10)"),
         ('transfer_screens_open_offer_trial_dialog', 'transfer_screens_open_offer_trial_dialog', '0x8f8be0', _F, 'cdecl',
          'abre a janela de oferecer um período de teste ao jogador, com 2 semanas',
          'opens the dialog that offers the player a trial, with 2 weeks'),
         ('transfer_screens_build_offer_trial_dialog', 'transfer_screens_build_offer_trial_dialog', '0x8f8c30', _F, 'cdecl',
          'monta a janela do período de teste: "Período de teste" com "-", as semanas (1 a 4) e "+", e os botões Voltar e Oferecer; sem o jogador, o gerente ou o clube, a caixa de erro',
          'builds the trial dialog: "Trial Period" with "-", the weeks (1 to 4) and "+", and the Back and Offer buttons; without the player, the manager or the club, the error box'),
         ('transfer_screens_handle_offer_trial_dialog', 'transfer_screens_handle_offer_trial_dialog', '0x8f9080', _F, 'cdecl',
          'responde à janela do período de teste: Voltar -9; Oferecer abre a confirmação com as semanas (-4); "-" e "+" mudam as semanas e os botões no lugar (0)',
          'answers the trial dialog: Back -9; Offer opens the confirmation with the weeks (-4); "-" and "+" change the weeks and the widgets in place (0)')])
