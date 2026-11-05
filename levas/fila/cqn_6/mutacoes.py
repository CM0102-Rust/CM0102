# Mutacoes (cqn_6), no calendario real de 2001/02 da Serie C1 e nos tocos. Como na cqn_5, as duas listas da C1 caem
# sempre na carreira de 8 ligas (clubes sem achar: provado pelo agente, 05/11); so a lista mantida aparece.
# - a lista da C1 nunca cai;
# - o toco do add_result devolve 0 (2124 chamadas na carreira): ESCAPOU na prova do agente (quem chama nao usa a
#   devolucao nessa carreira);
# - o toco do slot 25 devolve 0 (111 chamadas): acusada em 2001-143.
# A lista mantida derruba o jogo no primeiro dia (acusada pela queda, sem travar).
MUTACOES = [
    ('cqn6_lista_nunca_cai',
     'if cqn6_d(f + 8) == 0xffff_ffff || cqn6_d(f + 0xc) == 0xffff_ffff { // cqn6: um clube sem achar derruba a lista',
     'if false { // mutacao: a lista nunca cai',
     'cqn_6 calendario fixo'),
    ('cqn6_toco_de_dois_devolve_zero',
     '    1 // cqn6: o toco devolve 1',
     '    0 // mutacao: o toco devolve 0'),
    ('cqn6_toco_de_um_devolve_zero',
     'unsafe extern "thiscall" fn toco_um_com_um(_this: usize, _a: u32) -> u32 {\n    1\n}',
     'unsafe extern "thiscall" fn toco_um_com_um(_this: usize, _a: u32) -> u32 {\n    0\n}'),
]
