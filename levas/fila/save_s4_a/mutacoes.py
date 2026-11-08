# Mutacoes pela prova do save com a carga (levas/N/CARGA: o FeriasTesteSave carregado ate 2001-08-28 em modo
# rust; a gravacao do .qst nao passa pela carga, so a carga acusa). Medidas numa copia em 23/10/2026: a hora
# do dia lida no lugar da ultima hora (a hora fica a de antes do -load): impressao diferente; a data do jogo
# sem copiar (fica a data padrao) e o 0x7e61c0 depois do step_towards 0x22 pulado: o jogo cai na carga.
# Escaparam (equivalentes neste save, fora da lista): as ligas de frente lidas por cima das marcadas, a ultima
# data lida na data do jogo, a nacao padrao + 1, o 0x2710 do Match_Stats trocado por 0x10.
MUTACOES = [
    ('sv4_hora_do_dia', "        (0xae2c88, 1, 0x23ef, 0xa815d0),", "        (0xae2cfc, 1, 0x23ef, 0xa815d0),"),
    ('sv4_data_do_jogo', "                    copia_data(0xae2c90, tmp.as_mut_ptr(), data_do_jogo.as_ptr() as usize);", "                    let _ = (&mut tmp, &data_do_jogo);"),
    ('sv4_iniciar_0x22', "                    iniciar(o);", "                    let _ = (iniciar, o);"),
]
