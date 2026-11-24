# Mutacoes (sxB_6). Acusadas pelo comparador (o texto, a contagem devolvida, os ids e G_CONTRACT_LOOKUP_RESULT), com o
# modo CM_TEXTO_NOTICIAS que a cadeia liga em todo jogo comparando (a noticia da confianca da diretoria chama a rotina);
# a contagem tambem muda o texto da noticia (com ela o texto junta as duas frases). Num trecho que roda em toda
# chamada que passa das saidas de cara:
# - a contagem devolvida com um a mais;
# - a resposta do get_contract nao gravada no global (roda para todo jogador das listas do presidente).
MUTACOES = [
    ('sxB_opiniao_contagem',
     '    let named = (o.targets.len() + o.valuable.len() + o.disliked.len()) as u8;',
     '    let named = (o.targets.len() + o.valuable.len() + o.disliked.len() + 1) as u8; // mutation',
     'sxB_6 squad opinion'),
    ('sxB_opiniao_global',
     '    write_usize(G_CONTRACT_LOOKUP_RESULT, contract);',
     '    // mutation: the lookup result not stored',
     'sxB_6 squad opinion'),
]
