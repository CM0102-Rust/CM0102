# Mutacoes (cqv_6): cada construtor roda uma vez, na criacao da carreira de 8 ligas (contadores "<item> <copa> nova"); a
# copa montada um ano adiante leva as rodadas, os inscritos e os jogos para a temporada seguinte.
# As dos inscritos (o construtor os escolhe na criacao da carreira; a copa joga de fevereiro a maio de 2002): as vagas
# em ordem inversa (outros confrontos) e sem o primeiro nome (a vaga 0 vazia leva ao caminho dos estaduais: outros inscritos).
MUTACOES = [
    ('cqv6_copa_do_brasil_um_ano_adiante',
     'cqv6_copa_do_brasil_corpo(this, ano, dono) // cqv6: copa_do_brasil',
     'cqv6_copa_do_brasil_corpo(this, ano + 1, dono) // mutacao: um ano adiante',
     'cqv_6 copa_do_brasil nova'),
    ('cqv6_vagas_em_ordem_inversa',
     'cqv6_p32(cqv6_d(this + 0x14) + k * 6, c as u32); // cqv6: a vaga do nome',
     'cqv6_p32(cqv6_d(this + 0x14) + (70 - k) * 6, c as u32); // mutacao: as vagas em ordem inversa',
     'cqv_6 os 71 nomes'),
    ('cqv6_sem_o_primeiro_nome',
     'for (k, &t) in CQV6_NOMES.iter().enumerate() { // cqv6: os 71 nomes',
     'for (k, &t) in CQV6_NOMES.iter().enumerate().skip(1) { // mutacao: sem o Vasco',
     'cqv_6 os 71 nomes'),
]
