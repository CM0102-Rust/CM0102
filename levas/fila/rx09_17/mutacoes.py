# Mutacoes (rx09_17), com a Inglaterra ate 2005-12-31 (a primeira fase asiatica em 2005, a segunda em agosto/outubro):
# - cada grupo da primeira fase com as selecoes sorteadas na ordem inversa: as linhas e os jogos dos grupos mudam (a
#   carreira acusa; o comparador confere so o conjunto);
# - sem isentos, um so classificado direto em cada grupo da segunda fase (a campanha de 2006 nao tem isentos): a fase
#   seguinte muda;
# - o segundo grupo da segunda fase com os vencedores na ordem inversa: as linhas e os jogos do grupo mudam (a carreira
#   acusa; o comparador confere so o conjunto).
# O advance_stage fica sem mutacao propria: so constroi fases (as mutacoes possiveis param a campanha ou derrubam o
# jogo); as fases que ele chama tem as suas neste item e no rx09_18.
MUTACOES = [
    ('rx09_17_grupo_invertido',
     "write_usize(list + 4 * i, read_usize(draw + 16 * (k as usize + 1) + 4 * i)); // rx09_17: the group's drawn teams",
     "write_usize(list + 4 * i, read_usize(draw + 16 * (k as usize + 1) + 4 * (n as usize - 1 - i))); // mutation: reversed",
     'rx09_17 asian first round group'),
    ('rx09_17_um_classificado',
     'let first = if read_u8(this + WC_ASIA_EXEMPT) == 0 { 2 } else { 1 }; // rx09_17: two qualify with no exempt nation',
     'let first = if read_u8(this + WC_ASIA_EXEMPT) == 0 { 1 } else { 1 }; // mutation: one',
     'rx09_17 asian second round groups'),
    ('rx09_17_segundo_grupo_invertido',
     'for k in -1i32..9 {\n        if taken[(k + 1) as usize] == 0 {',
     'for k in (-1i32..9).rev() { // mutation: reversed\n        if taken[(k + 1) as usize] == 0 {',
     'rx09_17 asian second round groups'),
]
