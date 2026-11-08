# Mutacoes. A 0x76b600 roda a cada jogo do dia (5.160 chamadas na cobertura de 17/10); o ramo dos jogos entre
# selecoes que nao sao das tres competicoes especiais roda na carreira automatica ate 2002-06-01 (a primeira
# convocacao 0x86cc50, chamada so por ele, conta 128..256 na base da 257a: amistosos e eliminatorias da Copa de 2002).
# As tres mexem nos jogos (+0x22) e gols (+0x23) pela selecao das pessoas, que estao na tabela de pessoas (entra no
# hash diario do banco): um jogo valendo dois, os gols nao somados, as reservas que nao entraram contando jogo. Os
# ramos ficam colados no efeito.
MUTACOES = [
    ('cpw12_dois_jogos',
     '*((pessoa + 0x22) as *mut u8) = (*((pessoa + 0x22) as *const u8)).wrapping_add(1); // cpw12: um jogo',
     '*((pessoa + 0x22) as *mut u8) = (*((pessoa + 0x22) as *const u8)).wrapping_add(2); // mutacao: dois jogos',
     'cpw_12 national team match'),
    ('cpw12_sem_gols',
     '*((pessoa + 0x23) as *mut u8) = (*((pessoa + 0x23) as *const u8)).wrapping_add(*((vaga + 0x21) as *const u8)); // cpw12: os gols',
     'let _ = vaga; // mutacao: sem os gols',
     'cpw_12 national team match'),
    ('cpw12_reservas_contam',
     'if k >= 11 && *((vaga + 0x32) as *const u8) == 0xff { // cpw12: a reserva que não entrou',
     'if k >= 11 && *((vaga + 0x32) as *const u8) == 0xff && false { // mutacao: a reserva que nao entrou conta',
     'cpw_12 reserve not coming on'),
]
