# Mutacoes (com o ramo de cada uma; os tres comparadores rodam so a original: so o modo Rust e a carreira acusam). A
# suspensao nova sem ligar as regras (o jogador nao cumpre a suspensao: as escalacoes mudam). Na troca de clube, o
# clube novo nunca cobre o orgao (o transferido perde as vagas: cartoes acumulados e suspensoes, e as suspensoes
# seguintes mudam). No fim do emprestimo, idem para as vagas nao devolvidas. A terceira depende de emprestimos que
# terminem com o jogador em vaga de orgao (155 chamadas; PODE ESCAPAR se nenhum tiver vaga ocupada: ver o ramo).
MUTACOES = [
    ('cpo11_suspensao_sem_regras', 'ligar(ruling_body_of_rule(this, regra), id, vaga as u16 as u32); // liga a regra à suspensão',
     'let _ = (&ligar, vaga); // mutacao: a regra sem ligar a suspensao',
     'cpo_11 new suspension'),
    ('cpo11_transferido_sem_cobertura',
     'if ruling_body_covers_club(orgao, clube) {\n            continue;\n        }\n        let e = cpo11_entrada(orgao, id);\n        if e == 0 {',
     '// mutacao: o clube novo nunca cobre\n        let e = cpo11_entrada(orgao, id);\n        if e == 0 {',
     'cpo_11 transferred frees the slot'),
    ('cpo11_emprestimo_sem_cobertura',
     'if ruling_body_covers_club(orgao, clube) {\n            continue;\n        }\n        let e = cpo11_entrada(orgao, id);\n        if e != 0 {',
     '// mutacao: o clube do contrato nunca cobre\n        let e = cpo11_entrada(orgao, id);\n        if e != 0 {',
     'cpo_11 loan end frees the slot'),
]
