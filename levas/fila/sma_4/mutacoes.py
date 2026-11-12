# Mutacoes (sma_4). A rotina roda uma vez, na criacao do jogo (NationalTeams::NationalTeams 0x76a4e0), e as reservas e
# os primeiros elencos das selecoes saem dela; as convocacoes da temporada tiram jogadores dos clubes:
# - a reserva principal de cada nacao no maximo 60 em vez de 100 (outros convocaveis);
# - a reserva sub-21 no maximo 30 em vez de 50;
# - o elenco B com no maximo 16 em vez de 24.
MUTACOES = [
    ('sma4_reserva_principal_60',
     'let size = count.clamp(10, 100) as u8; // sma4: the senior pool size',
     'let size = count.clamp(10, 60) as u8; // mutation: a senior pool of at most 60',
     'sma_4 first squads'),
    ('sma4_reserva_sub21_30',
     'let size21 = count.clamp(10, 0x32) as u8; // sma4: the under-21 pool size',
     'let size21 = count.clamp(10, 0x1e) as u8; // mutation: an under-21 pool of at most 30',
     'sma_4 first squads'),
    ('sma4_elenco_b_16',
     "write_u8(rec + 0x56, 0x18); // sma4: the B squad's largest size",
     'write_u8(rec + 0x56, 0x10); // mutation: a B squad of at most 16',
     'sma_4 first squads'),
]
