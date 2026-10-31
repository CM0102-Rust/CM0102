# Mutacoes. A 0x7771c0 roda 684 vezes na carreira automatica ate 2002-06-01 (cobertura de 17/10), a cada convocacao
# para um jogo de selecao. Os registros dela sao lidos pela 0x776fc0 (ja em Rust, 7.711 chamadas): com mais de dois
# convocados na janela de um dia em volta da data, o jogo do clube nao e marcado ("cpp_12 convocado na janela" conta
# 64 e "cpp_12 visitante convocado" 8 na base da 257a). As tres mudam essas contas: o registro novo com um a mais, a
# soma na mesma data (dois times convocando do mesmo clube para a mesma data) perdida, e o ultimo clube da volta
# (que a original deixa sem registro) registrado. Os ramos ficam colados no efeito.
MUTACOES = [
    ('cpw13_um_a_mais',
     '*((e + 8) as *mut u8) = conta; // cpw13: a conta do registro novo',
     '*((e + 8) as *mut u8) = conta.wrapping_add(1); // mutacao: um a mais',
     'cpw_13 registro novo'),
    ('cpw13_sem_somar',
     '*((e + 8) as *mut u8) = (*((e + 8) as *const u8)).wrapping_add(conta); // cpw13: soma na data',
     'let _ = conta; // mutacao: sem somar na data',
     'cpw_13 soma na data'),
    ('cpw13_ultimo_clube',
     'crate::comparacao::ramo("cpw_13 último clube sem registro"); // cpw13: o último clube fica sem registro',
     'crate::comparacao::ramo("cpw_13 último clube sem registro"); cpw13_somar(this, anterior, conta, d.as_mut_ptr()); // mutacao: o ultimo clube registrado',
     'cpw_13 último clube sem registro'),
]
