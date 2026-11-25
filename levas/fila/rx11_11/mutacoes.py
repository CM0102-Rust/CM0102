# Mutacoes (rx11_11), para provar o comparador numa sessao do Lucas (item de TELAS: a cadeia nao as roda):
# - "Second Round" no lugar de "Qualified For Final" (acusada pelo comparador lado a lado do texto).
# Os destrutores ficam sem mutacao: so a original roda no comparador e o efeito (liberar o objeto) nao aparece na tela.
MUTACOES = [
    ('rx11_11_qualificado_para_a_final',
     '6 => qualifying_progress_sprintf(buf, 0xad4634, Some(comp)), // rx11_11: "Qualified For Final"',
     '6 => qualifying_progress_sprintf(buf, 0x9a7b94, None), // mutation: "Second Round"',
     'rx11_11 african progress text'),
]
