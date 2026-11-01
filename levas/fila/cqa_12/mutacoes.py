# Mutacoes (cqa_12), todas na subida da reputacao do vice da copa (0x86b0b0, pelo cqa12_colocado que o terceiro
# tambem usa): a reputacao do clube (+0x80) esta no banco (impressao diaria) e e lida pelas financas, pela midia e
# pelas transferencias. Sem a subida, com o teto de 100 por vez em vez de 350 ou com o limiar pela metade (peso x 250),
# a reputacao do vice de cada final ate 2002-06-01 (copas da liga e da FA, Liga dos Campeoes, copas das nacoes) muda.
# Escapam se nenhum vice estiver abaixo de peso x 500 (o contador mostra) ou, no teto, se a falta for sempre ate 100.
MUTACOES = [
    ('cqa12_vice_sem_reputacao', '        cqa12_pw(clube + 0x80, v); // cqa12: a reputação do colocado',
     '        let _ = v; // mutacao: a reputacao do colocado nao sobe',
     'cqa_12 reputacao do vice'),
    ('cqa12_teto_de_100', '        let soma = if falta > teto { teto } else { falta }; // cqa12: o teto da subida',
     '        let soma = if falta > 100 { 100 } else { falta }; let _ = teto; // mutacao: teto de 100',
     'cqa_12 reputacao do vice'),
    ('cqa12_limiar_pela_metade', '    let alvo = (peso as i32).wrapping_mul(limiar); // cqa12: o limiar da reputação',
     '    let alvo = (peso as i32).wrapping_mul(limiar / 2); // mutacao: o limiar pela metade',
     'cqa_12 reputacao do vice'),
]
