# As eliminatorias da Eurocopa: o construtor, os inscritos e as posicoes previstas (euro_champ_qual.cpp; leitor cx06,
# rodada 5, 24/11/2026). Simulacao com as 8 ligas ate 2002-06-01 (docs/cobertura_competicoes.txt: o construtor 1
# chamada, na criacao do jogo por create_other_int_comps; os inscritos e as posicoes previstas 1 cada, no dia do sorteio
# das eliminatorias, 10/03/2002 [inferencia: a data do set_up_rules ja em Rust]).
# - 0x57ce10 EuroChampQual::EuroChampQual thiscall(ano, registro) ret 8 -> this: League::League 0x682200, os campos
#   fixos (vtable 0x96a0d4; +0x42 4; +0x1c/+0x20 -1; +0x45, +0xf7, +0xee, +0xf2, +0xfb 0; +0xf6 1; +0x50 0; +0xff 7;
#   +0x100 8; +0x101 9), o ano da edicao (ano - 1 levado ao proximo (ano - 1990) % 4 == 0: 2002 para 2001), +0xd9 0x24,
#   a lista dos resultados (operator new 0x944cf1 de 0xe e LLIST::LLIST 0x68d020) em +0x10, +0x51 0xf;
#   League::load_from_temp_file 0x687b10 (push 1); set_up_rules 0x57e8f0 (direto, ja em Rust; 0: caixa 0x5e); Comp_Stats
#   (0x5ce, 0x49ee70; nulo: caixa 0x66); +0x44 0xff, +0x30 -1, +0x2c 10, +0xc = malloc(0x28); ano da temporada maior que
#   a edicao (so numa partida que comeca em 1999): create_groups_and_results 0x580bd0; edicao - ano > 2: +0x45 = 0.
# - 0x57f600 EuroChampQual::collect_entrants thiscall() ret 0 -> 0/1: +0xf2 51; +0xf6 conta as sedes europeias da
#   fase final (HostCountry::find_hosts 0x5fa730 thiscall, ret 0x14, com o id em [0x9cf7a4], o ano + 2, as duas sedes, 1;
#   sem a tabela [0xb5e134]: caixa 0x4c3); fora de 0..2: caixa 0x4dd e 1; +0xf2 = 51 - sedes; a lista antiga liberada e a
#   nova (malloc; nula: caixa 0x4ec e 0) com as selecoes europeias (continente +0x71 com o id [0x9cfa10]) na ordem da
#   tabela, menos as sedes; lista curta: caixa 0x506 e 0; qsort 0x945333 (cdecl) pela reputacao 0x4b07d0; 1.
# - 0x5806d0 EuroChampQual::set_predicted_positions thiscall() ret 0 (virtual 0x5c): sem estatisticas, caixa 0x6f3;
#   qsort dos inscritos; NationalTeams::release_competitions_of_continent 0x775070 (thiscall, ret 8) da Europa (registros
#   0xc6 de [0xae23a4]) com 1; cada inscrito: a faixa prevista (formato 1: 9x1, 3x10, 3x13, 3x16, 9x19, 9x28, 9x37, 46;
#   formato 2: 9x1, 1x10, 4x11, 4x15, 9x19, 9x28, 9x37, 46; outro: nenhuma) em Comp_Stats::set_club_final_position
#   0x4a2540 (thiscall, ret 8) e NationalTeams::set_current_competition 0x775220 (thiscall, ret 8).
# Iguais no cm0102-1080.exe (todas inteiras).
# Comparadores: so a original, com o plano (o construtor: this, vtable, registro, ano e importancia pelo
# compare_nations_tournament_ctor; os inscritos: 51 - sedes, todos nao nulos, em ordem de reputacao; as posicoes:
# predicted_by_reputation_check com as faixas).
SAVE = False
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
ROTINAS = [('construct_euro_champ_qual', 0x57ce10, [0x6a, 0xff, 0x68, 0x67, 0x88, 0x95, 0x00]),
           ('euro_champ_qual_collect_entrants', 0x57f600, [0x81, 0xec, 0x08, 0x02, 0x00, 0x00]),
           ('euro_champ_qual_set_predicted_positions', 0x5806d0, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('o construtor, os inscritos e as posições previstas das eliminatórias da Eurocopa',
        "the European Championship qualifiers' constructor, entrants and predicted positions",
        [('construct_euro_champ_qual', 'EuroChampQual::EuroChampQual', '0x57ce10', 'euro_champ_qual.cpp', 'thiscall, ret 8',
          'monta as eliminatórias da Eurocopa: seleções, 9 grupos (fases 0 a 7 mais o próprio grupo 1), a fase dos segundos colocados (8) e a repescagem (9), 10 fases; a edição é o ano da fase final seguinte (o próximo ano com (ano − 1990) múltiplo de 4, a partir do ano anterior ao da temporada: 2002 numa partida de 2001); importância 15; as regras, as estatísticas e, numa partida que comece em 1999, os grupos e resultados da edição de 1998',
          'builds the European Championship qualifiers: national teams, 9 groups (stages 0 to 7 plus group 1 itself), the runners-up stage (8) and the play-off (9), 10 stages; the edition is the next finals year (the next year with (year − 1990) a multiple of 4, from the year before the season: 2002 in a 2001 game); importance 15; the rules, the statistics and, in a game that starts in 1999, the groups and results of the 1998 edition'),
         ('euro_champ_qual_collect_entrants', 'EuroChampQual::collect_entrants', '0x57f600', 'euro_champ_qual.cpp', 'thiscall, ret 0',
          'lista os inscritos das eliminatórias: as seleções da Europa, menos as sedes da fase final, em ordem de reputação; o número de sedes europeias (0 a 2) dá o formato e o número de inscritos (51 menos as sedes)',
          'lists the qualifiers\' entrants: the European national teams but the finals hosts, by reputation; the number of European hosts (0 to 2) gives the format and the entrant count (51 minus the hosts)'),
         ('euro_champ_qual_set_predicted_positions', 'EuroChampQual::set_predicted_positions', '0x5806d0', 'euro_champ_qual.cpp', 'thiscall, ret 0',
          'dá as posições previstas das eliminatórias pela reputação: com uma sede, os 9 primeiros vencem os grupos (1), os 3 seguintes são os melhores segundos (10), depois 13, 16 (3 cada), 19, 28, 37 (9 cada) e 46; com duas sedes 1 (9), 10 (1), 11 e 15 (4 cada), 19, 28, 37 e 46; e põe as eliminatórias como a competição atual de cada seleção, depois de soltar as seleções europeias das suas competições',
          'sets the qualifiers\' predicted positions by reputation: with one host the first 9 win their groups (1), the next 3 are the best runners-up (10), then 13, 16 (3 each), 19, 28, 37 (9 each) and 46; with two hosts 1 (9), 10 (1), 11 and 15 (4 each), 19, 28, 37 and 46; and makes the qualifiers each nation\'s current competition, after releasing the European national teams from their competitions')])
CAMPOS = []
INTERESSA = []
