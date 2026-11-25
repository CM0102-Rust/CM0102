# Mutacoes (rxt_3). TELAS: a cadeia nao roda; o comparador do relatorio (lado a lado do texto, dos ids e do gerador)
# as acusa quando a noticia da observacao do adversario e aberta jogando comparando. Conferidas na emulacao (unicorn,
# a original e a copia no mesmo mundo): as tres mudam a saida.
# - a frase da forca da faixa 2 trocada pela da faixa 3 (todo adversario de faixa 2);
# - os nomes da frase da tabela meia-atacante trocados de lugar;
# - a comparacao de reputacao da frase dos cruzamentos invertida (time fraco).
MUTACOES = [
    ('rxt_3_forca_faixa_2',
     '        2 => 0xa77a54,',
     '        2 => 0xa77c48, // rxt_3 mutation',
     'rxt_3 opposition report'),
    ('rxt_3_tabela_invertida',
     '            story_lang!(para, 0xa77410, ap, bp);',
     '            story_lang!(para, 0xa77410, bp, ap); // rxt_3 mutation',
     'rxt_3 link-up play'),
    ('rxt_3_cruzamentos',
     '    if a[5] != 0 && a[0xb] == 0 && r.target.reputation() > own.reputation() {',
     '    if a[5] != 0 && a[0xb] == 0 && r.target.reputation() < own.reputation() { // rxt_3 mutation',
     'rxt_3 opposition report'),
]
