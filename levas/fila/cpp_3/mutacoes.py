# Mutacoes (com o ramo de cada uma). A diretoria do tecnico novo com a base 9000 em vez de 10000 (a confianca e os
# humores de todo tecnico contratado descem: as demissoes e os motivos mudam; a base vai a 10000 quase sempre porque a
# original compara com o double de 0x96d1b0, um denormal). O motivo da saida com um sorteio a mais (o sorteador anda
# e a carreira se separa na primeira saida de tecnico). O registro do tecnico sem criar a vaga (os tecnicos novos
# ficam sem registro: os pontos e o tecnico do mes mudam; se isso nao mudar o que a carreira le ate 2002-06-01,
# PODE ESCAPAR; comparando, acusa).
MUTACOES = [
    ('cpp_base_da_diretoria', '        q = 10000.0;',
     '        q = 9000.0; // mutacao: a base mais baixa',
     'cpp_3 base no teto'),
    ('cpp_motivo_sorteio', '    let temporada = em_temporada(clube, 0xae2c90);',
     '    let temporada = em_temporada(clube, 0xae2c90);\n    sortear(2); // mutacao: um sorteio a mais',
     'cpp_3 motivo da saida'),
    ('cpp_registro_sem_vaga', '            cpp3_por(r + 0x86, cpp3_d(pessoa)); // a vaga fica com ele\n            break r;',
     '            return 0; // mutacao: sem criar o registro',
     'cpp_3 criar registro'),
]
