# As seleções e o calendário dos clubes (national_teams.cpp): se o jogo do clube pode ser marcado
# (NationalTeams::can_schedule_club_fixture 0x776fc0, 7711 chamadas na carreira até 2002-06-01, dos choques de data
# do FIXTURE_MANAGER 0x5ad780: 0 quando o mandante ou o visitante tem mais de 2 convocados num dia em volta da data; a 0x5c6710 já em
# Rust; as datas 0x549ef0/0x54c8e0/0x54c5b0 originais), a ordem dos registros pela data (o comparador do qsort 0x778e90:
# as convocações de 9 bytes que a 0x7771c0 ordena, 684 chamadas, e que a 0x776fc0 percorre supondo a ordem; também as
# lesões 0x62e010/0x62e820 e as transferências 0x8cb9a0) e a nação classificada para a fase final
# (NationalTeams::add_qualified_nation 0x7779b0, das eliminatórias; acrescenta na LLIST +0x28 pela 0x68d0d0). O quadro
# SEH da 0x776fc0 só guarda o estado (datas sem destrutor). O único lixo é o alto das palavras +0x2a/+0x28 que a
# 0x776fc0 empurra para a 0x549ef0 (que só lê as palavras baixas: a cópia passa limpas). Sem sorteio. Iguais no
# cm0102-1080.exe. Simulação: as mutações pela carreira e a confirmação no jogo real comparando. Comparadores: a 0x776fc0
# e a 0x778e90 pelo valor; a 0x7779b0 só a original com o plano (aloca o nó).
SAVE = False
ROTINAS = [('pode_marcar_jogo_do_clube', 0x776fc0, [0x6a, 0xff, 0x68, 0x48, 0xdf, 0x95, 0x00]),
           ('ordem_das_convocacoes_pela_data', 0x778e90, [0x56, 0x8b, 0x74, 0x24, 0x0c]),
           ('acrescentar_nacao_classificada', 0x7779b0, [0x8b, 0x54, 0x24, 0x04, 0x83, 0xec, 0x08])]
DOCS = ('as seleções e o calendário dos clubes: se o jogo do clube pode ser marcado com os convocados, a ordem das convocações pela data e a nação classificada para a fase final',
        'national teams and the club calendar: whether the club fixture can be scheduled given the call-ups, the order of call-ups by date and the nation qualified for the finals',
        [('pode_marcar_jogo_do_clube', 'NationalTeams::can_schedule_club_fixture', '0x776fc0', 'national_teams.cpp', 'thiscall, ret 4',
          'jogo amistoso: 1; senão 0 quando o mandante ou o visitante (clube comum) tem mais de 2 convocados entre um dia antes e um depois da data do jogo, senão 1',
          'friendly fixture: 1; otherwise 0 when the home or away club (a plain club) has more than 2 players on call-up from a day before to a day after the fixture date, else 1'),
         ('ordem_das_convocacoes_pela_data', 'compare_call_up_records_by_date', '0x778e90', 'national_teams.cpp', 'cdecl',
          'comparador do qsort dos registros pela data em +0 (0x456770/0x456740), a mais cedo primeiro',
          'qsort comparator of records by the date at +0 (0x456770/0x456740), earliest first'),
         ('acrescentar_nacao_classificada', 'NationalTeams::add_qualified_nation', '0x7779b0', 'national_teams.cpp', 'thiscall, ret 8',
          'com a nação e a fase final não nulas, acrescenta o par à lista das novas classificadas (+0x28)',
          'with nation and finals both non-null, appends the pair to the new qualifiers list (+0x28)')])
CAMPOS = [('jogo +0x28/+0x2a: a data (ano, dia); +0x14 a competição; +0x1c, +0x20 mandante e visitante; valor, id e ponteiro; lê (pode_marcar_jogo_do_clube)',
           'fixture +0x28/+0x2a: the date (year, day); +0x14 the competition; +0x1c, +0x20 home and away clubs; value, id and pointer; reads (pode_marcar_jogo_do_clube)'),
          ('NationalTeams +0x20: as convocações por clube (6 bytes pelo id: +0 registros de 9 bytes, +5 a conta); ponteiro; lê (pode_marcar_jogo_do_clube)',
           'NationalTeams +0x20: the call-ups per club (6 bytes by id: +0 9-byte records, +5 the count); pointer; reads (pode_marcar_jogo_do_clube)'),
          ('convocação (9 bytes) +0: a data; +8 quantos convocados; valor; lê (pode_marcar_jogo_do_clube, ordem_das_convocacoes_pela_data)',
           'call-up (9 bytes) +0: the date; +8 how many called up; value; reads (pode_marcar_jogo_do_clube, ordem_das_convocacoes_pela_data)'),
          ('[0xae235c] clubes, [0xae2350] nações: o limite dos clubes comuns (sem as seleções e as sub-21); valor; lê (pode_marcar_jogo_do_clube)',
           '[0xae235c] clubs, [0xae2350] nations: the limit of plain clubs (without national and under-21 sides); value; reads (pode_marcar_jogo_do_clube)'),
          ('NationalTeams +0x28: a lista (LLIST) das novas classificadas (nação, fase final); ponteiro; grava (acrescentar_nacao_classificada)',
           'NationalTeams +0x28: the list (LLIST) of new qualifiers (nation, finals); pointer; writes (acrescentar_nacao_classificada)')]
INTERESSA = ['CMRTE']
