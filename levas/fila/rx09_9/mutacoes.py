# Mutacoes (rx09_9), com a Espanha e a Suecia ate 2002-09-01 e a base NORMAL:
# - a lista de cada secao de tras para a frente: as linhas e os jogos da fase mudam (criacao do jogo e virada de 2002;
#   a carreira acusa, o comparador confere so o conjunto);
# - o acesso armado sem sorteio (Cup::set_up_round na ordem da lista): outros confrontos em novembro de 2001;
# - a repescagem com os doze primeiros sorteados (os mesmos guardados em +0xee) em vez dos doze seguintes: outros clubes
#   jogam (o comparador e a carreira acusam).
MUTACOES = [
    ('rx09_9_secao_invertida',
     'write_usize(list + 4 * count as usize, c.addr()); // rx09_9: the next club of the section',
     'write_usize(list + 4 * (n - 1 - count) as usize, c.addr()); // mutation: from the end',
     'rx09_9 swedish second section stage'),
    ('rx09_9_acesso_sem_sorteio',
     'exe::cup_draw_round(read_usize(l.stages() + 4 * 5), 0); // rx09_9: the promotion ties drawn',
     'exe::cup_set_up_round(read_usize(l.stages() + 4 * 5), 0); // mutation: no draw',
     'rx09_9 swedish second promotion play-off'),
    ('rx09_9_repescagem_dos_doze_primeiros',
     'list[..12].copy_from_slice(&all[12..24]); // rx09_9: the next twelve play the runners-up',
     'list[..12].copy_from_slice(&all[0..12]); // mutation: the first twelve',
     'rx09_9 swedish second relegation play-off'),
]
