# Mutacoes (cx09_6) pela prova do save (mutacoes.py --prova-save N; CARGA), com um save que tenha a Belgica e a
# Croacia carregadas. Nenhuma abre caixa.
# - a Terceira Divisao belga grava so tres dos quatro clubes guardados: o .tmp dela no .qst muda (e a leitura seguinte
#   sai desalinhada);
# - a Primeira Divisao croata carregada nao le o numero de pares: o arquivo fica um byte atras e a liga carregada muda
#   (o comparador acusa a posicao do arquivo).
MUTACOES = [
    ('cx09_6_belgica_grava_tres_clubes',
     '    exe::crt_fwrite(this + BEL_THIRD_STORED_CLUBS, 4, 4, file)\n',
     '    exe::crt_fwrite(this + BEL_THIRD_STORED_CLUBS, 4, 3, file) // mutation\n'),
    ('cx09_6_croacia_nao_le',
     '    exe::crt_fread(this + CRO_A1_PLAYOFF_PAIRS, 1, 1, file);\n',
     '    exe::crt_fread(this + CRO_A1_PLAYOFF_PAIRS, 1, 0, file); // mutation\n'),
]
