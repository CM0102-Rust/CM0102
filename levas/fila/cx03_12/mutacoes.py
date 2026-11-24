# Mutacoes (cx03_12), carreira com as ligas da Suecia e de Portugal ate 2002-08-01. Nenhuma abre caixa.
# - o premio da Taca de Portugal 900.000: o comparador das rodadas acusa na criacao;
# - a Copa da Suecia vira uma semana depois (8 de julho): a edicao nova muda de data (a carreira);
# - o progresso das secoes 8n - 6: o comparador do progresso acusa a cada posicao gravada.
MUTACOES = [
    ('cx03_12_premio_taca_de_portugal',
     '    set_up_prize_rounds(Cup(this), 800_000)',
     '    set_up_prize_rounds(Cup(this), 900_000) // mutation'),
    ('cx03_12_suecia_vira_uma_semana_depois',
     '    update_cup_with_turnover_on(Cup(this), today, stage, 1, 6, "cx03_12 swedish cup next edition")',
     '    update_cup_with_turnover_on(Cup(this), today, stage, 8, 6, "cx03_12 swedish cup next edition") // mutation'),
    ('cx03_12_secao_8n_menos_6',
     '        let scale = |code: u8| (code << 3).wrapping_sub(7);',
     '        let scale = |code: u8| (code << 3).wrapping_sub(6); // mutation'),
]
