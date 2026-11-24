# Mutacoes (cx04_8), acusadas pelo comparador das historias (compare_story_post: os sorteios e a historia entregue aos
# humanos) e, quando mudam os sorteios, pela carreira:
# - o jogador infeliz com o treino sorteia duas vezes;
# - a briga resolvida poe o proprio jogador no lugar do outro (a historia entregue ao humano difere; so quando um
#   jogador do clube humano a dispara);
# - o relatorio critico usa <= no profissionalismo contra o sorteio de 7: a avaliacao e os sorteios seguintes mudam.
MUTACOES = [
    ('cx04_8_treino_dois_sorteios',
     '    set_news_slot(n, 5, exe::utils_random(3) as u32);\n    post_story(n, StoryTo::Club(player.club_job().addr()));',
     '    set_news_slot(n, 5, exe::utils_random(3) as u32);\n    exe::utils_random(3); // mutation\n    post_story(n, StoryTo::Club(player.club_job().addr()));',
     'cx04_8 unhappy with training story'),
    ('cx04_8_briga_o_mesmo',
     '    covered!("cx04_8 feud settled story");\n    set_news_person_slots(n, 4, other);',
     '    covered!("cx04_8 feud settled story");\n    set_news_person_slots(n, 4, player); // mutation',
     'cx04_8 feud settled story'),
    ('cx04_8_relatorio_profissionalismo',
     '        if (player.professionalism() as i32) < exe::utils_random(7) {',
     '        if (player.professionalism() as i32) <= exe::utils_random(7) { // mutation',
     'cx04_8 training report criticism'),
]
