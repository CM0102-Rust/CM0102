# The credits screen (contract_screens.cpp, part of it): opening it and the Sports Interactive, Jask/Xbox and
# Research sections. Reader tel33 (the screens round), 21/11/2026; the routines the tel31 reader left out.
# Bounds from the disassembly (and varredura_telas/telas_confirmadas.tsv): 0x4e5920..0x4e596f (79 bytes, opener;
# called by 0x7615d0 and 0x81bd70), 0x4e5970..0x4e6a66 (4342), 0x4e6a70..0x4e6dbb (843), 0x4e6dc0..0x4e92ef (9519);
# the three sections are called by credits_screen_build (0x513180) among the page 0 sections.
# Conventions checked in the disassembly: all cdecl with no arguments, ret 0 (the sections push ecx as the only
# local); ScreenManager::open_screen 0x7fe600 thiscall ret 0x14 (build 0x513180, input 0x5133c0, flag 0, leave 0,
# exit 0); set_screen_param 0x7ff270 thiscall ret 0xc (index, value, copy); get_screen_param 0x7ff010 thiscall ret 4;
# Display::add_item 0x55f990 thiscall ret 0x48; font_text_width 0x5e5ad0 cdecl; langlib_sprintf 0x66f4e0 cdecl
# (destination, format, and once the empty text 0xad9c64 as argument); 0x4e5970 copies its heading with an inline
# strcpy.
# Proof without the game (scratchpad extractor credits.py, the same as tel33_1): the op lists (texts and row patterns)
# read from the disassembly and from one emulated run, checked against the original emulated with unicorn, 300 random
# states each (page shown or not, item = parameter 0 or not, column/item/row moved after the adds, two register-
# garbage seeds; open_screen returning 0 or not): the same calls with the same arguments and the same final state
# (0xae1868..0xae187f, 0xde1f64), every instruction run, and every string the routine pushes in the op list.
# Equal in the cm0102-1080.exe (the four routines whole).
# Depends on tel32_1 (tel32_heading, tel32_blank_row, tel32_role_row, tel32_translated_role_row, tel32_cell,
# tel32_shown, tel32_next_item, tel32_bump_row, tel32_credits, TEL32_COLOUR_BDDE, TEL32_NO_TEXT) and through it on
# tel26_1, tel01_1 and tel22_1; and on tel29_1 (current_screen_node, screen_node_fields: the opener's comparer).
TELAS = True
ROTINAS = [('credits_screen_open', 0x4e5920, [0x6a, 0x00, 0x6a, 0x00, 0x6a, 0x00]),
           ('credits_add_sports_interactive_rows', 0x4e5970, [0x51, 0x56, 0x57, 0xbf, 0x40, 0x12, 0x9b, 0x00]),
           ('credits_add_jask_xbox_rows', 0x4e6a70, [0x51, 0x66, 0xa1, 0x7c, 0x18, 0xae, 0x00]),
           ('credits_add_research_rows', 0x4e6dc0, [0x51, 0x66, 0xa1, 0x7c, 0x18, 0xae, 0x00])]
DOCS = ('a tela dos créditos (2): abrir a tela e as seções da Sports Interactive, da Jask/Xbox e da pesquisa',
        'the credits screen (2): opening the screen and the Sports Interactive, Jask/Xbox and Research sections',
        [('credits_screen_open', 'credits_screen_open', '0x4e5920', 'contract_screens.cpp', 'cdecl',
          'abre a tela dos créditos (montar 0x513180, responder 0x5133c0) e, se abriu, zera os parâmetros 0, 1 e 2 da tela (começa na primeira página)',
          'opens the credits screen (build 0x513180, input 0x5133c0) and, when it opened, sets screen parameters 0, 1 and 2 to 0 (it starts on the first page)'),
         ('credits_add_sports_interactive_rows', 'credits_add_sports_interactive_rows', '0x4e5970', 'contract_screens.cpp', 'cdecl',
          'o título "Sports Interactive" (copiado, sem tradução, sem linha vazia antes) e 17 linhas de nome e função traduzida: os dois presidentes, o diretor-geral (Miles), oito de projeto e programação (Grant Appleyard com o nome também traduzido, num segundo texto) e seis outras',
          'the "Sports Interactive" heading (copied, not translated, with no empty row first) and 17 name and translated role rows: the two chairmen, the managing director (Miles), eight design and programming rows (Grant Appleyard with his name translated too, into a second text) and six others'),
         ('credits_add_jask_xbox_rows', 'credits_add_jask_xbox_rows', '0x4e6a70', 'contract_screens.cpp', 'cdecl',
          'uma linha vazia, o título "Jask Consultants" (traduzido) e a linha de projeto e programação do Xbox (Simon Page)',
          'an empty row, the "Jask Consultants" heading (translated) and the Xbox design and programming row (Simon Page)'),
         ('credits_add_research_rows', 'credits_add_research_rows', '0x4e6dc0', 'contract_screens.cpp', 'cdecl',
          'uma linha vazia, o título "Research" (traduzido) e 40 linhas de pesquisador com a nação (traduzida) como função, da Argentina à Iugoslávia (o da Coreia do Sul com o nome também traduzido, num segundo texto)',
          'an empty row, the "Research" heading (translated) and 40 researcher rows with the nation (translated) as the role, from Argentina to Yugoslavia (the South Korean one with his name translated too, into a second text)')])
