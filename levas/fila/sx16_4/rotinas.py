# officials_manager.cpp (leitor sx16, rodada 5, 24/11/2026): a ordenacao da tabela Official Stats (a tela de
# estatisticas dos arbitros de uma competicao, show_comp_official_stats_screen 0x79a530, ja Rust).
# - 0x79aba0 cdecl(widget) -> int, ret: 0x7ff010 get_screen_param thiscall(0xb74450, k) ret 4 (0x2d coluna, 0x2e
#   linhas, 0x2f quantas, 0x31 invertida, 0x30 area); 0x803320 get_area_scroll_row thiscall(sm, area) ret 4; 0x7ff270
#   set_screen_param thiscall(sm, k, valor, 0) ret 0xc; o id do item: [0xb74c4c + vaga*0x300] + widget*0x18c + 0xba966
#   (palavra; 0 -> [0xdd6bde]); 0x945333 qsort cdecl(linhas, n, 0x11, ordem); 0x7615d0 e 0x7643d0 cdecl(widget).
#   Devolve -11 depois de ordenar. Chamada pela 0x4965d0 (clique da tela da competicao, 0x496b47).
# - as 12 ordens: cdecl(a, b) -> int, qsort; o desempate por 0x90c1d0 utils_string_compare cdecl(nome a, nome b, 1, 0)
#   com o SECOND_NAME do arbitro (tabela 0xae23e8, 0x2b bytes por arbitro); a linha (0x11 bytes): +0 id, +4 nota media
#   f32, +8 amarelos (palavra), +0xc jogos (palavra), +0xe dureza (byte com sinal), +0xf vermelhos (byte sem sinal).
#   A 0x79b150 (jogos, decrescente) e tambem a ordem inicial da tela (0x79a63f).
# Prologos: os de 10 bytes terminam na fronteira (mov eax, [esp+4]; mov edx, [0xae23e8]).
# Caminho no jogo: Competicoes -> uma liga (Premier) -> menu da competicao -> Official Stats (estatisticas dos
# arbitros); clicar em cada cabecalho (Name, Games, Red, Yellow, Harshness, Rating) duas vezes.
TELAS = True
ROTINAS = [('official_stats_screen_handle_click', 0x79aba0, [0x83, 0xec, 0x08, 0x53, 0x55]),
           ('official_stats_compare_surname_asc', 0x79b040, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x15, 0xe8, 0x23, 0xae, 0x00]),
           ('official_stats_compare_surname_desc', 0x79b090, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x15, 0xe8, 0x23, 0xae, 0x00]),
           ('official_stats_compare_games_asc', 0x79b0e0, [0x8b, 0x44, 0x24, 0x04, 0x57]),
           ('official_stats_compare_games_desc', 0x79b150, [0x8b, 0x44, 0x24, 0x04, 0x57]),
           ('official_stats_compare_red_cards_asc', 0x79b1c0, [0x8b, 0x54, 0x24, 0x04, 0x57]),
           ('official_stats_compare_red_cards_desc', 0x79b230, [0x8b, 0x54, 0x24, 0x04, 0x57]),
           ('official_stats_compare_yellow_cards_asc', 0x79b2a0, [0x8b, 0x44, 0x24, 0x04, 0x57]),
           ('official_stats_compare_yellow_cards_desc', 0x79b310, [0x8b, 0x44, 0x24, 0x04, 0x57]),
           ('official_stats_compare_rating_desc', 0x79b380, [0x8b, 0x4c, 0x24, 0x04, 0x57]),
           ('official_stats_compare_rating_asc', 0x79b410, [0x8b, 0x4c, 0x24, 0x04, 0x57]),
           ('official_stats_compare_harshness_asc', 0x79b4a0, [0x8b, 0x44, 0x24, 0x04, 0x57]),
           ('official_stats_compare_harshness_desc', 0x79b510, [0x8b, 0x44, 0x24, 0x04, 0x57])]
_ORDENS = [('surname_asc', '0x79b040', 'pelo sobrenome do árbitro, crescente', "by the official's second name, ascending"),
           ('surname_desc', '0x79b090', 'pelo sobrenome, decrescente', 'by the second name, descending'),
           ('games_asc', '0x79b0e0', 'pelos jogos, crescente; empate pelo sobrenome', 'by games, ascending; ties by second name'),
           ('games_desc', '0x79b150', 'pelos jogos, decrescente (a ordem inicial da tela); empate pelo sobrenome', "by games, descending (the screen's first order); ties by second name"),
           ('red_cards_asc', '0x79b1c0', 'pelos vermelhos, crescente; empate pelo sobrenome', 'by red cards, ascending; ties by second name'),
           ('red_cards_desc', '0x79b230', 'pelos vermelhos, decrescente; empate pelo sobrenome', 'by red cards, descending; ties by second name'),
           ('yellow_cards_asc', '0x79b2a0', 'pelos amarelos, crescente; empate pelo sobrenome', 'by yellow cards, ascending; ties by second name'),
           ('yellow_cards_desc', '0x79b310', 'pelos amarelos, decrescente; empate pelo sobrenome', 'by yellow cards, descending; ties by second name'),
           ('rating_desc', '0x79b380', 'pela nota média, a maior primeiro; empate pelo sobrenome', 'by average rating, the higher first; ties by second name'),
           ('rating_asc', '0x79b410', 'pela nota média, a menor primeiro; empate pelo sobrenome', 'by average rating, the lower first; ties by second name'),
           ('harshness_asc', '0x79b4a0', 'pelo código de dureza, crescente; empate pelo sobrenome', 'by harshness code, ascending; ties by second name'),
           ('harshness_desc', '0x79b510', 'pelo código de dureza, decrescente; empate pelo sobrenome', 'by harshness code, descending; ties by second name')]
DOCS = ('a ordenação da tabela de estatísticas dos árbitros',
        "sorting the officials' statistics table",
        [('official_stats_screen_handle_click', 'official_stats_screen_handle_click', '0x79aba0', 'officials_manager.cpp', 'cdecl',
          'o clique na tela Official Stats: um cabeçalho de coluna ordena a tabela pela primeira ordem dela (nome crescente; jogos, vermelhos, amarelos, nota e dureza decrescentes); o segundo clique na mesma coluna inverte; guarda a linha de rolagem; o resto vai para a barra de menu',
          'the click on the Official Stats screen: a column header sorts the table by its first order (name ascending; games, reds, yellows, rating and harshness descending); a second click on the same column reverses it; keeps the scroll row; anything else goes to the menu bar')]
       + [('official_stats_compare_' + n, 'official_stats_compare_' + n, a, 'officials_manager.cpp', 'cdecl', pt, en) for n, a, pt, en in _ORDENS])
