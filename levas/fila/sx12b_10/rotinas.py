# Os textos de lesao das telas do jogador (injury.cpp; leitor sx12b, rodada 5, 23/11/2026).
# Quem chama: get_player_injury_history pela aba de historico da tela da pessoa (0x885820); describe_player_injury pela
# aba da proxima partida do clube (0x45ccf0) e pela tela de comparar jogadores (0x870650); format_days_out e
# injury_write_match_injury_text pela tela de comparar jogadores. Nenhum roda na carreira automatica.
# Caminho no jogo para o Lucas: Clube -> Proxima partida (a coluna de lesao); perfil de um jogador lesionado ->
# Comparar com outro jogador (a lesao, o tempo de volta e "Injured in ..."); perfil -> Historico (as lesoes passadas).
# Convencoes conferidas na desmontagem:
# - 0x62ec20: thiscall(this, pessoa), ret 4 -> eax; caixa 0xd88; FILE_LLIST::get_nodes 0x58f120 thiscall(lista,
#   [registro+0x14], [registro+0x18]) ret 8 (o registro fisico de 31 bytes pelo id).
# - 0x62ee30: thiscall(this, pessoa, saida, tamanho, artigo), ret 0x10 -> eax; caixa 0xe63; id >= [0xae2364]: volta com
#   eax = a contagem; update_player_fitness 0x62c680 (ja Rust) thiscall ret 4; sem lesao (+0xe = -1) langlib_sprintf(
#   saida, "None<%s - COMMENT - no injury>", 0xad9c64); senao injury_get_type_name 0x62ef30 (ja Rust) thiscall(this,
#   tipo, saida, tamanho, artigo) ret 0x10.
# - 0x630af0: thiscall(this, pessoa, saida, tamanho, completo, estilo), ret 0x14 -> eax; caixa 0x106d; Date::get_difference
#   0x54cc30 thiscall(0xae2c90, saida, data) ret 0xc: anos (palavra +2, sem sinal) x 365 + dias (palavra +0, com
#   sinal); format_duration_text 0x7e5780 cdecl(dias, saida, tamanho, estilo).
# - 0x631dc0: stdcall(registro, texto), ret 8; langlib_sprintf do placar (o maior primeiro) e da linha em 0xde1f64
#   (generos dos nomes curtos +0x52, nomes +0x38); Text::set_string 0x4ae660, add_link 0x4ae8a0 thiscall(texto, nome,
#   0x7d5, clube) e (texto, placar, 0x7d3, partida), append_string 0x4ae770; comp_is_reserve_or_minor_comp 0x7e6f20
#   thiscall([0xb74434], comp); Match_Stats::fixture_entry_matches 0x729500 thiscall([0xb6b7c0], +0x1c, +0x12, +0x16,
#   palavra +1, palavra +3) (o original empilha as palavras com a metade alta de residuo).
# Comparadores: get_player_injury_history so a original (a lista devolvida e alocada); describe_player_injury e
# format_days_out: a original roda (com a atualizacao da forma do dia) e a copia escreve sem a atualizacao num buffer
# proprio, comparados; o texto da partida pelo compare_text_writer.
# Fora (RELATIVAS): injury_get_type_name_gender 0x6306e0 comeca com call 0x66f4d0: entraria com (0x6306e0, 0).
TELAS = True
SAVE = False
ROTINAS = [('get_player_injury_history', 0x62ec20, [0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('describe_player_injury', 0x62ee30, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('format_days_out', 0x630af0, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('injury_write_match_injury_text', 0x631dc0, [0x8b, 0x4c, 0x24, 0x04, 0x83, 0xec, 0x0c])]
DOCS = ('os textos de lesão das telas do jogador',
        'the injury texts of the player screens',
        [('get_player_injury_history', 'Injury::get_player_injury_history', '0x62ec20', 'injury.cpp', 'thiscall, ret 4',
          'devolve a lista das lesões passadas de uma pessoa, lida do arquivo do histórico, ou nada sem o arquivo',
          'returns the list of a person\'s past injuries, read from the history file, or nothing without the file'),
         ('describe_player_injury', 'Injury::describe_player_injury', '0x62ee30', 'injury.cpp', 'thiscall, ret 0x10',
          'atualiza a forma do dia da pessoa e escreve o nome da lesão atual dela, ou "None"',
          'runs the person\'s daily fitness update and writes the name of his current injury, or "None"'),
         ('format_days_out', 'Injury::format_days_out', '0x630af0', 'injury.cpp', 'thiscall, ret 0x14',
          'atualiza a forma do dia e escreve o tempo que falta para a pessoa voltar a treinar ou, com a opção, a estar em forma: os dias da lesão menos os passados desde ela; sem lesão, o texto vazio',
          'runs the daily fitness update and writes the time left until the person trains again or, with the option, is fully fit: the injury\'s days minus those since it; with no injury, the text emptied'),
         ('injury_write_match_injury_text', 'injury_write_match_injury_text', '0x631dc0', 'injury.cpp', 'stdcall',
          'escreve a linha "Injured in" da partida em que o jogador se machucou: o clube, o placar (o maior número primeiro) e vitória, derrota ou empate contra o outro clube, os links dos dois clubes, o nome da competição entre parênteses e o link do placar para a partida quando as estatísticas a guardam',
          'writes the "Injured in" line of the match where the player got injured: the club, the score (the higher number first) and a win, defeat or draw against the other club, the links to both clubs, the competition\'s name in brackets and the score\'s link to the match when the statistics keep it')])
