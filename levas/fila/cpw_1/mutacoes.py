# Mutacoes (cpw_1), todas na satisfacao com o contrato (0x866620, 10359 chamadas ate 2002-06-01 na cobertura de
# 17/10, pelo passe diario do pessoal): o resultado escolhe o ramo do dia do jogador (1: pedir contrato novo ou sair;
# 2: ficar mais feliz, +0x44 do contrato; 3: as condicoes de quem esta bem). O limit da insatisfacao em 60 (e nao 50) e
# o da satisfacao em 70 (e nao 80) mudam o resultado de quem esta entre eles; o desconto do recem-contratado em +30 (e
# nao +50) deixa os recem-chegados menos satisfeitos. As tres mudam pedidos, felicidade e sorteios dos dias seguintes.
MUTACOES = [
    ('cpw1_insatisfacao_60', '    if !(razao >= read_f32_at(0x969560) as f64) { // cpw1: o limit da insatisfação',
     '    if !(razao >= 60.0) { // mutacao: o limit em 60',
     'cpw_1 satisfaction evaluated'),
    ('cpw1_satisfacao_70', '    if razao < read_f32_at(0x96ff70) as f64 { // cpw1: o limit da satisfação',
     '    if razao < 70.0 { // mutacao: o limit em 70',
     'cpw_1 satisfaction evaluated'),
    ('cpw1_recem_contratado_30', '        razao = razao + read_f32_at(0x969560) as f64 + adapt as f64 * read_f64_at(0x968eb0) - (dias / 2) as f64; // cpw1: o desconto do recém-contratado',
     '        razao = razao + 30.0 + adapt as f64 * read_f64_at(0x968eb0) - (dias / 2) as f64; // mutacao: +30',
     'cpw_1 recently signed'),
]
