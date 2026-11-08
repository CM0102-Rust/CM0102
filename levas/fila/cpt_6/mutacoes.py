# Mutacoes (cpt_6), todas nos coeficientes das nacoes (0x5d5f20, todo dia pelo laco do dia do jogo; o dia de dezembro
# [0x9cd216] cai em dezembro de 2001, dentro da carreira ate 2002-06-01). O bonus com fator 1 em vez de 3 (o ano
# corrente de toda nacao com marca < 12 e injury_word_0c sai diferente e passa a +0xd0). O ano corrente que nao zera (+0xd8
# fica com o bonus). O dia de dezembro invertido (a historia anda nos outros dias do mes da tabela e nao em dezembro). As
# tres mudam a historia das nacoes no banco (o hash diario dos blocos do banco).
MUTACOES = [
    ('cpt6_bonus_fator_1', '* read_f64_at(0x968930) + h[5]; // cpt6: o bônus de dezembro',
     '* read_f64_at(0x967890) + h[5]; // mutacao: fator 1',
     'cpt_6 bonus de dezembro'),
    ('cpt6_ano_corrente_fica', '    h[5] = 0.0; // cpt6: o ano corrente zerado',
     '    // mutacao: o ano corrente fica',
     'cpt_6 dezembro'),
    ('cpt6_dezembro_invertido', '    let dez = cpt6_w(0x9cd216) == cpt6_w(0xae2c90); // cpt6: o dia de dezembro',
     '    let dez = cpt6_w(0x9cd216) != cpt6_w(0xae2c90); // mutacao',
     'cpt_6 dezembro'),
]
