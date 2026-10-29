# Recordes do clube: somar o valor da transferência aos totais de venda e compra (Club_Records::add_transfer_totals
# 0x448ad0, pelas transferências 0x8ca96e/0x8cb314/0x8cb4a7), os recordes de jogos e gols na carreira pelo clube
# (club_records_update_career_records 0x448cb0, pelas estatísticas do jogador 0x7c040d/0x7c059b) e o recorde de nota
# da temporada (Club_Records::update_season_rating_record 0x452a90, pelo fim de temporada 0x447a9a/0x4497e6/0x451105).
# Chamadas: 0x4528d0, 0x452710 e 0x452cb0 (247a leva), 0x669950, 0x449590, 0x7e4920, 0x540a50, 0x7c3920 e 0x7c2150 por
# endereço. Sem SEH, sem sorteio. Comparadores: lado a lado; os registros que podem ser criados (realloc) são pedidos
# antes, na ordem da original; a fila de notícias sem folga para duas: só a original.
SAVE = False
ROTINAS = [('somar_totais_de_transferencia', 0x448ad0, [0x51, 0x8b, 0x44, 0x24, 0x0c]),
           ('recordes_de_carreira_no_clube', 0x448cb0, [0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('recorde_de_nota_da_temporada', 0x452a90, [0x81, 0xec, 0x14, 0x02, 0x00, 0x00])]
DOCS = ('os recordes do clube: somar o valor da transferência aos totais de venda e compra, os recordes de jogos e gols na carreira pelo clube e o recorde de nota da temporada',
        "club records: adding the transfer fee to the sale and purchase totals, the club's career appearances and goals records and the season rating record",
        [('somar_totais_de_transferencia', 'Club_Records::add_transfer_totals', '0x448ad0', 'club_records.cpp', 'thiscall, ret 0xc',
          'soma o valor ao recebido do vendedor e ao gasto do comprador na temporada, sobe os recordes de sempre e guarda a notícia de recorde (0x1f, 0x1e) acima de (+0x80 + 500) × 1000',
          "adds the fee to the seller's season income and the buyer's season spending, raises the all-time records and queues the record news (0x1f, 0x1e) above (+0x80 + 500) × 1000"),
         ('recordes_de_carreira_no_clube', 'club_records_update_career_records', '0x448cb0', 'club_records.cpp', 'thiscall, ret 0x10',
          'os recordes de sempre de jogos (+0x247) e gols (+0x261) pela liga do clube, com a referência do jogador (0x7e4920), quando o tipo vale para o clube (0x449590)',
          "the club's all-time league appearances (+0x247) and goals (+0x261) records, with the player reference (0x7e4920), when the record type applies to the club (0x449590)"),
         ('recorde_de_nota_da_temporada', 'Club_Records::update_season_rating_record', '0x452a90', 'club_records.cpp', 'thiscall, ret 4',
          'o jogador do elenco com a maior média (estatística 0x11) entre os que jogaram ao menos metade dos jogos vira o recorde de nota da temporada',
          'the squad player with the best average rating (statistic 0x11) among those who played at least half the games becomes the season rating record')])
CAMPOS = [('Club_Records registro da temporada e de sempre +0x365 gasto em transferências, +0x369 ano, +0x36b recebido, +0x36f ano; valor; lê e grava (somar_totais_de_transferencia)',
           'Club_Records season and all-time record +0x365 transfer spending, +0x369 year, +0x36b income, +0x36f year; value; reads and writes (somar_totais_de_transferencia)'),
          ('Club_Records +0x24 fila de notícias de recorde (6 bytes: id do clube, tipo), +0x28 capacidade, +0x2c conta; lê (somar_totais_de_transferencia)',
           'Club_Records +0x24 record news queue (6 bytes: club id, type), +0x28 capacity, +0x2c count; reads (somar_totais_de_transferencia)'),
          ('clube +0x80 base do mínimo da notícia de recorde (palavra); valor; lê (somar_totais_de_transferencia)',
           'club +0x80 base of the record news minimum (word); value; reads (somar_totais_de_transferencia)'),
          ('Club_Records registro de sempre +0x247 recorde de jogos na liga, +0x249 referência do jogador (0x18), +0x261 recorde de gols, +0x263 referência; valor; lê e grava (recordes_de_carreira_no_clube)',
           'Club_Records all-time record +0x247 league appearances record, +0x249 player reference (0x18), +0x261 goals record, +0x263 reference; value; reads and writes (recordes_de_carreira_no_clube)'),
          ('Club_Records +0x1c entrada do clube (0x6a bytes) +0x10 jogos da temporada; valor; lê (recorde_de_nota_da_temporada)',
           'Club_Records +0x1c club entry (0x6a bytes) +0x10 season games; value; reads (recorde_de_nota_da_temporada)'),
          ('Club_Records registro da temporada +0x1f1 nota (float), +0x1f5 referência do jogador, +0x20d jogos, +0x20e zerado; grava (recorde_de_nota_da_temporada)',
           'Club_Records season record +0x1f1 rating (float), +0x1f5 player reference, +0x20d games, +0x20e cleared; writes (recorde_de_nota_da_temporada)'),
          ('clube +0xd7 elenco (50 ponteiros); pessoa +0 id, +0x39 clube, +0x61 estatísticas; [0xae2364] primeiro id de técnico humano; [0xb74350] PlayerStats; lê (recorde_de_nota_da_temporada)',
           'club +0xd7 squad (50 pointers); person +0 id, +0x39 club, +0x61 stats; [0xae2364] first human manager id; [0xb74350] PlayerStats; reads (recorde_de_nota_da_temporada)')]
INTERESSA = ['CMRTE', 'SGE']
