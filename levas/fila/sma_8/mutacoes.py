# Mutacoes (sma_8). O texto nao volta para a simulacao: acusada pela comparacao (compare_text_writer), aceito pela
# sessao principal em 12/11/2026.
# - o jogador que se aposenta jovem com o texto do veterano.
MUTACOES = [
    ('sma8_aposentadoria_jovem',
     'if (person.age() as i8) < 0x1b { 0xa6ec2c } else { 0xa6eb44 } // sma8: the young retiring player',
     'if (person.age() as i8) < 0x1b { 0xa6eb44 } else { 0xa6eb44 } // mutation: always the veteran text',
     'sma_8 retirement body'),
]
