# Mutacoes (sx12_7). O hall da fama nao volta para a simulacao: acusadas pelos comparadores da rotina.
# - o primeiro byte desembaralhado trocado, com a soma ainda certa (sem a caixa "Error" da carga): o comparador lado a
#   lado acusa no inicio do jogo (o hall_of_fame.bin esta no jogo_testes);
# - duas entradas de mesma pontuacao com a mais nova primeiro: so roda quando um tecnico humano entra no hall da fama
#   (pode escapar na carreira de ferias ate 2002-06-01).
MUTACOES = [
    ('sx12_primeiro_byte_trocado',
     'write_u8(buffer + i as usize, v); // sx12_7: the unscrambled byte',
     'write_u8(buffer + i as usize, if i == 0 { v ^ 1 } else { v }); // mutation: the first byte changed (the sum still matches)',
     'sx12_7 hall of fame unscrambled'),
    ('sx12_mais_antiga_primeiro',
     '        -1 // sx12_7: the older entry first',
     '        1 // mutation: the newer entry first',
     'sx12_7 hall of fame equal scores'),
]
