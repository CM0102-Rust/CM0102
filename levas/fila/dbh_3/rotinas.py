# gravar_nomes roda na gravação do save (save_database 0x52dad0, use_zip = 1, por zwrite). abrir roda na
# criação do jogo (load_database pela pasta Data: o index.dat e cada tabela; 24 chamadas), que a prova do save
# faz antes de gravar o .qst. criar só roda em save_database(..., 0, ...) (a base gravada na pasta Data):
# a prova do save não passa por ela (a mutação escapa; ver mutacoes.py).
SAVE = True
ROTINAS = [('gravar_nomes', 0x550950, [0x8b, 0x44, 0x24, 0x08, 0x56]),
           ('abrir_arquivo_do_banco', 0x54d580, [0x8b, 0x44, 0x24, 0x08, 0x8b, 0x4c, 0x24, 0x04]),
           ('criar_arquivo_do_banco', 0x54d5b0, [0x8b, 0x44, 0x24, 0x08, 0x8b, 0x4c, 0x24, 0x04])]
DOCS = ('a gravação dos .dat: nomes; abrir e criar os .dat',
        'writing the .dat files: names; opening and creating the .dat files',
        [('gravar_nomes', 'db_files_write_or_read_records_0x3c', '0x550950', 'Db_files.cpp', 'cdecl',
          'grava de uma vez n registros de 60 bytes (first_names.dat, second_names.dat e common_names.dat, ids 13 a 15) por zwrite (save) ou fwrite (pasta Data); 1 se foi tudo (só grava, apesar do nome da comunidade)',
          'writes n 60-byte records (first_names.dat, second_names.dat and common_names.dat, ids 13 to 15) in one go, through zwrite (save) or fwrite (Data folder); 1 if everything went (it only writes, despite the community name)'),
         ('abrir_arquivo_do_banco', 'db_files_open_for_read', '0x54d580', 'Db_files.cpp', 'cdecl',
          'monta pasta + nome em 0xde1f64 (_makepath) e devolve fopen(caminho, "rb"); o comparador confere o caminho',
          'builds folder + name at 0xde1f64 (_makepath) and returns fopen(path, "rb"); the comparator checks the path'),
         ('criar_arquivo_do_banco', 'db_files_open_for_write', '0x54d5b0', 'Db_files.cpp', 'cdecl',
          'monta pasta + nome em 0xde1f64 (_makepath) e devolve fopen(caminho, "wb"); só ao gravar a base na pasta; o comparador confere o caminho',
          'builds folder + name at 0xde1f64 (_makepath) and returns fopen(path, "wb"); only when writing the database to the folder; the comparator checks the path')])
CAMPOS = [('TNames (60 bytes, [0xae23f0 / 0xae23f4 / 0xae23f8]): registro inteiro; lê (gravar_nomes)', 'TNames (60 bytes, [0xae23f0 / 0xae23f4 / 0xae23f8]): whole record; reads (gravar_nomes)'),
          ('0xde1f64 (caminho do .dat, 260): texto; grava (abrir_arquivo_do_banco, criar_arquivo_do_banco)', '0xde1f64 (.dat path, 260): text; writes (abrir_arquivo_do_banco, criar_arquivo_do_banco)')]
INTERESSA = ['SGE', 'CMRTE']
