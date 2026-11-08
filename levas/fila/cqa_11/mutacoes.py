# Mutacoes (cqa_11), todas na 0x868830, que roda a cada jogo (pelo match_wrap_up da partida do dia). Os registros dos
# tecnicos ficam fora do banco; a carreira os le no tecnico do mes (0x86f950, pelos pontos do mes +0xce de cada liga
# inglesa, todo mes) e no do ano (+0xba). Sem os pontos do mes do mandante, a vitoria do mandante valendo como empate
# ou a nota do mandante pela injury_word_0c do proprio clube, a soma de +0xce muda e o tecnico do mes de alguma divisao
# muda (o premio mexe na injury_word_0c). Podem escapar da carreira se nenhum tecnico do mes mudar ate 2002-06-01; o
# comparando acusa no primeiro jogo (lado a lado pelos dois registros).
MUTACOES = [
    ('cqa11_mes_sem_mandante', '            cqa11_somar(a + 0xce, q); // cqa11: os pontos do mês do mandante',
     '            let _ = q; // mutacao: sem os pontos do mes do mandante',
     'cqa_11 points of the month'),
    ('cqa11_vitoria_como_empate', '        ba = 900; // cqa11: o bônus da vitória do mandante',
     '        ba = 300; // mutacao: a vitoria do mandante vale como empate',
     'cqa_11 home win'),
    ('cqa11_reputacao_do_proprio', '        let q = match_rating_for_manager(p, 0, &mut saida, fora, ',
     '        let q = match_rating_for_manager(p, 0, &mut saida, casa, ',
     'cqa_11 home side rating'),
]
