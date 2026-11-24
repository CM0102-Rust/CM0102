# Mutacoes (sbE_5), com a Holanda ate 2002-08-01 (os grupos de maio/junho de 2002):
# - os clubes da Eredivisie na repescagem pelo status 1 em vez de 2 (outros clubes nos grupos: a carreira; o comparador
#   da fase so ve as duas fases de 4);
# - o veredito do grupo nao gravado na tabela (quem sobe e quem cai muda na virada: a carreira; acusada tambem pelo
#   comparador dos recordes, que confere o status da linha);
# - o texto de quem nao vence trocado pelo de rebaixamento (acusada pelo comparador de texto).
MUTACOES = [
    ('sbE_5_eredivisie_pelo_status_1',
     'if read_u8(exe::league_get_table_row(premier, k) + league_table_row::STATUS) == 2 { // sbE_5: the Eredivisie\'s play-off clubs',
     'if read_u8(exe::league_get_table_row(premier, k) + league_table_row::STATUS) == 1 { // mutation',
     'sbE_5 eerste divisie play-off groups'),
    ('sbE_5_veredito_sem_tabela',
     'write_u8(row + league_table_row::STATUS, s); // sbE_5: the play-off group\'s verdict',
     'let _ = (row, s); // mutation: the verdict not written',
     'sbE_5 eerste divisie play-off status'),
    ('sbE_5_texto_trocado',
     '(0xfe, _, true) => 0x9cee64,   // "... have failed to win their ... playoff group"',
     '(0xfe, _, true) => 0x9ceedc,   // mutation',
     'sbE_5 eerste divisie play-off text'),
]
