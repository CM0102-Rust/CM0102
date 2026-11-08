# Mutacoes (cqb_5), na lesao nova (0x62d250), que as copias Rust dos efeitos fisicos da partida, do dia das lesoes,
# da recaida e dos surtos chamam a cada lesao da carreira (contadores "lesao registrada" 16384, "cpv_8 batida vira
# lesao" 1024 e "cpv_11 lesão de 14 a 27 dias" 32 na base da 261a):
# - o fisioterapeuta alonga em vez de encurtar (35 + melhor no lugar de 35 - melhor): as lesoes ficam mais longas, os
#   jogadores faltam mais as partidas e as escalacoes e os resultados mudam;
# - a volta ao treino so no fim da lesao (+0x2a = os dias, sem o write_percent_pair do tipo): a forma (+8) cai mais tempo pelo
#   fisico do dia (0x62c680) e as escalacoes mudam;
# - os clubes que miram o jogador nao suspendem o interesse na lesao longa (0x842b00 nao roda): as listas de
#   observacao e as transferencias mudam (pega se na base alguma lesao passou de sorteio(50) + 30 dias: contador
#   "cqb_5 suspende as miras"; "lesao grave" 2048 e "lesao mais grave" 1024 na base sugerem que sim).
# Os contadores ficam colados no efeito: a duracao, a volta ao treino e a suspensao.
MUTACOES = [
    ('cqb5_fisioterapeuta_alonga',
     'let mut extra = (cqb5_w(0x9e7add + t) as i32).wrapping_mul(0x23 - melhor as i32) / 20; // cqb5: o fisioterapeuta encurta',
     'let mut extra = (cqb5_w(0x9e7add + t) as i32).wrapping_mul(0x23 + melhor as i32) / 20; // mutacao: o fisioterapeuta alonga',
     'cqb_5 duracao'),
    ('cqb5_volta_ao_treino_no_fim',
     'let volta = return_to_training_days(tipo, dias); // cqb5: a volta ao treino pelo write_percent_pair',
     'let volta = dias; // mutacao: a volta ao treino so no fim',
     'cqb_5 volta ao treino'),
    ('cqb5_miras_nao_suspendem',
     'suspender(cqb5_d(cqb5_d(0xde1f54) + 0x856), p, 0); // cqb5: os clubes que o miram suspendem',
     'let _ = suspender; // mutacao: os clubes que o miram nao suspendem',
     'cqb_5 suspende as miras'),
]
