# Mutacoes (cx04_7). As historias vao so para a caixa dos tecnicos humanos; o comparador (compare_story_post) ensaia a
# copia sem enviar e confere os sorteios e a historia entregue:
# - o olheiro sorteia duas vezes: os sorteios da copia nao sao o comeco dos do original (e a carreira muda);
# - a inquietacao no vestiario com o teste do tecnico humano invertido: a copia monta a historia para os clubes da
#   maquina (com os sorteios da redacao) e nao para o humano;
# - o jogador infeliz com o papel com a marca da lista de transferencias invertida: a historia entregue ao humano
#   difere (so quando um jogador do clube humano a dispara).
MUTACOES = [
    ('cx04_7_olheiro_dois_sorteios',
     '    let draw = exe::utils_random(3);\n    let mut item = [0u32; LOCAL_NEWS_WORDS];\n    let n = new_local_news(&mut item, 0xfbd);',
     '    let draw = exe::utils_random(3);\n    exe::utils_random(3); // mutation\n    let mut item = [0u32; LOCAL_NEWS_WORDS];\n    let n = new_local_news(&mut item, 0xfbd);',
     'cx04_7 scout watching story sent'),
    ('cx04_7_inquietacao_humano_invertido',
     '    if !player.club_job().is_null() && exe::news_club_has_human_manager(read_usize(G_NEWS), player.club_job()) == 0 {\n        return;\n    }\n    let mut item = [0u32; LOCAL_NEWS_WORDS];\n    let n = new_local_news(&mut item, 0xbc7);',
     '    if !player.club_job().is_null() && exe::news_club_has_human_manager(read_usize(G_NEWS), player.club_job()) != 0 { // mutation\n        return;\n    }\n    let mut item = [0u32; LOCAL_NEWS_WORDS];\n    let n = new_local_news(&mut item, 0xbc7);',
     'cx04_7 dressing room unrest story'),
    ('cx04_7_papel_lista_invertida',
     '    set_news_slot(n, 5, (exe::staff_is_transfer_listed(player, 0) != 0) as u32);',
     '    set_news_slot(n, 5, (exe::staff_is_transfer_listed(player, 0) == 0) as u32); // mutation',
     'cx04_7 unhappy with role story'),
]
