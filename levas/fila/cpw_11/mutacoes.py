# Mutacoes. A 0x685d30 roda 2.315 vezes na carreira automatica ate 2002-06-01 (cobertura de 17/10): todo jogo das
# ligas da carreira soma na tabela por ela (liga sem fases: +0x44 = +0x42 = 0xff). As tres mexem na tabela, que decide
# a classificacao, os rebaixamentos, os acessos e as copas europeias (e a ordem da tabela entra nas projecoes da
# 0x687710): sem a tabela, nenhum jogo conta; com uma linha a menos no fim, o lanterna nao soma os jogos; com a
# primeira linha de fora, o lider nao soma. O ramo fica colado na chamada, depois de todas as condicoes.
MUTACOES = [
    ('cpw11_sem_tabela',
     'aplicar(liga, p, cpw11_d(liga + 0xb1), cpw11_w(liga + 0x3e) as u16 as u32, 1, &mut pts_casa, &mut pts_fora); // cpw11: a tabela',
     'let _ = aplicar; // mutacao: sem a tabela',
     'cpw_11 resultado aplicado'),
    ('cpw11_ultima_linha_fora',
     'aplicar(liga, p, cpw11_d(liga + 0xb1), cpw11_w(liga + 0x3e) as u16 as u32, 1, &mut pts_casa, &mut pts_fora); // cpw11: a tabela',
     'aplicar(liga, p, cpw11_d(liga + 0xb1), cpw11_w(liga + 0x3e).wrapping_sub(1) as u16 as u32, 1, &mut pts_casa, &mut pts_fora); // mutacao: a ultima linha de fora',
     'cpw_11 resultado aplicado'),
    ('cpw11_primeira_linha_fora',
     'aplicar(liga, p, cpw11_d(liga + 0xb1), cpw11_w(liga + 0x3e) as u16 as u32, 1, &mut pts_casa, &mut pts_fora); // cpw11: a tabela',
     'aplicar(liga, p, cpw11_d(liga + 0xb1) + 0x3b, cpw11_w(liga + 0x3e).wrapping_sub(1) as u16 as u32, 1, &mut pts_casa, &mut pts_fora); // mutacao: a primeira linha de fora',
     'cpw_11 resultado aplicado'),
]
