# Database.cpp (leitor sx08, rodada 5, 23/11/2026): a carga do eng.lng (os nomes ingleses de clubes, nacoes,
# competicoes, cidades e estadios), EngLng::load.
# Convencao: thiscall(objeto de 0x40 bytes), sem argumentos na pilha, ret; devolve 1/0 em eax (os chamadores, os
# 7 primeiros usos 0x54545c..0x5459fc de nation/comp/club/stadium/..._get_english_name, ignoram). Sem SEH (o quadro
# SEH e dos chamadores, no new 0x40). 0x90bde0 cdecl(pasta, nome); fopen 0x945b00 cdecl(caminho, "rb"); 0x90c140
# cdecl(arquivo) o marcador; fread 0x945bfe cdecl(buf, tam, n, arq); malloc 0x944e46; fclose 0x945a09; a caixa de erro
# do Database.cpp (linhas 0x4dd2 sem arquivo, 0x4df1 contas que nao batem; a segunda fecha o arquivo antes).
# Roda uma vez na carreira de teste (cobertura: 1 chamada).
# Tipo: simulacao, provada pelo comparador (lado a lado: a original no objeto do jogo, a copia num objeto local; as
# tabelas da copia sao liberadas). Os nomes ingleses so aparecem em textos e telas com outro idioma: a carreira nao
# acusa mutacao desta rotina.
SAVE = False
ROTINAS = [('eng_lng_load_tables', 0x544fc0, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('a carga dos nomes ingleses (eng.lng)',
        'loading the English names (eng.lng)',
        [('eng_lng_load_tables', 'eng_lng_load_tables', '0x544fc0', 'Database.cpp', 'thiscall',
          'abre CM3_TEMP\\eng.lng (sem ele, caixa de erro e 0); lê o marcador de formato e o cabeçalho de 8 contas (invertendo os bytes no arquivo big-endian); se as contas de continentes, nações, cidades, estádios e competições de pessoal não batem com as tabelas carregadas, ou se as competições de nação mais as de clube não dão a conta de competições de clube menos 127, fecha, caixa de erro e 0 (a de clubes não é conferida); senão aloca e lê as 8 tabelas na ordem do arquivo (clubes 79 bytes, continentes 186, competições de nação 83, nações 109, estádios 52, cidades 52, competições de pessoal 79, competições de clube 83), fecha e devolve 1',
          'opens CM3_TEMP\\eng.lng (without it, error box and 0); reads the format marker and the 8-count header (swapping the bytes in a big-endian file); if the counts of continents, nations, cities, stadiums and staff competitions do not match the loaded tables, or the nation plus club competitions do not give the club competition count minus 127, closes, error box and 0 (the club count is not checked); otherwise allocates and reads the 8 tables in file order (clubs 79 bytes, continents 186, nation competitions 83, nations 109, stadiums 52, cities 52, staff competitions 79, club competitions 83), closes and returns 1')])
CAMPOS = [('objeto eng.lng ([0xae2408], 0x40 bytes): +0..+0x1c as 8 contas do cabeçalho (clubes, continentes, competições de nação, nações, cidades, competições de clube, estádios, competições de pessoal); +0x24 linhas de continente (186) e +0x30 linhas de cidade (52), sem nome no fields.rs; grava (eng_lng_load_tables)',
           'eng.lng object ([0xae2408], 0x40 bytes): +0..+0x1c the 8 header counts (clubs, continents, nation competitions, nations, cities, club competitions, stadiums, staff competitions); +0x24 continent rows (186) and +0x30 city rows (52), unnamed in fields.rs; writes (eng_lng_load_tables)')]
INTERESSA = []
