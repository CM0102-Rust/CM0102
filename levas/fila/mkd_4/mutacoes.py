# Mutacoes (mkd_4). A rotina roda em cada data de decisao do tipo 3 (as ofertas de contrato da IA, renovacoes
# sobretudo, marcadas pelo passe diario dos contratos 0x4c7ac0); todas mudam quem assina:
# - a razao sem o k da personalidade (outras ofertas passam da nota 2,75);
# - sem a elevacao da vontade a 3,15 (razao >= 0,9, ou > 0,7 com vontade > 3,5): ofertas boas de quem quer pouco
#   caem abaixo da nota 2,75 (troca o sem_piso_da_renovacao de 11/11, que e redundante: ver PRONTO.txt);
# - o corte da razao em 0,9 em vez de 0,6 (ofertas medianas viram 0).
MUTACOES = [
    ('mkd4_razao_sem_k',
     'score = v as f64 / base - k; // mkd4: the ratio less k',
     'score = v as f64 / base; // mutation: without k',
     'mkd_4 best offer'),
    ('mkd4_sem_elevar_a_vontade',
     'if lift && will <= read_f64(rdata::F64_3_15) {',
     'if false && lift && will <= read_f64(rdata::F64_3_15) { // mutation: the will is not lifted to 3.15',
     'mkd_4 best offer'),
    ('mkd4_corte_em_0_9',
     'if score < read_f64(rdata::F64_0_6) {',
     'if score < read_f64(rdata::F64_0_9) { // mutation: the cut at 0.9',
     'mkd_4 best offer'),
]
