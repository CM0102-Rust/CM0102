ROTINAS = [('primeiro_nome_pelo_peso', 0x539b90, [0x51, 0x8b, 0x44, 0x24, 0x08]),
           ('cores_da_competicao', 0x53b7c0, [0x66, 0xa1, 0x84, 0x31, 0xae, 0x00]),
           ('clube_tem_vaga_para_cargo', 0x53b9a0, [0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('o primeiro nome pelo peso, as cores da competição e se o clube tem vaga no cargo',
        "the first name by weight, the competition's colours and whether the club has room in the job",
        [('primeiro_nome_pelo_peso', 'database_pick_first_name_by_weight', '0x539b90', 'Database.cpp', 'cdecl',
          'um primeiro nome: nação com mais de 15 nomes, o de maior peso em 3 sorteios na faixa dela; senão sorteio na tabela inteira',
          'a first name: nation with more than 15 names, the heaviest of 3 draws in its range; otherwise a draw over the whole table'),
         ('cores_da_competicao', 'database_get_comp_colours', '0x53b7c0', 'Database.cpp', 'cdecl',
          'as cores (frente e fundo) da competição em pixels, do registro +0x5d ou da própria competição',
          "the competition's colours (foreground and background) in pixels, from the +0x5d record or the competition itself"),
         ('clube_tem_vaga_para_cargo', 'database_club_has_room_for_job', '0x53b9a0', 'Database.cpp', 'cdecl',
          'se o clube tem vaga para a pessoa no cargo 1..15 (casa vazia ou já dela; 12, 13 e 15 também no elenco)',
          'whether the club has room for the person in job 1..15 (an empty slot or one already theirs; 12, 13 and 15 also in the squad)')])
CAMPOS = [('[0xae2338] índice de nomes por nação (+0 primeira linha, +0x18 quantos primeiros nomes); valor; lê (primeiro_nome_pelo_peso)', '[0xae2338] per-nation name index (+0 first row, +0x18 first-name count); value; reads (primeiro_nome_pelo_peso)'),
          ('[0xae23f0] tabela de primeiros nomes (60 bytes; +0x3b peso); ponteiro; lê (primeiro_nome_pelo_peso)', '[0xae23f0] first-name table (60 bytes; +0x3b weight); pointer; reads (primeiro_nome_pelo_peso)'),
          ('[0xae2398] total de primeiros nomes; valor; lê (primeiro_nome_pelo_peso)', '[0xae2398] first-name total; value; reads (primeiro_nome_pelo_peso)'),
          ('[0xad6c30] estado do rand() da CRT; valor; lê e grava (primeiro_nome_pelo_peso)', '[0xad6c30] CRT rand() state; value; reads and writes (primeiro_nome_pelo_peso)'),
          ('competição +0x5d: registro com as cores (+0x90, +0x94); ponteiro; lê (cores_da_competicao)', 'competition +0x5d: record holding the colours (+0x90, +0x94); pointer; reads (cores_da_competicao)'),
          ('competição +0x61, +0x65: cores; ponteiro; lê (cores_da_competicao)', 'competition +0x61, +0x65: colours; pointer; reads (cores_da_competicao)'),
          ('cor +0x37, +0x38, +0x39: vermelho, verde, azul; valor; lê (cores_da_competicao)', 'colour +0x37, +0x38, +0x39: red, green, blue; value; reads (cores_da_competicao)'),
          ('[0xae3184], [0xaebe04] cores padrão; valor; lê (cores_da_competicao)', '[0xae3184], [0xaebe04] default colours; value; reads (cores_da_competicao)'),
          ('clube +0xbf, +0xc3 (3), +0xcf, +0xd3, +0xd7 (50), +0x19f (5), +0x1b3 (7), +0x1cf (3): casas dos cargos e do elenco; ponteiro; lê (clube_tem_vaga_para_cargo)', 'club +0xbf, +0xc3 (3), +0xcf, +0xd3, +0xd7 (50), +0x19f (5), +0x1b3 (7), +0x1cf (3): job and squad slots; pointer; reads (clube_tem_vaga_para_cargo)')]
INTERESSA = ['CMRTE', 'GPF2']
