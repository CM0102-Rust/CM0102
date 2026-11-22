# Mutacoes (tel02_2): a carreira automatica nao abre telas, entao a cadeia nao roda mutacoes nesta leva; estas provam
# o comparador lado a lado pela lista numa sessao do Lucas (JOGAR_COMPARANDO.bat):
# - a comissao nao jogadora uma linha acima (Info > General Info de qualquer clube com comissao);
# - o salario anual de 48 semanas (Info > Stats de um clube, salario mostrado por ano);
# - o lucro com o sinal trocado (Info > Finances do proprio clube);
# - as colunas das temporadas trocadas (Info > Income ou Expenditure do proprio clube).
MUTACOES = [
    ('tel02_2_linha_comissao',
     "let mut row = 1usize;",
     "let mut row = 0usize; // mutation: the staff list starts one row up",
     'tel02_2 info tab general'),
    ('tel02_2_salario_anual',
     "(weekly.wrapping_mul(52) as u32, 4)",
     "(weekly.wrapping_mul(48) as u32, 4) // mutation: a year of 48 weeks",
     'tel02_2 stats view'),
    ('tel02_2_lucro_invertido',
     "let diff = income[k].wrapping_sub(spent[k]); // tel02_2 profit is income minus expenditure",
     "let diff = spent[k].wrapping_sub(income[k]); // mutation: profit sign reversed",
     'tel02_2 info tab finances view'),
    ('tel02_2_colunas_temporada',
     "const LEDGER_BLOCKS: [usize; 4] = [finance::MONTH_PLAYERS_BOUGHT, 0x20, finance::SEASON_PLAYERS_BOUGHT, finance::LAST_SEASON_TOTALS];",
     "const LEDGER_BLOCKS: [usize; 4] = [finance::MONTH_PLAYERS_BOUGHT, 0x20, finance::LAST_SEASON_TOTALS, finance::SEASON_PLAYERS_BOUGHT]; // mutation: season columns swapped",
     'tel02_2 income view'),
]
