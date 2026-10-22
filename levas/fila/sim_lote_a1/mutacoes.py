# Mutacoes do fim do emprestimo, da contratacao pelas regras e das trocas que oferecem o jogador. Uso: python mutacoes.py mutacoes\levaN.py --carreira 2002-06-01
MUTACOES = [
    ('tr_recusa', "        return 0; // recusado, sem segunda pergunta", "        return 1; // recusado, sem segunda pergunta"),
    ('tr_sem_trocas', "        return 0; // sem trocas marcadas", "        return 1; // sem trocas marcadas"),
]
