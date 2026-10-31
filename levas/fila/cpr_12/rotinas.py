# O calendário dos jogos (fix_man.cpp): FIXTURE_MANAGER::find_fixtures (0x5acbe0, 288.865 chamadas na cobertura de
# 17/10: escalação, finanças do dia, amistosos, TV, diretoria), FIXTURE_MANAGER::collect_comp_fixtures_on_day
# (0x5abd90, 130: adiamento pela TV das ligas e rodadas das copas nas datas fixas) e fix_man_comp_has_fixture_on_date
# (0x5ae6d0, 335: eliminatórias da Eurocopa). Sem SEH; as listas do calendário vêm da 0x5abf10 (o ano comparado como
# palavra; o dia, a fatia e a nação mascarados: os bytes altos de sobra que a original empurra não pesam) e as cópias
# dos jogos vão para a lista do chamador pela 0x68d0d0 (a falha mostra a caixa, como a original). A 0x5ae6d0 é stdcall
# (o ecx que o chamador põe não é usado). Iguais no cm0102-1080.exe. Simulação: as mutações pela carreira e a
# confirmação no jogo real comparando. Comparadores: sem lista, pela devolução; com lista, a original enche a lista
# real e a cópia uma provisória (0x68d020/0x68d040), conferidas a conta e os itens acrescentados.
SAVE = False
ROTINAS = [('achar_jogos_do_calendario', 0x5acbe0, [0x81, 0xec, 0x24, 0x02, 0x00, 0x00]),
           ('jogos_da_competicao_no_dia', 0x5abd90, [0x81, 0xec, 0x08, 0x02, 0x00, 0x00]),
           ('competicao_tem_jogo_na_data', 0x5ae6d0, [0x81, 0xec, 0x04, 0x02, 0x00, 0x00])]
DOCS = ('o calendário dos jogos: achar os jogos de um clube ou competição entre duas datas, os jogos da competição num dia e se a competição tem jogo na data',
        'the fixture calendar: finding the fixtures of a club or competition between two dates, the competition fixtures on a day and whether the competition has a fixture on a date',
        [('achar_jogos_do_calendario', 'FIXTURE_MANAGER::find_fixtures', '0x5acbe0', 'fix_man.cpp', 'thiscall, ret 0x24',
          'conta (e copia para a lista, se dada) os jogos entre duas datas nas listas das nações-chave do clube ou da competição, com os filtros de tipo, lado do clube e competição',
          "counts (and copies into the list, if given) the fixtures between two dates in the key-nation lists of the club or competition, with type, club side and competition filters"),
         ('jogos_da_competicao_no_dia', 'FIXTURE_MANAGER::collect_comp_fixtures_on_day', '0x5abd90', 'fix_man.cpp', 'thiscall, ret 0x1c',
          'copia para a lista os jogos de um dia e fatia da competição, rodada e tipo pedidos e devolve quantos',
          'copies into the list the fixtures of one day and slice for the requested competition, round and type and returns how many'),
         ('competicao_tem_jogo_na_data', 'fix_man_comp_has_fixture_on_date', '0x5ae6d0', 'fix_man.cpp', 'stdcall, ret 8',
          'diz se a competição tem jogo na data em alguma das três fatias do calendário',
          'tells whether the competition has a fixture on the date in any of the three calendar slices')])
CAMPOS = [('jogo (0x4f bytes) +0: a competição; +0xc o clube mandante; +0x10 o visitante; +0x34 a rodada (palavra); +0x40/+0x41 o lado do mandante e do visitante; +0x42 o tipo; id e valor; lê (as três)',
           'fixture (0x4f bytes) +0: the competition; +0xc home club; +0x10 away club; +0x34 the round (word); +0x40/+0x41 home and away side; +0x42 the type; id and value; reads (all three)'),
          ('lista (LLIST) +0: a conta (palavra); +2 o começo; +6 a cauda; +0xa o cursor; valor e ponteiro; lê, anda o cursor e acrescenta (as três)',
           'list (LLIST) +0: the count (word); +2 head; +6 tail; +0xa cursor; value and pointer; reads, moves the cursor and appends (all three)'),
          ('[0xae23b4] os clubes (0x245 bytes); [0xae23d0] as competições (0x6b bytes); [0xae2a58] o calendário; ponteiro; lê (achar_jogos_do_calendario, competicao_tem_jogo_na_data)',
           '[0xae23b4] the clubs (0x245 bytes); [0xae23d0] the competitions (0x6b bytes); [0xae2a58] the calendar; pointer; reads (achar_jogos_do_calendario, competicao_tem_jogo_na_data)')]
INTERESSA = ['SGE']
