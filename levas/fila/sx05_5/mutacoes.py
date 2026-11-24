# Mutacoes (sx05_5). O texto nao volta para a simulacao: acusadas pelo comparador (compare_text_writer), como as de
# texto aceitas em 12/11/2026, nos corpos que a carreira ENG escreve (treino perdido e forma fisica dos outros clubes).
# - o treino perdido sem o vinculo do tecnico;
# - a forma fisica com o tecnico com o nome do clube e o do jogador trocados;
# - o AWOL "volta ao clube" com o clube e o jogador trocados.
MUTACOES = [
    ('sx05_5_treino_sem_tecnico',
     '        awol_link_person(out, &coach_name, coach_ids, coach as u32);\n    }\n    let c = club_at',
     '        // mutation: no coach link\n    }\n    let c = club_at',
     'sx05_5 missed training body'),
    ('sx05_5_forma_trocada',
     'fmt(tp, 0x98963c, g, g, cn, cp, np);',
     'fmt(tp, 0x98963c, g, g, np, cp, cn); // mutation: club and player swapped',
     'sx05_5 fitness worry body'),
    ('sx05_5_awol_volta_trocado',
     '0 => fmt(buf, 0x9887bc, g, g, cn, np),',
     '0 => fmt(buf, 0x9887bc, g, g, np, cn), // mutation: swapped',
     'sx05_5 awol body'),
]
