# Mutações (rx07_10). O texto só roda quando a caixa de um técnico humano mostra a notícia: o comparador de texto
# (lado a lado) acusa numa sessão do Lucas com a comparação; a carreira automática não o roda.
# - a manchete usa o texto do corpo (0x9ae604), não "<nome> seeks assurances".
MUTACOES = [
    ('rx07_10_manchete_do_corpo',
     '        fmt(NEWS_TEXT_BUFFER, 0x9ad8b8, np); // "<name> seeks assurances"',
     '        fmt(NEWS_TEXT_BUFFER, 0x9ae604, np); // "<name> seeks assurances" (mutation)',
     'rx07_10 assurances text'),
]
