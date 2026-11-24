# Mutacoes (sx01d_2). As duas rotinas nao aparecem na cobertura ENG ate 2002-06-01: so sao acusadas se rodarem (gesto
# obsceno filmado; jogador que quer ser titular). Mudam a sequencia do gerador:
# - o gesto obsceno sem o sorteio da vaga 5;
# - o jogador que quer ser titular sem pedir a resposta do tecnico do computador (que sorteia).
MUTACOES = [
    ('sx01d_gesto_sem_sorteio',
     'exe::newsitem_set_slot(n, 5, exe::utils_random(2) as u32);',
     'exe::newsitem_set_slot(n, 5, 0); // mutation: no draw',
     'sx01d_2 obscene gesture story'),
    ('sx01d_titular_sem_resposta',
     'exe::media_pick_ai_manager_response(this, club.manager(), player, 0x83)',
     '0 /* mutation */',
     'sx01d_2 wants first team story'),
]
