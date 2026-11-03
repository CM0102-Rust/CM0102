# Os árbitros do jogo novo (officials_manager.cpp), pedido de 02/11 (agente C, família das pessoas):
# - 0x79bf60 (OfficialsManager::set_up_officials, 1488 bytes): revê os registros (0x79caa0, já em Rust), fica com os
#   primeiros de cada nação pela qualidade (qsort 0x79d4e0), monta os blocos por nação com as vagas das ligas, sorteia
#   os árbitros que faltam (0x79c5c0, já em Rust) e reparte os árbitros da nação pelas vagas. 1 chamada por carreira (o
#   construtor 0x79b580 no jogo novo da carreira automática; cobertura de 02/11). As mutações estão nela.
# - 0x79d410 (officials_manager_compare_comp_level, 104 bytes): a ordem das vagas de competição de cada nação (qsort da
#   0x79bf60): passageira.
# Iguais no cm0102-1080.exe (as duas inteiras, conferido byte a byte).
# Trocas: a 0x79caa0 (a revisão), a 0x79c5c0 (os sorteados) e a 0x79c530 (a cópia do registro) já estavam no motor.rs
# (cpy_9, 261ª): ficaram de fora deste item (a escolha inicial as repetia; achado pelo desvio recusado no ddraw.log).
# Convenções conferidas nos pontos de chamada da original (lição da 258ª):
#   0x79bf60: thiscall (mov ecx, esi em 0x79b746), 0 (ret). Dentro: 0x79caa0 cdecl, 0; 0x945333 qsort cdecl (add esp,
#   0x18 junto do calloc), 4; 0x945501 calloc cdecl, 2 (n, tamanho); 0x79c530 thiscall (mov ecx, ebp = o destino; push
#   origem), 1; 0x9452ca free cdecl, 1; 0x944e46 malloc cdecl, 1; 0x4b0700 cdecl (add esp, 4), 1; 0x5c6710 thiscall (mov
#   ecx, [0xae2c74]), 1 (a competição); 0x79c5c0 (mov ecx, esi; push nação; push conta; ret 8: a Rust a declara
#   thiscall com o this não lido), 2; o fim fatal sem memória (aviso_em, 0x5cc7d0, 0x5a8f60, 0x945934 com −1).
#   0x79d410: cdecl (ret), 2: ponteiro passado ao qsort em 0x79c3f7.
# Leitura de pilha: nenhuma. A original lê de volta, depois do free da marcação e da tabela velha, só a tabela nova.
# Comparadores: 0x79bf60 só a original (aloca, libera, troca a tabela dos árbitros e as chamadas sorteiam: não pode
# rodar duas vezes) com o plano (os blocos, a conta igual à da tabela, as marcas). 0x79d410 pela devolução (só lê).
SAVE = False
ROTINAS = [('montar_os_arbitros', 0x79bf60, [0x81, 0xec, 0x0c, 0x02, 0x00, 0x00]),
           ('ordem_das_vagas_de_competicao', 0x79d410, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x4c, 0x24, 0x08])]
DOCS = ('os árbitros do jogo novo: os que ficam por nação, as vagas das competições e a parte de cada uma; a ordem das vagas pelo nível da competição',
        'the new game officials: the ones kept per nation, the competition slots and the share of each; the slot order by competition level',
        [('montar_os_arbitros', 'OfficialsManager::set_up_officials', '0x79bf60', 'officials_manager.cpp', 'thiscall',
          'no jogo novo: revê os árbitros, ordena por nação e qualidade, fica com 6 por nação (61 nas de clubes de base), monta os blocos por nação com uma vaga por liga (o pedido pela competição), sorteia os que faltam e reparte os árbitros da nação pelas vagas',
          'on a new game: reviews the officials, sorts them by nation and quality, keeps 6 per nation (61 in the youth club nations), builds the per-nation blocks with one slot per league (the demand from the competition), draws the missing ones and shares the nation officials among the slots'),
         ('ordem_das_vagas_de_competicao', 'officials_manager_compare_comp_level', '0x79d410', 'officials_manager.cpp', 'cdecl',
          'ordena as vagas de árbitros de uma nação pelo nível da competição (as inválidas por último ou primeiro), empate pelo id',
          'orders the official slots of a nation by competition level (the invalid ones last or first), tie by id')])
CAMPOS = [('árbitro (0x2b bytes em [0xae23e8], conta [0xae2390]): +0 índice, +0x16 nação, +0x22/+0x28 a qualidade na ordem; valor/ponteiro; lê e grava (montar_os_arbitros)',
           'official (0x2b bytes at [0xae23e8], count [0xae2390]): +0 index, +0x16 nation, +0x22/+0x28 the quality in the order; value/pointer; reads and writes (montar_os_arbitros)'),
          ('OfficialsManager +4 conta, +8 marcas (calloc), +0xc blocos por nação (0x102 bytes: +0 nação, +4 árbitros, +8 primeiro índice, +0xc pedido, +0x10 competições, +0x12 15 vagas de 16 bytes: +0 início, +4 parte, +8 pedido, +0xc competição); valor; grava',
           'OfficialsManager +4 count, +8 flags (calloc), +0xc per-nation blocks (0x102 bytes: +0 nation, +4 officials, +8 first index, +0xc demand, +0x10 competitions, +0x12 15 slots of 16 bytes: +0 start, +4 share, +8 demand, +0xc competition); value; writes'),
          ('nação +0x8e a quantidade padrão de árbitros (1..5; 0 vira 1), +0x11c; valor; lê e grava; competição (0x6b bytes em [0xae23d0]) +0x5d nação, +0x69 nível; competição de clube ([0xadadfc][k]) +0x3e o pedido de árbitros [inferência]; valor; lê',
           'nation +0x8e the default official count (1..5; 0 becomes 1), +0x11c; value; reads and writes; competition (0x6b bytes at [0xae23d0]) +0x5d nation, +0x69 level; club competition ([0xadadfc][k]) +0x3e the official demand [inference]; value; reads')]
INTERESSA = ['CMRTE', 'SGE']
