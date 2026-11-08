# Mutacoes (cqq_6), no construtor da primeira divisao francesa (a criacao da carreira de 8 ligas):
# - a divisao de baixo montada antes da CFA (os sorteios das duas ligas de fundo trocam de ordem desde o 1o dia);
# - a regra de disciplina 9 em vez de 0xc (as suspensoes da primeira francesa mudam).
# Trocadas (06/11): mudar a competicao de reserva dos clubes da CFA ou da divisao de baixo derrubava o jogo no 1o dia
# (as listas de 72/40 das ligas de fundo ficam com clubes nulos, como na cqn_4); acusadas pela queda.
MUTACOES = [
    ('cqq6_divisao_de_baixo_antes_da_cfa',
     '    let a = novo(0xee);\n    let ligas_cfa = if a != 0 { cfa(a, ano) } else { 0 };\n    let b = novo(0xee);\n    let ligas_baixo = if b != 0 { baixo(b, ano) } else { 0 };',
     '    let b = novo(0xee);\n    let ligas_baixo = if b != 0 { baixo(b, ano) } else { 0 };\n    let a = novo(0xee);\n    let ligas_cfa = if a != 0 { cfa(a, ano) } else { 0 };',
     'cqq_6 new french first division'),
    ('cqq6_outra_disciplina',
     '    *((this + 0x50) as *mut u8) = 0xc; // cqq6: a disciplina da primeira francesa',
     '    *((this + 0x50) as *mut u8) = 9; // mutacao: outra disciplina',
     'cqq_6 new french first division'),
]
