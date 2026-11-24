# Mutacoes (sx16_13, telas; provadas pelo comparador numa sessao do Lucas, pedindo o relatorio do fisioterapeuta).
# - o pico do meio-campo 30 (e nao 29): compare_physio_get_age_stage acusa para um meio-campista de 32 anos ou mais;
# - a base do fisioterapeuta fraco 50 (e nao 55): compare_physio_get_days_to_judge acusa.
MUTACOES = [
    ('sx16_13_pico_meio_campo',
     '        0x1d\n    } else {\n        0x1c\n    };',
     '        0x1e // mutation\n    } else {\n        0x1c\n    };'),
    ('sx16_13_base_fraco',
     '    let base = if skill > 0x8c { 0x19 } else if skill > 0x50 { 0x28 } else { 0x37 };',
     '    let base = if skill > 0x8c { 0x19 } else if skill > 0x50 { 0x28 } else { 0x32 }; // mutation'),
]
