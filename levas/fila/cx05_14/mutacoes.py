# Mutacoes (cx05_14) pela prova do save (mutacoes.py --prova-save N). Nenhuma abre caixa: o tamanho gravado nao muda,
# so o conteudo.
# - a Eurocopa grava o ano relativo da semeadura no lugar do dia: o .tmp da Eurocopa no .qst muda;
# - a minicopa grava o dia da data de inicio a partir do ano (deslocada 2 bytes): o .tmp da minicopa muda (so com
#   minicopas vivas no save).
MUTACOES = [
    ('cx05_14_eurocopa_grava_trocado',
     '    exe::crt_fwrite(this + EURO_CHAMP_SEEDING_DAY, 2, 1, file); // cx05_14: the seeding day',
     '    exe::crt_fwrite(this + EURO_CHAMP_SEEDING_YEAR_OFFSET, 2, 1, file); // mutation'),
    ('cx05_14_minicopa_data_deslocada',
     "    exe::crt_fwrite(this + 0xb2, 8, 1, file); // cx05_14: the mini cup's start date",
     "    exe::crt_fwrite(this + 0xb4, 8, 1, file); // mutation"),
]
