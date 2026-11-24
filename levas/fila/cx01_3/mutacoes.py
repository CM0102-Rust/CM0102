# Mutacoes (cx01_3), carreira ENG ate 2002-06-01. Nenhuma abre caixa.
# - o progresso com a melhor e a pior posicao trocadas: o comparador acusa (toda partida de liga e copa);
# - o texto do acesso negado (tipo 2) some: a comparacao com CM_TEXTOS=1 acusa, se a noticia sair ate a data;
# - o envio com o tipo trocado (2 por 1): so a original roda na comparacao; a carreira Rust so muda o texto da noticia,
#   que nao volta a simulacao: pode escapar (a rotina so roda no fim da temporada inglesa, com reserva ou estadio
#   barrado).
MUTACOES = [
    ('cx01_3_melhor_pior_trocadas',
     '    entry.set_best_position(best as u8);\n    entry.set_worst_position(worst as u8);',
     '    entry.set_best_position(worst as u8); // mutation\n    entry.set_worst_position(best as u8);'),
    ('cx01_3_texto_tipo2_some',
     '    if kind != 1 && kind != 2 {\n        return;\n    }',
     '    if kind != 1 { // mutation\n        return;\n    }'),
    ('cx01_3_envio_tipo_trocado',
     '    exe::newsitem_set_slot(p, 4, kind as u8 as i8 as i32 as u32);',
     '    exe::newsitem_set_slot(p, 4, (3 - kind as u8 as i8 as i32) as u32); // mutation'),
]
