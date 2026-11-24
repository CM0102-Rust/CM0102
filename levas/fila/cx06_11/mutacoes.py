# Mutacoes (cx06_11), com as 8 ligas e ATE 2004-11-01:
# - a repescagem so e criada na fase dos segundos (+0x100), nao na +0x101: sem a repescagem de nov/2003 (os jogos e os
#   classificados mudam);
# - com uma sede europeia so 2 melhores segundos se classificam direto: outra selecao vai a Eurocopa de 2004;
# - os vencedores da repescagem ganham a situacao 6 no lugar de 7: o comparador lado a lado acusa.
MUTACOES = [
    ('cx06_11_sem_repescagem',
     '    if stage == read_u8(this + 0x100) as i8 as i32 || stage == read_u8(this + 0x101) as i8 as i32 { // the runners-up / play-off stages',
     '    if stage == read_u8(this + 0x100) as i8 as i32 { // mutation',
     'cx06_11 qualifiers play-off stage'),
    ('cx06_11_dois_melhores_segundos',
     '    let best_runners_up: i8 = if one_host { 3 } else { 1 };',
     '    let best_runners_up: i8 = if one_host { 2 } else { 1 }; // mutation',
     'cx06_11 best runner-up qualified'),
    ('cx06_11_situacao_errada',
     '                        write_u8(exe::league_get_table_row(group, r) + league_table_row::STATUS, 7);',
     '                        write_u8(exe::league_get_table_row(group, r) + league_table_row::STATUS, 6); // mutation',
     'cx06_11 play-off winner marked'),
]
