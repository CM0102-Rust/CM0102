# Mutacoes (cx02_16) pela prova do save com um save que tenha uma dessas copas (CARGA). Nenhuma abre caixa.
# - a vaga vazia gravada como 0 e nao -1: o .tmp da copa muda (o .qst) e, carregado, a vaga vira o clube 0.
MUTACOES = [
    ('cx02_16_vaga_vazia_zero',
     '        write_u32(slot, if club != 0 { read_u32(club) } else { u32::MAX });',
     '        write_u32(slot, if club != 0 { read_u32(club) } else { 0 }); // mutation'),
]
