# Mutacoes (rx09_12) pela prova do save (mutacoes.py --prova-save N; CARGA), com a base NORMAL (a Segunda sueca so existe
# nela):
# - a gravacao com onze ids em vez de doze: o .tmp da Segunda sueca no .qst fica 4 bytes mais curto (e a carga seguinte
#   desalinha);
# - a carga le onze ids: o ultimo fica como estava e o arquivo fica 4 bytes atras (o comparador acusa).
MUTACOES = [
    ('rx09_12_grava_onze',
     'exe::crt_fwrite(this + 0xee, 4, 0xc, file) // rx09_12: the twelve promoted club ids written',
     'exe::crt_fwrite(this + 0xee, 4, 0xb, file) // mutation: eleven',
     'rx09_12 swedish second data saved'),
    ('rx09_12_le_onze',
     'exe::crt_fread(this + 0xee, 4, 0xc, file); // rx09_12: the twelve promoted club ids',
     'exe::crt_fread(this + 0xee, 4, 0xb, file); // mutation: eleven',
     'rx09_12 swedish second data loaded'),
]
