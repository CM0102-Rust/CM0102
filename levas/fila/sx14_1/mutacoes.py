# Mutacoes (sx14_1). O texto nao volta para a simulacao (update_board_confidence_after_match so le o retorno): as de
# texto sao acusadas pelo comparador (lado a lado no buffer 0xde1f64), como as de texto aceitas em 12/11/2026. A do
# retorno e acusada pelo comparador (retorno) e muda a carreira quando o clube do tecnico humano cai no veredito padrao
# (a noticia da diretoria passa a ser postada).
# - amistoso trocado: os casos que perguntam pelo amistoso escrevem o outro modelo (toda partida de liga ou copa com
#   marcas desses casos; os amistosos de julho);
# - a frase "estamos muito impressionados" trocada por "Tomara que..." (sequencia de vitorias perto da meta);
# - o veredito padrao devolve 1.
MUTACOES = [
    ('sx14_1_amistoso_trocado',
     'Some(t) if exe::friendly_is_friendly_competition(read_usize(G_FRIENDLY), comp) != 0 => {',
     'Some(t) if exe::friendly_is_friendly_competition(read_usize(G_FRIENDLY), comp) == 0 => { // mutation',
     'sx14_1 verdict'),
    ('sx14_1_frase_impressionados',
     '            return 0xa0c4c4; // "We are very impressed',
     '            return 0xa0c514; // mutation "We are very impressed',
     'sx14_1 run sentence'),
    ('sx14_1_padrao_devolve_1',
     '            format_board_verdict(NEWS_TEXT_BUFFER, template, BoardVerdictArgs::OpponentInCompWithScore, other, comp, b, score);\n            return 0;',
     '            format_board_verdict(NEWS_TEXT_BUFFER, template, BoardVerdictArgs::OpponentInCompWithScore, other, comp, b, score);\n            return 1; // mutation',
     'sx14_1 default verdict'),
]
