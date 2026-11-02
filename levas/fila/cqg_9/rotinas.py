# Os árbitros do jogo novo (officials_manager.cpp), pedido de 02/11 (agente C, família das pessoas):
# - 0x79bf60 (OfficialsManager::set_up_officials, 1488 bytes): revê os registros, fica com os primeiros de cada nação
#   pela qualidade, monta os blocos por nação com as vagas das competições e sorteia os árbitros que faltam. 1 chamada
#   por carreira (o construtor 0x79b580 no jogo novo da carreira automática; cobertura de 02/11). As mutações estão nela
#   e na 0x79c5c0.
# - 0x79c5c0 (officials_manager_add_random_officials, 496 bytes): os árbitros novos sorteados (nomes, notas, atributos,
#   nascimento); pela 0x79bf60, uma vez por nação que precisa.
# - 0x79caa0 (officials_manager_validate_officials, 880 bytes): a revisão dos registros (nação, notas e atributos fora
#   da faixa e nascimento antigo sorteados de novo); pela 0x79bf60.
# Iguais no cm0102-1080.exe (as três inteiras, conferido byte a byte).
# Convenções conferidas nos pontos de chamada da original (lição da 258ª):
#   0x79bf60: thiscall (mov ecx, esi em 0x79b746), 0 (ret). Dentro: 0x79caa0 cdecl, 0; 0x945333 qsort cdecl (add esp,
#   0x18 junto do calloc), 4; 0x945501 calloc cdecl, 2 (n, tamanho); 0x79c530 thiscall (mov ecx, ebp = o destino; push
#   origem), 1; 0x9452ca free cdecl, 1; 0x944e46 malloc cdecl, 1; 0x4b0700 cdecl (add esp, 4), 1; 0x5c6710 thiscall (mov
#   ecx, [0xae2c74]), 1 (a competição); 0x79c5c0 (mov ecx, esi; push nação; push conta; ret 8, o ecx não é lido:
#   stdcall), 2; o fim fatal sem memória (aviso_em, 0x5cc7d0, 0x5a8f60, 0x945934 com −1).
#   0x79c5c0: stdcall, 2 (ret 8: conta, nação). Dentro: 0x945667 realloc cdecl (add esp, 8), 2; 0x539b90 e 0x53a0e0
#   cdecl, 1 (a nação; os 14 dwords saem no add esp, 0x38 do fim de cada volta); 0x90cfc0 cdecl, 1; 0x945333 qsort
#   cdecl (add esp, 0x10), 4 (com 0x79d480).
#   0x79caa0: cdecl, 0 (ret; chamada em 0x79bf6c sem push). Dentro: 0x90cfc0 cdecl, 1; 0x549f70 thiscall (lea ecx, a
#   data local), 4 (1, 1, 0x776, −1).
# Leitura de memória: os árbitros novos da 0x79c5c0 ficam com +0..+3 e +0x10..+0x15 como o realloc deixou (a cópia faz
#   igual: não grava); o +0 vira o índice na 0x79bf60.
# Comparadores: 0x79caa0 lado a lado nos registros dos árbitros e no estado do sorteio (os sorteios da cópia fora da
# contagem). 0x79bf60 e 0x79c5c0 só a original (alocam, liberam e sorteiam) com o plano (os blocos, as contas).
SAVE = False
ROTINAS = [('montar_os_arbitros', 0x79bf60, [0x81, 0xec, 0x0c, 0x02, 0x00, 0x00]),
           ('rever_os_arbitros', 0x79caa0, [0x8b, 0x0d, 0x90, 0x23, 0xae, 0x00]),
           ('sortear_arbitros_novos', 0x79c5c0, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('os árbitros do jogo novo: a revisão dos registros, os que ficam por nação, as vagas das competições e os árbitros novos sorteados',
        'the new game officials: the record review, the ones kept per nation, the competition slots and the new random officials',
        [('montar_os_arbitros', 'OfficialsManager::set_up_officials', '0x79bf60', 'officials_manager.cpp', 'thiscall',
          'no jogo novo: revê os árbitros, ordena por nação e qualidade, fica com 6 por nação (61 nas de clubes de base), monta os blocos por nação com uma vaga por liga (o pedido pela competição) e sorteia os que faltam',
          'on a new game: reviews the officials, sorts them by nation and quality, keeps 6 per nation (61 in the youth club nations), builds the per-nation blocks with one slot per league (the demand from the competition) and draws the missing ones'),
         ('rever_os_arbitros', 'officials_manager_validate_officials', '0x79caa0', 'officials_manager.cpp', 'cdecl',
          'acerta cada árbitro: a nação pelos nomes ou sorteada, as notas e os atributos fora da faixa sorteados e o nascimento antes de 1910 levado aos anos 60',
          'fixes every official: the nation from the names or drawn, the out-of-range ratings and attributes drawn and the birth before 1910 moved to the 1960s'),
         ('sortear_arbitros_novos', 'officials_manager_add_random_officials', '0x79c5c0', 'officials_manager.cpp', 'stdcall',
          'acrescenta árbitros sorteados de uma nação (nomes, notas, atributos, nascimento) e reordena a tabela',
          'adds random officials of a nation (names, ratings, attributes, birth) and re-sorts the table')])
CAMPOS = [('árbitro (0x2b bytes em [0xae23e8], conta [0xae2390]): +0 índice, +4/+8 nomes, +0xc dia e +0xe ano do nascimento, +0x16 nação, +0x1a, +0x1e/+0x20/+0x22 notas [inferência: atual, potencial, reputação], +0x24..+0x2a atributos [inferência]; valor/ponteiro; lê e grava',
           'official (0x2b bytes at [0xae23e8], count [0xae2390]): +0 index, +4/+8 names, +0xc birth day and +0xe birth year, +0x16 nation, +0x1a, +0x1e/+0x20/+0x22 ratings [inference: current, potential, reputation], +0x24..+0x2a attributes [inference]; value/pointer; reads and writes'),
          ('OfficialsManager +4 conta, +8 marcas (calloc), +0xc blocos por nação (0x102 bytes: +0 nação, +4 árbitros, +8 primeiro índice, +0xc pedido, +0x10 competições, +0x12 15 vagas de 16 bytes: início, parte, pedido, competição); valor; grava',
           'OfficialsManager +4 count, +8 flags (calloc), +0xc per-nation blocks (0x102 bytes: +0 nation, +4 officials, +8 first index, +0xc demand, +0x10 competitions, +0x12 15 slots of 16 bytes: start, share, demand, competition); value; writes'),
          ('nação +0x71 continente, +0x8e a quantidade padrão de árbitros (1..5; 0 vira 1), +0x11c; valor; lê e grava; competição +0x5d nação; competição de clube ([0xadadfc][k]) +0x3e o pedido de árbitros [inferência]; valor; lê',
           'nation +0x71 continent, +0x8e the default official count (1..5; 0 becomes 1), +0x11c; value; reads and writes; competition +0x5d nation; club competition ([0xadadfc][k]) +0x3e the official demand [inference]; value; reads')]
INTERESSA = ['CMRTE', 'SGE', 'GPF2']
