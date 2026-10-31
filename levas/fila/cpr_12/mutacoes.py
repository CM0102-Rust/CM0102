# Mutacoes. A 0x5acbe0 e a busca de jogos do calendario que a escalacao (MATCH_MAN::pick_team), as financas do dia,
# a marcacao dos amistosos, o adiamento pela TV e a diretoria usam (288 mil chamadas ate 2002-06-01): o clube so como
# mandante (os jogos fora somem das buscas) e a janela um dia mais curta (o ultimo dia fica de fora) mudam as datas dos
# amistosos, a escalacao e as financas. A 0x5abd90 sem o filtro da rodada (130 chamadas: adiamentos pela TV e rodadas
# de copa nas datas fixas) muda jogos de outras rodadas junto. A 0x5acbe0 e quente: sem ramo.
MUTACOES = [
    ('cpr12_so_em_casa', 'if clube != casa && clube != cpr12_d(f + 0x10) {',
     'if clube != casa { // mutacao: so em casa'),
    ('cpr12_janela_curta', '(ano as u16 == ano1 as u16 && dia as u16 > dia1 as u16)',
     '(ano as u16 == ano1 as u16 && dia as u16 >= dia1 as u16) /* mutacao: sem o ultimo dia */'),
    ('cpr12_qualquer_rodada', 'ok = false; // outra rodada',
     '// mutacao: a rodada nao filtra',
     'cpr_12 jogo da rodada'),
]
