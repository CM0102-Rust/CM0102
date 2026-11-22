# Mutacao (tel29_4): a carreira automatica nao abre telas; esta prova o comparador numa sessao do Lucas
# (JOGAR_COMPARANDO.bat): a tela abre na pagina 2 (o comparador pega ao clicar em "Details" na noticia das regras
# novas de transferencia).
MUTACOES = [
    ('tel29_4_pagina',
     "exe::screenmanager_set_screen_param(G_SCREEN_MANAGER, 0, 1usize, 0); // tel29_4: the first page",
     "exe::screenmanager_set_screen_param(G_SCREEN_MANAGER, 0, 2usize, 0); // mutation: the second page",
     'tel29_4 transfer rules screen opened'),
]
