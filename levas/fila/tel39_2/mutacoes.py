# Mutacoes (tel39_2). A carreira automatica nao abre tela: a cadeia nao roda estas; provam os comparadores numa
# sessao do Lucas (JOGAR_COMPARANDO.bat):
# - a opcao da foto fixa gravada sem inverter (Opcoes do jogo > mudar a opcao das fotos de fundo e confirmar; tambem
#   na partida do programa, ao ler o game.cfg);
# - o registro dos filtros da busca com 0x80 no primeiro dword (abrir a busca de jogadores: o PLAYER_SEARCH e
#   construido).
MUTACOES = [
    ('tel39_2_foto_fixa',
     'write_u32(BACKGROUND_FIXED_FLAG, (on == 0) as u32);',
     'write_u32(BACKGROUND_FIXED_FLAG, (on != 0) as u32);'),
    ('tel39_2_filtro',
     'write_u32(this, 0x81);',
     'write_u32(this, 0x80);'),
]
