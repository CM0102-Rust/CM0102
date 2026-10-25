# O conversor das competições da versão 1 (0x54feb0, já em Rust, o chama pelo endereço com a versão 1) e dois
# nomes em inglês do eng.lng (o idioma do jogo diferente de 0 e 1). Nem a carreira nem a prova do save passam por
# elas (base e save versão 2; o jogo dos testes em inglês): SAVE = True só para agrupar com a leva do banco. A
# prova é o autoteste_dbj_2 (CM_AUTOTESTE=1). Os nomes têm quadro SEH só para o primeiro uso (new 0x40 e a carga
# do eng.lng 0x544fc0): a cópia o deixa com a original, como a 0x545880 (224a leva). O conversor não tem SEH
# nem variável local lida sem valor.
SAVE = True
ROTINAS = [('ler_competicoes_v1', 0x54ffd0, [0x8b, 0x44, 0x24, 0x10, 0x53]),
           ('nome_ingles_da_nacao', 0x5453f0, [0x6a, 0xff, 0x68, 0x5b, 0x79, 0x95, 0x00]),
           ('nome_curto_ingles_do_clube', 0x545770, [0x6a, 0xff, 0x68, 0xdb, 0x79, 0x95, 0x00])]
DOCS = ('o conversor das competições da versão 1 e os nomes em inglês da nação e o curto do clube',
        "the version 1 competition converter and the nation's English name and the club's English short name",
        [('ler_competicoes_v1', 'db_files_read_comps_v1', '0x54ffd0', 'Db_files.cpp', 'cdecl',
          'lê o club_comp.dat / nation_comp.dat versão 1 campo a campo (104 bytes no arquivo para registros de 107, buracos em +0x37 e +0x52), a reputação num byte; 0 na primeira leitura que não vem, 1 se veio tudo',
          'reads version 1 club_comp.dat / nation_comp.dat field by field (104 bytes on file for 107-byte records, holes at +0x37 and +0x52), the reputation in one byte; 0 at the first read that does not come, 1 if everything came'),
         ('nome_ingles_da_nacao', 'nation_get_english_name', '0x5453f0', 'Database.cpp', 'cdecl',
          'o nome da nação, ou o em inglês do eng.lng (linhas de 109 bytes, pelo id) quando outro idioma está carregado',
          "the nation's name, or its English one from eng.lng (109-byte rows, by id) when another language is loaded"),
         ('nome_curto_ingles_do_clube', 'club_get_english_short_name', '0x545770', 'Database.cpp', 'cdecl',
          'o nome curto do clube, ou o em inglês do eng.lng (linhas de 79 bytes, pelo mapa de ids ou pelo id) quando outro idioma está carregado',
          "the club's short name, or its English one from eng.lng (79-byte rows, through the id map or by id) when another language is loaded")])
CAMPOS = [('TComp (107 bytes, [0xae23d0 / 0xae23d8]): +0x69 reputação (u16; versão 1: byte); registro inteiro menos +0x37 e +0x52; grava (ler_competicoes_v1)', 'TComp (107 bytes, [0xae23d0 / 0xae23d8]): +0x69 reputation (u16; version 1: byte); whole record except +0x37 and +0x52; writes (ler_competicoes_v1)'),
          ('0x9b5984 idioma dos nomes; valor; lê (nome_ingles_da_nacao, nome_curto_ingles_do_clube)', '0x9b5984 names language; value; reads (nome_ingles_da_nacao, nome_curto_ingles_do_clube)'),
          ('0xae2408 tabela do eng.lng (+0 quantos clubes, +0x20 clubes em linhas de 79, +0x2c nações em linhas de 109); ponteiro; lê (as duas)', '0xae2408 eng.lng table (+0 club count, +0x20 clubs in 79-byte rows, +0x2c nations in 109-byte rows); pointer; reads (both)'),
          ('0xae23fc mapa de ids dos clubes do eng.lng; ponteiro; lê (nome_curto_ingles_do_clube)', '0xae23fc eng.lng club id map; pointer; reads (nome_curto_ingles_do_clube)')]
INTERESSA = ['SGE', 'CMRTE']
