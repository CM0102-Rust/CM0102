# Mutacoes, todas no fim da lesao (0x62e700: toda lesao que termina na carreira, pelo fisico do dia 0x62c680). O +0xe do
# registro e a lesao atual que a escalacao e o fisico consultam (0x62ed70): sem o -1 o jogador segue lesionado (numa
# entrada livre); a entrada 0 da tabela que nunca termina e o id da metade de cima que nao confere deixam jogadores
# lesionados para sempre: a escalacao e o motor das partidas mudam.
MUTACOES = [
    ('cpv10_fica_a_lesao_atual',
     'core::ptr::write_unaligned((cpv10_registro(this, cpv10_d(pessoa)) + 0xe) as *mut i32, -1); // cpv10: sem lesão atual',
     'let _ = cpv10_registro(this, cpv10_d(pessoa)); // mutacao: o +0xe fica',
     'cpv_10 fim da lesao'),
    ('cpv10_entrada_zero_nao_termina',
     'if i == -1 { // cpv10: a lesão atual',
     'if i <= 0 { // mutacao: a entrada 0 nao termina',
     'cpv_10 entrada zero'),
    ('cpv10_metade_de_cima_nao_termina',
     'if id >= cpv10_d(0xae2364) { // cpv10: o id da pessoa',
     'if id >= cpv10_d(0xae2364) / 2 { // mutacao: a metade de cima nao termina',
     'cpv_10 metade de cima'),
]
