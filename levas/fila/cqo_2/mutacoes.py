# Mutações (cqo_2): o público entra na renda dos jogos (as finanças dos clubes, no hash diário do banco) e no motor.
# - o clássico sem o ×1,5 do mandante (contador "cqo_2 classico");
# - os primeiros da liga com +10 por clube em vez de +100 (contador "cqo_2 posicao de cima");
# - o sorteio do total de 0 a 48 em vez de 0 a 49 (todo jogo; muda o total sem mudar a conta dos sorteios).
MUTACOES = [
    ('cqo2_classico_sem_bonus', 'pc = ftol(pc as f64 * f64_em(0x968940)); // cqo2: o clássico', 'pc = ftol(pc as f64 * 1.0); // mutacao: o clássico sem o bônus', 'cqo_2 classico'),
    ('cqo2_primeiros_sem_bonus', 'n * 100 // cqo2: os primeiros', 'n * 10 // mutacao: os primeiros quase sem bônus', 'cqo_2 posicao de cima'),
    ('cqo2_sorteio_do_total', 'let r = cqo2_sortear(0x32); // cqo2: o sorteio do total', 'let r = cqo2_sortear(0x31); // mutacao: o sorteio do total menor'),
]
