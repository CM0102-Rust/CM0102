# Mutacoes (cx09_20) pela carreira com a Finlandia ate 2002-08-01 (o grupo sul nasce na criacao do jogo e na virada; a
# repescagem contra a Veikkausliiga joga em outubro de 2001):
# - a repescagem leva o clube da Primeira com status 1 (e nao 2) na repescagem de acesso: o outro finalista joga, quem
#   sobe muda (o comparador da repescagem tambem acusa).
# O grupo sul usa o molde do grupo B belga (create_second_group_stage do cx09_13, que tem a mutacao dele): sem mutacao
# propria (mudar a chave ou os turnos abre a caixa do grupo incompleto).
MUTACOES = [
    ('cx09_20_finalista_errado',
     '    if let Some(k) = (0..count).find(|&k| read_u8(teams + 6 * k as usize + cup_team_entry::STATUS) == 2) {',
     '    if let Some(k) = (0..count).find(|&k| read_u8(teams + 6 * k as usize + cup_team_entry::STATUS) == 1) { // mutation'),
]
