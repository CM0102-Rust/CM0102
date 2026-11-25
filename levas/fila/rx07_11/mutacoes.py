# Mutações (rx07_11). O texto só roda quando a caixa de um técnico humano mostra a notícia: o comparador de texto
# (lado a lado) acusa numa sessão do Lucas com a comparação; a carreira automática não o roda.
# - o pedido do agente usa os textos do pedido do próprio jogador (com 100 sobrando).
MUTACOES = [
    ('rx07_11_agente_como_jogador',
     '            fmt(NEWS_TEXT_BUFFER, NEW_CONTRACT_TEXTS[1][k], 100, np);',
     '            fmt(NEWS_TEXT_BUFFER, NEW_CONTRACT_TEXTS[0][k], np); // mutation',
     'rx07_11 new contract text'),
]
