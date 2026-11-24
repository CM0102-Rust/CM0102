# Mutacoes (cx17_4) pela prova do save (mutacoes.py --prova-save N; CARGA). Nenhuma abre caixa (a leitura segue
# alinhada):
# - a Copa do Mundo grava o dia da semeadura no lugar do ano dela: o .tmp da Copa no .qst muda;
# - a sul-americana le a contagem de isentas no byte seguinte (+0xef): o +0xee carregado fica com o que havia
#   (o comparador da carga e a impressao da carga acusam).
MUTACOES = [
    ('cx17_4_grava_dia_no_lugar_do_ano',
     'r = exe::crt_fwrite(this + at, size, count, file); // cx17_4: the World Cup\'s fields',
     'r = exe::crt_fwrite(this + if size == 2 { world_cup_fields::SEEDING_DAY } else { at }, size, count, file); // mutation: the day twice',
     'cx17_4 world cup saved'),
    ('cx17_4_isentas_no_byte_seguinte',
     'if exe::crt_fread(this + wc_south_american_league_fields::EXEMPT_COUNT, 1, 1, file) != 1 {',
     'if exe::crt_fread(this + wc_south_american_league_fields::EXEMPT_COUNT + 1, 1, 1, file) != 1 { // mutation: the next byte',
     'cx17_4 south american qualifying loaded'),
]
