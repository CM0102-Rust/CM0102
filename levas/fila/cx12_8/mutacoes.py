# Mutacoes (cx12_8), com as 8 ligas ate 2002-06-01:
# - o vice pela fase de menos pontos no segundo lugar: so muda o historico quando o mesmo clube vence as duas fases da
#   J1 de 2001 (sem final; o caminho do covered "cx12_8 j1 won both stages"). Com final, o caminho e o salto de cauda
#   para a fase final, sem regra propria. O historico quase nao volta a simulacao ate junho de 2002, e o comparador
#   roda so a original (escreve o historico): a mutacao pode escapar; entao a prova diz se o caminho rodou.
MUTACOES = [
    ('cx12_8_j1_vice_da_outra_fase',
     'let runner_up_stage = if second_row.points() > first_row.points() { second } else { first }; // cx12_8: the better second place',
     'let runner_up_stage = if second_row.points() > first_row.points() { first } else { second }; // mutation: the other stage',
     'cx12_8 j1 won both stages'),
]
