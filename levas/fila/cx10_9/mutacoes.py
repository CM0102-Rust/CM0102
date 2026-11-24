# Mutacoes (cx10_9): textos, acusados pela comparacao com CM_TEXTOS=1 (o texto nao volta para a simulacao). Nenhuma abre
# caixa. A saida da crise e os planos do estadio sao raros ate 2002-06-01: podem escapar se a noticia nao sair.
MUTACOES = [
    ('cx10_9_credores_nome_curto',
     '        let (name, g) = club_long_name(exe::newsitem_get_slot(news, 0));\n        fmt(buf, 0x9c9d10, g, g, name);',
     '        let (name, g) = awol_club(exe::newsitem_get_slot(news, 0)); // mutation\n        fmt(buf, 0x9c9d10, g, g, name);'),
    ('cx10_9_planos_tipos_trocados',
     '            0 => fmt(buf, 0x9c9db4, g, g, name), // redevelop the stadium to enter <comp>',
     '            0 => fmt(buf, 0x9c9e34, g, g, name), // mutation'),
]
