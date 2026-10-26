# Do Database.cpp: as comparações do qsort dos clubes pelo nome curto (0x540080: telas de clubes e de busca, Setup
# 0x822260, semeadura da UEFA 0x906410) e das pessoas pelo nome completo (0x540270: menu 0x7612c0, gerente de
# transferências 0x8e55f0), e se a pessoa confere com os nomes e a data de nascimento (0x53e990, ~137 chamadores:
# telas, regeneração, transferências). Simulação: as mutações pela carreira até 2002-06-01 e a confirmação no jogo
# real comparando. Sem SEH (o mapa junta a 0x540080 com a 0x540130, que tem SEH; a 0x540080 em si não tem), sem
# sorteio; as locais de 0x7d0 bytes das duas ordens têm o primeiro byte zerado antes de lidas; a local de 0x200 da
# 0x53e990 só serve à mensagem da caixa (pessoa nula), caminho que fica com a original. As chamadas (0x90c1d0,
# 0x53dba0) são pelo endereço do jogo.
SAVE = False
ROTINAS = [('ordem_dos_clubes_pelo_nome_curto', 0x540080, [0x81, 0xec, 0xa0, 0x0f, 0x00, 0x00]),
           ('ordem_das_pessoas_pelo_nome_completo', 0x540270, [0x81, 0xec, 0xa0, 0x0f, 0x00, 0x00]),
           ('pessoa_confere_com_os_nomes', 0x53e990, [0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('as ordens dos clubes pelo nome curto e das pessoas pelo nome completo, e se a pessoa confere com os nomes',
        'the club order by short name and the staff order by full name, and whether a person matches the names',
        [('ordem_dos_clubes_pelo_nome_curto', 'database_compare_clubs_by_short_name', '0x540080', 'Database.cpp', 'cdecl',
          'comparação do qsort de clubes pelo nome curto (+0x38) sem maiúsculas (0x90c1d0); clube nulo: texto vazio',
          'qsort comparison of clubs by short name (+0x38), case-insensitive (0x90c1d0); null club: empty text'),
         ('ordem_das_pessoas_pelo_nome_completo', 'database_compare_staff_by_full_name', '0x540270', 'Database.cpp', 'cdecl',
          'comparação do qsort de pessoas pelo nome no estilo 1 (0x53dba0) sem maiúsculas; pessoa nula: texto vazio',
          'qsort comparison of staff by style-1 name (0x53dba0), case-insensitive; null person: empty text'),
         ('pessoa_confere_com_os_nomes', 'database_staff_matches_names', '0x53e990', 'Database.cpp', 'cdecl',
          '1 se a pessoa (com jogador ou não jogador) tem os ids de nome, sobrenome e nome comum pedidos (nos não nulos) e, com data, a mesma data de nascimento',
          '1 when the person (with a player or non-player record) has the requested first, second and common name ids (on the non-null ones) and, with a date, the same birth date')])
CAMPOS = [('TClub +0x38: nome curto; texto; lê (ordem_dos_clubes_pelo_nome_curto)', 'TClub +0x38: short name; text; reads (ordem_dos_clubes_pelo_nome_curto)'),
          ('TStaff +4/+8/+0xc: nome, sobrenome, nome comum (registro de nome +0x33 id); +0x10/+0x12 nascimento (dia, ano); +0x61/+0x69 jogador e não jogador; lê (pessoa_confere_com_os_nomes)',
           'TStaff +4/+8/+0xc: first, second, common name (name record +0x33 id); +0x10/+0x12 birth (day, year); +0x61/+0x69 player and non-player; reads (pessoa_confere_com_os_nomes)')]
INTERESSA = ['SGE', 'CMRTE']
