# Mutacoes (cqv_6): cada construtor roda uma vez, na criacao da carreira de 8 ligas (contadores "<item> <copa> new"); a
# copa montada um ano adiante leva as rodadas, os inscritos e os jogos para a temporada seguinte.
# As dos inscritos (o construtor os escolhe na criacao da carreira; a copa joga de fevereiro a maio de 2002): as vagas
# em ordem inversa (outros confrontos) e sem o primeiro nome (a vaga 0 vazia leva ao caminho dos estaduais: outros inscritos).
MUTACOES = [
    ('cqv6_copa_do_brasil_um_ano_adiante',
     'construct_bra_cup_body(this, year, record) // cqv6: the Copa do Brasil',
     'construct_bra_cup_body(this, year + 1, record) // mutation: one year ahead',
     'cqv_6 copa do brasil new'),
    ('cqv6_vagas_em_ordem_inversa',
     'write_u32(read_usize(this + cup::ENTRANT_BOOK) + k * 6, c as u32); // cqv6: the slot of the name',
     'write_u32(read_usize(this + cup::ENTRANT_BOOK) + (70 - k) * 6, c as u32); // mutation: the slots in reverse order',
     'cqv_6 the 71 names'),
    ('cqv6_sem_o_primeiro_nome',
     'for (k, &t) in BRA_CUP_ENTRANT_NAMES.iter().enumerate() { // cqv6: the 71 names',
     'for (k, &t) in BRA_CUP_ENTRANT_NAMES.iter().enumerate().skip(1) { // mutation: without Vasco',
     'cqv_6 the 71 names'),
]
