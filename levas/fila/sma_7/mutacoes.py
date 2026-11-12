# Mutacoes (sma_7). Os textos nao voltam para a simulacao: acusadas pela comparacao (compare_text_writer), aceito pela
# sessao principal em 12/11/2026.
# - a promocao no mesmo clube sem o nome do cargo antigo;
# - o motivo da pressao com o presidente sem o nome dele.
MUTACOES = [
    ('sma7_promocao_sem_cargo',
     'format(buf, 0xa0a274, gn, gn, gt, gt, newn, title, nm); // sma7: promoted at the club',
     'format(buf, 0xa0a274, gn, gn, gt, gt, newn, nm, nm); // mutation: without the old job\'s name',
     'sma_7 appointment body'),
    ('sma7_motivo_sem_presidente',
     'format(buf, t[0], g, g, 100, cl, ch, nm); // sma7: the reason with the chairman',
     'format(buf, t[0], g, g, 100, cl, nm, nm); // mutation: without the chairman\'s name',
     'sma_7 pressure body'),
]
