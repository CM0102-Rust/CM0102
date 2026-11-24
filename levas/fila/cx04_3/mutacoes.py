# Mutacoes (cx04_3) pela prova do save (mutacoes.py --prova-save N; CARGA). Nenhuma abre caixa:
# - a Copa da UEFA grava o byte de antes (+0xb1) no lugar do preliminar: o .tmp da UEFA no .qst muda;
# - o ranking da FIFA grava um byte da data a mais antes do mes da virada: o fifa_rankings.dat do .qst muda (a carga
#   le o mes desse byte e sobra um no fim, sem caixa);
# - o ranking da FIFA carregado fica com a contagem de resultados de cada nacao uma acima da capacidade: o
#   comparador (contagem = capacidade) acusa.
MUTACOES = [
    ('cx04_3_uefa_grava_outro',
     '    exe::crt_fwrite(this + uefa_cup::PRELIMINARY_CLUB_COUNT, 1, 1, file)\n}',
     '    exe::crt_fwrite(this + uefa_cup::PRELIMINARY_CLUB_COUNT - 1, 1, 1, file) // mutation\n}'),
    ('cx04_3_fifa_data_antes_do_mes',
     '    if exe::zwrite(this + fifa_rankings_fields::ROLLOVER_MONTH, 1, 1, file) != 1 {\n        show_error_at(FIFA_RANKINGS_CPP, 0x2b9);',
     '    exe::zwrite(this + fifa_rankings_fields::LAST_UPDATE_DATE, 1, 1, file); // mutation\n    if exe::zwrite(this + fifa_rankings_fields::ROLLOVER_MONTH, 1, 1, file) != 1 {\n        show_error_at(FIFA_RANKINGS_CPP, 0x2b9);'),
    ('cx04_3_fifa_contagem_acima',
     '        write_i16(e + fifa_ranking_entry::HISTORY_COUNT, read_i16(e + fifa_ranking_entry::HISTORY_CAPACITY));',
     '        write_i16(e + fifa_ranking_entry::HISTORY_COUNT, read_i16(e + fifa_ranking_entry::HISTORY_CAPACITY) + 1); // mutation'),
]
