SAVE = True
ROTINAS = [('empacotar_no_conteiner', 0x933610, [0x81, 0xec, 0x20, 0x04, 0x00, 0x00]),
           ('desempacotar_do_conteiner', 0x933320, [0x81, 0xec, 0x14, 0x03, 0x00, 0x00]),
           ('limpar_pasta_temporaria', 0x90cd50, [0xb8, 0x1c, 0x68, 0x00, 0x00])]
DOCS = ('o contêiner do save: empacotar e desempacotar os arquivos da pasta e limpar a pasta temporária',
        'the saved game container: packing and unpacking the folder files and clearing the temporary folder',
        [('empacotar_no_conteiner', 'zadd_matching_files', '0x933610', 'zipdir.cpp', 'cdecl',
          'cada arquivo da pasta que casa com o padrão (FindFirstFile) vira um item do contêiner: zopen gravando, fread/zwrite em blocos de 0x30d40, zclose',
          'each folder file matching the pattern (FindFirstFile) becomes a container item: zopen for writing, fread/zwrite in 0x30d40 blocks, zclose'),
         ('desempacotar_do_conteiner', 'zextract_matching_files', '0x933320', 'zipdir.cpp', 'cdecl',
          'cada item do contêiner cujo nome casa com o padrão vira um arquivo da pasta (zopen lendo, zread em blocos de 0x30d40, fwrite)',
          'each container item whose name matches the pattern becomes a folder file (zopen for reading, zread in 0x30d40 blocks, fwrite)'),
         ('limpar_pasta_temporaria', 'utils_clear_temp_folder', '0x90cd50', 'utils.cpp', 'cdecl',
          'monta em [0xde279c] a pasta CM3_TEMP e apaga os arquivos dela que casam com o padrão, em grupos de 100',
          'builds the CM3_TEMP folder path in [0xde279c] and deletes its files matching the pattern, in groups of 100')])
CAMPOS = [('ZFILE (0xa3191 bytes, do zopen) +0x41705: buffer de leitura (0x30d40), usado como bloco do arquivo lido; bytes; grava (empacotar_no_conteiner)', 'ZFILE (0xa3191 bytes, from zopen) +0x41705: read buffer (0x30d40), used as the block read from the file; bytes; writes (empacotar_no_conteiner)'),
          ('ZFILE +0x72445: buffer de escrita (0x30d40), usado como bloco do item lido; bytes; grava (desempacotar_do_conteiner)', 'ZFILE +0x72445: write buffer (0x30d40), used as the block read from the item; bytes; writes (desempacotar_do_conteiner)'),
          ('contêiner (arquivo) +0/+8: tipo (1/2 antigos viram 3/4) e quantidade de itens; valor; lê (desempacotar_do_conteiner)', 'container (file) +0/+8: type (old 1/2 become 3/4) and item count; value; reads (desempacotar_do_conteiner)'),
          ('índice do contêiner (+0xc, 0x10c por item) +8: nome do item; texto; lê e copia para a lista de 1000 × 0x104 (desempacotar_do_conteiner)', 'container index (+0xc, 0x10c per item) +8: item name; text; reads and copies into the 1000 × 0x104 list (desempacotar_do_conteiner)'),
          ('busca de arquivos (0x4c0aa0, 0x114 bytes) +0: alça do FindFirstFile; valor; lê (empacotar_no_conteiner, limpar_pasta_temporaria)', 'file search (0x4c0aa0, 0x114 bytes) +0: FindFirstFile handle; value; reads (empacotar_no_conteiner, limpar_pasta_temporaria)'),
          ('busca de arquivos +0x10: nome do arquivo achado; texto; lê (empacotar_no_conteiner, limpar_pasta_temporaria)', 'file search +0x10: found file name; text; reads (empacotar_no_conteiner, limpar_pasta_temporaria)'),
          ('[0xde279c] pasta temporária (CM3_TEMP + barra + [0xad9c64]); texto; grava (limpar_pasta_temporaria)', '[0xde279c] temporary folder (CM3_TEMP + backslash + [0xad9c64]); text; writes (limpar_pasta_temporaria)'),
          ('[0xad9c64] subpasta temporária; texto; lê (limpar_pasta_temporaria)', '[0xad9c64] temporary subfolder; text; reads (limpar_pasta_temporaria)'),
          ('[0xad6ad8] "{ Empty }" (vaga vazia da lista); texto; lê (desempacotar_do_conteiner)', '[0xad6ad8] "{ Empty }" (empty list slot); text; reads (desempacotar_do_conteiner)'),
          ('[0xb67a34] marca zerada depois da caixa de erro; valor; grava 0 (empacotar_no_conteiner, desempacotar_do_conteiner)', '[0xb67a34] flag cleared after the error box; value; writes 0 (empacotar_no_conteiner, desempacotar_do_conteiner)')]
INTERESSA = ['SGE']
