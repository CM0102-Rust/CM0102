# Mutacoes (mkc_6), na carreira com as ligas alema e irlandesa (LIGAS): o jogador de clube alemao nunca pode ter o
# emprestimo encerrado, nem com a flag (o can_loan_player recusa emprestar quem pertence a clube alemao e a IA nao pede
# emprestado de clube alemao); o emprestimo com clube irlandes termina fora da janela (a IA e a conferencia do
# emprestimo deixam de esperar a janela irlandesa); o clube irlandes nao conta os emprestimos (o limite de 5 por
# temporada some).
MUTACOES = [
    ('mkc6_alemanha_nunca_encerra',
     '    if flag != 0 {\n        return 1;\n    }\n    let player = Staff(player);\n    let key = read_usize(KEY_NATION_GERMANY);',
     '    let _ = flag; // mutacao: nem com a flag\n    let player = Staff(player);\n    let key = read_usize(KEY_NATION_GERMANY);',
     'mkc_6 germany no early end'),
    ('mkc6_irlanda_encerra_fora_da_janela',
     'date, 0) != 0 {\n        return 1; // mkc6: Ireland, the window open',
     'date, 0) != 0 || true {\n        return 1; // mutacao: fora da janela',
     'mkc_6 ireland window closed'),
    ('mkc6_irlanda_sem_conta',
     '        write_u8(p, read_u8(p).wrapping_add(1)); // mkc6: Ireland, the season count',
     '        let _ = p; // mutacao: sem conta',
     'mkc_6 ireland loan counted'),
]
