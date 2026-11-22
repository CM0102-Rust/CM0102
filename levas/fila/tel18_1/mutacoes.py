# Mutacoes (tel18_1): provam os comparadores numa sessao do Lucas (JOGAR_COMPARANDO.bat):
# - a janela da comparacao troca o subtitulo do 1o pelo do 2o jogador (perfil de um jogador > Compare > 1st player);
# - a janela de retirar a proposta da ao Sim o id do Nao (Transfers > uma proposta sua > Withdraw).
MUTACOES = [
    ('tel18_1_comparacao_subtitulo',
     "ts_player_yes_no_dialog(0xaa0a48, 0xaa09dc, 2, 1);",
     "ts_player_yes_no_dialog(0xaa0a24, 0xaa09dc, 2, 1); // mutation: the 2nd player subtitle",
     'tel18_1 comparison prompt'),
    ('tel18_1_retirar_ids',
     "let (no, yes) = (1, 2);",
     "let (no, yes) = (1, 1); // mutation: Yes answers as No",
     'tel18_1 withdraw bid dialog'),
]
