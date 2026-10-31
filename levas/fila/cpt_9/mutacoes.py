# Mutacoes. O progresso do treino roda para cada jogador a cada atualizacao (0x8b1750, na escalacao de cada partida):
# quente, sem contador. Os atributos mudados vao para o motor das partidas e para o valor do jogador. Na categoria 0, o
# primeiro atributo (+0x1b, de 1 a 20) anda o dobro do passo. Na categoria 3, o meio byte do progresso nao e guardado:
# o passo passa a depender so do progresso do dia e os atributos andam em outros momentos. Na categoria 1, os atributos
# de -125 a 125 andam cinco vezes o passo em vez de seis.
MUTACOES = [
    ('cpt9_categoria0_dobro', 'cpt9_soma20(f + 0x1b, dd); // cpt9: o primeiro atributo da categoria 0',
     'cpt9_soma20(f + 0x1b, dd * 2); // mutacao: o dobro do passo'),
    ('cpt9_categoria3_sem_guardar', 'cpt9_alto(e + 4, k); // cpt9: guarda o progresso da categoria 3',
     'let _ = k; // mutacao: nao guarda o progresso da categoria 3'),
    ('cpt9_categoria1_cinco', 'let seis = dd * 6; // cpt9: a categoria 1, seis vezes o passo',
     'let seis = dd * 5; // mutacao: cinco vezes o passo'),
]
