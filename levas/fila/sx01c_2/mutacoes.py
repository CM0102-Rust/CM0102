# Mutacoes (sx01c_2). A rotina roda a cada expulsao (o pos-jogo, modo Rust); a noticia nao volta para a simulacao,
# mas os sorteios sim:
# - sem o sorteio da vaga 8 (muda a sequencia do gerador);
# - a resposta do tecnico do computador nunca pedida (a 0x735870 sorteia; muda a sequencia do gerador).
MUTACOES = [
    ('sx01c_expulso_sorteio',
     'write(n, 8, exe::utils_random(2) as u32);',
     'write(n, 8, 0); // mutation: no draw',
     'sx01c_2 sent off story'),
    ('sx01c_expulso_resposta',
     'let response = if media_is_human_club(club.addr(), 0, 1) == 0 {',
     'let response = if media_is_human_club(club.addr(), 0, 1) == 2 { // mutation: never the AI response',
     'sx01c_2 sent off story'),
]
