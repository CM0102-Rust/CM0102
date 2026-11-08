# Mutacoes da tabela da liga. A da ordenacao poe cada clube uma posicao abaixo (+4 e historico): roda depois de cada
# rodada, a carreira ate 2002-06-01 deve acusar. A da liberacao zera a tabela da liga quando solta uma copia (as da
# 0x687710, todo dia): deve acusar. A do fim da liga da a liga por completa uma rodada antes (premios e registros antes
# da hora, em maio de 2002): deve acusar. A da minitabela (a posicao nao desce nos pares separados) so roda nas ligas
# com a regra 4 (179 chamadas): PODE ESCAPAR se os empates em pontos forem raros (o comparando acusa no primeiro).
MUTACOES = [
    ('cpa_ordem_posicao', '            *((r + 4) as *mut i8) = c; // a posição',
     '            *((r + 4) as *mut i8) = c.wrapping_add(1); // mutacao: uma posicao abaixo'),
    ('cpa_mini_posicao', '                pos = pos.wrapping_sub(1); // a posição desce',
     '                pos = pos.wrapping_sub(0); // mutacao: a posicao nao desce'),
    ('cpa_liberar_propria', '    if propria { // a da liga: zera a contagem e a tabela',
     '    if !propria { // mutacao: zera a da liga ao soltar uma copia'),
    ('cpa_completa_rodadas', '            if ((cpa1_b(tab + LEAGUE_TABLE_ROW_SIZE * i + 0x11) as i8) as i16) < rodadas { // ainda falta jogar',
     '            if ((cpa1_b(tab + LEAGUE_TABLE_ROW_SIZE * i + 0x11) as i8) as i16) < rodadas - 1 { // mutacao: uma rodada antes'),
]
