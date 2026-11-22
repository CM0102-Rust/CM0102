# Os filtros das classificacoes do ano dos premios e o registro de barra de menu de um tecnico humano
# (award_screens.cpp, Menubar.cpp; leitor tel39, rodada 2 das telas, 22/11/2026).
# Convencoes conferidas na desmontagem e nos pontos de chamada:
# - 0x414b20 e 0x414b40: cdecl(pessoa, partida) -> eax (0/1), ret; passadas por endereco (push 0x414b20 em 0x413517,
#   push 0x414b40 em 0x41352a) ao YEAR_RATINGS::record_match_ratings (0x9315d0, Rust), que as chama cdecl com
#   (pessoa, partida) para cada titular de cada partida: rodam na carreira depois de toda partida oficial.
# - 0x765480 get_human_manager_record: cdecl(tecnico) -> eax, ret; chamada pelo MATCH_DAY::play_matches (0x6b4560...),
#   pelas telas da partida (0x718edc...) e pela 0x4543ac: roda na carreira (o tecnico humano de ferias).
# Comparadores: os tres lado a lado (so devolvem, nao gravam nada).
# Regra estranha do original: na 0x414b40 a Holanda so conta pela competicao da partida; Franca e Suecia contam pela
#   nacao da pessoa ou da competicao, a Dinamarca so pela pessoa e a Belgica so pela competicao.
ROTINAS = [('award_rating_filter_reputation_over_2000', 0x414b20, [0x8b, 0x44, 0x24, 0x04, 0x85, 0xc0]),
           ('award_rating_filter_selected_nations', 0x414b40, [0x8b, 0x54, 0x24, 0x04, 0x56]),
           ('get_human_manager_record', 0x765480, [0x8b, 0x44, 0x24, 0x04, 0x85, 0xc0])]
DOCS = ('os filtros das classificações do ano dos prêmios e o registro da barra de menu do técnico humano',
        "the award year ratings filters and the human manager's menu bar record",
        [('award_rating_filter_reputation_over_2000', 'award_rating_filter_reputation_over_2000', '0x414b20', 'award_screens.cpp', 'cdecl',
          'filtro da classificação mundial do ano: o jogador conta quando a reputação atual da ficha passa de 2000',
          "the world year ratings filter: a player counts when his sheet's current reputation is over 2000"),
         ('award_rating_filter_selected_nations', 'award_rating_filter_selected_nations', '0x414b40', 'award_screens.cpp', 'cdecl',
          'filtro da segunda classificação do ano: conta o jogador dinamarquês, a partida de competição belga, o jogador ou a competição da França, a competição holandesa e o jogador ou a competição da Suécia',
          'the second year ratings filter: a Danish player, a match of a Belgian competition, a French player or competition, a Dutch competition and a Swedish player or competition count'),
         ('get_human_manager_record', 'get_human_manager_record', '0x765480', 'Menubar.cpp', 'cdecl',
          'o registro da barra de menu (16, de 0xf6 bytes) de um técnico humano pela posição entre os 16 últimos registros de pessoa; sem técnico, o da vaga atual; fora da faixa, nulo',
          "the menu bar record (16, 0xf6 bytes each) of a human manager by his place among the last 16 staff records; with no manager, the current slot's; outside the range, null")])
