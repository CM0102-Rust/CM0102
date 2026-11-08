# Mutacoes das qualidades e dos defeitos. As duas mudam QUANTOS sorteios a rotina faz: a das qualidades troca > por
# >= no primeiro sorteio de +0x39 (quem empata sorteia o de +0x3e); a dos defeitos poe os goleiros no bloco dos de
# linha (dois sorteios a mais por goleiro). A das qualidades a carreira ate 2002-06-01 deve acusar (a 0x546ab0 roda nas
# transferencias 0x8dbef0 e na midia 0x754a30, cobertas). A dos defeitos PODE ESCAPAR: os chamadores (midia 0x72c1f0 e
# 0x487c70) nao aparecem na cobertura da carreira; o comparando acusa se a midia a chamar.
MUTACOES = [
    ('dbs_forcas', '    if b(0x39) > random(50) && b(0x3e) > random(50) {',
     '    if b(0x39) >= random(50) && b(0x3e) > random(50) { // mutacao: >= no primeiro'),
    ('dbs_fraquezas', '    if b(0xf) < 20 { // os de linha',
     '    if b(0xf) <= 20 { // mutacao: os goleiros tambem'),
]
