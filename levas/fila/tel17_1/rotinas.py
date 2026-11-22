# The transfer bid screen (transfer_screens.cpp, part 1; reader tel17, the screens round, 21/11/2026). Equal in
# cm0102-1080.exe (no byte of 0x8e9550..0x8f2e10 differs). Needs tel17_2 (the shared ts_* helpers) and tel26_1.
# Conventions checked in the disassembly and at the call sites:
# - 0x8e9550 open_bid_screen_for_offer cdecl(slot, open) ret 0 (open = 0: only the params of the current screen).
# - 0x8e9760 build_bid_screen cdecl() ret 0, SEH frame (Text, LLIST, Cash temporaries); 11.4 KB up to the ret at
#   0x8ec419 (the map's 3462 is an instruction count); split into named pieces (ts_stepper_row, ts_clause_row,
#   ts_copy_offer), the whole routine hooked. Assert boxes at lines 0x208, 0x21b, 0x222.
# - 0x8ec420 handle_bid_screen_action cdecl(item) -> int, SEH frame; 11 KB up to 0x8eef56.
# - 0x8eef60 refresh_bid_total_and_buttons cdecl(bid, live slot) ret 0.
# Callees checked: TRANSFER_OFFER ctors 0x8e56a0 (thiscall, 5 args) and 0x8e5680 (none), has_same_terms 0x8e7f70
# (thiscall, 1), get_negotiation_slot 0x8c6200 (thiscall on the transfer manager, 1), HUMAN_MANAGER::get_bid_offer_slot
# 0x5fda30 (thiscall on [G_HUMAN_MANAGER], 1), Cash::set_from_amount_table 0x43f720 (thiscall, 3), step_down 0x43f580
# (1), set_from_display 0x43f4c0 (an i64), FOG_OF_WAR_MAP::get_knowledge_level 0x5b02f0 (1),
# transfer_manager_selling_club_must_accept 0x8cd240 (thiscall, 3: offer, bid, out reason),
# get_squad_status_reason 0x8c54c0 (thiscall, 1). Date::get_month takes no argument (the pushed 0 is the next call's).
# Comparers: the build side by side over the display list, the human's bid (0x41 bytes) and the contract lookup global
# (only the original when the bid is still to be created: operator new); the answer side by side for the edits of the
# bid (steps, toggles, popups, public, timing, Remove), only the original for the rest (they delete or replace the bid,
# change its exchange list or open screens); the refresh side by side; the opener only the original.
# DEFECTS of the original kept (candidates for docs/DEFEITOS_DO_ORIGINAL.md): the "budget" link (0x25) with a buyer that
# is not the human's club sets the fee to the item argument itself; the two tables' row counts go as the low byte of
# the bids popup's area dword; the 0x1c/0x1d actions check the offer's state twice.
TELAS = True
ROTINAS = [('transfer_screens_open_bid_screen_for_offer', 0x8e9550, [0x8b, 0x44, 0x24, 0x08, 0x85, 0xc0]),
           ('transfer_screens_build_bid_screen', 0x8e9760, [0x6a, 0xff, 0x68, 0xe8, 0x4a, 0x96, 0x00]),
           ('transfer_screens_handle_bid_screen_action', 0x8ec420, [0x6a, 0xff, 0x68, 0xd4, 0x4b, 0x96, 0x00]),
           ('transfer_screens_refresh_bid_total_and_buttons', 0x8eef60, [0x6a, 0xff, 0x68, 0x00, 0x4c, 0x96, 0x00])]
_F = 'transfer_screens.cpp'
DOCS = ('a tela da proposta de transferência', 'the transfer bid screen',
        [('transfer_screens_open_bid_screen_for_offer', 'transfer_screens_open_bid_screen_for_offer', '0x8e9550', _F, 'cdecl',
          'abre a tela da proposta para uma oferta e preenche os parâmetros: o jogador, a vaga da oferta, o número de série, se o clube do contrato está em temporada, se o comprador pode contratá-lo hoje, se a oferta já foi aceita; uma oferta nova passa a ser pública (2)',
          'opens the bid screen for an offer and fills its params: the player, the offer slot, its serial, whether the contract club is in season, whether the buyer can sign him today, whether the offer was accepted; a new offer becomes public (2)'),
         ('transfer_screens_build_bid_screen', 'transfer_screens_build_bid_screen', '0x8e9760', _F, 'cdecl',
          'monta a tela da proposta: cria a proposta do humano (copiada da oferta respondida ou da negociação dela), o nome do clube nas cores dele, as outras propostas pelo jogador num menu, o título, o valor do jogador e o orçamento de transferências (pelo conhecimento do jogador, com a névoa de guerra), as linhas da taxa, das parcelas, da porcentagem da próxima venda, das cláusulas de jogos pela liga e pela seleção e de tornar a proposta pública, com os botões - + = e os menus de valores, Troca, Remover, Repor, Proposta negociada, os jogadores de troca com o valor total, e a barra de botões (Voltar, Cancelar, Oferecer; Retirar; Recusar, Adiar, Negociar ou Aceitar)',
          "builds the bid screen: creates the human's bid (copied from the offer answered or its negotiation), the club's name in its colours, the other bids for the player in a popup, the title, the player's value and the transfer budget (by the knowledge of the player, with the fog of war), the rows of the fee, the instalments, the sell-on percentage, the league and international appearance clauses and making the bid public, with the - + = buttons and the value popups, Exchange, Remove, Reset, Negotiated Bid, the exchange players with the total value, and the button bar (Back, Cancel, Offer; Withdraw; Reject, Delay, Negotiate or Accept)"),
         ('transfer_screens_handle_bid_screen_action', 'transfer_screens_handle_bid_screen_action', '0x8ec420', _F, 'cdecl',
          'responde à tela da proposta: Voltar apaga a proposta; as ações abrem a confirmação, a recusa ou o aviso de aceitação obrigatória (ou, com a oferta mudada, a tela de oferta mudada); as edições mudam a proposta (passos da taxa e das cláusulas, parcelas de 6 em 6 até 24, porcentagem de 5 em 5 até 25, cláusulas ligadas com um quarto do valor do jogador, no mínimo 1000, jogos de 10 em 10 até 50, pública, prazo) e atualizam os botões no lugar; Repor e Proposta negociada copiam a oferta de novo; um jogador de troca sai da lista e o valor dele volta à taxa',
          "answers the bid screen: Back deletes the bid; the actions open the confirm, reject or must accept dialogs (or, with the offer changed, the offer changed screen); the edits change the bid (fee and clause steps, instalments by 6 up to 24, sell-on by 5 up to 25, clauses switched on at a quarter of the player's value, at least 1000, games by 10 up to 50, public, timing) and update the buttons in place; Reset and Negotiated Bid copy the offer again; an exchange player leaves the list and his value goes back to the fee"),
         ('transfer_screens_refresh_bid_total_and_buttons', 'transfer_screens_refresh_bid_total_and_buttons', '0x8eef60', _F, 'cdecl',
          'recalcula o valor total da proposta (taxa, cláusulas ligadas e jogadores de troca) e, com uma oferta respondida, o botão de ação (Negociar, Aceitar cinza ou Aceitar) e o Repor',
          'recalculates the bid total (fee, clauses switched on and exchange players) and, with an offer answered, the action button (Negotiate, grey Accept or Accept) and Reset')])
