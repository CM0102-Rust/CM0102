# Mutacoes (sx20_15), item de TELAS: a carreira automatica nao passa; servem para provar o comparador numa sessao do
# Lucas jogando comparando:
# - a frase do status com o status seguinte (o perfil de um jogador mostra a frase);
# - o batedor gravado na cobranca seguinte (Taticas, batedores da disputa por penaltis);
# - a quarta categoria do historico perdida (o historico de clube com trocas de tecnico, tipo 0x96, filtrado).
MUTACOES = [
    ('sx20_15_frase_seguinte',
     'langlib_text(out, sentence, &[]); // sx20_15: the squad status sentence',
     'langlib_text(out, SQUAD_STATUS_SENTENCES[(status as usize) % 8], &[]); // mutation: the next sentence'),
    ('sx20_15_cobranca_seguinte',
     'write_i8(at + TACTICS_ROW_SHOOTOUT_TAKERS + kick as usize, slot); // sx20_15: the taker\'s slot',
     'write_i8(at + TACTICS_ROW_SHOOTOUT_TAKERS + kick as usize + 1, slot); // mutation: the next kick'),
    ('sx20_15_sem_quarta',
     '|| (mask & 8 != 0 && (0x96..=0xc7).contains(&kind)) // sx20_15: the fourth category',
     '|| false // mutation: no fourth category'),
]
