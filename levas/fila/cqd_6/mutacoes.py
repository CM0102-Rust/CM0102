# Mutacoes (cqd_6), no dia do calendario (0x5aaf00, 1098 chamadas ate 2002-06-01, uma por dia) e na carga dos jogos do
# ano (0x5ac3b0, pelas trocas do bloco do outro ano: recarrega os jogos de 2002 que a carreira joga):
# - no dia 3 da semana, os jogos de reservas de amanha nao sao marcados (0x7e6380 com a data de amanha): menos jogos
#   de reservas, outros sorteios e outra forma dos jogadores;
# - os amistosos de ontem entre clubes sem tecnico humano ficam nas listas: as buscas de jogos numa janela de dias
#   (conflitos dos amistosos e da TV, 0x5acbe0) acham jogos a mais (contador "cqd_6 friendly discarded");
# - o jogo carregado entra na lista do dia seguinte: as partidas de 2002 mudam de data (ou nao sao jogadas).
# Os contadores ficam colados no efeito: depois de todas as condicoes de cada trecho.
MUTACOES = [
    ('cqd6_sem_reservas_de_amanha',
     'reservas(cqd6_d(0xb74434), pb); // cqd6: as reservas de amanhã',
     '// mutacao: sem as reservas de amanha',
     'cqd_6 tomorrow reserves'),
    ('cqd6_amistoso_fica',
     'tirar(lista, no); // cqd6: o amistoso de ontem sai da lista',
     '// mutacao: o amistoso de ontem fica na lista',
     'cqd_6 friendly discarded'),
    ('cqd6_jogo_no_dia_seguinte',
     'let dia = cqd6_w(j + 0x2a) as u16 as usize; // cqd6: o dia do jogo carregado',
     'let dia = { let x = cqd6_w(j + 0x2a) as u16 as usize; if x < 364 { x + 1 } else { x } }; // mutacao: o dia seguinte',
     'cqd_6 match loaded'),
]
