# Mutacoes (cx09_11) pela carreira com a Belgica ate 2002-08-01 (as repescagens jogam em maio de 2002 e a virada de
# julho sobe e desce quem elas decidem):
# - a repescagem de acesso leva o 15o da Segunda (e nao o 16o): outro clube joga e quem sobe ou cai muda;
# - na repescagem de rebaixamento os dois da Terceira entram com semente 2: o sorteio dos confrontos muda.
MUTACOES = [
    ('cx09_11_acesso_com_o_15o',
     '    list[6] = LeagueTableRow(upper.table() + 15 * league_table_row::SIZE).club() as usize;',
     '    list[6] = LeagueTableRow(upper.table() + 14 * league_table_row::SIZE).club() as usize; // mutation'),
    ('cx09_11_rebaixamento_sem_semente',
     '    let seeds: [u8; 14] = [2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 1, 1];',
     '    let seeds: [u8; 14] = [2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2]; // mutation'),
]
