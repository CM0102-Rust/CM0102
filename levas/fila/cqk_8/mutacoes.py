# Mutacoes (cqk_8): o progresso do clube nas estatisticas da competicao (+0x73..+0x75 da entrada do Comp_Stats: a
# posicao atual, a minima e a maxima). Rodam a cada atualizacao da tabela (17 mil, 2,4 mil e 6,3 mil chamadas na
# carreira de 8 ligas ate 2002-06-01).
# - Mineiro (e Paranaense, Carioca): nos turnos e no mata-mata (os codigos como vem), a posicao atual sai uma abaixo.
#   ESCAPOU na carreira de 8 ligas ate 2002-06-01 (04/11): o progresso desses tres estaduais nao e lido antes da virada;
#   fica como lacuna medida (a rotina e provada pela comparacao, 0 divergencias).
# - MLS: na temporada regular, o primeiro codigo 2 vira 4 em vez de 3.
# - Serie C: nos grupos, o segundo codigo 3 vira 0x12 em vez de 0x11.
# Escapou na carreira de 8 ligas (04/11) e saiu: nos pontos corridos do Mineiro, o terceiro codigo 3 em vez de 2 e a
# posicao atual uma abaixo quando o terceiro e menor que 3 (a tabela geral desses estaduais nao da titulo).
MUTACOES = [
    ('cqk8_atual_mineiro', 'cqk8_gravar(e, a, b, c); // cqk8: os códigos do Mineiro como vêm',
     'cqk8_gravar(e, a.wrapping_add(1), b, c); // mutacao: os códigos do Mineiro como vêm', 'cqk_8 mineiro como vem'),
    ('cqk8_codigo_2_mls', '3 // cqk8: o código 2 da MLS',
     '4 // mutacao: o código 2 da MLS', 'cqk_8 mls segundo'),
    ('cqk8_segundo_serie_c', 'cqk8_gravar(e, a, b2, c); // cqk8: os códigos dos grupos da Série C',
     'cqk8_gravar(e, a, if b == 3 { 0x12 } else { b2 }, c); // mutacao: os códigos dos grupos da Série C', 'cqk_8 serie c segundo 3'),
]
