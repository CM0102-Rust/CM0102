# Mutacoes (cqq_10), na Copa da UEFA e na Intertoto (clubes de ENG, ESP, ITA e FRA jogaveis; a carreira de 8 ligas vai
# de janeiro de 2001 a junho de 2002: a Intertoto de 2001 e a Copa da UEFA de 2001/02 ficam dentro da janela): o
# primeiro jogo da 1a rodada de qualificacao da UEFA em 17/8 em vez de 10/8; a rodada de novembro da UEFA em 30/11 em
# vez de 23/11 (uma semana: as datas sao levadas ao dia da semana); a classe dos inscritos da Intertoto zerada (o qsort das sementes muda o sorteio). Os contadores ficam
# colados no efeito.
MUTACOES = [
    ('cqq10_uefa_qualificacao_dia_17',
     'cql10r((0x1c, 6, 0, 4), (0xa, 7, 0, 3, 2, 0), 0xdc, (4, 0, 3), 7, (0x52, 0x29, 0, 0), (0, 2, 0xe), (0, 0xffff_ffff, 0)), // cqq10: a 1ª rodada de qualificação da UEFA',
     'cql10r((0x1c, 6, 0, 4), (0x11, 7, 0, 3, 2, 0), 0xdc, (4, 0, 3), 7, (0x52, 0x29, 0, 0), (0, 2, 0xe), (0, 0xffff_ffff, 0)), // mutacao: 17/8',
     'cqq_10 uefa fixtures'),
    ('cqq10_uefa_novembro_dia_30',
     'cql10r((3, 0xa, 0, 4), (0x17, 0xa, 0, 3, 2, 0), 0x1e, (0, 0, 3), 8, (0x20, 0x10, 8, 0), (0, 2, 0xe), (0, 0xffff_ffff, 0)),',
     'cql10r((3, 0xa, 0, 4), (0x1e, 0xa, 0, 3, 2, 0), 0x1e, (0, 0, 3), 8, (0x20, 0x10, 8, 0), (0, 2, 0xe), (0, 0xffff_ffff, 0)), // mutacao: 30/11',
     'cqq_10 uefa fixtures'),
    ('cqq10_intertoto_classe_zero',
     '*((e + 4) as *mut u8) = *((c + 0x1df) as *const u8); // cqq10: a classe pelo coeficiente',
     '*((e + 4) as *mut u8) = 0; // mutacao: classe zero',
     'cqq_10 intertoto entrants'),
]
