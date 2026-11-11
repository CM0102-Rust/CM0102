# Mutacoes (mkd_4). A rotina roda em cada data de decisao do tipo 3 (as ofertas de contrato da IA, renovacoes
# sobretudo, marcadas pelo passe diario dos contratos 0x4c7ac0); todas mudam quem assina:
# - a razao sem o k da personalidade (outras ofertas passam da nota 2,75);
# - sem o piso da renovacao (vontade nao sobe a 3,15 com razao >= 0,99: menos renovacoes);
# - o corte da razao em 0,9 em vez de 0,6 (ofertas medianas viram 0).
MUTACOES = [
    ('mkd4_razao_sem_k',
     'score = v as f64 / base - k; // mkd4: the ratio less k',
     'score = v as f64 / base; // mutation: without k',
     'mkd_4 best offer'),
    ('mkd4_sem_piso_da_renovacao',
     'will = 3.15; // mkd4: the renewal floor',
     'will = will + 0.0; // mutation: no renewal floor',
     'mkd_4 renewal floor'),
    ('mkd4_corte_em_0_9',
     'if score < read_f64(rdata::F64_0_6) {',
     'if score < read_f64(rdata::F64_0_9) { // mutation: the cut at 0.9',
     'mkd_4 best offer'),
]
