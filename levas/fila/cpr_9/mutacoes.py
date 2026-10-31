# Mutacoes. A busca de novas aposentadorias sem a do desempregado (0x7a8050 nao roda: os sorteios dela somem e os
# desempregados nao se aposentam; os regens mudam). Na saida dos cargos, as vagas de treinador ficam com o aposentado
# (o clube continua com o treinador no elenco tecnico; a distribuicao dos treinadores e o treino mudam) e a vaga do
# tecnico nao abre (o clube fica sem tecnico e sem procura: a 0x69ba50 nao roda). O registro da liberacao so grava o
# historico em arquivo (fora da impressao): sem mutacao util.
MUTACOES = [
    ('cpr9_sem_desempregado', '} else if desempregado(p, e.as_mut_ptr()) != 0 {',
     '} else if false && desempregado(p, e.as_mut_ptr()) != 0 { // mutacao: sem o desempregado',
     'cpr_9 desempregado'),
    ('cpr9_treinador_fica', 'limpar(0x19f, 5); // cpr9: as vagas de treinador',
     '// mutacao: as vagas de treinador ficam',
     'cpr_9 treinador'),
    ('cpr9_vaga_nao_abre', 'abrir(cpr9_d(0xb67a44), c, pessoa); // cpr9: a vaga do técnico abre',
     'let _ = abrir; // mutacao: a vaga nao abre',
     'cpr_9 tecnico'),
]
