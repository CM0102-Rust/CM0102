# A referencia de nome lida das vagas de uma noticia e o registro de periodo (valor, inicio e fim) gravado e lido nelas
# (record_utils.cpp; leitor sx18, rodada 5, 24/11/2026). Usa as pecas do sx18_4 (news_put_slots, news_take_slots,
# news_slot_date, date_slot_values, compare_news_slot_writer/_reader, RECORD_UTILS_CPP): integrar depois dele.
# Ler a referencia roda na carreira (258 vezes no mundo ate 2002-08-01: textos de suspensao, de recordes e da midia);
# o registro de periodo (0x7e5420, 0x7e5570; chamados de 0x4533xx/0x4addxx e 0x44cbxx/0x4a84xx, as series dos
# recordes de clube) nao rodou nessa carreira: entra pelo exe inteiro, provado so pelo comparador quando rodar.
# Convencoes conferidas na desmontagem: as tres cdecl, devolvem 1 (0 com a caixa de erro):
# - 0x7e52c0 (noticia, &k, referencia): quatro dwords e a data (Date::Date 0x549ef0 dia, ano; operator= 0x417c10) em
#   +0x10; linhas 0x1e4 e 0x1ec;
# - 0x7e5420 (registro, noticia, &k): +0, dia (movsx) e ano (movzx) de +4 e de +0xc; linhas 0x20c e 0x214;
# - 0x7e5570 (noticia, &k, registro): +0, a data de +4 e a de +0xc; linhas 0x234 e 0x23c.
SAVE = False
ROTINAS = [('read_staff_name_ref_from_news', 0x7e52c0, [0x81, 0xec, 0x10, 0x02, 0x00, 0x00]),
           ('write_date_span_record_to_news', 0x7e5420, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('read_date_span_record_from_news', 0x7e5570, [0x81, 0xec, 0x10, 0x02, 0x00, 0x00])]
DOCS = ('a referência de nome lida das vagas de uma notícia e o registro de período nelas',
        'a name reference read from a news item\'s slots and a span record in them',
        [('read_staff_name_ref_from_news', 'read_staff_name_ref_from_news', '0x7e52c0', 'record_utils.cpp', 'cdecl',
          'lê de volta a referência de nome de uma pessoa das vagas seguintes da notícia (os três ids de nome, o id e a data pelo dia e pelo ano) e avança o contador; notícia ou referência nulas: caixa de erro e 0',
          'reads a person\'s name reference back from the news item\'s next slots (the three name ids, the id and the date from the day and the year) and advances the counter; a null news item or reference: the error box and 0'),
         ('write_date_span_record_to_news', 'write_date_span_record_to_news', '0x7e5420', 'record_utils.cpp', 'cdecl',
          'grava um registro de período (o valor, a data de início e a de fim) nas vagas seguintes da notícia e avança o contador',
          'stores a span record (the value, the start date and the end date) in the news item\'s next slots and advances the counter'),
         ('read_date_span_record_from_news', 'read_date_span_record_from_news', '0x7e5570', 'record_utils.cpp', 'cdecl',
          'lê de volta o registro de período das vagas seguintes da notícia (o valor e as duas datas) e avança o contador',
          'reads the span record back from the news item\'s next slots (the value and the two dates) and advances the counter')])
CAMPOS = []
INTERESSA = []
