# Mutacoes (cpv_7): o peso da liga no teto salarial do pessoal com 0,25 em vez de 0,2 (todo clube de liga com nacao:
# muda o salario que o clube oferece e o interesse do candidato 0x844780, logo quem cada clube contrata), sem o aumento
# de 5% dos clubes grandes da Inglaterra, Italia e Espanha (os mesmos efeitos, so nesses clubes) e o bonus somado ao
# saldo em vez de tirado (todo jogo: o saldo de todos os clubes sobe em vez de cair; a band das financas 0x5986b0 muda
# e com ela os sorteios da divida 0x59dd40 e do clima 0x923630). A segunda pode escapar se nenhum desses clubes buscar
# pessoal ate 2002-06-01 (o contador mostra); a terceira, se nenhuma band mudar a tempo.
MUTACOES = [
    ('cpv7_peso_da_liga', '            w = match_pl_ftol(cpv7_w(liga + 0x69) as f64 * w as f64 * read_f64_at(0x968918)); // cpv7: o peso da liga',
     '            w = match_pl_ftol(cpv7_w(liga + 0x69) as f64 * w as f64 * read_f64_at(0x9689e0)); // mutacao: 0,25',
     'cpv_7 peso da liga'),
    ('cpv7_sem_clube_grande', '                    w = match_pl_ftol(w as f64 * read_f64_at(0x9689b8)); // cpv7: o clube grande da Inglaterra, Itália e Espanha',
     '                    w = match_pl_ftol(w as f64); // mutacao: sem o aumento',
     'cpv_7 clube grande'),
    ('cpv7_bonus_no_saldo', '    core::ptr::write_unaligned(this as *mut i64, s.wrapping_sub(valor as i64)); // cpv7: o bônus sai do saldo',
     '    core::ptr::write_unaligned(this as *mut i64, s.wrapping_add(valor as i64)); // mutacao: o bonus entra no saldo',
     'cpv_7 bonus pagos'),
]
