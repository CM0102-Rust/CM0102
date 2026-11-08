# Mutacoes (cqe_6), no periodo do emprestimo para um clube ingles (0x579c70 pela 0x8d0590, 203886 chamadas ate
# 2002-06-01); as regras da Escocia, Italia, Irlanda e Espanha nao rodam na carreira inglesa (passageiras):
# - o fim do emprestimo vai a quatro meses do comeco em vez de tres: os emprestimos duram mais e as datas de volta
#   mudam (0x842740 confere o periodo com o contrato e a janela);
# - o comeco vai para dois meses adiante: o periodo inteiro desloca;
# - o periodo da regra base passa a valer so abaixo de 21 anos (em vez de 24): de 21 a 23 anos, de julho a dezembro, o
#   periodo vira o de tres meses (16384+ chamadas pela base ate 2001-09-01).
MUTACOES = [
    ('cqe6_quatro_meses',
     '*fim_mes = (*ini_mes).wrapping_add(3); // cqe6: três meses',
     '*fim_mes = (*ini_mes).wrapping_add(4); // mutacao: quatro meses',
     'cqe_6 england three months'),
    ('cqe6_comeco_dois_meses',
     '*ini_mes = (mes(0xae2c90) as i8).wrapping_add(1); // cqe6: o mês seguinte (Inglaterra)',
     '*ini_mes = (mes(0xae2c90) as i8).wrapping_add(2); // mutacao: dois meses adiante',
     'cqe_6 england three months'),
    ('cqe6_base_abaixo_de_21',
     '< 0x18 { // cqe6: abaixo de 24 anos',
     '< 0x15 { // mutacao: abaixo de 21 anos',
     'cqe_6 england youth period'),
]
