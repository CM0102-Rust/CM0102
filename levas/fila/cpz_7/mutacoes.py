# Mutacoes (cpz_7), na 0x7e4750, que a estatistica da tatica (0x896290, 3 mil partidas sem humano ate 2002-06-01)
# chama para o mandante e para o visitante no fim de cada partida: o placar pro e contra do registro decide vitoria,
# empate ou derrota, os pontos e os gols da linha da tatica, que a escolha da formacao do computador (0x6e0230) le.
# Os gols do mandante pelos do visitante (empate sempre), o visitante sofrendo os proprios gols, ou o mandante achado
# pelo visitante (placar ao contrario dos dois lados) mudam as razoes das taticas, a formacao escolhida e as partidas
# seguintes. Os jogos de liga nao tem prorrogacao (+0x47 = -1): os contadores ficam no ramo do placar normal, depois
# das condicoes (ponteiros nao nulos, o lado, a prorrogacao), colados no placar gravado.
MUTACOES = [
    ('cpz7_mandante_com_os_gols_do_visitante',
     '*s(0x19) = cpz7_b(jogo + 0x43); // cpz7: os gols do mandante',
     '*s(0x19) = cpz7_b(jogo + 0x44); // mutacao: os gols do visitante',
     'cpz_7 placar do mandante'),
    ('cpz7_visitante_sofre_os_proprios_gols',
     '*s(0x1a) = cpz7_b(jogo + 0x43); // cpz7: os gols sofridos pelo visitante',
     '*s(0x1a) = cpz7_b(jogo + 0x44); // mutacao: os gols do proprio visitante',
     'cpz_7 placar do visitante'),
    ('cpz7_mandante_pelo_visitante',
     'if clube as u32 == cpz7_d(jogo + 0x1c) { // cpz7: o clube é o mandante',
     'if clube as u32 == cpz7_d(jogo + 0x20) { // mutacao: o mandante pelo visitante',
     'cpz_7 placar do mandante'),
]
