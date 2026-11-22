# The staff transfer status screen (transfer_screens.cpp, part 2; reader tel18, the screens round, 21/11/2026). Equal
# in cm0102-1080.exe. Needs tel17_1 (TS_MINUS, TS_PLUS, TS_FEE_STEPS, TS_FN_CASH_FROM_TABLE, TS_FN_CASH_STEP_DOWN,
# TS_COLOUR_POPUP), tel17_2 (ts_* helpers), tel18_1 (ts_news_slot) and tel26_1.
# Conventions checked in the disassembly and at the call sites:
# - 0x8f66d0 open_transfer_status_from_news cdecl(news id a, news id b) ret 0, SEH frame (a NewsItem, no destructor);
# - 0x8f6770 open_transfer_status_screen cdecl(player, a, b, c) ret 0 (params 7, 8, 9);
# - 0x8f6a10 build cdecl() ret 0, SEH frame (Text); assert box line 0x1d15 without the contract;
# - 0x8f7640 handle cdecl(item) -> int, SEH frame (a Cash temporary in the argument slot).
# Callees: CONTRACT_MANAGER::get_contract_for_club 0x4cf5f0 (thiscall, 2), calc_player_value 0x4d0c60 (thiscall, 5:
# the sixth push is the next call's), shortlist_manager_squad_status_text 0x84b330 (thiscall on [TM+0x856], 3),
# squad_status_allowed 0x84b4d0 (2), TRANSFER_MANAGER::set_player_squad_status 0x8ce0e0 (7: player, club, value, &out,
# reason byte, flags byte, 0; the original pushes the reason and the old status as bytes over leftover high bytes,
# the callee reads bytes), the list calls 0x842320 (2) / 0x844280 / 0x8442b0 / 0x8442d0 (1) on [TM+0x856], the
# widget setters 0x800580 / 0x801960 / 0x8022d0 / 0x801fc0 / 0x801c70 / 0x800d30 and redraw_screen 0x802d00.
# Comparers: the build side by side over the display list (only the original without the contract: assert box); the
# answers that only edit the params side by side (plus the contract lookup global); "-"/"+" (widgets and redraw),
# Set (contract, lists, news), the openers and the menu bar only the original.
# Defects of the original: Set rewrites the contract's squad status with its own old value (the new one goes through
# set_player_squad_status); "-" greys "+" with set_widget_data and "+" greys it with set_widget_id (kept).
TELAS = True
ROTINAS = [('transfer_screens_open_transfer_status_from_news', 0x8f66d0, [0x6a, 0xff, 0x68, 0x3b, 0x4e, 0x96, 0x00]),
           ('transfer_screens_open_transfer_status_screen', 0x8f6770, [0x56, 0x6a, 0x00, 0x6a, 0x00]),
           ('transfer_screens_build_transfer_status_screen', 0x8f6a10, [0x6a, 0xff, 0x68, 0x66, 0x4e, 0x96, 0x00]),
           ('transfer_screens_handle_transfer_status_screen', 0x8f7640, [0x6a, 0xff, 0x68, 0x80, 0x4e, 0x96, 0x00])]
_F = 'transfer_screens.cpp'
DOCS = ('a tela do status de transferência do jogador', "the staff transfer status screen",
        [('transfer_screens_open_transfer_status_from_news', 'transfer_screens_open_transfer_status_from_news', '0x8f66d0', _F, 'cdecl',
          'botão Responder da notícia: abre a tela do status de transferência do jogador da notícia (campo 2), com os números da notícia',
          "the news item's Respond button: opens the transfer status screen of the news item's player (slot 2), with the news item's ids"),
         ('transfer_screens_open_transfer_status_screen', 'transfer_screens_open_transfer_status_screen', '0x8f6770', _F, 'cdecl',
          'abre a tela do status de transferência: o valor do jogador como preço pedido e, pelo contrato com o clube, o status no elenco, se está na lista de transferências ou de empréstimo, se pediu para sair, as queixas e o preço automático; o valor dele para o clube como teto',
          "opens the transfer status screen: the player's value as the asking price and, from his contract with the club, the squad status, whether he is transfer or loan listed, whether he asked to leave, his grievances and the automatic price; his value for the club as the cap"),
         ('transfer_screens_build_transfer_status_screen', 'transfer_screens_build_transfer_status_screen', '0x8f6a10', _F, 'cdecl',
          'monta a tela do status de transferência: o texto do contrato (sem contrato, mês a mês, ou o valor), o preço pedido com "-", "+" (até 30 milhões) e Automático, os sete status no elenco (os não permitidos em cinza), a lista de transferências, de empréstimos e a pedido do jogador, e os botões Voltar e Definir; sem contrato, a caixa de erro (linha 0x1d15)',
          'builds the transfer status screen: the contract text (out of contract, month to month, or the value), the asking price with "-", "+" (up to 30 million) and Auto, the seven squad statuses (those not allowed greyed), the transfer list, the loan list and listed by request, and the Back and Set buttons; without a contract, the error box (line 0x1d15)'),
         ('transfer_screens_handle_transfer_status_screen', 'transfer_screens_handle_transfer_status_screen', '0x8f7640', _F, 'cdecl',
          'responde à tela do status de transferência: os botões mudam os parâmetros (automático, preço, status no elenco, as três listas, -11); "-" e "+" mudam o preço e os botões no lugar (0); Definir grava na notícia, no status do jogador, nas listas de transferência e empréstimo e nos bits do contrato (-9)',
          "answers the transfer status screen: the buttons change the params (automatic, price, squad status, the three lists, -11); \"-\" and \"+\" change the price and the widgets in place (0); Set writes the news item, the player's squad status, the transfer and loan lists and the contract's bits (-9)")])
