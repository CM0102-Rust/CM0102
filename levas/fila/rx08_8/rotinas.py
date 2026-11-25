# Os textos das estatisticas dos jogadores: os nomes das categorias, os titulos das colunas, as ultimas notas e uma
# estatistica (player_stats.cpp; leitor rx08, rodada 6, 25/11/2026). TELAS: rodam so nas telas (a ficha do jogador, a
# aba de estatisticas e as listas de busca/elenco com colunas de estatistica); a cobertura do mundo viu 0 chamadas.
# Caminho no jogo: ficha de um jogador -> Estatisticas (as categorias Liga/Copa/Continental/Internacional e as colunas
# Apps, Gls, Av R...), e a tela do elenco com a coluna das ultimas notas (Forma) [inferencia: o caminho exato da coluna].
# - 0x7c14b0 player_stats_get_category_name thiscall(categoria, buf, tamanho) ret 0xc (this nao lido): buf nulo ou
#   tamanho < 0x33: caixa 0x3ce e 0; buf[0] = 0; tabela de saltos 0x7c1644 (categorias 1..6), langlib_sprintf 0x66f4e0
#   (cdecl) com o texto (e 0xad9c64, a cadeia vazia, nos que tem COMMENT); outra: caixa 0x404 e 0; 1.
# - 0x7c1b20 player_stats_get_field_label thiscall(campo, buf, tamanho) ret 0xc: o mesmo com a tabela 0x7c1de4 (campos
#   1..17; o 13 e "Pass %" e o 17 "Av R"), caixas 0x5bc e 0x63f.
# - 0x7c1660 PlayerStats::format_recent_ratings thiscall(jogador, buf, tamanho) ret 0xc: buf[0] = 0 antes de tudo;
#   jogador nulo: caixa 0x41d (segue); buf nulo ou curto: caixa 0x424 (segue); id >= [0xae236c]: sprintf 0x944c9f
#   (buf, "-"); senao as 5 notas de 4 bits da entrada (+0x24 baixo/alto, +0x25 baixo/alto, +0x26 baixo), cada uma nao
#   nula "%d-" (sprintf num local e strcat); com 2 ou mais caracteres o ultimo sai (devolve 0); vazio: "-" (devolve 2).
# - 0x7c25b0 PlayerStats::format_player_stat thiscall(jogador, categoria, campo, buf, tamanho, a6, a7, a8) ret 0x20:
#   buf nulo ou curto: caixa 0x7c5 e 0; buf = "----" (campo 0x11) ou "-"; jogador nulo: caixa 0x7d7 e 0; id >= [0xae236c]:
#   1; PlayerStats::get_category_record 0x7c3920 thiscall(jogador, categoria) (ja Rust, pelo exe); nulo: 0; senao
#   PlayerStats::format_record_field 0x7c2750 thiscall(registro, campo, buf, tamanho, a6, a7, a8) ret 0x1c (original).
# Prologos (fronteira de instrucao, sem relativo): mov eax, [esp+8]; sub esp, 0x200 (10) nas duas primeiras; sub esp,
# 0x234 (6); sub esp, 0x200 (6). Iguais no cm0102-1080.exe (inteiras).
# Comparadores: lado a lado nos 51 primeiros bytes do buffer e no retorno (buffer nulo ou curto: so a original).
# Defeito do original: format_recent_ratings escreve buf[0] = 0 antes de conferir o buffer, e segue depois das caixas
# de jogador nulo (le [0]) e de buffer curto.
TELAS = True
SAVE = False
ROTINAS = [('player_stats_get_category_name', 0x7c14b0, [0x8b, 0x44, 0x24, 0x08, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('player_stats_get_field_label', 0x7c1b20, [0x8b, 0x44, 0x24, 0x08, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('format_recent_ratings', 0x7c1660, [0x81, 0xec, 0x34, 0x02, 0x00, 0x00]),
           ('format_player_stat', 0x7c25b0, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('os textos das estatísticas dos jogadores: categorias, colunas, últimas notas e uma estatística',
        'the player statistics texts: categories, columns, recent ratings and one stat',
        [('player_stats_get_category_name', 'player_stats_get_category_name', '0x7c14b0', 'player_stats.cpp', 'thiscall, ret 0xc',
          'o nome da categoria de estatística: 1 não competitivos (amistosos e reservas), 2 liga, 3 copa, 4 continental, 5 internacional, 6 clube profissional',
          'the statistics category name: 1 non competitive (friendlies and reserves), 2 league, 3 cup, 4 continental, 5 international, 6 senior club'),
         ('player_stats_get_field_label', 'player_stats_get_field_label', '0x7c1b20', 'player_stats.cpp', 'thiscall, ret 0xc',
          'o título curto da coluna de cada campo de estatística: jogos, gols, sofridos, pênaltis, assistências, gols e sofridos totais, vitórias, derrotas, amarelos, vermelhos, melhor em campo, passes certos, desarmes, dribles, chutes no alvo e nota média',
          'the short column heading of each stat field: apps, goals, conceded, penalties, assists, total goals and conceded, won, lost, yellow, red, man of the match, pass %, tackles, dribbles, shots on target and average rating'),
         ('format_recent_ratings', 'PlayerStats::format_recent_ratings', '0x7c1660', 'player_stats.cpp', 'thiscall, ret 0xc',
          'as cinco últimas notas do jogador, da mais antiga à mais nova, separadas por traço; sem nenhuma, "-"',
          "the player's last five match ratings, oldest first, dash separated; none, \"-\""),
         ('format_player_stat', 'PlayerStats::format_player_stat', '0x7c25b0', 'player_stats.cpp', 'thiscall, ret 0x20',
          'uma estatística do jogador numa categoria, como texto, pelo registro da categoria; sem registro, "-" ("----" na nota média)',
          'one stat of the player in one category, as text, from the category record; without a record, "-" ("----" for the average rating)')])
CAMPOS = []
INTERESSA = []
