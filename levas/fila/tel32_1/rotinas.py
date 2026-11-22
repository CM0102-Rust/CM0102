# The credits screen (contract_screens.cpp, part of it): the Testing, Feral, Eidos and Translation sections. Reader
# tel32 (the screens round), 21/11/2026; the four are called by credits_screen_build (0x513180) among the page 0 sections.
# Conventions checked in the disassembly: the four builders cdecl with no arguments (push ecx as the only local, ret);
# Display::add_item 0x55f990 thiscall (current slot's Display, 18 dwords, ret 0x48); get_screen_param 0x7ff010
# thiscall ret 4; font_text_width 0x5e5ad0 cdecl (font, text); langlib_sprintf 0x66f4e0 and crt_sprintf 0x944c9f
# cdecl (destination, format); langlib_get_language 0x66f4d0 cdecl.
# Proof without the game: the routines are straight line code over four row patterns; the op list (texts and patterns)
# was read from the disassembly and the model checked against the original code emulated with unicorn (200 random
# states per routine: pages, item numbers, column, parameter 0, language): the same add_item calls (fields, text) and
# the same final state. The high halves the original leaves in the registers it pushes for word fields (fields 8, 9,
# 11, 12 and 17: Guio::init keeps only the word) are zero in the copy.
# Equal in the cm0102-1080.exe (the routines whole).
# Depends on tel26_1 (tel26_add_item, tel26_copy_text, job_colour, JOB_TEXT, JOB_COLOUR_*), tel01_1
# (screens_current_display) and tel22_1 (comp_lang_format, comp_crt_format).
# Quirk of the original (kept): in a two-column name list the column only flips on the shown page, so the hidden page
# moves the row counter once per name and the shown one once per two names (the row values of the hidden page's
# headings then do not match the rows it has when shown [inference]).
TELAS = True
ROTINAS = [('credits_add_testing_rows', 0x4e92f0, [0x51, 0x66, 0xa1, 0x7c, 0x18, 0xae, 0x00]),
           ('credits_add_feral_rows', 0x4eb110, [0x51, 0x66, 0xa1, 0x7c, 0x18, 0xae, 0x00]),
           ('credits_add_eidos_rows', 0x4eb480, [0x51, 0x66, 0xa1, 0x7c, 0x18, 0xae, 0x00]),
           ('credits_add_translation_rows', 0x4eccd0, [0x51, 0x66, 0xa1, 0x7c, 0x18, 0xae, 0x00])]
DOCS = ('a tela dos créditos (1): as seções de testes, da Feral, da Eidos e das traduções',
        'the credits screen (1): the Testing, Feral, Eidos and Translation sections',
        [('credits_add_testing_rows', 'credits_add_testing_rows', '0x4e92f0', 'contract_screens.cpp', 'cdecl',
          'uma linha vazia, o título "Testing" (traduzido; marcado quando o número do item é o parâmetro 0 da tela) e os 32 nomes dos testadores em duas colunas; só com a página 0 na tela os itens das linhas entram, com a outra página só se contam as linhas (aí a coluna não vira e cada nome conta uma linha)',
          "an empty row, the \"Testing\" heading (translated; highlighted when the item number is screen parameter 0) and the 32 testers' names in two columns; only with page 0 on screen are the row items added, with the other page the rows are only counted (there the column does not flip and every name counts a row)"),
         ('credits_add_feral_rows', 'credits_add_feral_rows', '0x4eb110', 'contract_screens.cpp', 'cdecl',
          'uma linha vazia, o título "Feral Interactive" (copiado, sem tradução) e a linha do produtor (nome à esquerda, função traduzida à direita)',
          'an empty row, the "Feral Interactive" heading (copied, not translated) and the producer row (name on the left, translated role on the right)'),
         ('credits_add_eidos_rows', 'credits_add_eidos_rows', '0x4eb480', 'contract_screens.cpp', 'cdecl',
          'uma linha vazia, o título "Eidos Interactive" (copiado), 24 linhas de nome e função (traduzida) e, só com o jogo em inglês, o agradecimento pelas fotos do Man City (sem tradução)',
          'an empty row, the "Eidos Interactive" heading (copied), 24 name and role rows (role translated) and, with the game in English only, the Man City photos thanks row (not translated)'),
         ('credits_add_translation_rows', 'credits_add_translation_rows', '0x4eccd0', 'contract_screens.cpp', 'cdecl',
          'uma linha vazia, o título "Translation and Proof-Reading" (traduzido) e uma linha por tradutor com a língua (traduzida) como função',
          'an empty row, the "Translation and Proof-Reading" heading (translated) and one row per translator with the language (translated) as the role')])
