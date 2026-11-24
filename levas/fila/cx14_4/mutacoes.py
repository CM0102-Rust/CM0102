# Mutacoes (cx14_4), com a Turquia ate 2002-08-01:
# - quem cai nos grupos de classificacao fica com o status 0 na linha (a virada do verao de 2002,
#   tur_first_promote_lower_clubs_to_third_tier, rebaixa as linhas de status 3: muda quem cai; acusada tambem pelo
#   comparador dos registros);
# - as posicoes finais dos grupos de classificacao de 4 em diante (acusada pelo comparador, lado a lado, no fim da
#   temporada turca da primavera de 2002).
# O texto da noticia (0 chamadas na cobertura do mundo) fica sem mutacao: so o comparador o prova, se rodar.
MUTACOES = [
    ('cx14_4_rebaixado_sem_status',
     'plan.row = Some((row, 3)); // cx14_4: the classification group club goes down',
     'plan.row = Some((row, 0)); // mutation: status 0',
     'cx14_4 category b row status'),
    ('cx14_4_posicoes_de_4',
     '(k as u8).wrapping_add(3))); // cx14_4: places from 3',
     '(k as u8).wrapping_add(4))); // mutation: places from 4',
     'cx14_4 category b final positions'),
]
