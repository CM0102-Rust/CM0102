# Mutacoes (cx04_9), acusadas pelo comparador das historias (compare_story_post: os sorteios e a historia entregue aos
# humanos) e, quando mudam os sorteios, pela carreira:
# - a chance no time principal sorteia a redacao em qualquer clube (reputacao > 0): os sorteios mudam nos clubes
#   pequenos;
# - a historia de gratidao marca 2 na reputacao para qualquer jogador (a historia entregue ao humano difere);
# - a euforia da virada leva a sequencia uma acima (a historia entregue ao humano difere).
MUTACOES = [
    ('cx04_9_chance_sorteio_sempre',
     '    set_news_slot(n, 0xa, if club.reputation() > 0x186a {',
     '    set_news_slot(n, 0xa, if club.reputation() > 0 /* mutation */ {',
     'cx04_9 player pleased with chance story'),
    ('cx04_9_reputacao_sempre_2',
     '    set_news_slot(n, 5, if player.player().current_reputation() > 0x1c52 { 2 } else { 1 });',
     '    set_news_slot(n, 5, if player.player().current_reputation() > -1 { 2 } else { 1 }); // mutation',
     'cx04_9 player thanks manager story'),
    ('cx04_9_euforia_sequencia',
     '    set_news_slot(n, 0xb, run);',
     '    set_news_slot(n, 0xb, run + 1); // mutation',
     'cx04_9 player ecstatic story'),
]
