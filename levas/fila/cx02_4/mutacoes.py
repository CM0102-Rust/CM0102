# Mutacoes (cx02_4), carreira ENG ate 2002-06-01. Nenhuma abre caixa.
# - o premio da Copa da Liga 2.000.000: o dinheiro das rodadas muda (a carreira) e o comparador acusa na criacao;
# - a Charity Shield sem premio: o comparador acusa na criacao;
# - o progresso das secoes 2n e nao 2n - 1: o comparador acusa a cada posicao gravada.
MUTACOES = [
    ('cx02_4_premio_copa_da_liga',
     '    set_up_prize_rounds(Cup(this), 1_000_000)',
     '    set_up_prize_rounds(Cup(this), 2_000_000) // mutation'),
    ('cx02_4_charity_sem_premio',
     '    set_up_prize_rounds(Cup(this), 300_000)',
     '    set_up_prize_rounds(Cup(this), 0) // mutation'),
    ('cx02_4_progresso_2n',
     '        current = (current << 1).wrapping_sub(1);',
     '        current = current << 1; // mutation'),
]
