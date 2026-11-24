# Mutacoes (sx20_11), item de TELAS: a carreira automatica nao passa; servem para provar o comparador numa sessao do
# Lucas jogando comparando:
# - o salario anual com 48 semanas (com as opcoes em salario por ano, todo texto de salario diverge);
# - a oferta de teste com salario 100 (oferecer periodo de testes a um jogador livre);
# - a clausula de multa minima fica ao por outra clausula (tela de proposta de contrato).
MUTACOES = [
    ('sx20_11_ano_48_semanas',
     '(this.wage().wrapping_mul(52), 4) // sx20_11: a year of wages',
     '(this.wage().wrapping_mul(48), 4) // mutation: 48 weeks'),
    ('sx20_11_teste_100',
     'this.set_wage(200); // sx20_11: the trial wage',
     'this.set_wage(100); // mutation: trial wage 100'),
    ('sx20_11_multa_fica',
     'write_u8(c + contract::MINIMUM_FEE_RC, 0); // sx20_11: the minimum fee clause goes',
     '(); // mutation: the minimum fee clause stays'),
]
