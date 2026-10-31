# Mutacoes (cps_6), todas nas faltas ao treino (0x5d6aa0, todo dia pelo laco do dia do jogo; a competicoes_do_dia so
# repassa o dia as virtuais e a confianca da diretoria so roda num ramo raro do laco do dia). A primeira pessoa sem
# sorteio (um sorteio a menos por dia e outras pessoas conferidas: tudo o que sorteia depois muda). A falta pela soma
# sem o sorteio (um sorteio a menos por jogador que passa os filtros da soma). Sem a falta (a 0x41a360 sorteia, multa e
# mexe na forma do jogador: os sorteios e a forma mudam).
MUTACOES = [
    ('cps6_primeira_sem_sorteio', '    let r = cps6_sortear(n - q - 1); // cps6: a primeira pessoa',
     '    let r = (n - q - 1) / 2; // mutacao: sem o sorteio',
     'cps_6 faltas do dia'),
    ('cps6_soma_sem_sorteio', '            if cps6_sortear(s as i8 as i32) == 0 { // cps6: o sorteio da soma',
     '            if s == 0 { // mutacao: sem o sorteio',
     'cps_6 falta pela soma'),
    ('cps6_sem_falta', '            falta(cps6_d(0xad9c6c), p); // cps6: a falta',
     '            let _ = (falta, p); // mutacao: sem a falta',
     'cps_6 falta ao treino'),
]
