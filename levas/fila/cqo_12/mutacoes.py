# Mutacoes (cqo_12), na Copa do Brasil (o Brasil jogavel na carreira de 8 ligas; a copa de 2002 e play de fevereiro a
# julho, dentro da janela): a 1a rodada play em 2/3 em vez de 23/2; as vagas tiradas dos inscritos de tras para a
# frente (os sorteios mudam); uma substituicao so em vez de 3 (as partidas da copa mudam). Os contadores ficam colados
# no efeito.
MUTACOES = [
    ('cqo12_primeira_rodada_em_marco',
     'cql10r((0x12, 0, 0, 6), (0x17, 1, 0, 2, 2, 0), 0xa, (0, 0, 1), 4, (0x36, 0x1b, 0x36, 0), (0, 2, 0xe), (0, 0xffff_ffff, 0)), // cqo12: a 1ª rodada da Copa do Brasil',
     'cql10r((0x12, 0, 0, 6), (2, 2, 0, 2, 2, 0), 0xa, (0, 0, 1), 4, (0x36, 0x1b, 0x36, 0), (0, 2, 0xe), (0, 0xffff_ffff, 0)), // mutacao: 2/3',
     'cqo_12 tabela da copa do brasil'),
    ('cqo12_vagas_de_tras_para_frente',
     'let fonte = o; // cqo12: a inscrição de cada vaga',
     'let fonte = (cqo12_w(this + 0x3e) as usize - 1 - i as usize) * 6; // mutacao: de tras para a frente',
     'cqo_12 copa do brasil teams'),
    ('cqo12_uma_substituicao',
     '*((this + 0x4a) as *mut u8) = 3; // cqo12: as substituições da Copa do Brasil',
     '*((this + 0x4a) as *mut u8) = 1; // mutacao: uma so',
     'cqo_12 copa do brasil rules'),
]
