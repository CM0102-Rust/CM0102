# Mutacoes (mkc_6), na carreira com as ligas alema e irlandesa (LIGAS): o emprestimo com clube irlandes termina fora
# da janela (a IA e a conferencia do emprestimo deixam de esperar a janela irlandesa); cada emprestimo de clube
# irlandes conta cinco (o primeiro ja esgota o limite de 5 da temporada: o can_loan_player irlandes recusa esse clube
# nas listas e nos pedidos ate 20 de junho). A mutacao alema saiu (ver PRONTO.txt): so a tela de encerrar do humano
# passa flag 0. A de tirar a conta escapou: nenhum clube irlandes chega a 5 emprestimos na temporada.
MUTACOES = [
    ('mkc6_irlanda_encerra_fora_da_janela',
     'date, 0) != 0 {\n        return 1; // mkc6: Ireland, the window open',
     'date, 0) != 0 || true {\n        return 1; // mutacao: fora da janela',
     'mkc_6 ireland window closed'),
    ('mkc6_irlanda_conta_cinco',
     '        write_u8(p, read_u8(p).wrapping_add(1)); // mkc6: Ireland, the season count',
     '        write_u8(p, read_u8(p).wrapping_add(5)); // mutacao: cada emprestimo conta cinco',
     'mkc_6 ireland loan counted'),
]
