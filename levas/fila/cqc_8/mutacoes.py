# Mutacoes (cqc_8), todas na 0x695240 (5.859 chamadas ate 2002-06-01 na cobertura de 17/10: duas por partida com
# tecnico nos dois lados). A confianca da diretoria decide a demissao (0x69aba0 abaixo de 4275 em +6) e as vagas de
# tecnico, que mudam os elencos e os sorteios da carreira.
# - A confianca sobe 20% mais a cada ajuste (+6, +0xa, +8 e +0xc): todas as partidas.
# - A vitoria vale (c / 10 + 30) por gol de saldo em vez de + 20.
# - A nota do time no jogo soma 40 antes do fator (as partidas com o registro do jogo, a 0x6951e0 passa sempre).
MUTACOES = [
    ('cqc8_confianca_mais_20', '        let t = ftol_f64(v * cqc8_f(k)); // cqc8: a confiança pelo ajuste',
     '        let t = ftol_f64(v * cqc8_f(k) * 1.2); // mutacao: a confiança pelo ajuste', 'cqc_8 confianca'),
    ('cqc8_vitoria_30', '        aj = aj.wrapping_add((c16 / 10 + 20).wrapping_mul(a - b)); // cqc8: a vitória',
     '        aj = aj.wrapping_add((c16 / 10 + 30).wrapping_mul(a - b)); // mutacao: a vitória', 'cqc_8 vitoria'),
    ('cqc8_nota_do_jogo_mais_40', '        v = v * cqc8_f(0x96d1c8) - cqc8_f(0x96d1c0); // cqc8: a nota do jogo (× 67 − 470)',
     '        v = v * cqc8_f(0x96d1c8) - cqc8_f(0x96d1c0) + 40.0; // mutacao: a nota do jogo (× 67 − 470)', 'cqc_8 nota do jogo'),
]
