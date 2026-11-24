# Mutacoes (sx02_2). A lista volta a quem posta a noticia; o comparador lado a lado acusa qualquer clube a mais ou a
# menos (a carreira de ferias tem um tecnico humano ingles; as ofertas por jogadores ingleses o alcancam pela bandeira
# 0x10).
# - o alcance nacional desligado;
# - o limite nacional 0x1000 + bonus em vez de 0x1c20 + bonus (mais ofertas alcancam a nacao);
# - a divisao do clube do alvo desligada.
MUTACOES = [
    ('sx02_2_sem_alcance_nacional',
     'r.national = true; // sx02_2: national by reputation',
     'r.national = false; // mutation: no national reach',
     'sx02_2 national reach'),
    ('sx02_2_limite_nacional',
     'let national = cap(bonus + 0x1c20);',
     'let national = cap(bonus + 0x1000); // mutation: a lower national threshold',
     'sx02_2 national reach'),
    ('sx02_2_sem_divisao_do_alvo',
     'r.target_division = true; // sx02_2: the target\'s division',
     'r.target_division = false; // mutation: no target division',
     'sx02_2 national reach'),
]
