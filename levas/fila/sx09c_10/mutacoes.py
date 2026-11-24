# Mutacoes (sx09c_10). TELAS: a cadeia nao roda mutacoes; para provar o comparador numa sessao do Lucas (ou com o modo
# CM_TEXTO_NOTICIAS), acusadas pelo comparador de texto quando a noticia e aberta:
# - a reacao aceita usa o modelo da advertencia quando e multa (e vice-versa);
# - o retorno do tecnico com resultado 1 sai com o modelo da pressao.
MUTACOES = [
    ('sx09c_reacao_trocada',
     '2 => fmt(b, fine_accepted_template(!warning, reason, effects, personality, game), np), // sx09c_10: accepted',
     '2 => fmt(b, fine_accepted_template(warning, reason, effects, personality, game), np), // mutation',
     'sx09c_10 fine reaction body'),
    ('sx09c_retorno_pressao',
     'let tmpl = if s[8] == 1 {',
     'let tmpl = if s[8] == 0x7fff_ffff { // mutation',
     'sx09c_10 coach feedback body'),
]
