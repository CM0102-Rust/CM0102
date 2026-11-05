# Mutacoes (cqm_9), na tabela da J1, da J2 e da NSL (rodam na criacao da carreira de 8 ligas e na virada de cada liga):
# em cada uma, as linhas giradas uma casa (o k-esimo clube na linha k+1, o ultimo na 0): a grade de jogos sai da ordem
# das linhas e os confrontos de cada rodada mudam. O contador fica depois de a tabela encher.
# Prova do agente (05/11, carreira Rust de 8 ligas ate 2002-06-01): as tres ACUSADAS, mas pela queda: EXCECAO 0xc0000005 no
# ddraw.dll (+0x1302f8) no primeiro dia, logo depois da criacao (em segundos, sem travar): alguma rotina ja em Rust
# le a tabela da J1/J2/NSL esperando a ordem do banco nas linhas [inferencia; nao achei qual]. Os estaduais girados
# nao caem.
MUTACOES = [
    ('cqm9_j1_girada',
     'cqm9_japao(this, 0x9cf65c, 0, "cqm_9 j1 cheia")',
     'cqm9_japao(this, 0x9cf65c, 1, "cqm_9 j1 cheia")',
     'cqm_9 j1 cheia'),
    ('cqm9_j2_girada',
     'cqm9_japao(this, 0x9cf768, 0, "cqm_9 j2 cheia")',
     'cqm9_japao(this, 0x9cf768, 1, "cqm_9 j2 cheia")',
     'cqm_9 j2 cheia'),
    ('cqm9_nsl_girada',
     'cqm9_linha(this, k as u8, cqm9_w(this + 0x3e), 0, clube); // cqm9: a linha da NSL',
     'cqm9_linha(this, k as u8, cqm9_w(this + 0x3e), 1, clube); // mutacao: a linha da NSL',
     'cqm_9 nsl cheia'),
]
