# Mutacoes (sma_1). A nota escolhe quem o mercado de tecnicos contrata e quem a diretoria quer no lugar do tecnico
# (consider_sacking_manager, add_job_vacancy, best_manager_candidate_score), todos com o cargo 5:
# - o peso da tatica do tecnico 0,1 em vez de 0,002..0,008 (outro favorito para cada vaga);
# - sem o fator do idioma (os estrangeiros sobem);
# - sem o bonus de potencial dos jovens.
MUTACOES = [
    ('sma1_peso_da_tatica',
     'v = staff_job_weight(v, read_i8(np + n::TACTICS), k); // sma1: the manager\'s tactics weight',
     'v = staff_job_weight(v, read_i8(np + n::TACTICS), rdata::F64_0_1); // mutation: a heavy tactics weight',
     'sma_1 manager job'),
    ('sma1_sem_idioma',
     'v = ftol((fl as i32 as f64 * read_f64(rdata::F64_0_05) + read_f64(rdata::F64_0_5)) * v as f64); // sma1: the language factor',
     'v = v + 0 * (fl as i32); // mutation: without the language factor',
     'sma_1 manager job'),
    ('sma1_sem_potencial',
     'v += ftol(game_pow(gap as f64, read_f64(rdata::F64_2)) * patience as f64 * read_f64(rdata::F64_0_1)); // sma1: the potential bonus',
     'v += 0 * gap * patience; // mutation: without the potential bonus',
     'sma_1 young manager potential'),
]
