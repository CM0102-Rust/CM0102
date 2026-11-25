# Um campo das estatisticas de um jogador como texto, de um registro de temporada ou dos totais da carreira
# (player_stats.cpp; leitor rx08b, rodada 6, 25/11/2026). TELAS: so as telas da ficha do jogador as chamam (a cobertura
# do mundo viu 0 chamadas). Quem chama (desmontagem): 0x7c2750 <- 0x7c26c7 (PlayerStats::format_player_stat, rx08_8),
# 0x875954 e 0x886e22 (staff_screens, as abas de estatisticas e de historico); 0x7c3060 <- 0x88700f (a linha dos totais
# da aba de historico, history_tab_stat_total, ja Rust, pelo exe).
# Caminho no jogo: ficha de um jogador -> Estatisticas (Stats) e Historico (History): cada coluna (Apps, Gls, Con, Pens,
# Asts, ..., Pass %, Tck, Drb, Sh Tar, Av R) de cada temporada e a linha dos totais da carreira.
# - 0x7c2750 PlayerStats::format_record_field thiscall(registro, campo, buf, tamanho, subs, apps, zero) ret 0x1c; quadro
#   alinhado (and esp, -8); tabela de saltos 0x7c3014 (17 campos).
# - 0x7c3060 PlayerStats::format_totals_field thiscall(totais, campo, buf, tamanho, subs, apps, zero) ret 0x1c; tabela
#   0x7c38dc. O campo 4 sem subs mostra o segundo total (+0x14) onde o registro mostra o primeiro (DEFEITOS).
# Prologos push ebp; mov ebp, esp; and esp, -8 (6), sem relativo. Iguais no cm0102-1080.exe (inteiras).
# Comparadores: lado a lado nos 51 primeiros bytes do buffer e na resposta; buffer curto ou registro nulo: so a original.
TELAS = True
SAVE = False
ROTINAS = [('format_record_field', 0x7c2750, [0x55, 0x8b, 0xec, 0x83, 0xe4, 0xf8]),
           ('format_totals_field', 0x7c3060, [0x55, 0x8b, 0xec, 0x83, 0xe4, 0xf8])]
DOCS = ('um campo das estatísticas do jogador como texto', "one field of a player's statistics as text",
        [('format_record_field', 'PlayerStats::format_record_field', '0x7c2750', 'player_stats.cpp', 'thiscall, ret 0x1c',
          'regra: um campo da temporada do jogador como texto: a contagem, com os jogos depois de uma barra quando pedido; jogos e pênaltis podem mostrar uma parte entre parênteses; a nota média com duas casas, os números por jogo com uma, o acerto de passes e de chutes em porcentagem; nada escrito sem jogos (a não ser que se peça o zero)',
          "rule: one field of the player's season as text: the count, with the appearances after a slash when asked; appearances and penalties can show a part in brackets; the average rating with two decimals, the per-game figures with one, the passing and shooting success as percentages; nothing written without appearances (unless asked for a zero)"),
         ('format_totals_field', 'PlayerStats::format_totals_field', '0x7c3060', 'player_stats.cpp', 'thiscall, ret 0x1c',
          'regra: um campo dos totais da carreira como texto, como o da temporada, mas os números por jogo são por 90 minutos e pedem 150 minutos; o acerto de passes pede 20 tentativas; o de chutes, 25 chutes, ou 5 a 24 com no máximo 45 minutos por chute; sem os parênteses, a coluna dos pênaltis mostra o segundo total',
          'rule: one field of the career totals as text, like the season one, but the per-game figures are per 90 minutes and need 150 minutes; passing success needs 20 attempts; shooting success 25 shots, or 5 to 24 at no more than 45 minutes a shot; without the brackets, the penalties column shows the second total')])
CAMPOS = []
INTERESSA = []
