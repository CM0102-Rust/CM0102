# Mutacoes. As noticias de recorde de valor recebido e pago deixam de ir para a fila (a 0x448e30 as transforma em
# noticias depois: a carreira pega pelas noticias do jogo, os contadores provam que o ramo roda com as transferencias
# da carreira). O recorde de nota da temporada exigindo todos os jogos em vez da metade (no fim da temporada; o recorde
# da temporada vai para o de sempre e para as noticias de recorde pela 0x449810). Nenhuma nos recordes de carreira
# (sem efeito lido pela carreira que se conheca).
MUTACOES = [
    ('cpm_noticia_recebido', 'noticia(this, cpm6_d(vendedor) as u32, 0x1f); // recorde de valor recebido',
     '// mutacao: sem a noticia de recorde recebido', 'cpm_6 noticia de recorde recebido'),
    ('cpm_noticia_pago', 'noticia(this, cpm6_d(comprador) as u32, 0x1e); // recorde de valor pago',
     '// mutacao: sem a noticia de recorde pago', 'cpm_6 noticia de recorde pago'),
    ('cpm_nota_metade', 'if (jogou as i32) < *((entrada + 0x10) as *const i8) as i32 / 2 { // menos da metade dos jogos',
     'if (jogou as i32) < *((entrada + 0x10) as *const i8) as i32 { // mutacao: todos os jogos', 'cpm_6 recorde de nota da temporada'),
]
