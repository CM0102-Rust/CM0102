# Mutacoes (sx17_4), item de TELAS: a carreira automatica nao passa por estas rotinas; servem para provar o comparador
# numa sessao do Lucas jogando comparando:
# - a tarefa de 1..4 dias em vez de 1..3 (o comparador da tarefa confere os dias: acusa quando sai o 4);
# - o clube-alvo fica como esta ao encerrar a tarefa (o comparador confere o −1: acusa com clube-alvo);
# - a media como soma (sem dividir: o comparador da media acusa com mais de um jogo).
MUTACOES = [
    ('sx17_4_dias_da_tarefa',
     'let days = exe::utils_random(3) + 1; // sx17_4: the days of a nation, region, competition or youth assignment',
     'let days = exe::utils_random(4) + 1; // mutation: up to four days'),
    ('sx17_4_alvo_mantido',
     'write_i32(this + SCOUT_DAY_TARGET_CLUB, -1); // sx17_4: no target club',
     '// mutation: the target club stays'),
    ('sx17_4_media_sem_dividir',
     'sum as i16 as f64 / count as f64 // sx17_4: the average grade',
     'sum as i16 as f64 // mutation: the sum'),
]
