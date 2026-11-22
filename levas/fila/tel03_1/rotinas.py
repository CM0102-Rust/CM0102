# The small accessors of club_screens.cpp (reader tel03, the screens round, 21/11/2026): the cash setter, the four
# finance blocks, the four widget readers of the screen manager and the area scroll row.
# - 0x4671a0 Cash::set thiscall(amount) ret 4; 0x4671b0/0x4671c0/0x4671d0/0x4671e0 Finance::get_last_month,
#   get_this_month, get_last_year, get_this_year thiscall ret 0 (4- and 7-byte routines; the hook covers the alignment
#   nops); 0x468b10 Display::get_widget_id, 0x468b40 ScreenManager::get_widget_action_id, 0x468b90
#   get_widget_action_data, 0x468bd0 get_widget_drop_target, thiscall(short widget) ret 4 (the callers push the widget
#   as a dword, the routines read the word); 0x46ad90 Display::set_area_scroll_row thiscall(short area, int row) ret 8,
#   calling 0x403320 Area::set_scroll_row thiscall(row) ret 4.
# Callers: the finance getters and the cash setter only from the club info tab's income view (the tail of 0x460590);
#   the widget readers from every screen's input handler (58, 72, 84 and 4 call sites).
# Comparers: the return value (compare_value!) or the written dword side by side.
# How to reach them: any screen click (the widget readers); the club screen's Info tab, income view (finance); the
#   club history screen's tabs (scroll row).
TELAS = True
ROTINAS = [('cash_set', 0x4671a0, [0x8b, 0x44, 0x24, 0x04, 0x89, 0x01]),
           ('finance_get_last_month', 0x4671b0, [0x8d, 0x41, 0x20, 0xc3, 0x90]),
           ('finance_get_this_month', 0x4671c0, [0x8d, 0x41, 0x70, 0xc3, 0x90]),
           ('finance_get_last_year', 0x4671d0, [0x8d, 0x81, 0xc0, 0x00, 0x00, 0x00]),
           ('finance_get_this_year', 0x4671e0, [0x8d, 0x81, 0x10, 0x01, 0x00, 0x00]),
           ('display_get_widget_id', 0x468b10, [0x66, 0x8b, 0x44, 0x24, 0x04]),
           ('screenmanager_get_widget_action_id', 0x468b40, [0x66, 0x8b, 0x54, 0x24, 0x04]),
           ('screenmanager_get_widget_action_data', 0x468b90, [0x66, 0x8b, 0x54, 0x24, 0x04]),
           ('screenmanager_get_widget_drop_target', 0x468bd0, [0x66, 0x8b, 0x54, 0x24, 0x04]),
           ('display_set_area_scroll_row', 0x46ad90, [0x66, 0x8b, 0x44, 0x24, 0x04])]
DOCS = ('os pequenos acessos das telas do clube (caixa, blocos das finanças, itens da tela, rolagem)',
        'the small accessors of the club screens (cash, finance blocks, widgets, area scroll)',
        [('cash_set', 'cash_set', '0x4671a0', 'club_screens.cpp', 'thiscall',
          'Grava uma quantia de dinheiro: o dword em this recebe o valor.',
          'Stores a money amount: the dword at this gets the value.'),
         ('finance_get_last_month', 'finance_get_last_month', '0x4671b0', 'club_screens.cpp', 'thiscall',
          'Devolve o bloco do mês passado das finanças do clube (+0x20, 0x50 bytes).',
          'Returns the last month\'s block of the club finances (+0x20, 0x50 bytes).'),
         ('finance_get_this_month', 'finance_get_this_month', '0x4671c0', 'club_screens.cpp', 'thiscall',
          'Devolve o bloco do mês corrente das finanças do clube (+0x70).',
          'Returns the current month\'s block of the club finances (+0x70).'),
         ('finance_get_last_year', 'finance_get_last_year', '0x4671d0', 'club_screens.cpp', 'thiscall',
          'Devolve o bloco da temporada passada das finanças do clube (+0xc0).',
          'Returns the last season\'s block of the club finances (+0xc0).'),
         ('finance_get_this_year', 'finance_get_this_year', '0x4671e0', 'club_screens.cpp', 'thiscall',
          'Devolve o bloco da temporada corrente das finanças do clube (+0x110).',
          'Returns the current season\'s block of the club finances (+0x110).'),
         ('display_get_widget_id', 'display_get_widget_id', '0x468b10', 'club_screens.cpp', 'thiscall',
          'Devolve o dword do id de ação do item pedido da tela (o id na palavra baixa), ou 0 para o item −1.',
          'Returns the dword of the action id of the given item of the display (the id in the low word), or 0 for item −1.'),
         ('screenmanager_get_widget_action_id', 'screenmanager_get_widget_action_id', '0x468b40', 'club_screens.cpp', 'thiscall',
          'Devolve o id de ação (com sinal) do item clicado na tela do turno corrente; para o item −1 ou um item de id 0, o id de ação injetado do gerenciador de telas.',
          'Returns the action id (signed) of the clicked item on the current slot\'s display; for item −1 or an item whose id is 0, the screen manager\'s injected action id.'),
         ('screenmanager_get_widget_action_data', 'screenmanager_get_widget_action_data', '0x468b90', 'club_screens.cpp', 'thiscall',
          'Devolve o valor ligado ao item clicado na tela do turno corrente, ou 0 para o item −1.',
          'Returns the value attached to the clicked item on the current slot\'s display, or 0 for item −1.'),
         ('screenmanager_get_widget_drop_target', 'screenmanager_get_widget_drop_target', '0x468bd0', 'club_screens.cpp', 'thiscall',
          'Devolve o item onde um arrastar soltou o item pedido (palavra em +0x7e do item), ou −1 para o item −1.',
          'Returns the item a drag dropped the given item on (the word at item +0x7e), or −1 for item −1.'),
         ('display_set_area_scroll_row', 'display_set_area_scroll_row', '0x46ad90', 'club_screens.cpp', 'thiscall',
          'Põe a linha de rolagem de uma área da tela (Area::set_scroll_row); nada para a área −1.',
          'Sets the scroll row of an area of the display (Area::set_scroll_row); nothing for area −1.')])
