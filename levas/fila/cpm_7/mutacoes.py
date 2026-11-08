# Mutacoes. O amistoso perto da data sem conferir o dia antes e sem conferir o dia depois (o planejamento 0x5c4380
# aceita a primeira data em vez de pass_ball para a seguinte: os amistosos mudam de dia, e com eles as partidas e os
# sorteios da carreira). O amistoso planejado com o mandante e o visitante trocados (a partida muda de casa: publico,
# renda e o mando). Nenhuma na competicao na data (chamada pelas copas e ligas a cada data: quente, sem contador).
MUTACOES = [
    ('cpm_amistoso_dia_antes',
     'if item == antes {\n            crate::comparison::branch("cpm_7 amistoso no dia antes");',
     'if false && item == antes { // mutacao: sem o dia antes\n            crate::comparison::branch("cpm_7 amistoso no dia antes");',
     'cpm_7 amistoso no dia antes'),
    ('cpm_amistoso_dia_depois',
     'if item == depois {\n            crate::comparison::branch("cpm_7 amistoso no dia depois");',
     'if false && item == depois { // mutacao: sem o dia depois\n            crate::comparison::branch("cpm_7 amistoso no dia depois");',
     'cpm_7 amistoso no dia depois'),
    ('cpm_amistoso_mando',
     'cpm7_por(item + 0xc, cpm7_d(mandante)); // o mandante\n    cpm7_por(item + 0x10, cpm7_d(visitante)); // o visitante',
     'cpm7_por(item + 0xc, cpm7_d(visitante)); // mutacao: trocados\n    cpm7_por(item + 0x10, cpm7_d(mandante));',
     'cpm_7 amistoso planejado'),
]
