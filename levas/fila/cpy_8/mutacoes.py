# Mutacoes (cpy_8), na escolha do arbitro de cada jogo (0x79c7b0 pela montagem do dia, "montagem avisada" roda ~4000
# vezes na carreira): o arbitro sai do motor da partida (faltas, cartoes, impedimentos pela 0x6cfca0 e irmas) e o
# sorteio muda o estado do gerador.
# 1) o sorteio na band com a conta inteira (sorteio(conta) em vez de sorteio(conta - 1)): outro arbitro na maioria
#    dos jogos (contador "cpy_8 arbitro sorteado");
# 2) a band da liga ignorada (o bloco inteiro da nacao): os jogos das ligas inglesas com arbitros de fora da band da
#    divisao (contador "cpy_8 band da liga");
# 3) nas competicoes dos tipos 2 e 4 o comeco do bloco em vez do quarto de cima (contador "cpy_8 arbitro neutro");
#    pode escapar se nenhuma competicao desses tipos for play ate 2002-06-01 (o contador mostra).
MUTACOES = [
    ('cpy8_sorteio_conta_inteira', '        ini.wrapping_add(sortear(n.wrapping_sub(1))) // cpy8: o sorteio na band',
     '        ini.wrapping_add(sortear(n)) // mutacao: a conta inteira',
     'cpy_8 arbitro sorteado'),
    ('cpy8_sem_faixa_da_liga', '                            band // cpy8: a band da liga',
     '                            { let _ = band; (cpy8_i(b + 8), cpy8_i(b + 4)) } // mutacao: o bloco inteiro',
     'cpy_8 band da liga'),
    ('cpy8_neutro_do_comeco', '                    (cpy8_i(b + 8).wrapping_sub(match_pl_ftol(conta * -0.75)), match_pl_ftol(conta * 0.25)) // cpy8: o quarto de cima',
     '                    (cpy8_i(b + 8), match_pl_ftol(conta * 0.25)) // mutacao: o comeco do bloco',
     'cpy_8 arbitro neutro'),
]
