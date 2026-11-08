# Mutacoes (cqr_3). A serie final e a grande final da NSL de 2001/02 rodam uma vez cada na carreira de 8 ligas ate
# 2002-06-01 (contadores "cqr_3 serie final" e "cqr_3 grande final"); as tres mudam quem joga na serie final:
# - a serie final com o segundo ao setimo da tabela (o primeiro fora, o setimo dentro): outros jogos na serie final;
# - a serie final com os seis na ordem inversa: outra ordem da tabela da fase e outros jogos por rodada;
# - a serie final com o setimo no lugar do sexto: outro clube na serie final.
# (As mutacoes na grande final escaparam na prova do agente: a final e montada no fim de maio e play depois de
# 2002-06-01; trocadas, 06/11.)
MUTACOES = [
    ('cqr3_serie_final_sem_o_primeiro',
     '        *c = aus_nsl_table_row_club(this, i); // cqr3: os seis primeiros da tabela\n',
     '        *c = aus_nsl_table_row_club(this, i + 1); // mutacao: do segundo ao setimo\n',
     'cqr_3 serie final'),
    ('cqr3_serie_final_invertida',
     '        *c = aus_nsl_table_row_club(this, i); // cqr3: os seis primeiros da tabela\n',
     '        *c = aus_nsl_table_row_club(this, 5 - i); // mutacao: os seis na ordem inversa\n',
     'cqr_3 serie final'),
    ('cqr3_serie_final_com_o_setimo',
     '        *c = aus_nsl_table_row_club(this, i); // cqr3: os seis primeiros da tabela\n',
     '        *c = aus_nsl_table_row_club(this, if i == 5 { 6 } else { i }); // mutacao: o setimo no lugar do sexto\n',
     'cqr_3 serie final'),
]
# Prova do agente (06/11, carreira Rust de 8 ligas ate 2002-06-01; base com os 12 itens cqr/cqs/cqt_1..4, impressao b690295afd222f6a): as tres ACUSADAS (a serie final em 2002-087).
