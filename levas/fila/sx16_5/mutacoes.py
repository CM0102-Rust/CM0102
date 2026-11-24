# Mutacoes (sx16_5, telas; provadas pelo comparador numa sessao do Lucas).
# - a contagem limitada a 50 (e nao 100): compare_get_comp_official_count acusa ao abrir Official Stats da Premier
#   (mais de 50 arbitros na Inglaterra);
# - sem nota registrada vira 6 (e nao 7): compare_get_last_match_rating acusa no pedido sobre o arbitro.
MUTACOES = [
    ('sx16_5_limite_50',
     '        Some((_, count)) => count.min(0x64) as i16,',
     '        Some((_, count)) => count.min(0x32) as i16, // mutation'),
    ('sx16_5_sem_nota_6',
     '        0xff => 7,',
     '        0xff => 6, // mutation'),
]
