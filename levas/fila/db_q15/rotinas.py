ROTINAS = [('incluir_pessoa_no_clube', 0x53bd10, [0x81, 0xec, 0x08, 0x02, 0x00, 0x00])]
DOCS = ('a entrada da pessoa no clube',
        'adding a person to a club',
        [('incluir_pessoa_no_clube', 'database_add_staff_to_club', '0x53bd10', 'Database.cpp', 'cdecl',
          'põe a pessoa na casa do cargo no clube (cargos 1..15; jogador acima de 85 sem técnico humano sobe de 6/8 para 13/15), copia a data e grava clube e cargo na pessoa',
          'puts the person in the job slot at the club (jobs 1..15; a player above 85 without a human manager moves from 6/8 to 13/15), copies the date and writes club and job into the person')])
CAMPOS = [('clube +0xbf, +0xc3 (3), +0xcf, +0xd3, +0xd7 (50), +0x19f (5), +0x1b3 (7), +0x1cf (3): casas dos cargos e do elenco; ponteiro; lê e grava (incluir_pessoa_no_clube)', 'club +0xbf, +0xc3 (3), +0xcf, +0xd3, +0xd7 (50), +0x19f (5), +0x1b3 (7), +0x1cf (3): job and squad slots; pointer; reads and writes (incluir_pessoa_no_clube)'),
          ('staff +0x24, +0x28: seleção e cargo nela; ponteiro, valor; grava (incluir_pessoa_no_clube)', 'staff +0x24, +0x28: national team and job there; pointer, value; writes (incluir_pessoa_no_clube)'),
          ('staff +0x29, +0x3e: data de entrada na seleção / no clube (8 bytes); valor; grava (incluir_pessoa_no_clube)', 'staff +0x29, +0x3e: date joined national team / club (8 bytes); value; writes (incluir_pessoa_no_clube)'),
          ('staff +0x39, +0x3d: clube e cargo nele; ponteiro, valor; grava (incluir_pessoa_no_clube)', 'staff +0x39, +0x3d: club and job there; pointer, value; writes (incluir_pessoa_no_clube)'),
          ('staff +0x61: jogador; ponteiro; lê (incluir_pessoa_no_clube)', 'staff +0x61: player record; pointer; reads (incluir_pessoa_no_clube)'),
          ('jogador +0x05: habilidade atual (> 85 sobe o cargo); valor; lê (incluir_pessoa_no_clube)', 'player +0x05: current ability (> 85 raises the job); value; reads (incluir_pessoa_no_clube)'),
          ('[0xae2364] total de pessoas (id ≥ total − 16: só no cargo 5); valor; lê (incluir_pessoa_no_clube)', '[0xae2364] staff total (id ≥ total − 16: job 5 only); value; reads (incluir_pessoa_no_clube)'),
          ('[0xae235c], [0xae2350] totais de clubes e nações (faixa das seleções); valor; lê (incluir_pessoa_no_clube)', '[0xae235c], [0xae2350] club and nation totals (national team range); value; reads (incluir_pessoa_no_clube)')]
INTERESSA = ['CMRTE', 'SGE']
