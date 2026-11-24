# Mutacoes (cx13_7), com a Polonia ate 2002-08-01 (a repescagem em junho de 2002):
# - a repescagem sem esperar a Primeira acabar (montada com a classificacao da fase final do dia: outros clubes e
#   outras datas; quem sobe e quem cai muda);
# - os clubes da Primeira na repescagem pelo status 3 em vez de 2 (outros clubes jogam a repescagem; acusada tambem
#   pelo comparador, o plano contra a lista da fase criada).
# (Sem a mutacao "nenhuma repescagem": a fase 0 ficaria nula e o jogo cairia no dia seguinte.)
MUTACOES = [
    ('cx13_7_sem_esperar_a_primeira',
     'if complete(first, 0, 1) == 0 {',
     'if false && complete(first, 0, 1) == 0 {',
     'cx13_7 polish second next stage'),
    ('cx13_7_primeira_pelo_status_3',
     'if read_u8(exe::league_get_table_row(final_stage, k) + league_table_row::STATUS) == 2 {',
     'if read_u8(exe::league_get_table_row(final_stage, k) + league_table_row::STATUS) == 3 {',
     'cx13_7 polish play-off'),
]
