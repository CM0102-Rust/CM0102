# The transfer regulations information screen (new_transfer_rule_screens.cpp; reader tel36, the screens round,
# 21/11/2026). The opener 0x77b700 is not in the group (it stays with the exe). Uses tel36_1 (tel36_langlib), tel26_1
# (tel26_text_area, tel26_back_next, tel26_item_id, tel26_item_value, job_colour) and tel26_4 (manager_menu_bar).
# Conventions checked in the disassembly: the builder cdecl(void) with an SEH frame: four MenuItems (0xd4 bytes) built
# by the CRT "eh vector constructor iterator" 0x9463af stdcall(array, 0xd4, 4, 0x4568f0, 0x5a53c0) and destroyed by
# 0x9462bb stdcall(array, 0xd4, 4, 0x5a53c0); gui_add_tab_bar 0x5ed780 cdecl(tabs, 4, tab, &top, &bottom, 1);
# ScreenManager::is_forward_unavailable / is_back_unavailable(0) and gui_add_back_next_buttons(back, next) as
# tel26_back_next; the handler cdecl(short) -> eax.
# Equal in cm0102-1080.exe.
# Comparers: the builder side by side over the display list; the handler's tab choice side by side over the screen
# params, the menu bar only the original.
TELAS = True
ROTINAS = [('transfer_rules_info_screen_display', 0x77b850, [0x6a, 0xff, 0x68, 0xd9, 0xe0, 0x95, 0x00]),
           ('transfer_rules_info_screen_handle_click', 0x77bb30, [0x66, 0x8b, 0x15, 0x7a, 0x7c, 0xb7, 0x00])]
DOCS = ('a tela de informação das regras de transferência',
        'the transfer regulations information screen',
        [('transfer_rules_info_screen_display', 'transfer_rules_info_screen_display', '0x77b850', 'new_transfer_rule_screens.cpp', 'cdecl',
          'monta a tela das regras (atuais ou propostas): o título com o clube do humano (sem clube, "Information" nas cores do clube da chave fixa do Brighton), as quatro abas (visão geral, janelas de transferência, jovens, proteção de contrato) e o texto da aba escolhida (a visão geral muda entre atual e proposta; a aba 2 no idioma 0 usa o texto C puro), e os botões Voltar / Avançar',
          'builds the regulations screen (current or proposed): the title with the human\'s club (with no club, "Information" in the colours of the club of Brighton\'s fixed key), the four tabs (overview, transfer windows, young players, contract protection) and the chosen tab\'s text (the overview differs between current and proposed; tab 2 in language 0 uses the plain C text), and the Back / Next buttons'),
         ('transfer_rules_info_screen_handle_click', 'transfer_rules_info_screen_handle_click', '0x77bb30', 'new_transfer_rule_screens.cpp', 'cdecl',
          'uma aba clicada (id 1) vira a escolhida (parâmetro 0) e a tela é remontada (-11); o resto vai para a barra de menus',
          'a clicked tab (id 1) becomes the chosen one (parameter 0) and the screen is rebuilt (-11); the rest goes to the menu bar')])
