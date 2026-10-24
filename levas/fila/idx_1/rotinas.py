# Rodam na criação do jogo (carga da base): casar_chave_de_clube e casar_chave_de_nacao por set_key_ids
# (0x601a40, chamada em 0x52d8fa), uma vez por clube/nação da base; remapear_chave_de_clube pelo corte da
# tabela de clubes (database_trim_club_table 0x5267c0, chamada em 0x526d94), uma vez por clube. A gravação
# do .qst da prova do save cria o jogo lendo o banco (e a carreira automática também cria o seu).
SAVE = True
ROTINAS = [('casar_chave_de_clube', 0x6146b0, [0x56, 0x8b, 0x74, 0x24, 0x08]),
           ('remapear_chave_de_clube', 0x60ba80, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x0d, 0xa4, 0x00, 0x9d, 0x00]),
           ('casar_chave_de_nacao', 0x60d810, [0x56, 0x8b, 0x74, 0x24, 0x08])]
DOCS = ('as chaves fixas de clube e de nação pelo nome e a troca da chave de clube depois do corte da base',
        'the fixed club and nation keys by name and the club key remap after the database trim',
        [('casar_chave_de_clube', 'match_club_key_id', '0x6146b0', 'index.cpp', 'cdecl',
          'casa o nome do clube da base com as 313 chaves fixas de clube (372 nomes em 4 cadeias, _stricmp): o primeiro acerto de cada cadeia grava a linha na global da chave',
          'matches the database club name with the 313 fixed club keys (372 names in 4 chains, _stricmp): the first hit of each chain stores the row in the key global'),
         ('remapear_chave_de_clube', 'remap_club_key_id', '0x60ba80', 'index.cpp', 'cdecl',
          'depois do corte da tabela de clubes, a primeira chave de clube que guarda o id antigo passa a guardar o novo (devolve 1; nenhuma: 0)',
          'after the club table trim, the first club key holding the old id gets the new one (returns 1; none: 0)'),
         ('casar_chave_de_nacao', 'match_nation_key_id', '0x60d810', 'index.cpp', 'cdecl',
          'casa o nome da nação da base com as 209 chaves fixas de nação (217 nomes em 2 cadeias, _stricmp)',
          'matches the database nation name with the 209 fixed nation keys (217 names in 2 chains, _stricmp)')])
CAMPOS = [('[0x9d00a4..0x9d0524] e [0xb63c9c..0xb63d00]: as 313 chaves de clube (nomes reais na tabela CHAVES_DE_CLUBE do codigo.rs: MLS All Stars East/West, River Plate, Vélez Sarsfield, Everton, Tottenham Hotspur, Chelsea, Valencia C.F., FC St. Pauli, Arminia Bielefeld...; grafias diferentes dão na mesma chave); id (linha da tabela de clubes); grava (casar_chave_de_clube), lê e grava (remapear_chave_de_clube)',
           '[0x9d00a4..0x9d0524] and [0xb63c9c..0xb63d00]: the 313 club keys (real names in the CHAVES_DE_CLUBE table of codigo.rs: MLS All Stars East/West, River Plate, Vélez Sarsfield, Everton, Tottenham Hotspur, Chelsea, Valencia C.F., FC St. Pauli, Arminia Bielefeld...; different spellings map to the same key); id (club table row); writes (casar_chave_de_clube), reads and writes (remapear_chave_de_clube)'),
          ('[0x9cf200..0x9cf544]: as 209 chaves de nação (0x9cf200 Afghanistan, 0x9cf204 Albania...; nomes em CHAVES_DE_NACAO, 217 com as grafias); id (linha da tabela de nações); grava (casar_chave_de_nacao)',
           '[0x9cf200..0x9cf544]: the 209 nation keys (0x9cf200 Afghanistan, 0x9cf204 Albania...; names in CHAVES_DE_NACAO, 217 with the spellings); id (nation table row); writes (casar_chave_de_nacao)'),
          ('clube/nação +nome: o nome que a base dá (argumento das rotinas de casar); texto; lê (por _stricmp 0x951c50, sem distinguir maiúsculas; com o locale em [0xde3040+8])',
           'club/nation +name: the name the database gives (argument of the match routines); text; reads (through _stricmp 0x951c50, case-insensitive; with the locale at [0xde3040+8])')]
INTERESSA = ['CMRTE', 'SGE']
