# Mutacoes (mkc_2), na carreira de 8 ligas: o jogador de clube frances nunca pode ser devolvido (o emprestimo dele
# nao passa no can_loan_player); a segunda janela italiana (inverno) nao conta para devolver; o clube australiano pega
# emprestado fora da janela.
MUTACOES = [
    ('mkc2_franca_nunca_devolve',
     '        return 1; // mkc2: France, the window open',
     '        return 0; // mutacao: nunca',
     'mkc_2 france window closed'),
    ('mkc2_italia_sem_a_segunda_janela',
     ' || exe::transfer_window_contains_date(this, date, read_usize(this + 4) + 0xc) != 0',
     '',
     'mkc_2 italy windows closed'),
    ('mkc2_australia_sem_prazo',
     'G_CURRENT_DATE, 0) != 0 {\n        return 1; // mkc2: Australia, the window open',
     'G_CURRENT_DATE, 0) != 0 || true {\n        return 1; // mutacao: sem prazo',
     'mkc_2 australia deadline'),
]
