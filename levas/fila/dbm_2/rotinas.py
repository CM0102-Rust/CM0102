# O nome em inglês do estádio (molde da dbj_2), o leitor "o .lng do idioma existe?" e soltar as 8 linhas da tabela
# do eng.lng. A 0x535930 roda em toda carga (load_database 0x5286e4, idioma das preferências): a carreira e a
# prova do save passam por ela; o caminho do eng.lng (o estádio) e a 0x544f30 (só com a tabela carregada, idioma
# ≠ 0) não: a prova é o autoteste_dbm_2 (CM_AUTOTESTE=1 ou =banco). SAVE = True para agrupar com a leva do banco.
# O estádio tem quadro SEH só no primeiro uso (fica com a original); as outras duas, sem SEH e sem variável local
# lida sem valor. A 0x544f30 solta memória: o comparador só chama a original (como as liberações da 232a).
SAVE = True
ROTINAS = [('nome_ingles_do_estadio', 0x545990, [0x6a, 0xff, 0x68, 0x1b, 0x7a, 0x95, 0x00]),
           ('arquivo_do_idioma_existe', 0x535930, [0x8b, 0x44, 0x24, 0x04, 0x85, 0xc0]),
           ('liberar_tabelas_em_ingles', 0x544f30, [0x56, 0x8b, 0xf1, 0x8b, 0x46, 0x20])]
DOCS = ('o nome em inglês do estádio, o .lng do idioma existe? e soltar as tabelas em inglês',
        "the stadium's English name, does the language's .lng exist? and freeing the English tables",
        [('nome_ingles_do_estadio', 'stadium_get_english_name', '0x545990', 'Database.cpp', 'cdecl',
          'o nome do estádio, ou o em inglês do eng.lng (linhas de 52 bytes, pelo id) quando outro idioma está carregado',
          "the stadium's name, or its English one from eng.lng (52-byte rows, by id) when another language is loaded"),
         ('arquivo_do_idioma_existe', 'database_language_file_exists', '0x535930', 'Database.cpp', 'cdecl',
          'monta CM3_TEMP\\<idioma>.lng (0, 1, 11 e 12: eng.lng; 2 a 10: fra, ger, ita, esp, dan, hol, nor, por, swe) e diz se existe; outro idioma, 0',
          'builds CM3_TEMP\\<language>.lng (0, 1, 11 and 12: eng.lng; 2 to 10: fra, ger, ita, esp, dan, hol, nor, por, swe) and tells whether it exists; any other language, 0'),
         ('liberar_tabelas_em_ingles', 'eng_lng_free_tables', '0x544f30', 'Database.cpp', 'thiscall',
          'solta (free) as 8 linhas não nulas da tabela do eng.lng, de +0x20 a +0x3c, sem zerar os ponteiros',
          'frees the 8 non-null rows of the eng.lng table, +0x20 to +0x3c, without clearing the pointers')])
CAMPOS = [('0x9b5984 idioma dos nomes; valor; lê (nome_ingles_do_estadio)', '0x9b5984 names language; value; reads (nome_ingles_do_estadio)'),
          ('0xae2408 tabela do eng.lng (+0x34 estádios em linhas de 52; +0x20..+0x3c as 8 linhas); ponteiro; lê (nome_ingles_do_estadio, liberar_tabelas_em_ingles)',
           '0xae2408 eng.lng table (+0x34 stadiums in 52-byte rows; +0x20..+0x3c the 8 rows); pointer; reads (nome_ingles_do_estadio, liberar_tabelas_em_ingles)'),
          ('0xde279c buffer do caminho (0x90bde0); texto; grava (arquivo_do_idioma_existe)', '0xde279c path buffer (0x90bde0); text; writes (arquivo_do_idioma_existe)')]
INTERESSA = ['SGE', 'CMRTE']
