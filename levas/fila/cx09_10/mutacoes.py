# Mutacoes (cx09_10) pela carreira com a Belgica ate 2002-08-01 (as repescagens e a final acabam em maio de 2002; a
# virada de julho usa os status das linhas):
# - o clube da Terceira que vence a repescagem de acesso fica com status 0xfe (e nao 0): nao sobe, a virada muda e o
#   comparador do plano acusa;
# - a final do titulo com o lider do grupo B em casa: o jogo e o campeao mudam.
MUTACOES = [
    ('cx09_10_vencedor_nao_sobe',
     '                1 if own => plan(row, Some(0), BelThirdRecord::Promotion, id),',
     '                1 if own => plan(row, Some(0xfe), BelThirdRecord::Promotion, id), // mutation'),
    ('cx09_10_final_invertida',
     '    let pair = [LeagueTableRow(l.table()).club() as usize, LeagueTableRow(League(read_usize(l.stages())).table()).club() as usize];',
     '    let pair = [LeagueTableRow(League(read_usize(l.stages())).table()).club() as usize, LeagueTableRow(l.table()).club() as usize]; // mutation'),
]
