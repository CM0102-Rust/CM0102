# Mutacoes (mkd_2). O salario padrao entra no contrato desejado (0x85fc50: o salario que a pessoa pede), na vontade de
# vir (0x846920: o portao do salario contra o teto do clube) e na folha (0x598370); muda quem aceita, quem o clube
# consegue pagar e por quanto:
# - u em vez de u ao quadrado (os salarios dos jogadores caem a uma fracao: todos passam pelo portao do salario);
# - sem o teto do salario maximo do clube (os pedidos passam do que o clube pode pagar: outras recusas);
# - o declinio pelos dias no clube sem o termo dos dias (1,375 fixo: os recem-chegados pedem mais).
MUTACOES = [
    ('mkd2_sem_quadrado',
     'let product = st * u.wrapping_mul(u) as u64 as f64 * d(rdata::F64_1ENEG_08); // mkd2: the reputation squared',
     'let product = st * u as f64 * 100.0 * d(rdata::F64_1ENEG_08); // mutation: u instead of u squared',
     'mkd_2 player wage'),
    ('mkd2_sem_teto_do_clube',
     'value = mw as f32; // mkd2: the club\'s maximum wage',
     'value = value; // mutation: without the club\'s maximum wage',
     'mkd_2 player wage'),
    ('mkd2_declinio_fixo',
     'let mut k = f(F32_1_375) - x * f(F32_0_0125); // mkd2: the decline by days',
     'let mut k = f(F32_1_375) - 0.0 * x * f(F32_0_0125); // mutation: no decline by days',
     'mkd_2 days at the club'),
]
