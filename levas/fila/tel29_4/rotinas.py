# A tela do regulamento de transferências (new_transfer_rule_screens.cpp; leitor tel29, rodada das telas,
# 21/11/2026): abrir a tela a partir de uma notícia (o botão "Details" da notícia de regras novas, nma_8).
# Convenção conferida na desmontagem (0x77b700..0x77b845): cdecl (técnico ou 0, id da notícia), quadro SEH só para a
# NewsItem local (0xe0 bytes; NewsItem::NewsItem 0x77bbe0 thiscall ret 8); ScreenManager::open_screen (0x7fe600) com
# 0x77b850/0x77bb30; News::find_item_by_id (0x786b70, thiscall, ret 0xc; já Rust); NewsItem::get_slot (0x785830,
# thiscall, ret 4); ScreenManager::set_screen_param (0x7ff270) 0 = 1 e 1 = a vaga 0 da notícia.
# Comparador: só a original (abre a tela), conferindo que a tela nova é esta e o parâmetro 0 (a página) é 1.
TELAS = True
ROTINAS = [('transfer_rules_info_screen_open', 0x77b700, [0x6a, 0xff, 0x68, 0xab, 0xe0, 0x95, 0x00])]
DOCS = ('a tela do regulamento de transferências: abrir a partir de uma notícia',
        'the transfer regulations screen: opening it from a news item',
        [('transfer_rules_info_screen_open', 'transfer_rules_info_screen_open', '0x77b700', 'new_transfer_rule_screens.cpp', 'cdecl',
          'abre a tela de informação do regulamento de transferências na primeira página e, vinda de uma notícia do técnico, passa a vaga 0 da notícia como segundo parâmetro (notícia não achada: a caixa de erro)',
          "opens the transfer regulations information screen at its first page and, from one of the manager's news items, passes the item's slot 0 as the second parameter (item not found: the error box)")])
