# Mutacoes (cqb_13), todas na 0x5ae380 (7 chamadas ate 2002-06-01, pela add_fixture quando dois jogos caem no mesmo
# dia): o dia da semana alvo um a frente (o jogo que sai vai para outra data), a prioridade invertida (sai o outro
# jogo) e o "ainda nao jogado" invertido (com o novo vencendo, sai o novo). As datas dos jogos sao lidas pela tabela e
# pela impressao do banco. Podem escapar se as remarcacoes cairem, com a data errada, num dia que a 0x5ab510 acerta do
# mesmo jeito (conflito na data nova manda procurar outra) ou se, na carreira, as duas prioridades forem sempre iguais
# com a mesma injury_word_0c (os contadores "cqb_13 dia alvo" e "cqb_13 novo vence" mostram quantas vezes cada trecho rodou).
MUTACOES = [
    ('cqb13_alvo_mais_1', '    ajustar(&mut a, alvo, &mut b); // cqb13: o dia da semana alvo',
     '    ajustar(&mut a, alvo % 7 + 1, &mut b); // mutacao: um dia a frente',
     'cqb_13 dia alvo'),
    ('cqb13_prioridade_invertida', '    let novo_vence = pn > pv // cqb13: a prioridade maior fica',
     '    let novo_vence = pn < pv // mutacao: a prioridade menor fica',
     'cqb_13 dia alvo'),
    ('cqb13_ja_jogado', "    let sai_o_velho = novo_vence && *((v + 0x43) as *const u8) == 0xff; // cqb13: o da lista ainda não jogado",
     "    let sai_o_velho = novo_vence && *((v + 0x43) as *const u8) != 0xff; // mutacao: so o ja jogado sai",
     'cqb_13 novo vence'),
]
