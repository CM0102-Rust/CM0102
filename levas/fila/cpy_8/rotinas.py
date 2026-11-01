# O árbitro de cada jogo (officials_manager.cpp): OfficialsManager::choose_match_official 0x79c7b0 (pela montagem do
# dia 0x6b3c10, contador "montagem avisada": todo jogo pendente do dia), OfficialsManager::clear_busy_flags 0x79bf40
# (laço do dia 0x5cccd0 e corrida até a data de início 0x5cc850) e OfficialsManager::renew_officials 0x79cee0 (começo
# da temporada de cada liga, 83 chamadores). Iguais no cm0102-1080.exe; sem SEH; o quadro de 0x208 da 0x79cee0 é só da
# caixa de erro. Comparadores lado a lado: a 0x79c7b0 pela palavra de saída, pelo estado do sorteio e pela devolução;
# a 0x79bf40 pelos bytes de ocupado; a 0x79cee0 pelas fichas de todos os árbitros, pelas linhas de estatísticas, pelo
# sorteio e por [0xb67a34].
# Convenções conferidas nos pontos de chamada das rotinas chamadas por endereço:
# - 0x5c6710 e_amistoso: `mov ecx, [0xae2c74]; push eax; call` = thiscall(amistosos, comp), 1 argumento (0x79c7e4).
# - 0x90cfc0 sortear: `push; call; add esp, 4` = cdecl(n) (0x79c85b, 0x79c8f5, 0x79c906, 0x79ca15, 0x79cf44...).
# - 0x4b0700 competicao_conta: `push ecx; call; add esp, 4` = cdecl(id) (0x79cf21).
# - 0x539c60 primeiro_nome_da_nacao: `push eax; call` (duas vezes) e `add esp, 0xc` depois do push 0xa e do sorteio
#   (0x79d07d..0x79d0ae: 2 nomes + o sorteio(10) num só ajuste) = cdecl(nacao).
# - 0x945640 (_ftol) é o `ftol` do motor.rs; a caixa de erro pelo `aviso_em` (0x944cff, 0x90d130, 0x944c9f, 0x5e8290,
#   todas cdecl como no motor.rs).
# - Os pontos de chamada da 0x79c7b0 (0x6b3d87: `mov ecx, [0xb74340]`, 2 push), da 0x79bf40 (0x5ccb21, 0x5cdabe: `mov
#   ecx, [0xb74340]`, sem push) e da 0x79cee0 (`mov ecx, [0xb74340]`, 1 push) conferem com thiscall.
SAVE = False
ROTINAS = [('escolher_arbitro_da_partida', 0x79c7b0, [0x83, 0xec, 0x08, 0x53, 0x55, 0x56]),
           ('limpar_arbitros_ocupados', 0x79bf40, [0x8b, 0x51, 0x04, 0x33, 0xc0]),
           ('renovar_arbitros', 0x79cee0, [0x81, 0xec, 0x08, 0x02, 0x00, 0x00])]
DOCS = ('o árbitro de cada jogo: a escolha do árbitro, as marcas de ocupado do dia e a renovação dos árbitros da liga',
        'the referee of each match: the referee choice, the daily busy flags and the renewal of the league referees',
        [('escolher_arbitro_da_partida', 'OfficialsManager::choose_match_official', '0x79c7b0', 'officials_manager.cpp', 'thiscall, ret 8',
          'a faixa de árbitros pela competição (a da liga no bloco da nação, o quarto de cima de uma nação neutra do continente nas competições dos tipos 2 e 4, a nação do mandante no amistoso), um sorteado nela e o seguinte livre',
          'the referee range for the competition (the league range in the nation block, the top quarter of a neutral nation of the continent in type 2 and 4 competitions, the home nation for a friendly), a random one in it and the next free one'),
         ('limpar_arbitros_ocupados', 'OfficialsManager::clear_busy_flags', '0x79bf40', 'officials_manager.cpp', 'thiscall, ret 0',
          'zera os bytes de árbitro ocupado no dia',
          'clears the daily referee busy bytes'),
         ('renovar_arbitros', 'OfficialsManager::renew_officials', '0x79cee0', 'officials_manager.cpp', 'thiscall, ret 4',
          'no começo da temporada da liga, troca os árbitros da faixa com mais de 45 anos (ou mais de 40, um em três) por novos sorteados e zera a linha de estatísticas do primeiro',
          'at the league season start, replaces the referees of the range over 45 (or over 40, one in three) with new random ones and clears the stats row of the first one')])
CAMPOS = [('OfficialsManager +0 linhas de estatísticas (17 bytes por id: +4 média, +8, +0xa, +0xc palavras, +0xf cartões), +4 conta e +8 vetor dos bytes de ocupado, +0xc blocos por nação (0x102 bytes: +4 conta, +8 primeiro, +0x10 número de faixas, +0x12 faixas de 16 bytes: +0 primeiro, +4 conta, +0xc id da competição); ponteiros/valores; lê (escolher_arbitro_da_partida, renovar_arbitros), grava os bytes de ocupado (limpar_arbitros_ocupados) e a linha de estatísticas (renovar_arbitros)',
           'OfficialsManager +0 stats rows (17 bytes per id: +4 average, +8, +0xa, +0xc words, +0xf cards), +4 count and +8 busy byte array, +0xc per-nation blocks (0x102 bytes: +4 count, +8 first, +0x10 range count, +0x12 16-byte ranges: +0 first, +4 count, +0xc competition id); pointers/values; reads (escolher_arbitro_da_partida, renovar_arbitros), writes the busy bytes (limpar_arbitros_ocupados) and the stats row (renovar_arbitros)'),
          ('árbitro (0x2b bytes em [0xae23e8], [0xae2390] deles) +0 id, +4 e +8 nomes, +0xc dia e +0xe ano de nascimento, +0x16 nação, +0x25 nota; ids/ponteiros/valores; lê +0 (escolher_arbitro_da_partida), lê e grava +4, +8, +0xc, +0xe, +0x25 (renovar_arbitros)',
           'referee (0x2b bytes at [0xae23e8], [0xae2390] of them) +0 id, +4 and +8 names, +0xc birth day and +0xe year, +0x16 nation, +0x25 rating; ids/pointers/values; reads +0 (escolher_arbitro_da_partida), reads and writes +4, +8, +0xc, +0xe, +0x25 (renovar_arbitros)'),
          ('jogo +0 (−2 tirado), +0x14 competição, +0x1c e +0x20 clubes, +0x2e árbitro (palavra, a saída); competição +0 id, +0x59 continente, +0x5d nação; objeto da competição ([0xadadfc]) +0x42 tipo; clube +0x53 nação; nação (0x122 bytes em [0xae23a8]) +0 id, +0x71 continente; ids/ponteiros; lê (escolher_arbitro_da_partida, renovar_arbitros)',
           'fixture +0 (−2 removed), +0x14 competition, +0x1c and +0x20 clubs, +0x2e referee (word, the output); competition +0 id, +0x59 continent, +0x5d nation; competition object ([0xadadfc]) +0x42 type; club +0x53 nation; nation (0x122 bytes at [0xae23a8]) +0 id, +0x71 continent; ids/pointers; reads (escolher_arbitro_da_partida, renovar_arbitros)'),
          ('[0xae2c90] data do jogo (dia, ano); [0xae2350] nações, [0xae234c] continentes; valores; lê',
           '[0xae2c90] game date (day, year); [0xae2350] nations, [0xae234c] continents; values; reads')]
INTERESSA = ['CMRTE', 'GPF2']
