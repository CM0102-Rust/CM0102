# Mutacoes. Todas no resultado no historico da selecao (append_result 0x58dea0, duas vezes por jogo internacional), que
# a permissao de trabalho 0x8c7950 conta pela count_results_in_period 0x58d710 (cpt_1) nos dois anos ate o mes de hoje:
# o ano seguinte tira o jogo da contagem ate virar o ano, o mes seguinte o tira ate virar o mes (os jogos de setembro,
# outubro e novembro de 2001 e os amistosos de 2002) e sem somar a conta a entrada nova e sobrescrita pela proxima. A
# contagem muda permissoes aceitas e recusadas (e os sorteios delas). A band de forca e as pontuacoes so pesam os
# pontos (tela e peso dos proximos resultados), fora da impressao: sem mutacao nelas.
MUTACOES = [
    ('cpt2_ano_seguinte', 'core::ptr::write_unaligned((p + 3) as *mut u16, ano); // cpt2: o ano do jogo',
     'core::ptr::write_unaligned((p + 3) as *mut u16, ano.wrapping_add(1)); // mutacao: o ano seguinte',
     'cpt_2 result'),
    ('cpt2_mes_seguinte', '*((p + 2) as *mut u8) = mes; // cpt2: o mes do jogo',
     '*((p + 2) as *mut u8) = mes.wrapping_add(1); // mutacao: o mes seguinte',
     'cpt_2 result'),
    ('cpt2_sem_somar_a_conta', '*((e + 0xa) as *mut i16) = c.wrapping_add(1); // cpt2: mais um resultado',
     '*((e + 0xa) as *mut i16) = c; // mutacao: a conta nao sobe',
     'cpt_2 result'),
]
