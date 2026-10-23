SAVE = True
ROTINAS = [('casar_nome_com_padrao', 0x933a00, [0x51, 0x53, 0x55, 0x8b, 0x6c, 0x24, 0x14]),
           ('id_do_conteiner', 0x9331c0, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('tamanho_dos_itens_do_padrao', 0x933860, [0x81, 0xec, 0x04, 0x02, 0x00, 0x00])]
DOCS = ('o contêiner do save: as consultas (o padrão do nome, o id e o tamanho dos itens)',
        'the saved game container: the queries (name pattern, id and item size)',
        [('casar_nome_com_padrao', 'zipdir_match_filename_pattern', '0x933a00', 'zipdir.cpp', 'cdecl',
          'casa o nome de um item do contêiner com um padrão com `*` (`*.` vai ao último ponto), sem diferenciar maiúsculas; a vaga "{ Empty }" nunca casa',
          'matches a container item name against a pattern with `*` (`*.` goes to the last dot), case-insensitively; the "{ Empty }" slot never matches'),
         ('id_do_conteiner', 'zget_archive_id', '0x9331c0', 'zipdir.cpp', 'cdecl',
          'lê o cabeçalho do contêiner e devolve o dword +4 (0x16 .sav, 0x17 .qst; 0x14 se falhar)',
          'reads the container header and returns dword +4 (0x16 .sav, 0x17 .qst; 0x14 on failure)'),
         ('tamanho_dos_itens_do_padrao', 'zget_matching_size', '0x933860', 'zipdir.cpp', 'cdecl',
          'soma os tamanhos descomprimidos dos itens que casam com o padrão (int com sinal: acima de 2 GB dá negativo, como na original)',
          'adds up the uncompressed sizes of the items matching the pattern (signed int: negative above 2 GB, as in the original)')])
CAMPOS = [('contêiner (arquivo) +0: tipo (3 sem compressão, 4 RLE; 1/2 antigos viram 3/4); valor; lê (id_do_conteiner, tamanho_dos_itens_do_padrao)', 'container (file) +0: type (3 uncompressed, 4 RLE; old 1/2 become 3/4); value; reads (id_do_conteiner, tamanho_dos_itens_do_padrao)'),
          ('contêiner (arquivo) +4: id do arquivo (0x16 .sav, 0x17 .qst); valor; lê (id_do_conteiner)', 'container (file) +4: archive id (0x16 .sav, 0x17 .qst); value; reads (id_do_conteiner)'),
          ('contêiner (arquivo) +8: quantidade de itens; valor; lê (tamanho_dos_itens_do_padrao)', 'container (file) +8: item count; value; reads (tamanho_dos_itens_do_padrao)'),
          ('índice do contêiner (+0xc, 0x10c por item) +4: tamanho descomprimido do item; valor; lê e soma com sinal (tamanho_dos_itens_do_padrao)', 'container index (+0xc, 0x10c per item) +4: uncompressed item size; value; reads and adds as signed (tamanho_dos_itens_do_padrao)'),
          ('índice do contêiner +8: nome do item (até 0x104 bytes); texto; lê (tamanho_dos_itens_do_padrao, casar_nome_com_padrao)', 'container index +8: item name (up to 0x104 bytes); text; reads (tamanho_dos_itens_do_padrao, casar_nome_com_padrao)'),
          ('[0xad6ad8] "{ Empty }" (vaga vazia); texto; lê (casar_nome_com_padrao)', '[0xad6ad8] "{ Empty }" (empty slot); text; reads (casar_nome_com_padrao)'),
          ('[0xaa2156] tabela de maiúsculas (3 bytes por caractere, via 0x90c2f0); valor; lê (casar_nome_com_padrao)', '[0xaa2156] upper-case table (3 bytes per character, via 0x90c2f0); value; reads (casar_nome_com_padrao)'),
          ('[0xb67a34] marca zerada depois da caixa de erro; valor; grava 0 (id_do_conteiner, tamanho_dos_itens_do_padrao, sem memória)', '[0xb67a34] flag cleared after the error box; value; writes 0 (id_do_conteiner, tamanho_dos_itens_do_padrao, out of memory)')]
INTERESSA = ['SGE']
