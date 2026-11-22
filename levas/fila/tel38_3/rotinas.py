# A base da interface (3): os botoes Voltar/Avancar e de pagina (gui_utils.cpp) e o objeto de widget (Guio.cpp): os
# campos, o destaque, o texto, as imagens e o formato das cores. Leitor tel38, rodada das telas, 21/11/2026.
# Usa os auxiliares do tel01_1 (screens_current_display, screens_forwarder_regions, DisplayAddAreaFn), do tel26_1
# (display_list, display_item, display_area, display_area_at, display_item_at), do tel29_1 (copy_c_string,
# c_string_len) e o SCREEN_MANAGER do motor; chama os auxiliares do tel38_1 pelo endereco (FN_SCREENMANAGER_*).
# Convencoes conferidas na desmontagem: gui_add_back_next_buttons cdecl(back, next) -> 1 (quadro de 0x7d4: os bytes
# {1, 1, 3, 1} e o texto em +4; a area recebe o ponteiro +2); gui_add_page_buttons cdecl(direita, y, pagina short,
# maximo short, ids de <<, <, >, >>) -> a borda esquerda da vaga do << (o original reaproveita as vagas dos argumentos 2
# e 3 como locais); Guio::get_fields thiscall, 19 ponteiros, ret 0x4c; highlight_on/off thiscall ret 0;
# set_text_and_refresh thiscall(texto, redesenhar) ret 8; free_images thiscall ret 0; convert_colours_pixel_format
# thiscall(para_565, quais) ret 8. Chamadas: Display::add_area 0x55fba0 thiscall ret 0x2c; langlib_sprintf 0x66f4e0 e
# sprintf 0x944c9f cdecl variadicos; font_text_width 0x5e5ad0 cdecl(fonte, texto); Guio::draw 0x5ee590 thiscall ret 4;
# 0x5e3fd0 cdecl(x, y, imagem) (o fundo de volta); 0x5e2eb0 cdecl(x1, y1, x2, y2) (mostra o retangulo); gui_free_image
# 0x5e42b0 cdecl; 0x5e49f0 cdecl(cor, &c0, &c1, &c2, formato) (separa os canais, escreve bytes); graphics_rgb_to_pixel
# 0x5e4800 cdecl -> ax.
# Iguais no cm0102-1080.exe (as rotinas inteiras; o Guio::init, que o 1080 altera, fica de fora).
# Comparadores: os dois construtores lado a lado na lista de exibicao (contagens, as proximas vagas de area e de item,
# as duas ordens, o pacote de rede, o texto de rascunho 0xde1f64) e no retorno; get_fields lado a lado nas 19 saidas;
# convert lado a lado nas tres cores; highlight_on/off, set_text_and_refresh e free_images so a original (desenham ou
# liberam imagens), conferindo o plano (as marcas e o destaque da area; o texto; os ponteiros nulos).
TELAS = True
ROTINAS = [('gui_add_back_next_buttons', 0x5edcc0, [0x81, 0xec, 0xd4, 0x07, 0x00, 0x00]),
           ('gui_add_page_buttons', 0x5ede20, [0x53, 0x55, 0x66, 0x8b, 0x6c, 0x24, 0x14]),
           ('guio_get_fields', 0x5ee4a0, [0x8b, 0x54, 0x24, 0x04, 0x8b, 0x41, 0x0c]),
           ('guio_highlight_on', 0x5eec20, [0x56, 0x8b, 0xf1, 0xc7, 0x86, 0x88, 0x01, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00]),
           ('guio_highlight_off', 0x5eecb0, [0x56, 0x8b, 0xf1, 0xc7, 0x86, 0x88, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]),
           ('guio_set_text_and_refresh', 0x5eed50, [0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('guio_free_images', 0x5eef00, [0x55, 0x57, 0x8b, 0xf9, 0x8b, 0x47, 0x4c]),
           ('guio_convert_colours_pixel_format', 0x5eef80, [0x8b, 0x44, 0x24, 0x04, 0x83, 0xec, 0x24])]
DOCS = ('a base da interface (3): os botões Voltar/Avançar e de página e o widget: os campos, o destaque, o texto, as imagens e o formato das cores',
        'the interface base (3): the Back/Next and page buttons and the widget: its fields, highlight, text, images and colour format',
        [('gui_add_back_next_buttons', 'gui_add_back_next_buttons', '0x5edcc0', 'gui_utils.cpp', 'cdecl',
          'os botões Voltar e Avançar: uma área de duas casas no pé da tela, (100, 555)–(790, 590); cada botão que não pode ser usado vira um texto cinza na fonte 0x2c; ids −2 e −3',
          'the Back and Next buttons: a two-cell area at the bottom of the screen, (100, 555)–(790, 590); a button that cannot be used becomes grey text in font 0x2c; ids −2 and −3'),
         ('gui_add_page_buttons', 'gui_add_page_buttons', '0x5ede20', 'gui_utils.cpp', 'cdecl',
          'os botões de página de uma lista, da direita para a esquerda: última, próxima, o texto "Pg página/máximo", anterior e primeira; a primeira e a última só com id; os que não levam a lugar nenhum ficam com id 0',
          'the page buttons of a list, right to left: last, next, the "Pg page/max" text, previous and first; first and last only with an id; those that lead nowhere get id 0'),
         ('guio_get_fields', 'Guio::get_fields', '0x5ee4a0', 'Guio.cpp', 'thiscall, ret 0x4c',
          'copia os campos do widget para 19 ponteiros do chamador; o texto de um item de área pula o recuo de cinco espaços',
          "copies the widget's fields to the caller's 19 pointers; an area item's text skips the five-space indent"),
         ('guio_highlight_on', 'Guio::highlight_on', '0x5eec20', 'Guio.cpp', 'thiscall',
          'destaca o widget: a marca e a hora, o redesenho quando o destaque aparece e o item destacado da área',
          "highlights the widget: the mark and the time, the redraw when the highlight shows and the area's highlighted item"),
         ('guio_highlight_off', 'Guio::highlight_off', '0x5eecb0', 'Guio.cpp', 'thiscall',
          'tira o destaque: a marca e a hora, o redesenho e, se era ele, a área fica sem item destacado',
          'takes the highlight off: the mark and the time, the redraw and, when it was this one, the area without a highlighted item'),
         ('guio_set_text_and_refresh', 'Guio::set_text_and_refresh', '0x5eed50', 'Guio.cpp', 'thiscall, ret 8',
          'troca o texto do widget (e o destino de um campo de texto), larga a imagem do texto antigo e, pedido e com a tela ao vivo, redesenha na hora',
          "sets the widget's text (and a text entry's target), drops the old text image and, when asked and the display is live, redraws at once"),
         ('guio_free_images', 'Guio::free_images', '0x5eef00', 'Guio.cpp', 'thiscall',
          'libera o fundo guardado e a imagem do texto do widget (a imagem partilhada só desconta)',
          "frees the widget's saved background and text image (the shared image is only counted down)"),
         ('guio_convert_colours_pixel_format', 'Guio::convert_colours_pixel_format', '0x5eef80', 'Guio.cpp', 'thiscall, ret 8',
          'converte as cores escolhidas do widget do formato 5-5-5 ou 5-6-5 para o da tela; defeito: com mais de uma escolhida, todas recebem a última separada',
          "converts the widget's chosen colours from the 5-5-5 or 5-6-5 format to the screen's; defect: with more than one chosen, all get the last one split")])
