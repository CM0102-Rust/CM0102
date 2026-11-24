# Mutacoes (cx10_6), carreira ENG ate 2002-06-01. Nenhuma abre caixa.
# - o construtor sem a data: o comparador acusa na primeira chamada (os recordes e as estatisticas a constroem logo na
#   criacao do jogo);
# - a saida da crise ao clube seguinte: a noticia vai a outro clube (so a caixa do tecnico; pode escapar se nenhum
#   clube sair da crise ate a data, ou se nenhum dos dois for humano);
# - os planos do estadio sem o tipo: so o texto muda (a noticia so sai na subida de divisao, fim de temporada): escapa
#   na janela da cadeia; fica sem mutacao propria, como os envios de noticia do cx01_3.
MUTACOES = [
    ('cx10_6_construtor_sem_data',
     '    exe::date_date_54a110(this);\n    this\n}',
     '    let _ = FN_DATE_DATE_54A110; // mutation\n    this\n}'),
    ('cx10_6_credores_clube_seguinte',
     '    exe::new_newsitem(p, 0x1783, 0);\n    exe::newsitem_set_slot(p, 0, club_id);\n    exe::news_send_to_club_manager(read_usize(G_NEWS), p, club_at(club_id as i32 as isize));',
     '    exe::new_newsitem(p, 0x1783, 0);\n    exe::newsitem_set_slot(p, 0, club_id);\n    exe::news_send_to_club_manager(read_usize(G_NEWS), p, club_at(club_id as i32 as isize + 1)); // mutation',
     'cx10_6 receivers appeased news'),
]
