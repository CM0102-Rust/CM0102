# Mutacoes (mke_5), nos textos das noticias de contrato (comparador lado a lado do texto e dos vinculos): o texto
# errado e acusado pelo comparador na primeira noticia escrita; a carreira nao muda (so texto):
# - o corpo da noticia de fim de contrato (outro clube, nao proprio) usa o modelo do tipo 1;
# - a noticia de contrato a vencer em tres meses (outro clube) usa o modelo de dois meses.
MUTACOES = [
    ('mke5_fim_modelo_errado',
     'fmt(NEWS_TEXT_BUFFER, 0x9aec58, 100, np); // mke5: the contract end body',
     'fmt(NEWS_TEXT_BUFFER, 0x9aedc8, 100, np); // mutation: the kind 1 template',
     'mke_5 contract end at the club'),
    ('mke5_vencimento_trocado',
     '    (0x9aac74, 0x9aab88, 0x9aab30),\n',
     '    (0x9aaa68, 0x9aab88, 0x9aab30),\n',
     'mke_5 talks state stored'),
]
