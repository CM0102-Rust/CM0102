# Mutacoes (r7_03_1). TELAS: a cadeia nao roda; o texto nao volta para a simulacao, e o comparador do escritor
# (compare_text_writer: o texto, os vinculos e o gerador) as acusa quando a noticia e aberta jogando comparando.
# Conferidas na emulacao (unicorn, a original e a copia no mesmo mundo): as tres mudam o rastro.
# - o empate contado como vitoria (todo relatorio de jogo empatado);
# - o veredito de capacidade 3 e potencial 4 trocado (todo relatorio de jogo com essas notas);
# - os vinculos do jogador com o codigo do clube (todo corpo cujo jogador ainda confere).
MUTACOES = [
    ('r7_03_1_empate_como_vitoria',
     '    let k = if goals > against { 0 } else if goals == against { 1 } else { 2 }; // r7_03_1: win, draw, defeat',
     '    let k = if goals >= against { 0 } else if goals == against { 1 } else { 2 }; // r7_03_1 mutation',
     'r7_03_1 watched report'),
    ('r7_03_1_veredito_3_4',
     '        (3, 4) => &WATCHED_GOOD_PROSPECT,',
     '        (3, 4) => &WATCHED_DECENT_PLAYER,',
     'r7_03_1 watched report'),
    ('r7_03_1_vinculo_do_jogador',
     '        exe::text_add_link(out, t.player.as_ptr(), 0x7d7, p); // r7_03_1: the player\'s links',
     '        exe::text_add_link(out, t.player.as_ptr(), 0x7d5, p); // r7_03_1 mutation',
     'r7_03_1 plain report'),
]
