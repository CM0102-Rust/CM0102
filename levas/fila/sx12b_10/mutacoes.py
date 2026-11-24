# Mutacoes (sx12b_10). Os textos de lesao das telas do jogador nao rodam na carreira automatica: a prova e o Lucas
# jogando comparando (tela de comparar jogadores, aba proxima partida do clube); os comparadores acusam.
# - o tipo da lesao lido um byte adiante (outro nome de lesao);
# - um dia a menos no tempo de volta;
# - a vitoria escrita como empate.
MUTACOES = [
    ('sx12b_tipo_da_lesao',
     '    let kind = read_u8(read_usize(this + injury_fields::INJURY_ENTRIES) + slot as usize * 0x30); // sx12b_10: the injury type',
     '    let kind = read_u8(read_usize(this + injury_fields::INJURY_ENTRIES) + slot as usize * 0x30 + 1); // mutation: the next byte'),
    ('sx12b_um_dia_a_menos',
     '    format(total - elapsed, out, size, style) // sx12b_10: the days left',
     '    format(total - elapsed - 1, out, size, style) // mutation: a day less'),
    ('sx12b_vitoria_como_empate',
     '        (ga, gb, 0x9e9cfc) // "... win over ..."',
     '        (ga, gb, 0x9e9c10) // mutation: a draw'),
]
