# Mutacoes (com o ramo de cada uma). O emprestimo assinado sem a vaga de jogador no clube novo (cargo 0 na 0x53bd10:
# o emprestado fica fora do elenco do clube que o recebe; as escalacoes mudam). O fim do emprestimo sem a volta ao
# dono (+0x39 fica no clube do emprestimo). A troca de lugar no clube com a marca de elenco trocada (principal vira
# reserva e vice-versa; a 0x4cbd20 roda pelas 0x4cc0b0/0x4cc0f0 e pela 0x4cb02b depois das partidas). Se nao houver
# emprestimo da maquina ate 2002-06-01 as duas primeiras PODEM ESCAPAR (os ramos dizem; comparando, so a original).
MUTACOES = [
    ('cpp_emprestimo_sem_vaga', 'cpp2_por_no_clube(novo, pessoa, 0xb, 0xae2c90, 1, 0); // o jogador no clube novo',
     'cpp2_por_no_clube(novo, pessoa, 0, 0xae2c90, 1, 0); // mutacao: sem a vaga de jogador',
     'cpp_2 assinar emprestimo'),
    ('cpp_fim_sem_volta', 'cpp2_por(pessoa + 0x39, volta as u32); // a pessoa volta ao dono',
     'let _ = volta; // mutacao: a pessoa nao volta ao dono',
     'cpp_2 encerrar emprestimo'),
    ('cpp_troca_marca', 'cpp2_marcar(pessoa, l as u32); // 1 principal, 2 reserva',
     'cpp2_marcar(pessoa, 3 - l as u32); // mutacao: a marca trocada',
     'cpp_2 trocar no clube'),
]
