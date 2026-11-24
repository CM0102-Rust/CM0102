# Mutacoes (sx04b_5), em 13 de julho de 2002 (ATE 2002-08-01): o jogador e a equipe da Copa do Mundo mudam a
# reputacao dos premiados (a carreira acusa depois) e o registro do historico (o comparador acusa):
# - o jogador do torneio exige um jogo a mais como titular;
# - a equipe do torneio aceita quem foi titular so na fracao (>= em vez de >).
MUTACOES = [
    ('sx04b_5_um_jogo_a_mais',
     'if (read_u8(row + ROW_APPEARANCES) as i32 - read_u8(row + ROW_SUB_APPEARANCES) as i32) < least { // sx04b_5: enough starts',
     'if (read_u8(row + ROW_APPEARANCES) as i32 - read_u8(row + ROW_SUB_APPEARANCES) as i32) < least + 1 { // mutation: one more start',
     'sx04b_5 tournament player candidate'),
    ('sx04b_5_equipe_na_fracao',
     'if appearances - read_u8(row + ROW_SUB_APPEARANCES) as i32 <= least { // sx04b_5: more starts than the share',
     'if appearances - (read_u8(row + ROW_SUB_APPEARANCES) as i32) < least { // mutation: the share is enough',
     'sx04b_5 tournament team candidate'),
]
