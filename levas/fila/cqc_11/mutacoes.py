# Mutacoes (cqc_11): as tres na 0x5cbdf0 (20 chamadas ate 2002-06-01, pelo dia dos amistosos internacionais). Os
# amistosos criados viram partidas no calendario (quem joga, quando e onde): com o limit em 600, menos pares passam;
# com o mando trocado, o amistoso muda de estadio; sem marcar a nacao de fora dos jogos perto da data, ela fica livre
# e entra nos pares (e cada nacao livre a mais e um sorteio de 500 a mais). Podem escapar se nenhuma soma cair entre
# 400 e 600 (improvavel: o sorteio de 500 sozinho passa de 400 em 1/5 das vezes), se nenhum amistoso for criado nas
# 20 chamadas (o contador "cqc_11 friendly between national teams" mostra quantos) ou se as nacoes de fora dos jogos perto da
# data ja estiverem usadas por outro jogo (contador "cqc_11 nations with a match").
MUTACOES = [
    ('cqc11_limite_600', '                        if v > 400 { // cqc11: a soma acima de 400',
     '                        if v > 600 { // mutacao: limit 600',
     'cqc_11 friendly between national teams'),
    ('cqc11_mando_trocado', '                                let casa = sortear(2) as u32; // cqc11: o mando sorteado',
     '                                let casa = 1 - sortear(2) as u32; // mutacao: mando trocado',
     'cqc_11 friendly between national teams'),
    ('cqc11_fora_livre', '            *((usadas.wrapping_add(b as usize)) as *mut u8) = 1; // cqc11: as nações com jogo perto da data',
     '            // mutacao: a de fora fica livre',
     'cqc_11 nations with a match'),
]
