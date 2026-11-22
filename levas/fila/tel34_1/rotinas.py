# The Game Credits screen and the Web Sites rows (contract_screens.cpp; reader tel34, the screens round, 21/11/2026:
# screens the community map did not see, passed by pointer). Uses the helpers of tel26_1 (tel26_add_item,
# tel26_add_area, tel26_add_button, tel26_back_next, tel26_item_id/value, tel26_copy_text, display_area_at, JOB_TEXT,
# job_colour) and match_screens (current_human_display, get/set_screen_param, compare_by_display_list_with).
# Conventions checked in the disassembly: every routine cdecl, no arguments, ret 0 (the row builders leave ecx
# unused: Ghidra calls them fastcall), except credits_screen_handle_action cdecl(short item) -> eax.
# Display::add_item 0x55f990 thiscall on the current human's Display (18 dwords, ret 0x48); Display::add_area 0x55fba0
# (11, ret 0x2c); ScreenManager::add_button_item 0x804030 (12); font_text_width 0x5e5ad0 cdecl(3, text);
# font_get_height 0x5e5cd0 cdecl(3); background_set_section_picture 0x41c8e0 cdecl(10, 0); get_game_title 0x839570
# cdecl -> char*; get_heading_colour 0x839590 cdecl(k) (asked 1 then 0); add_screen_title 0x803f30 thiscall(SM,
# colour 0, colour 1, title); langlib_get_language 0x66f4d0 (returns [0x9eb2dc]); Area::set_scroll_row 0x403320
# thiscall(area, row); ScreenManager::get_area_scroll_row 0x803320 thiscall(SM, area); fgetc 0x9469e0, stricmp
# 0x951c50, fopen/fclose. The page 0 sections (0x4e5970, 0x4e6a70, 0x4e6dc0, 0x4e92f0, 0x4eb480, 0x4eb110,
# 0x4eccd0) and 0x4eda90 belong to another group (tel33): called by address.
# The colour words go zero-extended (the original pushes them with whatever the register's high half held; the
# display keeps words). The section helpers are named tel34_credits_* so they do not clash with tel33's (which builds
# the same kind of rows in the same file and could reuse them).
# Defects of the original: web_sites_add_fan_club_links keeps one index register for both buffers, so after a link
# the scan goes on writing the first buffer from the length of the unused rest of the line (kept).
# Left out: shared_noop_ret0_5a53c0 (find_screens.cpp, 1 byte: no room for the 5-byte detour).
TELAS = True
ROTINAS = [('credits_add_additional_testing_rows', 0x50e000, [0x51, 0x66, 0xa1, 0x7c, 0x18, 0xae, 0x00]),
           ('credits_add_special_thanks_rows', 0x511320, [0x51, 0x66, 0xa1, 0x7c, 0x18, 0xae, 0x00]),
           ('credits_add_best_wishes_rows', 0x512d40, [0x51, 0x66, 0xa1, 0x7c, 0x18, 0xae, 0x00]),
           ('credits_screen_build', 0x513180, [0x51, 0x56, 0x33, 0xf6, 0x6a, 0x02]),
           ('credits_screen_handle_action', 0x5133c0, [0x66, 0x81, 0x3d, 0xde, 0x6b, 0xdd, 0x00, 0x2b, 0x04]),
           ('web_sites_screen_open', 0x5134c0, [0x6a, 0x00, 0x6a, 0x00, 0x6a, 0x00]),
           ('web_sites_add_publisher_rows', 0x513500, [0x51, 0x56, 0x57, 0xbf, 0x80, 0x1b, 0x9b, 0x00]),
           ('web_sites_add_fan_club_links', 0x513750, [0x81, 0xec, 0x04, 0x02, 0x00, 0x00])]
DOCS = ('os créditos do jogo e os sites (1): as seções próprias dos créditos, a tela dos créditos e as linhas da editora e dos sites de torcedores',
        'the game credits and the web sites (1): the credits\' own sections, the credits screen and the publisher and fan club rows',
        [('credits_add_additional_testing_rows', 'credits_add_additional_testing_rows', '0x50e000', 'contract_screens.cpp', 'cdecl',
          'a seção Additional Testing dos créditos: o título (com a entrada no menu View) e os 56 testadores externos em duas colunas; fora da página mostrada só os contadores andam',
          'the Additional Testing section of the credits: the heading (with its View menu entry) and the 56 external testers in two columns; off the page shown only the counters move'),
         ('credits_add_special_thanks_rows', 'credits_add_special_thanks_rows', '0x511320', 'contract_screens.cpp', 'cdecl',
          'a seção Special Thanks: o título e as linhas nome/função; 17 delas só em inglês; o nome da bola de espuma e "todo fã do CM" no idioma do jogo (francês, italiano, espanhol, dinamarquês, português, sueco ou inglês)',
          'the Special Thanks section: the heading and the name/role rows; 17 of them in English only; the sponge ball and "every CM fan" names in the game\'s language (French, Italian, Spanish, Danish, Portuguese, Swedish or English)'),
         ('credits_add_best_wishes_rows', 'credits_add_best_wishes_rows', '0x512d40', 'contract_screens.cpp', 'cdecl',
          'a seção Best Wishes To: o título (sem entrada no menu View) e os três casais "On getting married"',
          'the Best Wishes To section: the heading (no View menu entry) and the three couples "On getting married"'),
         ('credits_screen_build', 'credits_screen_build', '0x513180', 'contract_screens.cpp', 'cdecl',
          'monta a tela dos créditos: título, menu, Voltar/Avançar, o botão View com a lista das seções, a lista rolável com as seções da página 0 e da página 1 (mostra a página do parâmetro 2), rolada até a linha do parâmetro 0',
          'builds the credits screen: title, menu bar, Back/Next, the View button with the section list, the scrolling list with the page 0 and page 1 sections (showing the page of parameter 2), scrolled to the row of parameter 0'),
         ('credits_screen_handle_action', 'credits_screen_handle_action', '0x5133c0', 'contract_screens.cpp', 'cdecl',
          'guarda a rolagem da lista; uma entrada do menu View (id 100) mostra a página e a linha da seção (-11, remonta); o resto vai para a barra de menu',
          'keeps the list scroll; a View menu entry (id 100) shows the section\'s page and line (-11, rebuild); anything else goes to the menu bar'),
         ('web_sites_screen_open', 'web_sites_screen_open', '0x5134c0', 'contract_screens.cpp', 'cdecl',
          'abre a tela dos sites e zera os parâmetros 0 e 1',
          'opens the web sites screen and clears parameters 0 and 1'),
         ('web_sites_add_publisher_rows', 'web_sites_add_publisher_rows', '0x513500', 'contract_screens.cpp', 'cdecl',
          'as linhas da editora: o título Eidos Interactive, "The publishers" com www.eidos.co.uk e uma linha vazia',
          'the publisher rows: the Eidos Interactive heading, "The publishers" with www.eidos.co.uk and an empty row'),
         ('web_sites_add_fan_club_links', 'web_sites_add_fan_club_links', '0x513750', 'contract_screens.cpp', 'cdecl',
          'lê CM3_DATA\\fancluburls.txt: se existe, uma linha vazia, o título Unofficial Sites e uma linha por endereço "http://" do arquivo (o resto da linha é lido e descartado)',
          'reads CM3_DATA\\fancluburls.txt: when it exists, an empty row, the Unofficial Sites heading and one row per "http://" address in the file (the rest of the line is read and dropped)')])
