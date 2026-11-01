# O save do calendário (fix_man.cpp): fix_man_save_club_dates (0x5af040, do FIXTURE_MANAGER::save 0x5ac060 no passo
# 0x18 do save_game_data, 0x5ac394: o club_dates.tmp, uma palavra por clube de [0xae2a5c]; 5.040 bytes no .qst da
# prova), fix_man_load_club_dates (0x5aee90, do construtor do calendário 0x5a9cf0, 0x5a9e38: na carga do save lê o
# club_dates.tmp; na carreira nova, sem o arquivo, zera a tabela; 1 chamada na cobertura de 17/10) e
# fix_man_save_fixture_leg_checks (0x5af4a0, do mesmo save, 0x5ac399: o check_fixture_legs.tmp, a conta e os itens
# de 15 bytes da lista [0xae2a48]; passageira: no .qst da prova a lista está vazia e o arquivo não sai). Os
# .tmp do calendário entram no contêiner do save como os outros (MAPA_SAVE.md, a tabela dos .tmp globais; tmp_globais.py lê os
# dois). Iguais
# no cm0102-1080.exe; sem SEH; sem x87.
# Convenções conferidas nos pontos de chamada da original (lição da 258ª):
# - 0x90bde0 (caminho em CM3_TEMP): push modo (fica para a seguinte); push nome; push "CM3_TEMP"; call; add esp, 8
#   -> cdecl(pasta, nome).
# - 0x945b00 (fopen): push caminho (com o modo já empurrado); call; add esp, 8 -> cdecl(caminho, modo).
# - 0x945ce6 (fwrite) e 0x945bfe (fread): 4 push; call; add esp, 0x10 -> cdecl(ptr, tamanho, quantos, arquivo).
# - 0x945a09 (fclose): push; call; add esp, 4 -> cdecl(arquivo).
# - 0x944e46 (malloc): push; call; add esp, 4 -> cdecl(n).
# - 0x68d1b0 (próximo item da lista): mov ecx, 0xae2a48; call; sem push nem add esp -> thiscall(lista), ret 0.
# - a caixa de erro (aviso_em do motor.rs) com o fonte 0x9cc208 (fix_man.cpp), como as outras.
# Comparadores: só a original (gravam ou leem arquivos do save; a carga aloca a tabela global); a prova é a do save
# (o .qst gravado com e sem a leva) e, pela 0x5aee90, a carga do FeriasTesteSave (CARGA).
SAVE = True
CARGA = True  # a 0x5aee90 lê o club_dates.tmp só na carga: a prova do save também carrega o save de férias
ROTINAS = [('gravar_datas_dos_clubes', 0x5af040, [0xa1, 0x5c, 0x2a, 0xae, 0x00]),
           ('carregar_datas_dos_clubes', 0x5aee90, [0xa1, 0x5c, 0x2a, 0xae, 0x00]),
           ('gravar_conferencia_das_pernas', 0x5af4a0, [0x81, 0xec, 0x04, 0x02, 0x00, 0x00])]
DOCS = ('o save do calendário: gravar e carregar as datas dos clubes e gravar a conferência das pernas',
        'saving the fixture calendar: writing and reading the club dates and writing the leg checks',
        [('gravar_datas_dos_clubes', 'fix_man_save_club_dates', '0x5af040', 'fix_man.cpp', 'cdecl',
          'grava em CM3_TEMP\\club_dates.tmp a tabela [0xae2a5c], uma palavra por clube ([0xae235c]); sem a tabela ou com gravação curta, a caixa "Error" (fix_man.cpp 0x1011, 0x101b)',
          'writes the [0xae2a5c] table, one word per club ([0xae235c]), into CM3_TEMP\\club_dates.tmp; without the table or on a short write, the "Error" box (fix_man.cpp 0x1011, 0x101b)'),
         ('carregar_datas_dos_clubes', 'fix_man_load_club_dates', '0x5aee90', 'fix_man.cpp', 'cdecl',
          'aloca a tabela [0xae2a5c] (uma palavra por clube) e a lê de CM3_TEMP\\club_dates.tmp, ou a zera sem o arquivo; já alocada, sem memória ou com leitura curta, a caixa "Error" (0xfe4, 0xfec, 0xff6)',
          'allocates the [0xae2a5c] table (one word per club) and reads it from CM3_TEMP\\club_dates.tmp, or zeroes it without the file; already allocated, out of memory or on a short read, the "Error" box (0xfe4, 0xfec, 0xff6)'),
         ('gravar_conferencia_das_pernas', 'fix_man_save_fixture_leg_checks', '0x5af4a0', 'fix_man.cpp', 'cdecl',
          'com a lista [0xae2a48] não vazia, grava em CM3_TEMP\\check_fixture_legs.tmp a conta (2 bytes) e os itens de 15 bytes; gravação curta de um item, a caixa "Error" (0x10e0)',
          'with the [0xae2a48] list not empty, writes the count (2 bytes) and the 15-byte items into CM3_TEMP\\check_fixture_legs.tmp; on a short item write, the "Error" box (0x10e0)')])
CAMPOS = [('[0xae2a5c] as datas dos clubes (uma palavra por clube: bit 0 o primeiro ano, o resto o último); ponteiro; lê e grava no save (gravar_datas_dos_clubes), aloca e grava da carga (carregar_datas_dos_clubes)',
           '[0xae2a5c] the club dates (one word per club: bit 0 the first year, the rest the last); pointer; reads and writes into the save (gravar_datas_dos_clubes), allocates and writes from the load (carregar_datas_dos_clubes)'),
          ('[0xae2a48] a lista de conferência das pernas (LLIST: +0 conta, +2 cabeça, +0xa cursor; itens de 15 bytes); lê e grava o cursor (gravar_conferencia_das_pernas)',
           '[0xae2a48] the fixture leg check list (LLIST: +0 count, +2 head, +0xa cursor; 15-byte items); reads and writes the cursor (gravar_conferencia_das_pernas)'),
          ('[0xae235c] quantidade de clubes; valor; lê (gravar_datas_dos_clubes, carregar_datas_dos_clubes)',
           '[0xae235c] club count; value; reads (gravar_datas_dos_clubes, carregar_datas_dos_clubes)')]
INTERESSA = ['SGE']
