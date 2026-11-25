# Mutacoes (rx09_10), com a Espanha e a Suecia ate 2002-09-01 e a base NORMAL:
# - o vencedor do acesso com a linha marcada 0xfe em vez de 0: nao sobe na virada (novembro de 2001 / comeco de 2002);
# - os textos do acesso trocados (quem sobe recebe "miss out on promotion"): o comparador de texto acusa em novembro de
#   2001.
# A lista dos clubes fica sem mutacao: o comparador a confere vaga a vaga quando roda.
MUTACOES = [
    ('rx09_10_vencedor_marcado_fe',
     "plan.row = Some((row, 0)); // rx09_10: the promotion play-off winner's row",
     'plan.row = Some((row, 0xfe)); // mutation: marked out',
     'rx09_10 swedish second row status'),
    ('rx09_10_textos_trocados',
     'if outcome as u8 == 1 { 0x989c18 } else { 0x987528 }, text); // rx09_10: win / miss out on promotion',
     'if outcome as u8 == 1 { 0x987528 } else { 0x989c18 }, text); // mutation: swapped',
     'rx09_10 swedish second promotion text'),
]
