# Mutações (cqj_1), nas vagas da Libertadores (0x4c6430 roda 2 vezes até 2002-06-01, pela virada do ano da Conmebol
# 0x4c61b0; com o Brasil na carreira de 8 ligas, a Libertadores de 2002 joga-se antes de junho):
# - a vaga da cota não é dada: cada nação fica com menos clubes (contador "cqj_1 cota da nacao");
# - o Brasil com o São Paulo em vez do Palmeiras (até 2002, as vagas fixas; contador "cqj_1 brasil fixo");
# - a busca do n-ésimo clube sem pular os que já têm vaga: a vaga cai num clube já classificado e se perde (contador
#   "cqj_1 ja com vaga").
MUTACOES = [
    ('cqj1_sem_vaga_da_cota',
     'enter_libertadores(c, 2); // cqj1: a vaga da cota',
     '// mutacao: sem a vaga da cota',
     'cqj_1 cota da nacao'),
    ('cqj1_sao_paulo_no_lugar_do_palmeiras',
     'if enter_libertadores(cqj1_clube(cqj1_d(0x9d0274)), 2) {',
     'if enter_libertadores(cqj1_clube(cqj1_d(0x9d0270)), 2) { // mutacao: o São Paulo',
     'cqj_1 brasil fixo'),
    ('cqj1_sem_pular_ja_com_vaga',
     'p += 12; // cqj1: pula o clube que já tem vaga',
     'if falta == 0 { achou = true; break; } p += 12; falta = falta.wrapping_sub(1); // mutacao: conta o que já tem vaga',
     'cqj_1 ja com vaga'),
]
