# Mutacoes (mke_5), nos textos das noticias de contrato (comparador lado a lado do texto e dos vinculos): o texto
# errado e acusado pelo comparador na primeira noticia escrita; a carreira nao muda (so texto):
# - o corpo da noticia de fim de contrato (outro clube, nao proprio) usa o modelo do tipo 1;
# - a noticia da conversa de contrato troca o titulo pelo corpo (roda em toda escrita do texto; trocada em 13/11/2026:
#   o modelo de tres meses da noticia de vencimento escapava, esse caso nao saiu na carreira). A carreira a acusou
#   (a impressao das partidas mudou).
MUTACOES = [
    ('mke5_fim_modelo_errado',
     'fmt(NEWS_TEXT_BUFFER, 0x9aec58, 100, np); // mke5: the contract end body',
     'fmt(NEWS_TEXT_BUFFER, 0x9aedc8, 100, np); // mutation: the kind 1 template',
     'mke_5 contract end at the club'),
    ('mke5_titulo_e_corpo_trocados',
     '    let mut name = [0u8; 0x12c];\n    if full != 0 {\n        let (s1, s2, s3, s4, s5, s10)',
     '    let mut name = [0u8; 0x12c];\n    if full == 0 { // mutation: the headline and the body swapped\n        let (s1, s2, s3, s4, s5, s10)',
     'mke_5 talks state stored'),
]
