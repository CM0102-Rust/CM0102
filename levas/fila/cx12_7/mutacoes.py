# Mutacoes (cx12_7), com as 8 ligas ate 2002-06-01:
# - todo vencedor da segunda fase da J1 de 2001 vira campeao sem final (STAGE_COUNT = 2): sem a final de dezembro de
#   2001 (quando o vencedor da segunda fase nao e o da primeira), outro campeao, outros registros e outra temporada
#   arquivada; o comparador dos registros tambem acusa (retorno 1 no lugar de 2);
# - o vencedor da final recebe o texto do perdedor: o texto nao volta a simulacao; acusa o comparador do texto
#   (compare_text_writer), com o modo CM_TEXTO_NOTICIAS e um tecnico humano como destinatario;
# - a pior colocacao 5 deixa de ser a faixa (2..=4): a carreira quase nao le a previsao ate o fim da temporada; acusa o
#   comparador, lado a lado nos 3 bytes da entrada, a cada atualizacao de status.
MUTACOES = [
    ('cx12_7_j1_campeao_sem_final',
     'if read_usize(first_stage.table()) == club { // cx12_7: the first stage winner too',
     'if read_usize(first_stage.table()) == club || stage == 1 { // mutation: every second stage winner',
     'cx12_7 j1 champions rows'),
    ('cx12_7_j1_vencedor_da_final_perde',
     '(2, 1) if !news => Some((WIN_LEAGUE_TEXT, true)), // cx12_7: the final won',
     '(2, 1) if !news => Some((LOSE_FINAL_TEXT, true)), // mutation: the loser text',
     'cx12_7 j1 own fate text'),
    ('cx12_7_j1_faixa_da_pior',
     'if (2..=5).contains(&w) { // cx12_7: the worst code of the band',
     'if (2..=4).contains(&w) { // mutation: the band without 5',
     'cx12_7 j1 season progress'),
]
