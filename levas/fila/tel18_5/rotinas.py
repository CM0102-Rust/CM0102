# The loan bid screen (transfer_screens.cpp, part 2; reader tel18, the screens round, 21/11/2026). Equal in
# cm0102-1080.exe. Needs tel17_1 (TS_MINUS, TS_PLUS, TS_COLOUR_POPUP, ts_at, ts_ab), tel17_2 (ts_* helpers,
# transfer_screens_open_bid_confirm_dialog), tel17_3 (ts_find_news_item, ts_open_offer_changed, TS_LOAN_BID_*, the
# contract offer field offsets), tel18_1 (TS_CANCEL), tel18_4 (ts_contract_club_any), tel18_6
# (transfer_screens_open_confirm_loan_offer) and tel26_1.
# Conventions checked in the disassembly and at the call sites:
# - 0x8f93b0 open_loan_bid_from_news cdecl(news id a, news id b) ret 0 (SEH frame with a NewsItem of 0xe0 bytes, never
#   destroyed); 0x8f9690 open_loan_bid_screen cdecl(player) -> int (1 opened, 0 refused or not opened).
# - 0x8f9a20 build cdecl() ret 0 (SEH frame: a Text of 8 bytes, destroyed; a STAFF_CONTRACT of 0x50 bytes and a Date,
#   never destroyed), asserting at lines 0x23f8 (no player), 0x2406 (no slot manager), 0x240d (no club); the whole
#   routine is hooked, its body in ts_build_loan_bid.
# - 0x8fac30 handler cdecl(item) -> int.
# - Finance::get_max_wage 0x5968d0 takes 2 (ret 8): the third 0 pushed before it is set_screen_param's.
# Callees: NewsItem::NewsItem 0x77bbe0, News::find_item_by_id 0x786b70 (3), NewsItem::get_slot 0x785830 (1),
# TRANSFER_MANAGER::get_offer_slot 0x8c61e0 (1), withdraw_transfer_offer 0x8bfcb0 (3), find_negotiated_loan_contract
# 0x8c72d0 (thiscall 2, Rust), can_loan_player 0x8d0040 (3), get_loan_period 0x8d0590 (7), can_end_loan 0x8d0530 (5),
# CONTRACT_MANAGER::get_contract 0x4cf5a0 (1), find_contract_offer 0x4cc9e0 (2), FinanceTable::get_club_finance
# 0x5a0590 (1), confirm_push 0x591840 (cdecl 7), STAFF_CONTRACT::format_wage_text 0x85e750 (3), Date::Date 0x549f70
# (4), Date::get_short_month_date_string 0x54b2f0 (3), Date::get_month 0x54cfb0, Date::get_day_of_month 0x54cf70,
# Text::set_string / append_string / add_link (3) / draw_wrapped (8) / clear, Display::add_item 0x55f990 (18),
# transfer_screens_open_confirm_loan_offer 0x8fb5e0 (cdecl 7; the month and the share as bytes over stale high bytes).
# Comparers: the build side by side over the display list, the params and the contract lookup global (only the
# original on the assert paths; the STAFF_CONTRACT and the Date are its own stack locals); the handler's param edits
# and Back side by side over the params, the screens, the offer withdrawal and the menu bar only the original; the
# openers only the original.
# Defects and doubts: the greyed buttons push the grey word over stale high halves (a stack slot, the offer pointer's
# high half) and the table's rows byte over three stale bytes, ported zero-extended (the callees read the low part);
# Negotiate, Accept and Reject check the offer's serial against param 9, which neither opener sets (only the news
# Respond of tel17_3 does), so from this screen it is whatever an earlier screen left there; Accept is enabled by the
# loan's end alone, Offer also needs the contribution within the maximum wage; the build reads the contract club's
# gender with no check (a player without a contract would crash, as in the original); the wage share is truncated to
# tens, not rounded (tel17_3's news button rounds).
TELAS = True
ROTINAS = [('transfer_screens_open_loan_bid_from_news', 0x8f93b0, [0x6a, 0xff, 0x68, 0xbb, 0x4e, 0x96, 0x00]),
           ('transfer_screens_open_loan_bid_screen', 0x8f9690, [0x0f, 0xbf, 0x05, 0x7a, 0x7c, 0xb7, 0x00]),
           ('transfer_screens_build_loan_bid_screen', 0x8f9a20, [0x6a, 0xff, 0x68, 0xfc, 0x4e, 0x96, 0x00]),
           ('transfer_screens_handle_loan_bid_screen', 0x8fac30, [0x83, 0xec, 0x18, 0x53, 0x55])]
_F = 'transfer_screens.cpp'
DOCS = ('a tela da proposta de empréstimo', 'the loan bid screen',
        [('transfer_screens_open_loan_bid_from_news', 'transfer_screens_open_loan_bid_from_news', '0x8f93b0', _F, 'cdecl',
          'responde ao botão da notícia de uma proposta de empréstimo negociada: se a proposta não mudou, abre a tela da proposta de empréstimo (recusada, retira a proposta) e preenche os termos com o contrato de empréstimo negociado; senão, a tela de proposta alterada',
          'answers the news button of a negotiated loan offer: when the offer did not change, opens the loan bid screen (refused, withdraws the offer) and fills the terms from the negotiated loan contract; otherwise the offer changed screen'),
         ('transfer_screens_open_loan_bid_screen', 'transfer_screens_open_loan_bid_screen', '0x8f9690', _F, 'cdecl',
          'abre a tela da proposta de empréstimo pelo clube do gerente: se o empréstimo não é possível, a caixa com o motivo (0); senão os parâmetros (salário, teto salarial, período do empréstimo, termos de uma proposta de contrato do tipo 6) e 1',
          "opens the loan bid screen for the manager's club: when the loan is not possible, the box with the reason (0); else the params (wage, wage cap, loan period, the terms of a contract offer of type 6) and 1"),
         ('transfer_screens_build_loan_bid_screen', 'transfer_screens_build_loan_bid_screen', '0x8f9a20', _F, 'cdecl',
          'monta a tela da proposta de empréstimo: o salário atual e o teto da diretoria, a data de fim do empréstimo e a contribuição salarial com "-" e "+", a contribuição em dinheiro, os jogos de copa e contra o clube de origem; Restaurar e Proposta negociada quando responde a uma proposta; os botões Voltar e Oferecer, ou Rejeitar e Aceitar/Negociar, ou Cancelar e Oferecer',
          'builds the loan bid screen: the current wage and the board limit, the loan expiry date and the wage contribution with "-" and "+", the contribution in money, the cup matches and the matches against the parent club; Reset and Negotiated Bid when answering an offer; the Back and Offer buttons, or Reject and Accept/Negotiate, or Cancel and Offer'),
         ('transfer_screens_handle_loan_bid_screen', 'transfer_screens_handle_loan_bid_screen', '0x8fac30', _F, 'cdecl',
          'responde à tela da proposta de empréstimo: Voltar -9; os passos da data e da contribuição, o valor da contribuição, os dois vínculos, Restaurar e Proposta negociada mudam os parâmetros (-11); Oferecer e Negociar abrem a confirmação, Aceitar e Rejeitar a confirmação da proposta (-4, ou a tela de proposta alterada); Cancelar retira a proposta (-9)',
          'answers the loan bid screen: Back -9; the date and contribution steps, the contribution value, the two links, Reset and Negotiated Bid change the params (-11); Offer and Negotiate open the confirmation, Accept and Reject the bid confirmation (-4, or the offer changed screen); Cancel withdraws the offer (-9)')])
