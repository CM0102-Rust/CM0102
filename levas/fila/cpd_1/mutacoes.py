# Mutacoes do fim das convocacoes e da poda. A da competicao inverte o filtro (2 chamadas: todos os outros registros
# terminam amanha) e a da nacao inverte o filtro da nacao (34): mudam a volta dos jogadores aos clubes; a carreira ate
# 2002-06-01 acusa se isso mudar alguma escalacao, e AS DUAS PODEM ESCAPAR (os registros ficam no gerenciador das
# selecoes, fora da impressao do banco; o comparando acusa na primeira chamada). A da poda nao conta as vencidas (9
# chamadas): os registros velhos ficam; PODE ESCAPAR pelo mesmo motivo.
MUTACOES = [
    ('cpd_fim_competicao', 'cpd1_d(r + 4) == cpd1_d(comp) /* a competição */', 'cpd1_d(r + 4) != cpd1_d(comp) /* mutacao: as outras */'),
    ('cpd_fim_nacao', 'cpd1_d(r) == cpd1_d(nacao) /* a nação */', 'cpd1_d(r) != cpd1_d(nacao) /* mutacao: as outras */'),
    ('cpd_poda_vencidas', 'vencidas = vencidas.wrapping_add(1); // uma vencida', 'vencidas = vencidas.wrapping_add(0); // mutacao: nenhuma vencida'),
]
