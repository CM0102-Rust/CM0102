# Mutacoes (cx09_7) pela prova do save (mutacoes.py --prova-save N; CARGA), com um save que tenha a Finlandia
# carregada. Nenhuma abre caixa.
# - grava so tres dos quatro clubes guardados: o .tmp da liga no .qst muda;
# - le so tres: o arquivo fica 4 bytes atras e a liga carregada muda (o comparador acusa a posicao do arquivo).
MUTACOES = [
    ('cx09_7_finlandia_grava_tres',
     '    exe::crt_fwrite(this + FIN_FIRST_STORED_CLUBS, 4, 4, file)\n',
     '    exe::crt_fwrite(this + FIN_FIRST_STORED_CLUBS, 4, 3, file) // mutation\n'),
    ('cx09_7_finlandia_le_tres',
     '    exe::crt_fread(this + FIN_FIRST_STORED_CLUBS, 4, 4, file);\n',
     '    exe::crt_fread(this + FIN_FIRST_STORED_CLUBS, 4, 3, file); // mutation\n'),
]
