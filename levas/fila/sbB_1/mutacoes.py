# Mutacoes (sbB_1). O texto nao volta para a simulacao: acusadas pela comparacao da rotina (side by side sobre o
# buffer), com o modo CM_TEXTO_NOTICIAS que a cadeia liga em todo jogo comparando (a noticia 0x1772 vai a todos os
# clubes por alcance, cerca de 40 mil vezes ate 2002-06-01). Cada uma num trecho comum:
# - a faixa da liga 3..6 (a do meio, -500..500, a mais comum) lida como a 2;
# - a faixa da diretoria deslocada de uma (todo texto a partir de 30 dias);
# - os dois paragrafos do fecho trocados de lugar (todo fecho com o paragrafo da torcida).
MUTACOES = [
    ('sbB_faixa_liga',
     '        say(fp, CONFIDENCE_LATER[wealth][league.min(3) as usize][board][k]);',
     '        say(fp, CONFIDENCE_LATER[wealth][league.min(2) as usize][board][k]); // mutation',
     'sbB_1 later'),
    ('sbB_faixa_diretoria',
     '    let board = get_board_confidence_band(this, club.addr()) as usize;',
     '    let board = (get_board_confidence_band(this, club.addr()) as usize).saturating_sub(1); // mutation',
     'sbB_1 early days'),
    ('sbB_fecho_trocado',
     '        story_crt!(buf, 0x991948, first, sp); // "%s\\n\\n%s"',
     '        story_crt!(buf, 0x991948, sp, first); // mutation',
     'sbB_1 fans paragraph'),
]
