# Mutações (cqi_2), nas vagas europeias distribuídas no começo da carreira (uma vez, pela 0x9035a0; com as 8 ligas até
# 2002-06-01 decidem quem joga a Liga dos Campeões e a Intertoto de 2001/02):
# - a tabela dos clubes sem o nível da divisão: a ordem dos clubes dentro da nação muda e outros clubes ganham as vagas
#   (contador "cqi_2 division level");
# - as 3 primeiras nações com 3 clubes nos grupos e 1 na terceira preliminar em vez de 2 e 2 (contador "cqi_2 duas nos
#   grupos");
# - a Intertoto contando 2 por clube: para em 30 clubes em vez de 60 (contador "cqi_2 club in intertoto").
MUTACOES = [
    ('cqi2_sem_nivel_da_divisao',
     'core::ptr::write_unaligned((r + 9) as *mut u16, core::ptr::read_unaligned((linha + 0x69) as *const u16)); // cqi2: o nível da divisão',
     'core::ptr::write_unaligned((r + 9) as *mut u16, 0); // mutacao: sem o nível da divisão',
     'cqi_2 division level'),
    ('cqi2_tres_nos_grupos',
     '([0, 2, 2, 0], 0) // cqi2: 1ª a 3ª nação',
     '([0, 3, 1, 0], 0) // mutacao: 3 nos grupos',
     'cqi_2 two in the groups'),
    ('cqi2_intertoto_conta_dois',
     'colocados += 1; // cqi2: mais um na Intertoto',
     'colocados += 2; // mutacao: conta 2',
     'cqi_2 club in intertoto'),
]
