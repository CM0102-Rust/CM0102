# The transfer bid dialogs: confirm, reject, must accept and cannot sign (transfer_screens.cpp, part 1; reader tel17,
# the screens round, 21/11/2026). Equal in cm0102-1080.exe (no byte of 0x8e9550..0x8f2e10 differs).
# Conventions checked in the disassembly and at the call sites:
# - 0x8ef1e0 open_bid_confirm_dialog cdecl(edited slot, action, live slot) ret 0.
# - 0x8ef240 build_bid_confirm_dialog cdecl() ret 0, SEH frame (Date); assert box line 0xd42 without the slot's manager.
# - 0x8ef9f0 handle_bid_confirm_action cdecl(item) -> int (-10 done, -9 No, else the menu bar).
# - 0x8efda0 build_reject_bid_dialog cdecl() ret 0, SEH frame (LLIST, Text); assert box line 0xeed.
# - 0x8f06f0 handle_reject_bid_action cdecl(item) -> int; the squad status byte of the reason is kept over the low
#   byte of the item argument and pushed as a dword (the copy passes the same dword).
# - 0x8f0bc0 build_offer_must_accept_dialog cdecl() ret 0 (no null check of the slot's manager).
# - 0x8f0e40 open_cannot_sign_dialog cdecl(player, club, short reason, char mode) ret 0, SEH frame (LLIST).
# - 0x8f0f90 build_cannot_sign_dialog cdecl() ret 0.
# - 0x8f16c0 handle_back_dialog_action cdecl(item) -> int.
# The transfer manager helpers are thiscall on [G_TRANSFER_MANAGER]: set_player_squad_status 0x8ce0e0 (7 args, ret
# 0x1c), get_squad_status_for_reason 0x8c5800 (2), get_rejection_reason_text 0x8c5690 (3), is_rejection_reason_valid
# 0x8c5560 (2).
# Comparers: the builds side by side over the display list (+ G_CONTRACT_LOOKUP_RESULT where the inline contract
# lookups write it); only the original for the openers (they open screens), for the Yes answers (they carry out the
# transfer action: withdraw, reject, accept, negotiate, delay, submit) and for the cannot sign dialog with reason 0x21
# (it marks the linked offer, +0x2d = 1); the answers that only return a code or set a param side by side.
# Shared helpers of the tel17 items live here (ts_*): tel17_1, tel17_3 and tel17_4 use them.
TELAS = True
ROTINAS = [('transfer_screens_open_bid_confirm_dialog', 0x8ef1e0, [0x6a, 0x00, 0x6a, 0x00, 0x6a, 0x01]),
           ('transfer_screens_build_bid_confirm_dialog', 0x8ef240, [0x6a, 0xff, 0x68, 0x1b, 0x4c, 0x96, 0x00]),
           ('transfer_screens_handle_bid_confirm_action', 0x8ef9f0, [0x51, 0x53, 0x55, 0x56, 0x57]),
           ('transfer_screens_build_reject_bid_dialog', 0x8efda0, [0x6a, 0xff, 0x68, 0x46, 0x4c, 0x96, 0x00]),
           ('transfer_screens_handle_reject_bid_action', 0x8f06f0, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('transfer_screens_build_offer_must_accept_dialog', 0x8f0bc0, [0x81, 0xec, 0x3c, 0x01, 0x00, 0x00]),
           ('transfer_screens_open_cannot_sign_dialog', 0x8f0e40, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('transfer_screens_build_cannot_sign_dialog', 0x8f0f90, [0x81, 0xec, 0xa0, 0x01, 0x00, 0x00]),
           ('transfer_screens_handle_back_dialog_action', 0x8f16c0, [0x56, 0x8b, 0x74, 0x24, 0x08])]
_F = 'transfer_screens.cpp'
DOCS = ('as janelas da proposta de transferência: confirmar, recusar, aceitação obrigatória e não pode contratar',
        'the transfer bid dialogs: confirm, reject, must accept and cannot sign',
        [('transfer_screens_open_bid_confirm_dialog', 'transfer_screens_open_bid_confirm_dialog', '0x8ef1e0', _F, 'cdecl',
          'abre a janela Sim/Não que confirma uma ação sobre a proposta e guarda nos parâmetros a cópia editada, a ação (1 enviar, 2 cancelar, 0x1b recusar, 0x1c negociar, 0x1d aceitar, 0x23 adiar) e a proposta viva',
          'opens the Yes/No dialog that confirms a bid action and keeps in its params the edited copy, the action (1 submit, 2 cancel, 0x1b reject, 0x1c negotiate, 0x1d accept, 0x23 delay) and the live offer'),
         ('transfer_screens_build_bid_confirm_dialog', 'transfer_screens_build_bid_confirm_dialog', '0x8ef240', _F, 'cdecl',
          'monta a janela de confirmação: a foto do jogador, o título nas cores do clube do humano, "Proposta de transferência/empréstimo por <jogador>", o texto da ação (enviar com a data em que se concretiza, pelo fim do contrato ou da temporada; adiar, cancelar, negociar, aceitar ou recusar o empréstimo do clube comprador), a barra de menus e os botões Não (3) e Sim (2); sem o gerente da vaga, a caixa de erro (linha 0xd42)',
          "builds the confirm dialog: the player's picture, the title in the human's club colours, \"Transfer/Loan offer for <player>\", the action text (submit with the completion date, by contract end or season end; delay, cancel, negotiate, accept or reject the buyer's loan offer), the menu bar and the No (3) and Yes (2) buttons; without the slot's manager, the error box (line 0xd42)"),
         ('transfer_screens_handle_bid_confirm_action', 'transfer_screens_handle_bid_confirm_action', '0x8ef9f0', _F, 'cdecl',
          'responde à confirmação: Sim executa a ação (o clube recusa e adia, retirar ou recusar a proposta, enviar, recusar o empréstimo, negociar e, com a taxa mudada, o status do jogador no elenco segue a proposta, aceitar o empréstimo ou a proposta) e apaga a cópia editada (-10); Não -9; o resto vai para a barra de menus',
          "answers the confirm dialog: Yes carries out the action (the club rejects and delays, withdraw or reject the offer, submit, reject the loan, negotiate and, with the fee changed, the player's squad status follows the bid, accept the loan or the bid) and deletes the edited copy (-10); No -9; the rest goes to the menu bar"),
         ('transfer_screens_build_reject_bid_dialog', 'transfer_screens_build_reject_bid_dialog', '0x8efda0', _F, 'cdecl',
          'monta a janela de recusar a proposta: a lista dos motivos (0, 0x200 e 0x400 sempre; 0x2000, 0x1000, 0xa e 0x800 só os válidos para a proposta), o escolhido em amarelo; com mais de uma proposta aberta pelo jogador, o botão Recusar todas (cinza se o motivo não vale para todas) entre Não e Sim',
          'builds the reject bid dialog: the list of reasons (0, 0x200 and 0x400 always; 0x2000, 0x1000, 0xa and 0x800 only the valid ones for the offer), the chosen one in yellow; with more than one open bid for the player, the Reject All Bids button (grey when the reason is not valid for all) between No and Yes'),
         ('transfer_screens_handle_reject_bid_action', 'transfer_screens_handle_reject_bid_action', '0x8f06f0', _F, 'cdecl',
          'responde à recusa: um motivo vai para o parâmetro 2 (-11); Recusar todas acerta o status do jogador pelo motivo e recusa todas as propostas abertas; Sim recusa a proposta viva com o motivo e acerta o status; as duas apagam a cópia editada (-10); Não -9',
          "answers the reject dialog: a reason goes to param 2 (-11); Reject All Bids sets the player's squad status by the reason and rejects every open bid; Yes rejects the live bid with the reason and sets the status; both delete the edited copy (-10); No -9"),
         ('transfer_screens_build_offer_must_accept_dialog', 'transfer_screens_build_offer_must_accept_dialog', '0x8f0bc0', _F, 'cdecl',
          'monta a janela que diz que a proposta não pode ser recusada, negociada ou adiada: pela cláusula de rescisão (o motivo 0x1b) ou pela diretoria do clube do humano; botão Voltar',
          "builds the dialog that says the bid cannot be rejected, negotiated or delayed: because of the release clause (reason 0x1b) or the human club's board; Back button"),
         ('transfer_screens_open_cannot_sign_dialog', 'transfer_screens_open_cannot_sign_dialog', '0x8f0e40', _F, 'cdecl',
          'abre a janela de por que o clube não pode contratar, emprestar ou testar o jogador; nos motivos 0x12 e 0x20 o parâmetro 3 recebe o clube da proposta acertada (status 0xb a 0xe ou 0x13)',
          'opens the dialog on why the club cannot sign, loan or trial the player; for the reasons 0x12 and 0x20 param 3 gets the club of the agreed bid (status 0xb to 0xe or 0x13)'),
         ('transfer_screens_build_cannot_sign_dialog', 'transfer_screens_build_cannot_sign_dialog', '0x8f0f90', _F, 'cdecl',
          'monta a janela de não pode contratar: o título pelo modo (proposta cancelada, não pode emprestar, testar, contratar) e o texto pelo motivo (envolvido em outra negociação, jogadores sauditas, contrato acertado com outro clube, que marca a proposta ligada como conhecida do jogador, fim do contrato com o clube atual, transferência acertada, clube que não pode emprestar, regra de nacionalidade do País Basco, do clube ou da Espanha); botão Voltar',
          "builds the cannot sign dialog: the title by the mode (bid cancelled, cannot loan, trial, sign) and the text by the reason (involved in other talks, Saudi players, contract agreed with another club, which marks the linked offer as known to the player, end of contract at the current club, transfer agreed, club that cannot loan, nationality rule of the Basque Country, the club's nation or Spain); Back button"),
         ('transfer_screens_handle_back_dialog_action', 'transfer_screens_handle_back_dialog_action', '0x8f16c0', _F, 'cdecl',
          'responde ao botão Voltar das janelas de não pode contratar e de aceitação obrigatória (-9); o resto vai para a barra de menus',
          'answers the Back button of the cannot sign and must accept dialogs (-9); the rest goes to the menu bar')])
