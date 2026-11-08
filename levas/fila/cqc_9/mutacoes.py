# Mutacoes (cqc_9), todas na 0x6a3770 (689 chamadas ate 2002-06-01 na cobertura de 17/10). A meta e as marcas das
# expectativas entram na confianca da diretoria pelas partidas e nas demissoes; os sorteios do presidente mudam a contagem
# de sorteios da carreira.
# - A meta parte da posicao de perigo + 1 (todos os clubes com linha na liga).
# - As confiancas abaixo de 6500 voltam com 6x em vez de 5x (todos os clubes nessa band).
# - O primeiro sorteio do presidente passa de 50 a 60 (os clubes com presidente fora da marca 1 de +2).
MUTACOES = [
    ('cqc9_meta_mais_1', '    por(cqc9_u(lg + 0x72) as i32); // cqc9: a meta pela posição de perigo',
     '    por(cqc9_u(lg + 0x72) as i32 + 1); // mutacao: a meta pela posição de perigo', 'cqc_9 target'),
    ('cqc9_confianca_6x', '            let v = (x as f64 * cqc9_f(0x9688a0) + cqc9_f(k)) * cqc9_f(0x967880); // cqc9: a confiança de volta',
     '            let v = (x as f64 * (cqc9_f(0x9688a0) + 1.0) + cqc9_f(k)) * cqc9_f(0x967880); // mutacao: a confiança de volta', 'cqc_9 confidence back'),
    ('cqc9_presidente_60', '        let r = cqc9_sortear(0x32); // cqc9: o sorteio do presidente',
     '        let r = cqc9_sortear(0x3c); // mutacao: o sorteio do presidente', 'cqc_9 chairman draw'),
]
