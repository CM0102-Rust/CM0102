# The confirm trial and loan offer dialogs and the end loan dialog (transfer_screens.cpp, part 2; reader tel18, the
# screens round, 21/11/2026). Equal in cm0102-1080.exe. Needs tel17_2 (ts_* helpers), tel17_4 (TsTextAreaLinks),
# tel18_3 (TS_NETWORK_GAME), tel18_4 (ts_no_yes_bar, ts_contract_club_any) and tel26_1.
# Conventions checked in the disassembly and at the call sites:
# - 0x8fb530 open_confirm_trial_offer cdecl(player, weeks) ret 0 (called by 0x8f9080 with the weeks byte over the item
#   argument's high bytes); 0x8fb5e0 open_confirm_loan_offer cdecl(player, month, year, a4, a5, a6, offer) ret 0
#   (called twice by the loan bid handler 0x8fac30); 0x8fbcc0 open_end_loan_dialog cdecl(player) ret 0.
# - 0x8fb6a0 / 0x8fbdf0 builds cdecl() ret 0, asserting at lines 0x26f4 / 0x27f0 without the slot's manager.
# - 0x8fb9e0 / 0x8fc0e0 handlers cdecl(item) -> int (SEH frames: a STAFF_CONTRACT of 0x50 bytes and a Date on the
#   stack, never destroyed; a Text of 8 bytes, destroyed).
# Callees: STAFF_CONTRACT::STAFF_CONTRACT 0x85e410 (thiscall), init_trial_offer 0x85e480 (thiscall, 3, ret 0xc),
# CONTRACT_MANAGER::add_contract_offer 0x4cc490 (7), Date::Date 0x549f70 (4), Date::get_day_of_month 0x54cf70,
# TRANSFER_MANAGER::add_loan_negotiation 0x8bee50 (5, Rust), make_loan_offer 0x8bed60 (6), can_end_loan 0x8d0530 (5),
# CONTRACT_MANAGER::schedule_loan_end 0x4ccca0 (2, Rust), confirm_push 0x591840 (cdecl, 7), confirm_push_text_object
# 0x5919b0 (cdecl, 7), Text::Text 0x4ae590, Text::add_link 0x4ae8a0 (3), Text::~Text 0x4ae630.
# Comparers: the builds side by side over the display list (only the original on the assert paths); No side by side;
# Yes (the offers, the loan end and its box), the network command and the menu bar only the original; the openers only
# the original.
TELAS = True
ROTINAS = [('transfer_screens_open_confirm_trial_offer', 0x8fb530, [0x6a, 0x00, 0x6a, 0x00, 0x6a, 0x01]),
           ('transfer_screens_open_confirm_loan_offer', 0x8fb5e0, [0x6a, 0x00, 0x6a, 0x00, 0x6a, 0x01]),
           ('transfer_screens_build_confirm_offer_dialog', 0x8fb6a0, [0x81, 0xec, 0x38, 0x03, 0x00, 0x00]),
           ('transfer_screens_handle_confirm_offer_dialog', 0x8fb9e0, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('transfer_screens_open_end_loan_dialog', 0x8fbcc0, [0x0f, 0xbf, 0x05, 0x7a, 0x7c, 0xb7, 0x00]),
           ('transfer_screens_build_end_loan_dialog', 0x8fbdf0, [0x81, 0xec, 0x38, 0x03, 0x00, 0x00]),
           ('transfer_screens_handle_end_loan_dialog', 0x8fc0e0, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00])]
_F = 'transfer_screens.cpp'
DOCS = ('as confirmações das propostas de período de teste e de empréstimo e a janela de encerrar o empréstimo', 'the confirm trial and loan offer dialogs and the end loan dialog',
        [('transfer_screens_open_confirm_trial_offer', 'transfer_screens_open_confirm_trial_offer', '0x8fb530', _F, 'cdecl',
          'abre a confirmação da proposta de período de teste (parâmetros: o jogador e as semanas; os da proposta de empréstimo -1 e 0)',
          "opens the confirm trial offer dialog (params: the player and the weeks; the loan offer's -1 and 0)"),
         ('transfer_screens_open_confirm_loan_offer', 'transfer_screens_open_confirm_loan_offer', '0x8fb5e0', _F, 'cdecl',
          'abre a confirmação da proposta de empréstimo (parâmetros: o jogador, semanas -1, o mês e o ano do fim, os termos e a proposta em negociação)',
          'opens the confirm loan offer dialog (params: the player, weeks -1, the end month and year, the terms and the offer under negotiation)'),
         ('transfer_screens_build_confirm_offer_dialog', 'transfer_screens_build_confirm_offer_dialog', '0x8fb6a0', _F, 'cdecl',
          'monta a confirmação: nas cores do clube do gerente, o subtítulo e a pergunta pelo caso (teste de 1 ou n semanas, negociar a proposta de empréstimo de um clube, ou enviar a proposta de empréstimo) com o link do jogador; botões Não (1) e Sim (2); sem o gerente, a caixa de erro',
          "builds the confirmation: in the manager's club colours, the subtitle and the question by the case (a trial of 1 or n weeks, negotiating a club's loan offer, or submitting a loan offer) with the player's link; No (1) and Yes (2) buttons; without the manager, the error box"),
         ('transfer_screens_handle_confirm_offer_dialog', 'transfer_screens_handle_confirm_offer_dialog', '0x8fb9e0', _F, 'cdecl',
          'responde à confirmação: Não -9; Sim (-10) registra a proposta de contrato de teste, ou data o fim do empréstimo (dia do mês até 28) e negocia a proposta existente ou faz a proposta de empréstimo nova',
          "answers the confirmation: No -9; Yes (-10) adds the trial contract offer, or dates the loan's end (day of the month up to 28) and negotiates the existing offer or makes the new loan offer"),
         ('transfer_screens_open_end_loan_dialog', 'transfer_screens_open_end_loan_dialog', '0x8fbcc0', _F, 'cdecl',
          'abre a janela de encerrar o empréstimo (chamar de volta ou rescindir); se não pode, uma caixa com o motivo ("Não é possível rescindir/chamar de volta")',
          'opens the end loan dialog (recall or terminate); when it cannot, a box with the reason ("Cannot terminate/recall")'),
         ('transfer_screens_build_end_loan_dialog', 'transfer_screens_build_end_loan_dialog', '0x8fbdf0', _F, 'cdecl',
          'monta a janela de encerrar o empréstimo: "Rescindir o empréstimo" se o jogador joga no clube do gerente, senão "Chamar de volta", com os links do jogador e do clube; botões Não (1) e Sim (2); sem o gerente, a caixa de erro',
          "builds the end loan dialog: \"Terminate loan\" when the player plays for the manager's club, else \"Recall\", with the player's and the club's links; No (1) and Yes (2) buttons; without the manager, the error box"),
         ('transfer_screens_handle_end_loan_dialog', 'transfer_screens_handle_end_loan_dialog', '0x8fc0e0', _F, 'cdecl',
          'responde à janela de encerrar o empréstimo: comando de rede pendente 3 -9; Não -9; Sim agenda o fim do empréstimo e mostra a caixa "volta para o clube amanhã" (-4)',
          'answers the end loan dialog: pending network command 3 -9; No -9; Yes schedules the loan end and shows the "will return tomorrow" box (-4)')])
