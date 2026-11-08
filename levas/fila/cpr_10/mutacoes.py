# Mutacoes. O jogador lesionado aparece como recuperando a forma (1, nao 2) e o que recupera a forma como pronto (0,
# nao 1): a escalacao automatica (0x6dbc50), os starters e as buscas dos clubes leem o estado a cada partida e
# escolhem outros jogadores (quentes: sem contador). A primeira data de decisao sobre as ofertas cai em 2 a 6 dias, nao
# 1 a 5 (as respostas as ofertas e as transferencias mudam de data). Os candidatos a tecnico do ano so tem
# comparacao (premio de fim de temporada, que a carreira pode nao ler): sem mutacao.
MUTACOES = [
    ('cpr10_lesionado_como_recuperando', 'return 2; // cpr10: lesionado',
     'return 1; // mutacao: lesionado como recuperando'),
    ('cpr10_recuperando_como_pronto', '(dias < cpr10_w(l + 0x28) as i32) as u8 // cpr10: recuperando a forma',
     '0 // mutacao: recuperando como pronto'),
    ('cpr10_decisao_mais_tarde', 'injury_date_add_days(0xae2c90, (sortear(5) + 1) as u32, &mut d); // cpr10: a decisão em 1 a 5 dias',
     'injury_date_add_days(0xae2c90, (sortear(5) + 2) as u32, &mut d); // mutacao: 2 a 6 dias',
     'cpr_10 nova decisao'),
]
