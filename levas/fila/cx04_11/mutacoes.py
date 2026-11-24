# Mutacoes (cx04_11): o valor do seguro na historia vai pela metade; acusada pelo comparador das historias
# (compare_story_post) quando a historia chega a um tecnico humano. A aposentadoria por lesao e rara: se nenhuma
# acontecer no clube humano ate 2002-06-01, a mutacao escapa (explicar na leva ou tirar o item).
MUTACOES = [
    ('cx04_11_seguro_metade',
     '    set_news_slot(n, 4, amount);',
     '    set_news_slot(n, 4, amount / 2); // mutation',
     'cx04_11 insurance payout story'),
]
