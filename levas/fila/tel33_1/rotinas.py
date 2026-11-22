# The credits screen (contract_screens.cpp, part of it): the Additional Research section. Reader tel33 (the screens
# round), 21/11/2026; called by credits_screen_build (0x513180) among the page 1 sections.
# 0x4eda90..0x50dffa (132458 bytes, one function: 41330 instructions, no padding inside): cdecl with no arguments
# (push ecx as the only local, ret). Calls: langlib_sprintf 0x66f4e0 cdecl (destination, format); get_screen_param
# 0x7ff010 thiscall ret 4; font_text_width 0x5e5ad0 cdecl (font, text); Display::add_item 0x55f990 thiscall ret 0x48
# (6 sites inline) and ScreenManager::add_item 0x45cb60 (696 sites; the same item on the current slot's Display).
# Shape: the title, the heading, then 698 copies of the two-column name code (one per name, the texts by address) and
# a closing half-row step: a loop over a table of the 698 name texts with the tel32_1 helpers.
# Proof without the game (scratchpad extractor credits.py): the names read from the disassembly (each pushed once per
# column branch, checked pairwise), the model run against the original emulated with unicorn, 300 random states
# (page shown or not, item = parameter 0 or not, column/item/row moved after the adds to drive every branch, two
# register-garbage seeds each): the same calls with the same arguments and the same final state (0xae1868..0xae187f,
# 0xde1f64), and all 41330 instructions run. The word fields 8, 9, 11, 12 and 17 are zero-extended (as tel32_1;
# the original leaves known high halves in two code variants, names 2 and 3 and the rest; Guio::init keeps the word).
# Equal in the cm0102-1080.exe (the routine whole).
# Depends on tel32_1 (tel32_heading, tel32_name_columns, tel32_next_item, tel32_bump_row, tel32_credits) and through it
# on tel26_1, tel01_1 and tel22_1 (comp_lang_format).
TELAS = True
ROTINAS = [('credits_add_additional_research_rows', 0x4eda90, [0x51, 0x68, 0x1c, 0x4b, 0x9b, 0x00])]
DOCS = ('a tela dos créditos (3): a seção de pesquisa adicional',
        'the credits screen (3): the Additional Research section',
        [('credits_add_additional_research_rows', 'credits_add_additional_research_rows', '0x4eda90', 'contract_screens.cpp', 'cdecl',
          'o título "Additional Research" (traduzido; marcado quando o número do item é o parâmetro 0 da tela) e os 698 nomes dos pesquisadores em duas colunas, sem linha vazia antes; só com a página 1 na tela os itens entram (com a outra só se contam as linhas, uma por nome); no fim, uma linha pela metade (a coluna da direita vazia) fecha a linha; o original é um trecho corrido de 132 KB com o mesmo código de linha repetido por nome',
          'the "Additional Research" heading (translated; highlighted when the item number is screen parameter 0) and the 698 researchers\' names in two columns, with no empty row first; only with page 1 on screen are the items added (with the other page the rows are only counted, one per name); at the end a half row (the right column empty) is closed; the original is one straight 132 KB run of the same row code repeated per name')])
