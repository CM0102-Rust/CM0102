# Mutacoes (cx10_12), carreira com a Irlanda ate 2002-06-01. Nenhuma abre caixa. O progresso e acusado pelo comparador
# a cada rodada; as vagas fixas so quando o calendario e sorteado (na criacao do jogo, que a carreira de teste refaz):
# a leitura trocada muda o calendario irlandes (carreira) e o comparador acusa.
MUTACOES = [
    ('cx10_12_irlanda_pior_sem_troca',
     '    if worst == 0xa {\n        covered!("cx10_12 irish worst code 10");\n        worst = 0xb;',
     '    if worst == 0xa {\n        covered!("cx10_12 irish worst code 10");\n        worst = 0xa; // mutation'),
    ('cx10_12_vaga_sem_entrada_zero',
     '            return read_u8(e + 4) as u32;\n        }\n    }\n    0xff',
     '            return read_u8(e + 4) as u32;\n        }\n    }\n    0 // mutation'),
    ('cx10_12_vaga_nao_toma_de_outra_divisao',
     '        if held == -1 || read_usize(club_at(held as isize).addr() + club::DIVISION) != League(this).competition().addr() {',
     '        if held == -1 { // mutation'),
]
