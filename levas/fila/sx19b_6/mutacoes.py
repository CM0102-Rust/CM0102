# Mutacoes (sx19b_6), item de TELAS: a carreira automatica nao passa; servem para provar os comparadores numa sessao do
# Lucas jogando comparando:
# - o jogador de comparacao 1 nunca muda (o comparador do set_compare_player acusa o par planejado);
# - a competicao de selecao fica ate a ultima data, sem a folga de 7 dias (o comparador da lista acusa perto do fim
#   de uma data FIFA).
MUTACOES = [
    ('sx19b_6_comparacao_parada',
     'write_usize(a, p1); // sx19b_6: compare player 1',
     '// mutation: compare player 1 kept'),
    ('sx19b_6_sem_folga',
     '!date_vs_today(date_bytes(exe::date_sub_days(last.as_ptr() as usize, tmp.as_mut_ptr() as usize, 7)), true) // sx19b_6: still current',
     '!date_vs_today(last, true) // mutation: no slack'),
]
