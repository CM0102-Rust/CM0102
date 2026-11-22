# As telas do clube (1): as datas e os repassadores do gerente de telas que moram em club_screens.cpp, e a
# disponibilidade de um jogador para a partida. Leitor tel01 (rodada das telas), 21/11/2026.
# Convencoes conferidas na desmontagem: Date::operator> / operator< thiscall, ret 8 (a outra data por valor, dois
# dwords; o ano comparado sem sinal, o dia com sinal); set_area_scroll_row thiscall ret 8; add_area thiscall ret 0x2c
# (11 dwords repassados ao Display::add_area 0x55fba0); add_popup_area thiscall ret 4; add_ticked_popup_item thiscall
# ret 0x10 (18 dwords ao Display::add_item 0x55f990); add_item thiscall ret 0x48 (18 dwords repassados);
# is_player_available_for_match cdecl, ret 0 (bool em eax).
# Iguais no cm0102-1080.exe (as rotinas inteiras).
# Fora: MenuItem::MenuItem (0x4568f0, 4 bytes: nao cabe o desvio de 5 bytes).
# Comparadores: as datas e a disponibilidade pelo valor; os repassadores lado a lado no que escrevem (o pacote de rede
# a partir de [0xae290c], as contagens de areas e itens do Display atual, a proxima vaga de area e de item, as marcas
# de estouro) e o retorno; add_item e add_ticked_popup_item tambem na ordem dos itens livres e na area do item
# (scrman_item_regions, 22/11/2026: sem isso a copia acrescentava o item de novo); set_area_scroll_row lado a lado
# na area. Usa display_list e display_area_at do tel26_1 (entra antes).
TELAS = True
ROTINAS = [('date_operator_greater', 0x456740, [0x66, 0x8b, 0x41, 0x02, 0x66, 0x8b, 0x54, 0x24, 0x06]),
           ('date_operator_less', 0x456770, [0x66, 0x8b, 0x41, 0x02, 0x66, 0x8b, 0x54, 0x24, 0x06]),
           ('screenmanager_set_area_scroll_row', 0x4567a0, [0x66, 0x8b, 0x54, 0x24, 0x04]),
           ('screenmanager_add_area', 0x4567f0, [0x8b, 0x44, 0x24, 0x2c, 0x8b, 0x54, 0x24, 0x28]),
           ('screenmanager_add_popup_area', 0x456850, [0x8b, 0x44, 0x24, 0x04, 0x66, 0x8b, 0x15, 0xaa, 0x31, 0xae, 0x00]),
           ('screenmanager_add_ticked_popup_item', 0x456890, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x54, 0x24, 0x10]),
           ('screenmanager_add_item', 0x45cb60, [0x8b, 0x44, 0x24, 0x48, 0x8b, 0x54, 0x24, 0x44]),
           ('is_player_available_for_match', 0x45cbe0, [0x53, 0x55, 0x56, 0x8b, 0x74, 0x24, 0x10])]
DOCS = ('as telas do clube (1): as datas, os repassadores do gerente de telas e a disponibilidade de um jogador para a partida',
        'the club screens (1): the dates, the screen manager\'s forwarders and a player\'s availability for a match',
        [('date_operator_greater', 'Date::operator>', '0x456740', 'club_screens.cpp', 'thiscall',
          'uma data é posterior a outra quando o ano é maior (sem sinal) ou, no mesmo ano, quando o dia do ano é maior (com sinal)',
          'a date is later than another when its year is greater (unsigned) or, in the same year, when its day of the year is greater (signed)'),
         ('date_operator_less', 'Date::operator<', '0x456770', 'club_screens.cpp', 'thiscall',
          'uma data é anterior a outra quando o ano é menor (sem sinal) ou, no mesmo ano, quando o dia do ano é menor (com sinal)',
          'a date is earlier than another when its year is smaller (unsigned) or, in the same year, when its day of the year is smaller (signed)'),
         ('screenmanager_set_area_scroll_row', 'ScreenManager::set_area_scroll_row', '0x4567a0', 'club_screens.cpp', 'thiscall',
          'rola uma área da tela atual até uma linha; a área -1 (nenhuma) não faz nada',
          'scrolls an area of the current screen to a row; area -1 (none) does nothing'),
         ('screenmanager_add_area', 'ScreenManager::add_area', '0x4567f0', 'club_screens.cpp', 'thiscall',
          'repassa os onze argumentos de uma área à lista de exibição (Display) da vaga atual e devolve o índice da área',
          'passes an area\'s eleven arguments on to the current slot\'s display list (Display) and returns the area index'),
         ('screenmanager_add_popup_area', 'ScreenManager::add_popup_area', '0x456850', 'club_screens.cpp', 'thiscall',
          'uma área suspensa (a lista que abre de um botão): área vazia com as marcas 0x130 na cor das listas suspensas, dona o botão',
          'a popup area (the list a button opens): an empty area with flags 0x130 in the popup colour, owned by the button'),
         ('screenmanager_add_ticked_popup_item', 'ScreenManager::add_ticked_popup_item', '0x456890', 'club_screens.cpp', 'thiscall',
          'um item marcado (0x2002) de uma lista suspensa, com o texto, o identificador e o dado, nas cores das listas suspensas',
          'a ticked item (0x2002) of a popup list, with its text, id and data, in the popup colours'),
         ('screenmanager_add_item', 'ScreenManager::add_item', '0x45cb60', 'club_screens.cpp', 'thiscall',
          'repassa os dezoito argumentos de um item à lista de exibição (Display) da vaga atual e devolve o índice do item',
          'passes an item\'s eighteen arguments on to the current slot\'s display list (Display) and returns the item index'),
         ('is_player_available_for_match', 'is_player_available_for_match', '0x45cbe0', 'club_screens.cpp', 'cdecl',
          'um jogador pode jogar a partida a menos que esteja machucado, emprestado pelo próprio clube, inelegível, convocado, suspenso na competição ou precisando de permissão de trabalho; sem partida, sempre pode',
          'a player can play the match unless he is injured, on loan from this club, ineligible, away on international duty, banned in the competition or in need of a work permit; with no fixture he always can')])
