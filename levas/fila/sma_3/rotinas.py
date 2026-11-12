# Os registros dos clubes: a foto da temporada e os campos da notícia de recorde batido (club_records.cpp; leitor sma,
# 12/11/2026). Convenções conferidas:
# - season_history_entry_construct (0x448260): thiscall(this), ret 0, devolve this; chamada só pelo fim de temporada
#   da nação (0x447880, original) com uma local; as construtoras 0x54a110 (Date), 0x645800 (só a Date do registro de
#   jogo, original), 0x448670, 0x450d40, 0x4485d0 e 0x448620 (Rust): mov ecx, campo -> thiscall;
# - club_records_write_record_news_fields (0x453120): cdecl(id do clube, registro, tipo (char), notícia), ret 0,
#   devolve 1 ou 0; chamada pela 0x447880 (original) e pela 0x448e30 (Rust); 0x785790 (newsitem_set_slot): push valor;
#   push índice (só o byte baixo); ecx = notícia -> thiscall, ret 8; 0x7e4de0, 0x7e5160 e 0x7e5420: push &índice;
#   push notícia; push campo -> cdecl (avançam o índice); o tipo 20 grava o float +0x1f1 × 100 (fmul dword 0x968960,
#   _ftol).
# Comparadores: lado a lado (os 0x389 bytes da foto; os 0xe0 bytes da notícia e o retorno); os casos com a caixa de
# erro (sem registro, sem notícia, tipo fora de 1..32) só a original.
SAVE = False
ROTINAS = [('season_history_entry_construct', 0x448260, [0x6a, 0xff, 0x68, 0x41, 0x54, 0x95, 0x00]),
           ('club_records_write_record_news_fields', 0x453120, [0x81, 0xec, 0x04, 0x02, 0x00, 0x00])]
DOCS = ('os registros dos clubes: a foto da temporada e os campos da notícia de recorde',
        'club records: the season snapshot and the record news fields',
        [('season_history_entry_construct', 'season_history_entry_construct', '0x448260', 'club_records.cpp', 'thiscall',
          'constrói a foto da temporada de um clube (0x389 bytes): as Datas dos seus registros de jogo, de série, de detentor e de transferência, na ordem do original; devolve o próprio objeto',
          'constructs a club\'s season snapshot (0x389 bytes): the Dates of its match, run, holder and transfer records, in the original\'s order; returns the object itself'),
         ('club_records_write_record_news_fields', 'club_records_write_record_news_fields', '0x453120', 'club_records.cpp', 'cdecl',
          'grava na notícia de um recorde batido o id do clube (campo 0), o tipo do recorde (campo 1) e os campos daquele tipo (1 a 32: bytes, palavras, inteiros, a porcentagem × 100, registros de jogo, de série e o nome do detentor, que avançam o índice sozinhos); sem registro, sem notícia ou com tipo fora de 1..32, a caixa de erro e 0',
          'writes into a broken record\'s news item the club id (slot 0), the record type (slot 1) and that type\'s fields (1 to 32: bytes, words, dwords, the percentage × 100, match records, run records and the holder\'s name, which advance the index themselves); without a record, without a news item or with a type outside 1..32, the error box and 0')])
