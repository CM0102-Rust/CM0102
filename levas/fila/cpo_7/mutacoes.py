# Mutacoes (com o ramo de cada uma), todas na gratidao (25 chamadas na carreira ate 2002-06-01; as outras duas rodam
# uma vez so e so mexem em texto e caixa de noticias). Os tres comparadores rodam so a original: so o modo Rust e a
# carreira acusam. A primeira tira o sorteio(10) da troca de tipo (o gerador anda um a menos), a segunda nao sobe +0x44
# da ficha do jogador (lido de novo pela gratidao e pela resposta do tecnico 0x735870, tipos 0x28..0x3b) e a terceira
# tira o sorteio(3) dos tipos 4, 9, 33, 34 e 37. Com 25 chamadas, cada ramo PODE nao aparecer na carreira (ver os
# contadores); a primeira e a terceira sao as mais provaveis.
MUTACOES = [
    ('cpo_gratidao_tipo_sem_sorteio', 'let s = sorteio(10);\n            t = if cpo7_c(jogador + 0x5d) >= s { 0x12 } else { 0x21 };',
     'let s = 0; // mutacao: sem o sorteio\n            t = if cpo7_c(jogador + 0x5d) >= s { 0x12 } else { 0x21 };',
     'cpo_7 gratidao tipo'),
    ('cpo_gratidao_nao_sobe', '*((ficha + 0x44) as *mut u8) += 1;',
     '// mutacao: sem subir +0x44',
     'cpo_7 gratidao sobe'),
    ('cpo_gratidao_sem_sorteio_3', 'crate::comparison::branch("cpo_7 gratidao sorteio 3");\n            sorteio(3)',
     'crate::comparison::branch("cpo_7 gratidao sorteio 3");\n            0 // mutacao: sem o sorteio',
     'cpo_7 gratidao sorteio 3'),
]
