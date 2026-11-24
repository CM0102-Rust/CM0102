# Mutacoes (sbE_3), com a Alemanha ate 2002-08-01:
# - a TV da Bundesliga em dobro (as financas de todos os clubes da Bundesliga desde a criacao: a carreira);
# - a linha do sul subindo 1 grau por passada (outra divisao: acusada pelo comparador lado a lado na virada de 2002);
# - o par que volta so pela metade (acusada pelo comparador lado a lado).
MUTACOES = [
    ('sbE_3_tv_em_dobro',
     'eng_prm_pay_tv_rights(this, 0x4c4b40, 0); // sbE_3: the TV money of each club',
     'eng_prm_pay_tv_rights(this, 0x989680, 0); // mutation: twice the TV money',
     'sbE_3 german regional divisions gathered'),
    ('sbE_3_linha_do_sul',
     'south_line += read_f64(0x96b198); // sbE_3: 0.278 a pass',
     'south_line += 1.0; // mutation: a degree a pass',
     'sbE_3 german regional split'),
    ('sbE_3_par_pela_metade',
     'set(k, old[k]); // sbE_3: the pair takes its old sections back',
     'set(k, new_k); // mutation: only one of the pair goes back',
     'sbE_3 german regional split'),
]
