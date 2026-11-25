# Mutacoes (rx09_21) pela prova do save (mutacoes.py --prova-save N; CARGA): as fases de liga do save de ferias (os
# grupos da Liga dos Campeoes e das copas com fases de grupo) sao lidas pela carga.
# - a competicao de cada fase carregada e a seguinte do banco (id + 1): a fase carregada muda (a impressao da carga e o
#   comparador acusam);
# - a fase sem rodadas com a data de hoje um ano adiante: a rodada unica muda (a impressao acusa quando houver uma).
MUTACOES = [
    ('rx09_21_competicao_seguinte',
     'read_usize(G_CLUB_COMP_TABLE).wrapping_add((comp_id as usize).wrapping_mul(0x6b)) }); // rx09_21: the competition record',
     'read_usize(G_CLUB_COMP_TABLE).wrapping_add((comp_id as usize + 1).wrapping_mul(0x6b)) }); // mutation: the next one',
     'rx09_21 league stage rows loaded'),
    ('rx09_21_hoje_um_ano_adiante',
     "write_u16(d + 2, read_u16(G_CURRENT_DATE_YEAR).wrapping_sub(l.year() as u16)); // rx09_21: today's round, as an offset from the year",
     'write_u16(d + 2, read_u16(G_CURRENT_DATE_YEAR).wrapping_sub(l.year() as u16).wrapping_add(1)); // mutation: a year later',
     'rx09_21 league stage for loading'),
]
