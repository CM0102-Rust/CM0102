# Os gols do jogador pelo clube (player_stats_get_goals_for_club 0x7c3bb0, pela MATCH_PLAYER::setup 0x6eaea0 em cada
# jogador de cada partida, gravados em +0x3d do jogador da partida), o registro da seleção do ano
# (Staff_Records::add_team_of_year_record 0x86c730, pela TEAM_AWARD::record_winners 0x8affb0) e a ordem dos clubes pelos
# pontos do técnico (staff_records_compare_clubs_by_manager_points 0x870220, o comparador do qsort da 0x86fab0 já em
# Rust, os candidatos a técnico do ano). A 0x86c730 tem SEH só da data local (destrutor vazio 0x5a53c0) e usa os
# ajudantes do histórico da 253ª (cpq4_*). Chamadas originais pelo endereço: 0x7c3e40 (monta a tabela da carreira,
# malloc), 0x9452ca (free), 0x86f5b0 (o registro do técnico); o resto já em Rust. Iguais no cm0102-1080.exe. Trocas:
# das candidatas da família, a 0x7c3a40 (apps_for_club, irmã desta) já é porta do pós-jogo; a build_career_history
# 0x7c3e40 (733 bytes, malloc e compactação) fica para outra leva; as de texto e de tela (0x7c2750, 0x7c3060,
# 0x86e0b0, 0x86f300) ficam de fora. Simulação: as mutações pela carreira até 2002-06-01. Comparadores: os gols e a
# ordem pela devolução (a original e a cópia; na ordem, a original cria os registros que faltam antes), sem os casos
# de erro; o registro só a original (o histórico vai para o arquivo).
SAVE = False
ROTINAS = [('gols_pelo_clube', 0x7c3bb0, [0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x04, 0x02, 0x00, 0x00]),
           ('registrar_a_selecao_do_ano', 0x86c730, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('ordem_pelos_pontos_do_tecnico', 0x870220, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x4c, 0x24, 0x08])]
DOCS = ('os gols do jogador pelo clube, o registro da seleção do ano e a ordem dos clubes pelos pontos do técnico',
        "the player's goals for a club, the team of the year record and the order of the clubs by manager points",
        [('gols_pelo_clube', 'player_stats_get_goals_for_club', '0x7c3bb0', 'player_stats.cpp', 'stdcall, ret 8',
          'soma os gols das temporadas do jogador pelo clube na tabela da carreira (0x7c3e40), em 16 bits',
          "sums the goals of the player's seasons for the club in the career table (0x7c3e40), in 16 bits"),
         ('registrar_a_selecao_do_ano', 'Staff_Records::add_team_of_year_record', '0x86c730', 'staff_records.cpp', 'thiscall, ret 0x14',
          'o histórico da seleção do ano da pessoa (tipo 0x32: o clube, o prêmio e dois valores)',
          'the team of the year history of the person (type 0x32: the club, the award and two values)'),
         ('ordem_pelos_pontos_do_tecnico', 'staff_records_compare_clubs_by_manager_points', '0x870220', 'staff_records.cpp', 'cdecl',
          'ordena dois clubes pelos pontos da temporada do técnico na liga (mais pontos primeiro; sem registro, por último)',
          "orders two clubs by their manager's league season points (highest first; no record last)")])
CAMPOS = [('pessoa +0x61: ficha de jogador; ponteiro; lê (gols_pelo_clube)',
           'person +0x61: player record; pointer; reads (gols_pelo_clube)'),
          ('tabela da carreira (0x7c3e40): entradas de 0x2b bytes, seis por temporada e clube; +0xd9 do grupo: id do clube; +0xdf: gols; id e valor; lê (gols_pelo_clube)',
           'career table (0x7c3e40): 0x2b-byte entries, six per season and club; group +0xd9: club id; +0xdf: goals; id and value; reads (gols_pelo_clube)'),
          ('jogador da partida +0x3d: gols pelo clube; valor; grava (MATCH_PLAYER::setup, pela gols_pelo_clube)',
           'match player +0x3d: goals for the club; value; written (MATCH_PLAYER::setup, by gols_pelo_clube)'),
          ('clube +0xcf: técnico; ponteiro; lê (ordem_pelos_pontos_do_tecnico)',
           'club +0xcf: manager; pointer; reads (ordem_pelos_pontos_do_tecnico)'),
          ('registro do técnico (0x86f5b0 do [0xdd7f00]) +0xba: pontos da temporada na liga; valor; lê (ordem_pelos_pontos_do_tecnico)',
           'manager record (0x86f5b0 of [0xdd7f00]) +0xba: league season points; value; reads (ordem_pelos_pontos_do_tecnico)'),
          ('Staff_Records +0 e +0xc: arquivo e listas do histórico das pessoas; acrescenta (registrar_a_selecao_do_ano)',
           'Staff_Records +0 and +0xc: staff history file and lists; appends (registrar_a_selecao_do_ano)')]
INTERESSA = ['CMScout']
