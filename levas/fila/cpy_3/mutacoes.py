# Mutacoes (cpy_3), na reparticao da verba (0x83b6e0, 17425 chamadas ate 2002-06-01 na cobertura de 17/10, pela
# atualizacao da lista do clube 0x83a730). A verba da vaga de titular aberta tackle por abertas x 3 em vez de x 2 muda
# o +7 da vaga, que a busca do alvo (0x83f3a0) usa como teto do preco: outros alvos e outros lances. Sem a reducao pela
# sobra, a vaga leva mais verba e a busca acha alvos mais caros. Sem preencher a vaga (0x83f3a0), o clube do computador
# fica sem alvo nela: os lances e as transferencias da carreira mudam.
MUTACOES = [
    ('cpy3_verba_de_titular_por_tres', '                        Some(a * 2) // cpy3: a verba da vaga de titular',
     '                        Some(a * 3) // mutacao: abertas x 3',
     'cpy_3 verba da vaga de titular'),
    ('cpy3_sem_reducao_pela_sobra', '                cpy3_pi(v + 7, e.wrapping_sub(x)); // cpy3: a redução da vaga pela sobra',
     '                let _ = (e, x); // mutacao: sem a reducao',
     'cpy_3 reducao pela sobra'),
    ('cpy3_vaga_sem_preencher', '            preencher(this, v, clube, 0); // cpy3: a vaga é preenchida',
     '            let _ = preencher; // mutacao: a vaga fica sem alvo',
     'cpy_3 vaga preenchida'),
]
