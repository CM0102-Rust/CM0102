# Mutacoes (gra_17). A rotina roda todo domingo para todos os clubes com tecnico (desde o inicio da carreira, julho de
# 2001); a confianca da diretoria e da torcida e as reputacoes do tecnico ficam no save e decidem demissoes, propostas
# de emprego e noticias:
# - a torcida presa entre um terco (e nao a metade) e o dobro da confianca da diretoria: todo clube, toda semana;
# - a faixa de reputacao do tecnico + 17 (e nao + 16): muda a velocidade das tres reputacoes, toda semana;
# - nas primeiras 4 rodadas, + 8 (e nao + 5) acima da posicao final anterior (a partir de 18/08/2001);
# - a serie de vitorias pesa 20 (e nao 25) por jogo ao quadrado (a partir da 2a vitoria seguida na liga).
MUTACOES = [
    ('gra17_torcida_um_terco',
     '    let half = confidence / 2;\n',
     '    let half = confidence / 3;\n',
     'gra_17 decay'),
    ('gra17_faixa_de_reputacao',
     'let b = ((v as i32 / 1000) as i8).wrapping_add(16);',
     'let b = ((v as i32 / 1000) as i8).wrapping_add(17);',
     'gra_17 manager reputation'),
    ('gra17_primeiras_rodadas',
     'if current < last { 5 } else if current == last { 1 } else { -5 }',
     'if current < last { 8 } else if current == last { 1 } else { -5 }',
     'gra_17 early weeks'),
    ('gra17_serie_de_vitorias',
     '(read_i16(e + comp_stats_club::WINNING_RUN), 25)',
     '(read_i16(e + comp_stats_club::WINNING_RUN), 20)',
     'gra_17 winning run'),
]
