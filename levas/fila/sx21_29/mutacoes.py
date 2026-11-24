# Mutacoes (sx21_29). Com a Belgica carregada: mudam quem os clubes belgas contratam (carreira e comparador).
MUTACOES = [
    ('sx21_29_segunda_janela_livre',
     'return refuse(); // sx21_29: only from a more reputed division',
     'return base(); // mutation: from any division',
     'sx21_29 belgium second window from below'),
    ('sx21_29_comprador_sem_vendedor',
     'return sign(this, buyer, target, seller, date, reason); // sx21_29: the buyer signs the target',
     'return sign(this, buyer, target, 0, date, reason); // mutation: as a free agent',
     'sx21_29 belgium buyer check'),
]
