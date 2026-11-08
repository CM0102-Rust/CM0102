# Mutacoes (cps_5), nas duas rotinas diarias (0x5ce0b0 e o laco das pessoas da 0x5d6d10, fora dos dias multiplos de
# 50; o atrito com o tecnico quase nunca chega a queixa e fica sem mutacao). A parte do moral sem sorteio (o sorteador
# anda uma vez a menos por jogador de um clube em sete, todo dia: tudo o que sorteia depois muda). O primeiro desconto
# do tecnico sem sorteio (um sorteio a menos por pessoa que passa os tres filtros). O moral sobe dois em vez de um (o
# 0x544710 sorteia com outro limit e o moral muda: as partidas mudam).
MUTACOES = [
    ('cps5_moral_sem_sorteio', '                let r2 = cps5_sortear(m / 12); // cps5: a parte do moral',
     '                let r2 = m / 24; // mutacao: sem o sorteio',
     'cps_5 drawn morale'),
    ('cps5_desconto_sem_sorteio', '    if cps5_sortear(cps5_sb(p + 0x59)) == 0 { // cps5: o primeiro desconto',
     '    if cps5_sb(p + 0x59) < 10 { // mutacao: sem o sorteio',
     'cps_5 drawn manager'),
    ('cps5_moral_sobe_dois', '                        moral(p, 1, 1); // cps5: sobe o moral',
     '                        moral(p, 2, 1); // mutacao',
     'cps_5 morale rises'),
]
