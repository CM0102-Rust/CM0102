# Mutações (cqi_1), na classificação da UEFA feita no começo da carreira (0x9035a0 na virada do ano do jogo; com as 8
# ligas até 2002-06-01 as vagas decidem quem joga a Liga dos Campeões, a Copa da UEFA e a Intertoto de 2001/02):
# - sem a temporada mais antiga na soma do coeficiente: a ordem das nações muda e com ela as vagas (contador "cqi_1
#   soma dos coeficientes");
# - a França com 3 vagas na Intertoto de 2001 em vez de 4: um clube francês a menos na Intertoto (contador "cqi_1
#   intertoto da franca"; as vagas +0xc e +0xd da Liga dos Campeões e da Copa da UEFA não são lidas pela distribuição,
#   que usa faixas fixas pela posição: mutar nelas escaparia);
# - a busca do n-ésimo clube sem pular os que já têm vaga: as vagas da Copa da UEFA e da Intertoto caem em clubes que já
#   estão na Liga dos Campeões e se perdem (contador "cqi_1 already with a place").
MUTACOES = [
    ('cqi1_soma_sem_a_mais_antiga',
     'let soma = cqi1_f(p + 0x100) + cqi1_f(p + 0xf8) + cqi1_f(p + 0xf0) + cqi1_f(p + 0xe8) + cqi1_f(p + 0xe0); // cqi1: as 5 temporadas',
     'let soma = cqi1_f(p + 0x100) + cqi1_f(p + 0xf8) + cqi1_f(p + 0xf0) + cqi1_f(p + 0xe8); // mutacao: sem a mais antiga',
     'cqi_1 coefficient sum'),
    ('cqi1_franca_tres_na_intertoto',
     '4 // cqi1: a França na Intertoto em 2001',
     '3 // mutacao: 3 vagas',
     'cqi_1 france intertoto'),
    ('cqi1_sem_pular_ja_com_vaga',
     'p += 12; // cqi1: pula o clube que já tem vaga',
     'if falta == 0 { achou = true; break; } p += 12; falta = falta.wrapping_sub(1); // mutacao: conta o que já tem vaga',
     'cqi_1 already with a place'),
]
