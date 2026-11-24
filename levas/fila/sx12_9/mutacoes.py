# Mutacoes (sx12_9), item de telas: a cadeia nao as roda; provam o comparador numa sessao do Lucas jogando comparando
# (historia de um clube ingles e a escolha das ligas na criacao do jogo).
MUTACOES = [
    ('sx12_temporada_dois_anos',
     'story_crt!(text, 0x9eb024, y as u32, (y as u32 + 1) as i32 % 10); // sx12_9: "%d/%d"',
     'story_crt!(text, 0x9eb024, y as u32, (y as u32 + 2) as i32 % 10); // mutation: the year after the next',
     'sx12_9 two-year season'),
    ('sx12_divisao_extra',
     '0x9eb03c // sx12_9: "Second Division B"',
     '0x9eb02c // mutation: "Second Division" for Portugal and Spain',
     'sx12_9 extra division name'),
]
