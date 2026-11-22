# O texto quebrado em linhas (comp_text.cpp; leitor tel31, rodada das telas, 21/11/2026): Text::draw_wrapped, a quebra
# de linha que ~42 rotinas de tela chamam (notícias, perfis, caixas de texto).
# Convenção conferida na desmontagem (0x4aeaf0..0x4afa2f): thiscall, ecx = o Text (+0 o texto, +4 a lista dos links),
# ret 0x20 (área, x1, x2, ponteiro da linha, fonte, cor, flags, máximo de linhas no byte baixo); devolve 1 (0 sem
# texto). Chama font_text_width 0x5e5ad0 cdecl(fonte, texto) (só a palavra baixa da fonte), strstr 0x945a60,
# LLIST::get_next 0x68d1b0 thiscall e Display::add_item 0x55f990 (o Display do humano atual, [0xb74c4c +
# [0xb77c7a] * 0x300]) por endereço; strchr 0x946540 e os sprintf "%c" viram código (puros).
# 1080: o cm0102-1080.exe troca só o destino de duas chamadas a font_text_width (0x4af58b e 0x4af66f, as que medem
# onde um link começa e acaba) por um trecho próprio que escala a largura (0x602174): a cópia chama o destino lido do
# exe em execução (o rel32 em 0x4af58c e 0x4af670); o prólogo e o resto da rotina são iguais nos dois exes.
# Usa os auxiliares do tel26_1 (tel26_add_item, tel26_error_box, exe_immediate, display_list, display_item_at,
# display_area_at) e do tel29_1 (copy_c_string, c_string_len): entra depois dos dois; se um sair, este sai.
# Defeitos do original: o tamanho do texto é tomado antes de testar o ponteiro nulo (texto nulo derruba o jogo); o
# depósito da palavra que passa para a linha seguinte não é zerado antes do primeiro uso (o primeiro corte sem espaço
# nem código de cor começa a linha seguinte com lixo da pilha; a cópia começa vazia); mais de 50 linhas passam das
# tabelas de 50 posições na pilha (a cópia entrega esse texto à original).
TELAS = True
ROTINAS = [('text_draw_wrapped', 0x4aeaf0, [0x55, 0x8b, 0xec, 0x81, 0xec, 0x3c, 0x0a, 0x00, 0x00])]
DOCS = ('o texto quebrado em linhas: a quebra de linha das telas',
        'the wrapped text: the line breaking of the screens',
        [('text_draw_wrapped', 'Text::draw_wrapped', '0x4aeaf0', 'comp_text.cpp', 'thiscall, ret 0x20',
          'quebra o texto entre duas colunas (corta no último espaço ou antes do último código de cor #xxxx#; quebra de linha vira espaço e fecha a linha; espaços antes de "?" viram "|"; com máximo de linhas e linhas demais, a coluna direita recua 22 pixels e quebra de novo, uma vez), põe cada linha como item de texto na lista de exibição da tela do humano (centrada com a flag 1, só a primeira com a flag 2), cada link como itens de link sobre os caracteres que cobre, e avança a linha',
          'breaks the text between two columns (cut at the last space or before the last #xxxx# colour code; a line feed becomes a space and closes the line; spaces before "?" become "|"; with a line limit and too many lines, the right column moves 22 pixels left and it breaks again, once), adds each line as a text item to the display list of the human\'s screen (centred with flag 1, only the first with flag 2), each link as link items over the characters it covers, and moves the row on')])
