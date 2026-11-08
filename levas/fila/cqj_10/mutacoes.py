# Mutacoes (cqj_10). A nao classificacao (0x86c1d0) e a eliminacao (0x86c000) encerram as convocacoes da selecao na
# competicao (end_national_duty): sem elas, os jogadores das selecoes eliminadas (da Copa Africana de janeiro e
# fevereiro de 2002, com jogadores nas ligas da Inglaterra, Franca, Espanha e Italia; da Copa America de julho de
# 2001, com jogadores do Brasil e das ligas europeias) seguem convocados e faltam aos jogos dos clubes. A carreira pega
# pelas escalacoes. Os contadores ficam colados no efeito, depois das condicoes da fase e do desfecho.
MUTACOES = [
    ('cqj10_africana_sem_fim_nos_grupos',
     'european_cup_add_failed_to_qualify_record(clube, comp, 0x41c, 0x19); // cqj10: quem cai nos grupos da Copa Africana',
     'let _ = comp; // mutacao: quem cai nos grupos da Copa Africana segue convocado',
     'cqj_10 africana fora nos grupos'),
    ('cqj10_africana_sem_fim_no_mata_mata',
     'european_cup_add_knocked_out_record(clube, comp, cqj10_w(jogo + 0x32) as u16 as u32, cqj10_w(jogo + 0x30) as u16 as u32, peso); // cqj10: quem cai no mata-mata da Copa Africana',
     'let _ = peso; // mutacao: quem cai no mata-mata da Copa Africana segue convocado',
     'cqj_10 africana fora no mata-mata'),
    ('cqj10_copa_america_sem_fim_nos_grupos',
     'european_cup_add_failed_to_qualify_record(clube, comp, 0x41c, 0x19); // cqj10: quem cai nos grupos da Copa América',
     'let _ = comp; // mutacao: quem cai nos grupos da Copa America segue convocado',
     'cqj_10 copa america fora nos grupos'),
]
