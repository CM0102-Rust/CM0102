# Mutacoes (sx15_4). O texto nao volta para a simulacao: acusadas pela comparacao (compare_text_writer compara o texto
# e os vinculos), como as de texto aceitas em 12/11/2026.
# - a manchete de varios jogadores com o modelo da segunda selecao trocado pelo da primeira (e vice-versa);
# - o vinculo do nome da selecao no corpo sempre como segunda selecao (0x7d6): roda em todo corpo com adversarios.
MUTACOES = [
    ('sx15_4_manchete_jogadores',
     'let model = if special { 0xa5b51c } else if second == 0 { 0xa5b5f0 } else { 0xa5b588 }; // sx15_4: the players headline',
     'let model = if special { 0xa5b51c } else if second == 0 { 0xa5b588 } else { 0xa5b5f0 }; // mutation: the headlines swapped',
     'sx15_4 call-up headline'),
    ('sx15_4_vinculo_selecao',
     '    exe::text_add_link(out, team_name.as_ptr(), kind, team);',
     '    exe::text_add_link(out, team_name.as_ptr(), 0x7d6, team); // mutation: always the second team',
     'sx15_4 call-up body'),
]
