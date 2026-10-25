# Mutacoes das normalizacoes e do segundo nome: as normalizacoes rodam para todo jogador na criacao (segundo
# laco de database_player_initialisation, 0x538a62/0x538a68) e o segundo nome nos arbitros novos
# (OfficialsManager::set_up_officials); jogadores e arbitros vao para o .qst da prova do save.
MUTACOES = [
    ('dbi_normalizar_atributos_fisicos_do_jogador', '    let alvo = if s < -40 { -40 } else if s > 0 { 0 } else { s }; // alvo físico', '    let alvo = if s < -40 { -40 } else if s > 0 { 0 } else { s }; // alvo físico\n    let alvo = alvo - 1; // mutacao: o alvo do reequilíbrio físico − 1'),
    ('dbi_segundo_nome_pelo_peso', '            return linha(escolhido); // o segundo nome de maior peso', '            return linha(escolhido.wrapping_add(1)); // mutacao: o nome seguinte'),
    ('dbi_normalizar_atributos_do_jogador', '    reequilibrar_vetor_do_jogador(v, (lados as i8).wrapping_add(2), ponderados as i8, jogador, -20, &[],', '    reequilibrar_vetor_do_jogador(v, (lados as i8).wrapping_add(2), ponderados as i8, jogador, -21, &[], // mutacao: alvo −21'),
]
