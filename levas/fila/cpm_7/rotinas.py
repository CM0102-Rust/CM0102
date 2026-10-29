# Amistosos: se a competição vale na data do amistoso (Friendly::is_comp_current_on_date 0x5c29b0, pelas copas e
# ligas 0x45d9cd..0x4b43b2), se o candidato já tem amistoso no dia, no anterior ou no seguinte
# (Friendly::candidate_has_nearby_date 0x5c45a0, pelo planejamento 0x5c4380) e acrescentar o amistoso planejado
# (Friendly::add_planned_friendly 0x5c4b60, idem). Chamadas: 0x54c5b0 e 0x54c8e0 (datas), 0x68d1b0 (próximo da lista),
# 0x945667 (realloc) e 0x417c10 (cópia da data) por endereço. Os quadros SEH das duas primeiras só cobrem as datas
# locais (funclets de 2 bytes: destrutores vazios). Sem sorteio. Comparadores: a primeira pelo valor (quente, sem
# contador nem mutação), a segunda lado a lado (o cursor da lista) e a terceira pelo plano (realloc: só a original).
SAVE = False
ROTINAS = [('competicao_na_data_do_amistoso', 0x5c29b0, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('amistoso_perto_da_data', 0x5c45a0, [0x6a, 0xff, 0x68, 0x00, 0xa0, 0x95, 0x00]),
           ('acrescentar_amistoso_planejado', 0x5c4b60, [0x81, 0xec, 0x08, 0x02, 0x00, 0x00])]
DOCS = ('os amistosos: se a competição vale na data, se o clube já tem amistoso perto da data e acrescentar o amistoso planejado',
        'friendlies: whether a competition is current on the date, whether the club already has a friendly near the date and adding the planned friendly',
        [('competicao_na_data_do_amistoso', 'Friendly::is_comp_current_on_date', '0x5c29b0', 'friendly.cpp', 'thiscall, ret 8',
          '1 para a competição comum; para o torneio amistoso, só com a data a menos de 7 dias da data da vaga',
          'returns 1 for an ordinary competition; for a friendly tournament, only when the date is within 7 days of the slot date'),
         ('amistoso_perto_da_data', 'Friendly::candidate_has_nearby_date', '0x5c45a0', 'friendly.cpp', 'thiscall (this sem uso), ret 8',
          '1 se o clube já tem amistoso planejado na data, no dia antes ou no dia depois',
          'returns 1 when the club already has a planned friendly on the date, the day before or the day after'),
         ('acrescentar_amistoso_planejado', 'Friendly::add_planned_friendly', '0x5c4b60', 'friendly.cpp', 'thiscall, ret 0x10',
          'acrescenta o amistoso (data e três ids) ao vetor de 0x14 bytes, que cresce de 100 em 100',
          'appends the friendly (date and three ids) to the 0x14-byte array, which grows by 100')])
CAMPOS = [('Friendly +4 competição do amistoso; ponteiro; +0xaf primeiro id (byte); +0xb1 vagas do torneio (0x69 bytes: +0 ocupada, +4 data); lê (competicao_na_data_do_amistoso)',
           'Friendly +4 friendly competition; pointer; +0xaf first id (byte); +0xb1 tournament slots (0x69 bytes: +0 used, +4 date); reads (competicao_na_data_do_amistoso)'),
          ('[0xae2378] número de competições; valor; lê (competicao_na_data_do_amistoso)',
           '[0xae2378] competition count; value; reads (competicao_na_data_do_amistoso)'),
          ('candidato a amistoso +0x12 lista de amistosos planejados (itens: dia +0, ano +2); lê, grava o cursor +0xa (amistoso_perto_da_data)',
           'friendly candidate +0x12 planned friendlies list (items: day +0, year +2); reads, writes the cursor +0xa (amistoso_perto_da_data)'),
          ('Friendly +0x34d8 amistosos planejados (0x14 bytes: data +0, ids +8, +0xc, +0x10), +0x34e0 conta, +0x34e4 capacidade; grava (acrescentar_amistoso_planejado)',
           'Friendly +0x34d8 planned friendlies (0x14 bytes: date +0, ids +8, +0xc, +0x10), +0x34e0 count, +0x34e4 capacity; writes (acrescentar_amistoso_planejado)')]
INTERESSA = ['CMRTE']
