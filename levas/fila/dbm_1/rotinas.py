# Três nomes em inglês do eng.lng (idioma dos nomes [0x9b5984] diferente de 0 e 1), no molde da dbj_2. Nem a
# carreira nem a prova do save passam pelo caminho do eng.lng (o jogo dos testes em inglês): SAVE = True para
# agrupar com a leva do banco; a prova é o autoteste_dbm_1 (CM_AUTOTESTE=1 ou =banco, idiomas 0 a 13, tabela
# falsa). O quadro SEH só existe no primeiro uso (new 0x40 e a carga do eng.lng 0x544fc0): a cópia o deixa com
# a original, como a 0x545880 (224a) e a dbj_2 (232a).
SAVE = True
ROTINAS = [('sigla_inglesa_da_nacao', 0x5454c0, [0x6a, 0xff, 0x68, 0x7b, 0x79, 0x95, 0x00]),
           ('nome_ingles_da_competicao', 0x5455a0, [0x6a, 0xff, 0x68, 0x9b, 0x79, 0x95, 0x00]),
           ('nome_ingles_da_competicao_de_pessoal', 0x5456a0, [0x6a, 0xff, 0x68, 0xbb, 0x79, 0x95, 0x00])]
DOCS = ('os nomes em inglês da sigla da nação, da competição e da competição de pessoal',
        "the English names of the nation's three-letter code, the competition and the staff competition",
        [('sigla_inglesa_da_nacao', 'nation_get_english_three_letter_name', '0x5454c0', 'Database.cpp', 'cdecl',
          'a sigla da nação, ou a em inglês do eng.lng (linhas de 109 bytes, pelo id, sigla em +0x4f) quando outro idioma está carregado',
          "the nation's three-letter code, or its English one from eng.lng (109-byte rows, by id, code at +0x4f) when another language is loaded"),
         ('nome_ingles_da_competicao', 'comp_get_english_name', '0x5455a0', 'Database.cpp', 'cdecl',
          'o nome da competição, ou o em inglês do eng.lng (linhas de 83 bytes: de clube pelo id, de nação pelo id menos as de clube) quando outro idioma está carregado',
          "the competition's name, or its English one from eng.lng (83-byte rows: club ones by id, nation ones by id minus the club count) when another language is loaded"),
         ('nome_ingles_da_competicao_de_pessoal', 'staff_comp_get_english_name', '0x5456a0', 'Database.cpp', 'cdecl',
          'o nome da competição de pessoal, ou o em inglês do eng.lng (linhas de 79 bytes, pelo id) quando outro idioma está carregado',
          "the staff competition's name, or its English one from eng.lng (79-byte rows, by id) when another language is loaded")])
CAMPOS = [('0x9b5984 idioma dos nomes; valor; lê (as três)', '0x9b5984 names language; value; reads (all three)'),
          ('0xae2408 tabela do eng.lng (+0x14 quantas competições de clube, +0x28 competições de nação e +0x38 de clube em linhas de 83, +0x2c nações em linhas de 109, +0x3c competições de pessoal em linhas de 79); ponteiro; lê (as três)',
           '0xae2408 eng.lng table (+0x14 club competition count, +0x28 nation and +0x38 club competitions in 83-byte rows, +0x2c nations in 109-byte rows, +0x3c staff competitions in 79-byte rows); pointer; reads (all three)')]
INTERESSA = ['SGE', 'CMRTE']
