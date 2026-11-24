# Mutacoes (sbB_2). O texto nao volta para a simulacao: acusadas pela comparacao do escritor (compare_text_writer: o
# texto, os vinculos e o gerador), com o modo CM_TEXTO_NOTICIAS que a cadeia liga em todo jogo comparando (a noticia
# 0x138e sai de send_scout_report_news 0x7f9a60 para o tecnico do clube). Cada uma num trecho comum:
# - a manchete com o numero de olheiros trocado (toda manchete);
# - o vinculo do nome completo do jogador com outro codigo (todo corpo cujo jogador ainda confere);
# - o modo (2: impressionou) trocado na frase da partida (todo corpo com partida).
MUTACOES = [
    ('sbB_manchete_olheiros',
     '        let tpl = match (r.count == 1, r.mode == 2) {',
     '        let tpl = match (r.count != 1, r.mode == 2) { // mutation',
     'sbB_2 scout report headline'),
    ('sbB_vinculo_jogador',
     '        exe::text_add_link(out, full.as_ptr(), 0x7d7, r.player());',
     '        exe::text_add_link(out, full.as_ptr(), 0x7d8, r.player()); // mutation',
     'sbB_2 scout report body'),
    ('sbB_modo_partida',
     '    let mode = (r.mode != 2) as usize;',
     '    let mode = (r.mode == 2) as usize; // mutation',
     'sbB_2 match'),
]
