# Mutacoes (mwd_25): a TV da Primeira Divisao turca paga 100 mil em vez de 1 milhao (as financas dos 18 clubes mudam desde
# a criacao da carreira: o banco); na virada, o sorteio entre os melhores da divisao mais baixa embaralha os 60 primeiros.
MUTACOES = [
    ('mwd25_tv_turca',
     'eng_prm_pay_tv_rights(this, 0xf4240, 0); // mwd25: the Turkish top flight TV money',
     'eng_prm_pay_tv_rights(this, 0x186a0, 0); // mutation: 100,000',
     'mwd_25 turkish category b groups merged'),
    ('mwd25_sobem_outros',
     'exe::comp_util_sort_and_shuffle_top_clubs(list, n, 10); // mwd25: the best thirty shuffled',
     'exe::comp_util_sort_and_shuffle_top_clubs(list, n, 20); // mutation: the best sixty shuffled',
     'mwd_25 turkish lowest division up'),
]
