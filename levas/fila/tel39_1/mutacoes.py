# Mutacoes (tel39_1). A carreira automatica nao abre tela: a cadeia nao roda estas; provam os comparadores numa
# sessao do Lucas (JOGAR_COMPARANDO.bat):
# - o menu do perfil lista 2 nomes a menos com historico tambem sem ele (perfil de um jogador de clube com mais de
#   26 no elenco, sem historico escrito: o botao de menu do canto);
# - Remover e Ok ficam ativos com o apelido vazio (perfil > menu > Apelido, sem apelido).
MUTACOES = [
    ('tel39_1_menu_nomes',
     'let max: i32 = if history { 0x1a } else { 0x1c };',
     'let max: i32 = if history { 0x1a } else { 0x1a };'),
    ('tel39_1_apelido_vazio',
     'let empty = read_u8(nickname) == 0;',
     'let empty = false;'),
]
