# Diversos, 7º item: o olheiro da busca (PLAYER_SEARCH::get_scout 0x7ae950, 159 chamadas na carreira até 2002-06-01;
# player_search.cpp; 0x53e990 já em Rust), a sequência atual do clube na liga (MANAGER_MANAGER::get_current_run
# 0x690c30, 16; manager_manager.cpp; a linha das estatísticas pela 0x4a4850 original, que pode reordenar a tabela, e
# os dias pela 0x54cdb0), construir o recordista (record_holder_construct 0x450d40, 101.461; club_records.cpp, irmã das
# 0x450c60/0x450d30 da 243ª) e o técnico rival que comenta na mídia (Media::pick_rival_manager_id 0x72b940, 923;
# media.cpp; 0x53b900, 0x5480f0, 0x4b02e0 já em Rust). Simulação: as mutações pela carreira até 2002-06-01 e a
# confirmação no jogo real comparando. Sem SEH, sem caixa. Locais: a 0x72b940 passa a vaga do 1º argumento como o
# contador da 0x5480f0 (que zera o byte antes de contar) e o limite do sorteio fica na pilha como 3º argumento da
# 0x4b02e0 (que lê dois); a cópia faz o mesmo. Código morto no original: o teste "nota + 750 < nota" dos rejeitados
# nunca passa (o sorteio(3) dali não acontece). A 0x72b940 SORTEIA: comparador lado a lado pelo estado do sorteio, a
# cópia em without_counting. Os outros: o olheiro pelo valor; a sequência com a original nas saídas e a cópia em locais
# (a 0x4a4850 reordena só na primeira); o recordista lado a lado pela data. Nenhuma em laço quente.
SAVE = False
ROTINAS = [('olheiro_da_busca', 0x7ae950, [0x56, 0x8b, 0xf1, 0x8b, 0x86, 0x80, 0x01, 0x00, 0x00]),
           ('sequencia_atual_do_clube', 0x690c30, [0x8b, 0x54, 0x24, 0x04, 0x53]),
           ('construir_recordista', 0x450d40, [0x56, 0x8b, 0xf1, 0x8d, 0x4e, 0x10]),
           ('tecnico_rival_para_comentar', 0x72b940, [0x83, 0xec, 0x50, 0x53, 0x56])]
DOCS = ('o olheiro da busca, a sequência atual do clube, construir o recordista e o técnico rival que comenta na mídia',
        'the search scout, the club current run, building the record holder and the rival manager who comments in the media',
        [('olheiro_da_busca', 'PLAYER_SEARCH::get_scout', '0x7ae950', 'player_search.cpp', 'thiscall',
          'a pessoa do índice +0x180 se ainda confere com os nomes e a data guardados (0x53e990); senão 0',
          'the person at index +0x180 when it still matches the stored names and date (0x53e990); else 0'),
         ('sequencia_atual_do_clube', 'MANAGER_MANAGER::get_current_run', '0x690c30', 'manager_manager.cpp', 'thiscall, ret 0xc',
          'a primeira sequência (tipos 1, 3, 4, 2) de mais de 1 jogo com o último há menos de 10 dias, pela linha do clube nas estatísticas da liga; senão 5 e 0',
          'the first run (types 1, 3, 4, 2) of more than 1 game with the last one under 10 days ago, from the club line in the league statistics; else 5 and 0'),
         ('construir_recordista', 'record_holder_construct', '0x450d40', 'club_records.cpp', 'thiscall',
          'constrói o recordista: só a data em +0x10 (0x54a110)',
          'builds the record holder: only the date at +0x10 (0x54a110)'),
         ('tecnico_rival_para_comentar', 'Media::pick_rival_manager_id', '0x72b940', 'media.cpp', 'thiscall, ret 4',
          'o técnico de um rival (+0xb3/+0xb7/+0xbb) da mesma liga e bem maior, ou de um clube sorteado em confronto na liga, que passa nos sorteios; devolve o id ou −1',
          'the manager of a rival (+0xb3/+0xb7/+0xbb) of the same league and much bigger, or of a drawn club in a league clash, who passes the draws; returns the id or −1')])
CAMPOS = [('PLAYER_SEARCH +0x174/+0x178/+0x17c nomes, +0x180 índice do olheiro, +0x184 data; linha das estatísticas da liga +0x19/+0x2d/+0x41/+0x55 sequências e as datas do último jogo; clube +0x57 liga, +0x80 reputação, +0xb3/+0xb7/+0xbb rivais, +0xcf técnico; técnico +0x57, +0x5c, +0x69; lê (olheiro_da_busca, sequencia_atual_do_clube, tecnico_rival_para_comentar); recordista +0x10 data; escreve (construir_recordista)',
           'PLAYER_SEARCH +0x174/+0x178/+0x17c names, +0x180 scout index, +0x184 date; league statistics line +0x19/+0x2d/+0x41/+0x55 runs and the last game dates; club +0x57 league, +0x80 reputation, +0xb3/+0xb7/+0xbb rivals, +0xcf manager; manager +0x57, +0x5c, +0x69; reads (olheiro_da_busca, sequencia_atual_do_clube, tecnico_rival_para_comentar); record holder +0x10 date; writes (construir_recordista)')]
INTERESSA = ['SGE', 'CMRTE']
