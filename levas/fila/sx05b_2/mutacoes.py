# Mutacoes (sx05b_2). O texto nao volta a simulacao: acusadas pelo comparador (compare_text_writer); as datas da
# semana pelo comparador lado a lado e pela carreira (o periodo da equipe da semana).
# - o corpo de um jogador sem o periodo;
# - sem o vinculo da competicao;
# - a data de atualizacao copiada do inicio.
MUTACOES = [
    ('sx05b_2_um_sem_periodo',
     '            1 => fmt(buf, 0xa8cbcc, g, g, name(0), comp_name, pp),',
     '            1 => fmt(buf, 0xa8cbcc, g, g, name(0), comp_name, 0xad9c64usize), // mutation: no period',
     'sx05b_2 team award body'),
    ('sx05b_2_sem_competicao',
     '    exe::text_add_link(text, (comp + 4) as *const u8, 0x7d4, comp);\n}',
     '    let _ = comp; // mutation: no competition link\n}'),
    ('sx05b_2_atualizacao_do_inicio',
     '    exe::date_operator_3d(this + ta::UPDATE_DATE, o, update);',
     '    exe::date_operator_3d(this + ta::UPDATE_DATE, o, start); // mutation'),
]
