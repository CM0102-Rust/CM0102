# Mutacoes (cx04_10), acusadas pelo comparador das historias (compare_story_post: a historia entregue aos humanos e o
# alcance):
# - a multa da FA vai sempre ao clube (o alcance da nacao some): os tecnicos humanos da nacao deixam de recebe-la (os
#   da Inglaterra quando um clube grande e multado);
# - a mudanca do sistema de transferencias leva o tipo trocado (a historia que todos os humanos recebem difere);
# - o credito pelo premio leva o premio do jogador no lugar do premio (a historia entregue ao humano difere).
MUTACOES = [
    ('cx04_10_multa_sempre_clube',
     '    if kind != 0 && club.reputation() > 0x128e {',
     '    if kind != 0 && club.reputation() > i16::MAX { // mutation',
     'cx04_10 fa fine story to the club'),
    ('cx04_10_sistema_tipo_trocado',
     '    set_news_slot(n, 0, kind as u8 as i8 as i32 as u32);',
     '    set_news_slot(n, 0, (kind as u8 as i8 as i32 as u32) ^ 1); // mutation',
     'cx04_10 transfer system story'),
    ('cx04_10_premio_trocado',
     '    set_news_slot(n, 0xa, read_u32(award));',
     '    set_news_slot(n, 0xa, read_u32(player.addr())); // mutation',
     'cx04_10 award credit story'),
]
