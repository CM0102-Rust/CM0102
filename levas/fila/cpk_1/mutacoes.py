# Mutacoes dos recordes do clube (o comparando roda a original: so o modo Rust e a carreira acusam). Recordes de sempre
# e da temporada sem a conta subir: todos os clubes criados depois dividem o mesmo registro (o recorde de valor pago
# da 0x4486c0 muda as transferencias; os recordes batidos mudam as noticias). A noticia de recorde sem a conta subir:
# so a ultima de cada leva fica (a 0x448e30 manda menos noticias; noticias quase nao entram na impressao: PODE ESCAPAR).
MUTACOES = [
    ('cpk_recordes_sempre_conta', 'cpk1_por(this + 0xc, n.wrapping_add(1)); // a conta dos recordes de sempre',
     'cpk1_por(this + 0xc, n); // mutacao: a conta nao sobe'),
    ('cpk_recordes_temporada_conta', 'cpk1_por(this + 0x18, n.wrapping_add(1)); // a conta dos recordes da temporada',
     'cpk1_por(this + 0x18, n); // mutacao: a conta nao sobe'),
    ('cpk_noticia_recorde_conta', 'cpk1_por(this + 0x2c, n.wrapping_add(1)); // mais uma notícia',
     'cpk1_por(this + 0x2c, n); // mutacao: a conta nao sobe'),
]
