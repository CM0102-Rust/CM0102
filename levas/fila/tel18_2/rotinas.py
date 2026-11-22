# The transfers screen (transfer_screens.cpp, part 2; reader tel18, the screens round, 21/11/2026). Equal in
# cm0102-1080.exe. Needs tel17_1 (TS_COLOUR_POPUP), tel17_2 (ts_* helpers), tel26_1 (display helpers, JOB_COLOUR_*).
# Conventions checked in the disassembly and at the call sites:
# - 0x8f4fa0 open_transfers_screen cdecl() ret 0 (open_screen flag 0 with the leave routine 0x8f6690);
# - 0x8f5080 build_transfers_screen cdecl() ret 0, SEH frame (a Cash temporary); 0x8f5f40 handle cdecl(item) -> int;
#   0x8f6690 close cdecl() ret 0 (the leave routine).
# Callees: get_past_transfers 0x8cc030 (thiscall on the transfer manager, 6: month, year, month, year, &list, &count;
# the months go on to Date::Date as bytes), date_get_month_name 0x54d3b0 (cdecl, the month as a byte),
# background_set_nation_picture 0x41c100 (cdecl, 3), Date::get_short_weekday_date_string_no_year 0x54b550 (thiscall,
# 3), gui_add_page_buttons 0x5ede20 (cdecl, 8), NATION_MENU_SPLIT 0x66a280/0x66a350/0x66a360/0x66a430 as in tel26_3.
# The original pushes the month, the row count and the colours as bytes or words over leftover high bytes (the
# callees read only the byte or the word; Area::init keeps the colour as a word): the copy pushes them zero-extended.
# Comparers: the build side by side over the display list and the params once the list exists (only the original
# while it is malloc'd); the answer side by side for the page buttons and, with no list to free, for the period and
# nation buttons; the opener, the close and the menu bar only the original.
TELAS = True
ROTINAS = [('transfer_screens_open_transfers_screen', 0x8f4fa0, [0x6a, 0x00, 0x68, 0x90, 0x66, 0x8f, 0x00]),
           ('transfer_screens_build_transfers_screen', 0x8f5080, [0x6a, 0xff, 0x68, 0x26, 0x4e, 0x96, 0x00]),
           ('transfer_screens_handle_transfers_screen', 0x8f5f40, [0x83, 0xec, 0x10, 0x53, 0x55]),
           ('transfer_screens_close_transfers_screen', 0x8f6690, [0x6a, 0x04, 0xb9, 0x50, 0x44, 0xb7, 0x00])]
_F = 'transfer_screens.cpp'
DOCS = ('a tela de transferências', 'the transfers screen',
        [('transfer_screens_open_transfers_screen', 'transfer_screens_open_transfers_screen', '0x8f4fa0', _F, 'cdecl',
          'abre a tela de transferências no mês e no ano de hoje, todas as nações, por mês, página 1',
          "opens the transfers screen on today's month and year, all nations, by month, page 1"),
         ('transfer_screens_build_transfers_screen', 'transfer_screens_build_transfers_screen', '0x8f5080', _F, 'cdecl',
          'monta a tela de transferências: o menu do período (mês ou ano) e as setas, o menu das nações (todas ou uma das 35 nações principais com liga, em duas partes com cabeçalho), a lista das transferências passadas do período (sem lista, pede ao gerente de transferências, tira as que não envolvem a nação e ordena pela taxa) em páginas: data, jogador, clube vendedor, comprador e taxa (ou troca, empréstimo, Bosman, livre); sem nenhuma, "Nenhuma transferência aconteceu."',
          "builds the transfers screen: the period menu (month or year) and the arrows, the nations menu (all or one of the 35 key nations with a league, in two parts with headers), the list of the period's past transfers (without one, asks the transfer manager, drops those not involving the nation and sorts by fee) in pages: date, player, seller, buyer and fee (or exchange, loan, Bosman, free); with none, \"No transfers have taken place.\""),
         ('transfer_screens_handle_transfers_screen', 'transfer_screens_handle_transfers_screen', '0x8f5f40', _F, 'cdecl',
          'responde à tela de transferências: guarda a rolagem da tabela; mês ou ano, mês ou ano anterior e seguinte (entre o início do jogo e hoje) e a nação refazem a lista (página 1, -11); os botões de página andam 1 ou 10 páginas; o resto vai para a barra de menus',
          "answers the transfers screen: keeps the table's scroll; month or year, previous and next month or year (between the game's start and today) and the nation rebuild the list (page 1, -11); the page buttons move 1 or 10 pages; the rest goes to the menu bar"),
         ('transfer_screens_close_transfers_screen', 'transfer_screens_close_transfers_screen', '0x8f6690', _F, 'cdecl',
          'ao sair da tela de transferências, libera a lista',
          'on leaving the transfers screen, frees the list')])
