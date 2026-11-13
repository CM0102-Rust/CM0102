# Mutacoes (sma_7). Os textos nao voltam para a simulacao: acusadas pela comparacao (compare_text_writer), aceito pela
# sessao principal em 12/11/2026.
# - o corpo da nomeacao liga o clube como pessoa (0x7d7 em vez de 0x7d5; roda em todo corpo; trocada em 13/11/2026: a
#   promocao no mesmo clube sem o nome do cargo escapava, esse caso nao saiu na carreira);
# - o motivo da pressao com o presidente sem o nome dele.
MUTACOES = [
    ('sma7_vinculo_do_clube_trocado',
     '    exe::text_add_link(out, newn as *const u8, 0x7d5, new);',
     '    exe::text_add_link(out, newn as *const u8, 0x7d7, new); // mutation: the club linked as a person',
     'sma_7 appointment body'),
    ('sma7_motivo_sem_presidente',
     'format(buf, t[0], g, g, 100, cl, ch, nm); // sma7: the reason with the chairman',
     'format(buf, t[0], g, g, 100, cl, nm, nm); // mutation: without the chairman\'s name',
     'sma_7 pressure body'),
]
