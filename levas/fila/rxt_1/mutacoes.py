# Mutacoes (rxt_1). TELAS: a cadeia nao roda; o texto nao volta para a simulacao, e o comparador do escritor
# (compare_text_writer: o texto, os vinculos e o gerador) as acusa quando a noticia e aberta jogando comparando.
# Conferidas na emulacao (unicorn, a original e a copia no mesmo mundo): as tres mudam o rastro.
# - a contagem "um" escrita como "varios" (toda frase de busca com um jogador achado);
# - o vinculo do lugar a busca com outro codigo (toda noticia cujo olheiro e o da busca);
# - a ordem das respostas sobre o clube observado (todo relatorio de observacao de clube).
MUTACOES = [
    ('rxt_1_um_como_varios',
     '    let k = match found { 0 => 0, 1 => 1, _ => 2 } + if completed { 0 } else { 3 }; // rxt_1: none, one, several',
     '    let k = match found { 0 => 0, 1 => 2, _ => 2 } + if completed { 0 } else { 3 }; // rxt_1 mutation',
     'rxt_1 search body'),
    ('rxt_1_vinculo_da_busca',
     '        exe::text_add_link(out, place as *const u8, 0x7e4, r.search as i32 as usize); // rxt_1: the place to the search',
     '        exe::text_add_link(out, place as *const u8, 0x7e5, r.search as i32 as usize); // rxt_1 mutation',
     'rxt_1 search headline'),
    ('rxt_1_respostas_trocadas',
     '        for off in [1usize, 0, 2, 0xa, 5, 6, 7, 9, 3, 4, 0xb] { // rxt_1: where each answer goes',
     '        for off in [0usize, 1, 2, 0xa, 5, 6, 7, 9, 3, 4, 0xb] { // rxt_1 mutation',
     'rxt_1 club scouting report'),
]
