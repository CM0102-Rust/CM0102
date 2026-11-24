# Mutacoes (sx09b_3). O texto nao volta para a simulacao: acusadas pelo comparador (lado a lado) quando o texto da
# noticia de suspensao de um jogador do tecnico humano e escrito.
# - a forma da regra da vaga 0 (as federacoes nacionais, o caso comum) trocada: 8 vira 0 e 0 vira 8;
# - "domestic" no lugar do texto da propria regra (a suspensao so nacional, o caso comum).
MUTACOES = [
    ('sx09b_forma_vaga_0',
     "0 => if read_u8(b + 0x20) != 0xff { 8 } else { 0 }, // sx09b_3: +0x20 of the ruling body (unnamed byte)",
     "0 => if read_u8(b + 0x20) != 0xff { 0 } else { 8 }, // mutation",
     "sx09b_3 the rule's own text"),
    ('sx09b_texto_proprio_domestico',
     "        covered!(\"sx09b_3 the rule's own text\");\n        own_text(speaker);",
     "        covered!(\"sx09b_3 the rule's own text\");\n        story_lang!(out, 0x9c3454); // mutation",
     "sx09b_3 the rule's own text"),
]
