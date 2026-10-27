# Bloco das competições, a tabela da liga (league.cpp): ordenar a tabela (0x6827d0, 15.464 chamadas na carreira:
# depois de cada rodada, nas telas e nas cópias da 0x687710), a minitabela do confronto direto (0x682b30, 179: a
# regra 4 da 0x6827d0; entra no lugar da 0x686940 update_staff_records_for_status, que só despacha para as três
# rotinas grandes do staff_records.cpp com SEH, efeitos de reputação e notícias: sem comparador de verdade possível),
# liberar a tabela (0x687970, 6.093: as cópias da 0x687710 e a da liga) e conferir o fim da liga (0x689ad0, 26.377).
# Simulação: as mutações pela carreira até 2002-06-01 e a confirmação no jogo real comparando. Sem SEH, sem sorteio,
# sem variável local lida sem valor (a 0x6827d0 grava todas antes de ler; a linha temporária da troca só é lida
# depois de copiada). Caixa: só a da 0x682b30 sem memória (fica com a original). Comparadores: a ordenação e a
# minitabela pelas linhas, o histórico de posições e as globais 0xb679fc/0xb67a00; a liberação só a original (com
# ramo, como as da 232ª: liberar duas vezes soltaria os mesmos blocos); a 0x689ad0 lado a lado no caminho que só lê
# (o comum: algum clube sem todos os jogos; +0xa7 na região) e só a original no fim da liga (registros 0x86a070,
# prêmios 0x682e10, fases pela virtual +0x10, aviso +0x94). Em laço quente: a 0x689ad0 (26 mil) e a 0x6827d0 (15 mil);
# correm o risco da 0x547770 (233ª), de o desvio mudar o resíduo de pilha que uma original lê depois.
SAVE = False
ROTINAS = [('ordenar_tabela_da_liga', 0x6827d0, [0x83, 0xec, 0x4c, 0x53, 0x55]),
           ('mini_tabela_do_confronto', 0x682b30, [0x81, 0xec, 0x10, 0x02, 0x00, 0x00]),
           ('liberar_tabela_da_liga', 0x687970, [0x51, 0x53, 0x55, 0x8b, 0x6c, 0x24, 0x10]),
           ('liga_completa', 0x689ad0, [0x53, 0x56, 0x8b, 0xf1, 0x57])]
DOCS = ('a tabela da liga: ordenar, a minitabela do confronto direto, liberar e conferir o fim',
        'the league table: sorting, the head-to-head mini table, freeing and checking completion',
        [('ordenar_tabela_da_liga', 'League::sort_table', '0x6827d0', 'league.cpp', 'thiscall',
          'ordena a tabela pelas regras (+0xc5): a regra 6 só ordena (0x683210); a 4 monta a minitabela e, sendo a primeira, ordena pelo confronto (0x683410) e passa bolha nos empates; grava a posição (+4) e o histórico de posições (+9); devolve 1',
          'sorts the table by the rules (+0xc5): rule 6 only sorts (0x683210); rule 4 builds the mini table and, when first, sorts by head-to-head (0x683410) and bubbles the ties; stores the position (+4) and the position history (+9); returns 1'),
         ('mini_tabela_do_confronto', 'League::build_h2h_table', '0x682b30', 'league.cpp', 'thiscall',
          'minitabela dos clubes empatados em pontos com a soma dos confrontos diretos, ordenada pela 0x6834f0; a posição (+4) começa no número de clubes e desce a cada par separado',
          'mini table of the clubs level on points with their summed head-to-head results, sorted by 0x6834f0; the position (+4) starts at the club count and drops at each separated pair'),
         ('liberar_tabela_da_liga', 'League::free_table', '0x687970', 'league.cpp', 'thiscall',
          'solta os vetores +5 e +9 das linhas e a tabela (nula: a da liga, e então zera +0x3e e +0xb1)',
          'frees the +5 and +9 row buffers and the table (null: the league one, then clears +0x3e and +0xb1)'),
         ('liga_completa', 'League::check_complete', '0x689ad0', 'league.cpp', 'thiscall',
          '1 quando todos os clubes jogaram as +0xa7 rodadas (e as fases, se pedidas); no fim da liga, os registros (0x86a070), os prêmios (0x682e10), o aviso (+0x94) e +0xdd',
          '1 when every club played the +0xa7 rounds (and the stages, if asked); at the end of the league, the records (0x86a070), the prize money (0x682e10), the notice (+0x94) and +0xdd')])
CAMPOS = [('League +0x3e clubes (curto), +0x3c turnos, +0xa7 total de rodadas (curto; −1 = calcular), +0xad coluna, +0xb1 tabela, +0xc5 regras, +0xa3 comparação própria, +0x44 fase, +0xdd completa, +0xe6, +0xa9, +0x2c/+0x30/+0xc fases; lê e grava (as quatro)',
           'League +0x3e clubs (short), +0x3c legs, +0xa7 total rounds (short; −1 = compute), +0xad column, +0xb1 table, +0xc5 rules, +0xa3 own compare, +0x44 stage, +0xdd complete, +0xe6, +0xa9, +0x2c/+0x30/+0xc stages; reads and writes (all four)'),
          ('linha da tabela (0x3b bytes): +0 clube, +4 posição, +5 confrontos (entradas de 14), +9 histórico de posições, +0x11 jogos, +0x19 pontos; lê e grava (ordenar_tabela_da_liga, mini_tabela_do_confronto)',
           'table row (0x3b bytes): +0 club, +4 position, +5 head-to-head (14-byte entries), +9 position history, +0x11 played, +0x19 points; reads and writes (ordenar_tabela_da_liga, mini_tabela_do_confronto)'),
          ('[0xb679fc] liga sendo ordenada, [0xb67a00] comparação própria; grava (ordenar_tabela_da_liga)',
           '[0xb679fc] league being sorted, [0xb67a00] own compare; writes (ordenar_tabela_da_liga)')]
INTERESSA = ['SGE', 'CMRTE']
