# Mutacoes (nma_6). A rotina roda 372 vezes na carreira ENG ate 2002-06-01; a historia 0x1f vem do boato de
# transferencia (process_transfer_interest, com a resposta da IA) e as outras das historias novas da IA (0x72d1ed):
# - a torcida nao muda quando o clube segura um favorito (resposta 1 a 0x1f);
# - o jogador segurado contra a vontade nao passa a antipatizar com o tecnico (resposta 1 a 0x1f);
# - o humor do elenco depois do boato com outra resposta: sorteio(6) em vez de sorteio(5) (muda quem perde moral);
# - os efeitos da resposta 1 as outras historias com as marcas 0x480 em vez de 0x500.
MUTACOES = [
    ('nma6_mantido_sem_torcida',
     'add_clamped_confidence(manager_manager_row(club) + 8, 200); // nma6: kept, a favourite: supporters +200',
     'add_clamped_confidence(manager_manager_row(club) + 8, 0); // mutation: the supporters do not move',
     'nma_6 transfer interest kept'),
    ('nma6_mantido_sem_antipatia',
     'exe::staff_add_disliked_staff(player, manager, 1); // nma6: kept against his will: he dislikes the manager',
     '// mutation: no dislike of the manager',
     'nma_6 transfer interest kept'),
    ('nma6_humor_do_elenco',
     '5 // nma6: the squad\'s mood after the rumour',
     '6 // mutation: draw(6)',
     'nma_6 transfer interest other answer'),
    ('nma6_outras_marcas',
     '1 => 0x500, // nma6: the flags of answer 1',
     '1 => 0x480, // mutation: the flags of answer 2',
     'nma_6 other story'),
]
