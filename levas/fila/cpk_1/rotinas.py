# Recordes do clube (club_records.cpp): achar ou criar os recordes de sempre (Club_Records::get_all_time_record
# 0x452710, ~135 mil chamadas na carreira; lida pela 0x4486c0 das transferências) e os da temporada
# (Club_Records::get_season_record 0x4528d0, ~120 mil), e guardar a notícia de recorde batido
# (Club_Records::queue_record_news 0x452cb0, lida pela 0x448e30). Os três crescem vetores com realloc (0x945667) e o
# registro novo é reiniciado pela 0x452fe0 (já em Rust). Troca: a candidata 0x444930 (28 bytes no mapa) é na verdade
# a rotina de ~11 KB com SEH do fim da partida; entram as duas irmãs da 0x452cb0. Simulação: as mutações pela carreira
# e a confirmação no jogo real comparando. Sem SEH, sem sorteio; caixa só sem memória ou clube nulo (a original).
# Comparadores: a original roda (realoca) e o resultado é conferido com o plano da cópia.
SAVE = False
ROTINAS = [('recordes_de_sempre', 0x452710, [0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('recordes_da_temporada', 0x4528d0, [0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('guardar_noticia_de_recorde', 0x452cb0, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('os recordes do clube: achar ou criar os recordes de sempre e os da temporada e guardar a notícia de recorde batido',
        'club records: finding or creating the all-time and season records and queueing the broken record news',
        [('recordes_de_sempre', 'Club_Records::get_all_time_record', '0x452710', 'club_records.cpp', 'thiscall, ret 8',
          'os recordes de sempre do clube (índice +0 da entrada de 0x6a bytes, vetor de 0x38b bytes em +4); com criar, o índice novo é a conta (+0xc), o vetor cresce de 50 em 50 e o registro é reiniciado (0x452fe0)',
          'the all-time records of the club (index +0 of the 0x6a-byte entry, 0x38b-byte vector at +4); with create, the new index is the count (+0xc), the vector grows by 50 and the record is reset (0x452fe0)'),
         ('recordes_da_temporada', 'Club_Records::get_season_record', '0x4528d0', 'club_records.cpp', 'thiscall, ret 8',
          'o mesmo para os recordes da temporada (índice +4, vetor +0x10, capacidade +0x14, conta +0x18)',
          'the same for the season records (index +4, vector +0x10, capacity +0x14, count +0x18)'),
         ('guardar_noticia_de_recorde', 'Club_Records::queue_record_news', '0x452cb0', 'club_records.cpp', 'thiscall, ret 8',
          'acrescenta (id do clube, tipo do recorde) ao vetor de 6 bytes das notícias de recorde (+0x24), que cresce de 5 em 5; sem memória, caixa e 0',
          'appends (club id, record type) to the 6-byte record news vector (+0x24), growing by 5; without memory, error box and 0')])
CAMPOS = [('recordes do clube +0x1c: tabela de entradas de 0x6a bytes por id de clube (+0 índice dos recordes de sempre, +4 da temporada; −1 sem); índice; lê e grava (recordes_de_sempre, recordes_da_temporada)',
           'club records +0x1c: table of 0x6a-byte entries by club id (+0 all-time records index, +4 season index; −1 none); index; reads and writes (recordes_de_sempre, recordes_da_temporada)'),
          ('recordes do clube +4/+8/+0xc: vetor de 0x38b bytes dos recordes de sempre, capacidade, conta; ponteiro e valores; lê e grava (recordes_de_sempre)',
           'club records +4/+8/+0xc: 0x38b-byte all-time records vector, capacity, count; pointer and values; reads and writes (recordes_de_sempre)'),
          ('recordes do clube +0x10/+0x14/+0x18: vetor dos recordes da temporada, capacidade, conta; ponteiro e valores; lê e grava (recordes_da_temporada)',
           'club records +0x10/+0x14/+0x18: season records vector, capacity, count; pointer and values; reads and writes (recordes_da_temporada)'),
          ('recordes do clube +0x24/+0x28/+0x2c: vetor de 6 bytes das notícias de recorde (+0 id do clube, +4 tipo), capacidade, conta; lê e grava (guardar_noticia_de_recorde)',
           'club records +0x24/+0x28/+0x2c: 6-byte record news vector (+0 club id, +4 type), capacity, count; reads and writes (guardar_noticia_de_recorde)')]
INTERESSA = ['SGE', 'CMRTE']
