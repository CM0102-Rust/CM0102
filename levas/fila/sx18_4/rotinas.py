# O registro de partida do clube e a referencia de nome de uma pessoa nas vagas numeradas de uma noticia
# (record_utils.cpp; leitor sx18, rodada 5, 24/11/2026). Rodam muito na carreira (mundo ate 2002-08-01: gravar o
# registro 99.950 vezes, ler 1.128, gravar a referencia 6.580): as noticias de partida, de recordes, de suspensao e da
# midia guardam o que o texto vai ler depois. O sx18_5 usa as pecas deste (news_put_slots, news_take_slots,
# news_slot_date, date_slot_values, compare_news_slot_writer/_reader): integrar este antes.
# Convencoes conferidas na desmontagem: as tres cdecl, devolvem 1 (0 com a caixa de erro):
# - 0x7e4de0 (registro, noticia, &k): 0x785790 NewsItem::set_slot thiscall(k, valor) ret 8, onze vagas: dia (movsx),
#   ano (movzx), +8, +0xc e +0xe (movsx), +0x10 (movsx byte), +0x11, +0x15, +0x19 e +0x1a (movsx byte), +0x1b; linhas
#   0x160 (registro nulo) e 0x168 (noticia nula);
# - 0x7e4fa0 (noticia, &k, registro): 0x785830 NewsItem::get_slot thiscall(k) ret 4; Date::Date 0x549ef0 (dia, ano) e
#   operator= 0x417c10 na data; os outros campos com a largura do original; linhas 0x18e e 0x196;
# - 0x7e5160 (referencia, noticia, &k): seis vagas: os tres ids de nome, o id da pessoa, dia (movsx) e ano (movzx) da
#   data +0x10; linhas 0x1bb e 0x1c3.
# Comparadores: gravar so a original (as vagas de uma noticia se gravam uma vez) e as vagas lidas de volta contra os
# valores da copia; ler a copia num registro local com o seu proprio k e depois a original, os bytes comparados.
SAVE = False
ROTINAS = [('write_match_record_to_news', 0x7e4de0, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('read_match_record_from_news', 0x7e4fa0, [0x81, 0xec, 0x10, 0x02, 0x00, 0x00]),
           ('write_staff_name_ref_to_news', 0x7e5160, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('o registro de partida e a referência de nome nas vagas de uma notícia',
        'a match record and a name reference in a news item\'s slots',
        [('write_match_record_to_news', 'write_match_record_to_news', '0x7e4de0', 'record_utils.cpp', 'cdecl',
          'grava os onze campos do registro de partida do clube (dia, ano, id da partida, prefixo e nome da rodada, lado, clube, adversário, gols a favor e contra, vaga das estatísticas) nas vagas seguintes da notícia e avança o contador; registro ou notícia nulos: caixa de erro e 0',
          'stores the eleven fields of the club match record (day, year, fixture id, round prefix and name, venue, club, opponent, goals for and against, statistics slot) in the news item\'s next slots and advances the counter; a null record or news item: the error box and 0'),
         ('read_match_record_from_news', 'read_match_record_from_news', '0x7e4fa0', 'record_utils.cpp', 'cdecl',
          'lê de volta o registro de partida das vagas seguintes da notícia (a data pelo dia e pelo ano) e avança o contador; notícia ou registro nulos: caixa de erro e 0',
          'reads the match record back from the news item\'s next slots (the date from the day and the year) and advances the counter; a null news item or record: the error box and 0'),
         ('write_staff_name_ref_to_news', 'write_staff_name_ref_to_news', '0x7e5160', 'record_utils.cpp', 'cdecl',
          'grava a referência de nome de uma pessoa (os três ids de nome, o id e a data de nascimento) nas vagas seguintes da notícia e avança o contador',
          'stores a person\'s name reference (the three name ids, the id and the birth date) in the news item\'s next slots and advances the counter')])
CAMPOS = []
INTERESSA = []
