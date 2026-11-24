# Mutacoes (cx04_2) pela prova do save (mutacoes.py --prova-save N; CARGA). Nenhuma abre caixa (as leituras seguem
# alinhadas: so muda o valor que fica no objeto ou o que vai para o arquivo):
# - a Intertoto grava o numero de rodadas e de times (+0x3c, o campo errado) no lugar da marca: o .tmp da Intertoto no .qst muda;
# - a Copa da UEFA carregada fica com o byte preliminar trocado (bit 0 invertido): a copa carregada muda (a impressao da carga e o
#   comparador acusam).
MUTACOES = [
    ('cx04_2_intertoto_grava_outro',
     '    exe::crt_fwrite(this + intertoto_cup::WINNERS_SENT_TO_UEFA, 4, 1, file)\n}',
     '    exe::crt_fwrite(this + cup::ROUND_COUNT, 4, 1, file) // mutation\n}'),
    ('cx04_2_uefa_preliminar_trocado',
     '        exe::crt_fread(this + uefa_cup::PRELIMINARY_CLUB_COUNT, 1, 1, file);\n    } else {',
     '        exe::crt_fread(this + uefa_cup::PRELIMINARY_CLUB_COUNT, 1, 1, file);\n        write_u8(this + uefa_cup::PRELIMINARY_CLUB_COUNT, read_u8(this + uefa_cup::PRELIMINARY_CLUB_COUNT) ^ 1); // mutation\n    } else {'),
]
