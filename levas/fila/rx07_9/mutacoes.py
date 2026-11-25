# Mutações (rx07_9). Os textos só rodam quando a caixa de um técnico humano mostra a notícia: o comparador de texto
# (lado a lado) acusa numa sessão do Lucas com a comparação; a carreira automática não os roda.
# - a volta do empréstimo usa sempre o texto do empréstimo encerrado;
# - o pedido retirado ignora o jogador deixado de fora (bits 0x14).
MUTACOES = [
    ('rx07_9_sempre_encerrado',
     '    let recalled = by == main_club;',
     '    let recalled = false; // mutation',
     'rx07_9 loan recalled text'),
    ('rx07_9_ignora_fora',
     '            let template = if reason & 0x14 != 0 && left_out != 0 { 0x9ad6b4 } else { 0x9ad608 };',
     '            let template = 0x9ad608; // mutation',
     'rx07_9 request withdrawn other reason'),
]
