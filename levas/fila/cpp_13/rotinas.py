# As copas e a divisão dos clubes (Cup.cpp, league.cpp): a ordem dos cabeças de chave do sorteio, decrescente (o
# comparador do qsort 0x522190, dos sorteios da Copa da UEFA 0x902b90, da Intertoto 0x6336c0, das copas do Japão
# 0x6624f0/0x665a00 e da copa da liga escocesa 0x7f0400) e crescente (0x5221c0, da copa da liga inglesa 0x56ccc0): o
# nível da entrada (+4) e, no empate, a reputação do clube (+0x80); e o teto da divisão dos clubes
# (League::limit_club_division_tier 0x68a980, do fim de temporada de 14 ligas, eng_prm 0x574c10 entre elas: ajusta o
# byte +0x64 do clube pela situação da linha da tabela). Sem SEH, sem sorteio, sem chamadas; os três primeiros
# argumentos da 0x68a980 são bytes empurrados como dword (só o byte baixo conta). Iguais no cm0102-1080.exe. Os
# arquivos das copas e das ligas não entraram na cobertura de 17/10: as chamadas na carreira não foram contadas (os
# sorteios da Copa da UEFA e da copa da liga inglesa são da temporada 2001/02; o fim de temporada das ligas pode cair
# depois de 2002-06-01: o ramo diz). Simulação: as mutações pela carreira e a confirmação no jogo real comparando.
# Comparadores: as duas ordens pelo valor; o teto lado a lado no byte +0x64 dos clubes da tabela.
SAVE = False
ROTINAS = [('ordem_dos_cabecas_de_chave_decrescente', 0x522190, [0x8b, 0x4c, 0x24, 0x04, 0x8b, 0x54, 0x24, 0x08]),
           ('ordem_dos_cabecas_de_chave_crescente', 0x5221c0, [0x8b, 0x4c, 0x24, 0x04, 0x8b, 0x54, 0x24, 0x08]),
           ('teto_da_divisao_dos_clubes', 0x68a980, [0x56, 0x57, 0x8b, 0xf9, 0x33, 0xf6])]
DOCS = ('as copas e a divisão dos clubes: a ordem dos cabeças de chave do sorteio (decrescente e crescente) e o teto da divisão dos clubes no fim da temporada',
        'cups and club divisions: the order of seeds in the draw (descending and ascending) and the club division tier at the end of the season',
        [('ordem_dos_cabecas_de_chave_decrescente', 'cup_compare_entries_by_level_desc', '0x522190', 'Cup.cpp', 'cdecl',
          'comparador do qsort das entradas da copa pelo nível (+4), o maior primeiro; no empate pela reputação do clube (+0x80), a menor primeiro',
          'qsort comparator of cup entries by level (+4), highest first; on a tie by club reputation (+0x80), lowest first'),
         ('ordem_dos_cabecas_de_chave_crescente', 'cup_compare_entries_by_level_asc', '0x5221c0', 'Cup.cpp', 'cdecl',
          'comparador do qsort das entradas da copa pelo nível (+4), o menor primeiro; no empate pela reputação do clube (+0x80), a menor primeiro',
          'qsort comparator of cup entries by level (+4), lowest first; on a tie by club reputation (+0x80), lowest first'),
         ('teto_da_divisao_dos_clubes', 'League::limit_club_division_tier', '0x68a980', 'league.cpp', 'thiscall, ret 0x10',
          'para cada linha da tabela, leva o byte +0x64 do clube ao valor (para baixo ou para cima) quando a situação da linha não é a de fora e quando é a de dentro (0xfd = nenhuma)',
          'for each table row, moves the club byte +0x64 to the value (down or up) when the row status is not the excluded one and when it is the included one (0xfd = none)')])
CAMPOS = [('entrada de copa +0: o clube; ponteiro; +4 o nível; valor; lê (as duas ordens)',
           'cup entry +0: the club; pointer; +4 the level; value; reads (both orders)'),
          ('clube +0x80: a reputação; valor; lê (as duas ordens)', 'club +0x80: the reputation; value; reads (both orders)'),
          ('liga +0x3e: a conta das linhas da tabela; +0xb1 a tabela (0x3b bytes: +0 o clube, +0x37 a situação no fim da temporada); valor e ponteiro; lê (teto_da_divisao_dos_clubes)',
           'league +0x3e: the table row count; +0xb1 the table (0x3b bytes: +0 the club, +0x37 the end-of-season status); value and pointer; reads (teto_da_divisao_dos_clubes)'),
          ('clube +0x64: o teto da divisão (byte com sinal); valor; lê e grava (teto_da_divisao_dos_clubes)',
           'club +0x64: the division tier (signed byte); value; reads and writes (teto_da_divisao_dos_clubes)')]
INTERESSA = ['CMRTE', 'SGE']
