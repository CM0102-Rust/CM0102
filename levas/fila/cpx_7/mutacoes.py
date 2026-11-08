# Mutacoes (cpx_7) pela prova do save (mutacoes.py --prova-save N): cada uma encurta um item do .qst (o zwrite devolve a
# quantidade de registros, entao nenhuma cai na caixa de erro nem no fatal_error_at). Sem o 4o campo (contador): a base da
# prova do save roda com as rotinas desligadas.
# - o historico dos clubes com registros de 0xf bytes: club history.dat 2.520 bytes menor e deslocado;
# - as estatisticas dos arbitros por nacao com 0x101 bytes: off.dat 213 bytes menor e deslocado;
# - as estatisticas simuladas com 0x43 bytes: simulated_stats.dat 125 bytes menor e deslocado.
MUTACOES = [
    ('cpx7_historico_curto', '    if zip_write(cpx7_d(this + 8), 0x10, cpx7_d(this + 0xc) as u32, z) != cpx7_d(this + 0xc) as u32 { // cpx7: os registros do histórico',
     '    if zip_write(cpx7_d(this + 8), 0xf, cpx7_d(this + 0xc) as u32, z) != cpx7_d(this + 0xc) as u32 { // mutacao: registro curto'),
    ('cpx7_nacao_curta', '    if zip_write(cpx7_d(this + 0xc), 0x102, nacoes(), z) != nacoes() { // cpx7: os registros das nações',
     '    if zip_write(cpx7_d(this + 0xc), 0x101, nacoes(), z) != nacoes() { // mutacao: registro curto'),
    ('cpx7_simulado_curto', '    if zip_write(cpx7_d(this + 4), 0x44, cpx7_d(this) as u32, z) != cpx7_d(this) as u32 { // cpx7: os registros simulados',
     '    if zip_write(cpx7_d(this + 4), 0x43, cpx7_d(this) as u32, z) != cpx7_d(this) as u32 { // mutacao: registro curto'),
]
