# Mutacoes do olheiro (atacante rapido, meias criativos, band de forca): as duas primeiras invertem a resposta; a da
# band tira a ordenacao (soma a habilidade dos primeiros do elenco na ordem guardada, nao dos melhores: a media cai e a
# band muda na maioria dos clubes). Mesma lacuna do dbo_1: so o relatorio de um clube (run_scout_day 0x7b3e20) as
# chama e a resposta so vai para a noticia; PODEM ESCAPAR da carreira ate 2002-06-01; os comparadores rodam no jogo
# real comparando.
MUTACOES = [
    ('dbo_atacante_rapido', '    (n > 0) as u8 // atacante rapido (dbo)', '    (n == 0) as u8 // mutacao: a resposta invertida (atacante rapido)'),
    ('dbo_meias_criativos', '    (n > 1) as u8 // meias criativos (dbo)', '    (n <= 1) as u8 // mutacao: a resposta invertida (meias criativos)'),
    ('dbo_faixa_de_forca', '    ordenar(v.as_mut_ptr() as usize, n as u32, 4, 0x549c30); // os melhores primeiro (dbo)', '    let _ = ordenar; // mutacao: sem a ordenacao (os primeiros do elenco, nao os melhores)'),
]
