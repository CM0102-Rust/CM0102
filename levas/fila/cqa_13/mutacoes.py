# Mutacoes (cqa_13), todas na 0x758200, que roda no fim de cada transferencia (jogador de 19 anos ou mais com injury_word_0c
# 4250). A goal_crowd_reaction do novo clube (+8 da linha em [[0xb67a44]]) esta na impressao do banco: sem os +500 da contratacao
# de um craque (potencial acima de 6250), a impressao muda no dia. Sem tirar a marca 0x400 com o elenco forte (limit
# 100 em vez de 20..32) e com a moral do companheiro caindo 50 em vez de 25, os contratos mudam (a moral e as marcas
# entram na vontade de sair, 0x866f60, e nos pedidos de transferencia). Podem escapar se nenhum craque mudar de clube ate
# 2002-06-01, se nenhum contrato do elenco novo tiver a marca 0x400 ou se o caminho da moral nao rodar (os contadores
# mostram); o comparando acusa na primeira vez que o ramo roda sem a copia parar.
MUTACOES = [
    ('cqa13_torcida_sem_500', '            let x = cqa13_w(linha + 8).wrapping_add(0x1f4); // cqa13: a goal_crowd_reaction comemora',
     '            let x = cqa13_w(linha + 8).wrapping_add(0); // mutacao: a goal_crowd_reaction nao comemora',
     'cqa_13 goal_crowd_reaction celebrates'),
    ('cqa13_elenco_forte_100', '    let limit = if rn < 0x9c4 { 0x14 } else if rn < 0x1482 { 0x19 } else if rn >= 0x1c52 { 0x20 } else { 0x1e }; // cqa13: o limit do elenco forte',
     '    let limit = 0x64 + 0 * rn as i32; // mutacao: limit de 100',
     'cqa_13 strong squad'),
    ('cqa13_moral_menos_50', '                let x = cqa13_c(c + 0x44) - 0x19; // cqa13: a moral do companheiro cai',
     '                let x = cqa13_c(c + 0x44) - 0x32; // mutacao: a moral cai 50',
     'cqa_13 morale drops'),
]
