# A competicao reserva: se os dois clubes podem escalar os reservas, os jogos reservas de hoje que nao podem ser jogados
# e as subcompeticoes B e sub-21 (record_utils.cpp; leitor sx18, rodada 5, 24/11/2026). Rodam na carreira ENG: a
# criacao das subcompeticoes na criacao do jogo (0x82970c/0x82c6fc); o cancelamento todo dia (0x6b3c9e); a pergunta
# por ele e pela marcacao dos jogos reservas (ReserveComp::schedule_reserve_fixtures 0x7e6380, ainda original).
# Convencoes conferidas na desmontagem:
# - 0x7e6ba0 thiscall(partida, conferir_data) ret 8, devolve 0/1: 0x542740 database_club_has_reserve_squad cdecl(clube);
#   0x62c400 Injury::get_player_injury_status thiscall([0xb63d28], pessoa) ret 4; 0x53ab90 database_get_staff_age
#   cdecl(pessoa, &data); 0x600a50 human_manager_is_human_club cdecl(clube, 1, 1, 0, 0); 0x549f70 Date(0x1a, 0xb, ano,
#   -1) e 0x549ef0 Date(dia, ano) para o 26 de dezembro; os contadores sao bytes com sinal;
# - 0x7e6a70 thiscall() ret 0: 0x5ae6a0 fix_man_get_fixture_list_for_date thiscall([0xae2a58], hoje, fatia do dia,
#   k) ret 0xc para k = 0..[0xb64738]; LLIST local (0x68d020/0x68d0d0 0x4f bytes/0x68d1b0/0x68d040); 0x5abc30
#   FIXTURE_MANAGER::remove_fixture thiscall(copia) ret 4;
# - 0x7e61c0 thiscall() ret 0: operator new 0xa3, Comp::Comp 0x48c580, vtables 0x96f074 (B) e 0x96efe4 (sub-21), +4 o
#   registro da competicao ([0xae23d0] + id x 0x6b).
# Comparadores: a pergunta lado a lado (compare_value!); o cancelamento so a original, com a lista da copia conferida
# (nenhuma das partidas que a copia removeria fica); as subcompeticoes so a original e as vagas da tabela conferidas.
# Fica de fora ReserveComp::ReserveComp (0x7e5fc0): e a checagem do CD (procura cm0102.exe nas unidades de CD e repete
# "Insert CD"), desligada pelo patch_portatil.py nas copias portatil e 1080; desviada, so rodaria no exe original e
# travaria sem CD.
SAVE = False
ROTINAS = [('can_clubs_field_reserve_sides', 0x7e6ba0, [0x6a, 0xff, 0x68, 0xf0, 0x07, 0x96, 0x00]),
           ('cancel_unplayable_reserve_fixtures', 0x7e6a70, [0x6a, 0xff, 0x68, 0xc8, 0x07, 0x96, 0x00]),
           ('reserve_comp_create_sub_comps', 0x7e61c0, [0x6a, 0xff, 0x68, 0x46, 0x07, 0x96, 0x00])]
DOCS = ('a competição reserva: os reservas dos dois clubes, os jogos reservas impossíveis de hoje e as subcompetições',
        'the reserve competition: both clubs\' reserve sides, today\'s unplayable reserve fixtures and the sub competitions',
        [('can_clubs_field_reserve_sides', 'ReserveComp::can_clubs_field_reserve_sides', '0x7e6ba0', 'record_utils.cpp', 'thiscall, ret 8',
          'diz se os dois clubes podem escalar os reservas: conferindo a data, não em partida com nome de rodada, com mandante turco fora da divisão turca, nos prefixos 0x418, 0x419 e 0xa0 nem em 26 de dezembro; mandante na primeira escocesa ou turco exige 15 jovens (até 21 anos); os dois precisam ter time reserva; cada um, com menos de 11 disponíveis, no máximo 36 inscritos e vagas vazias que completem 14; o humano precisa de 11 disponíveis e um inscrito entre eles; e os jovens bastando',
          'tells whether both clubs can field reserve sides: checking the date, not for a fixture with a round name, a Turkish home club outside the Turkish division, the prefixes 0x418, 0x419 and 0xa0, nor on 26 December; a home club in the Scottish Premier Division or Turkish needs 15 young players (21 or under); both need a reserve squad; each, with fewer than 11 available, at most 36 listed and empty slots making up 14; a human club needs 11 available and one listed among them; and enough young players'),
         ('cancel_unplayable_reserve_fixtures', 'ReserveComp::cancel_unplayable_reserve_fixtures', '0x7e6a70', 'record_utils.cpp', 'thiscall',
          'tira as partidas reservas de hoje (reserva, CFA e divisão inferior francesas) em que um dos clubes não pode escalar os reservas',
          'removes today\'s reserve fixtures (reserve, French CFA and lower division) in which one of the clubs cannot field a reserve side'),
         ('reserve_comp_create_sub_comps', 'ReserveComp::create_sub_comps', '0x7e61c0', 'record_utils.cpp', 'thiscall',
          'registra a competição reserva na tabela, cria as competições internacionais B e sub-21 e aponta a CFA e a divisão inferior francesas e a divisão inferior espanhola para os objetos das primeiras divisões',
          'registers the reserve competition in the table, creates the B and Under 21 international competitions and points the French CFA and lower division and the Spanish lower division at the first divisions\' objects')])
CAMPOS = [('competição reserva +0xa3 e +0xa7: as competições internacionais B e sub-21 (grava); liga +0xee e +0xf2: os objetos das divisões inferiores (lê)',
           'reserve competition +0xa3 and +0xa7: the B and Under 21 international competitions (writes); league +0xee and +0xf2: the lower divisions\' objects (reads)')]
INTERESSA = []
