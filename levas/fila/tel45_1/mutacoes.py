# O laco das telas e o do cliente de rede: a carreira automatica nao passa por eles, entao a cadeia nao roda mutacoes
# nesta leva. Estas provam os comparadores numa sessao do Lucas com o JOGAR_COMPARANDO.bat:
# - o pacote de saida do monitor com o progresso 1 no lugar de 0: qualquer tela que termine (Continuar; o comparador
#   do monitor refaz o pacote pela copia e acusa);
# - o pacote de saida do cliente com a palavra 0xfffe: so numa partida em rede, como cliente, ao sair.
MUTACOES = [
    ('tel45_1_progresso_um', '    status_message_packet(sm, GUI_SCRATCH_TEXT, 0, 0); // tel45_1: progress 0 of 0', '    status_message_packet(sm, GUI_SCRATCH_TEXT, 1, 0); // mutation: progress 1'),
    ('tel45_1_saida_palavra', '    scrman_net_put(sm, 0xffff, 2, 0x86); // tel45_1: all players', '    scrman_net_put(sm, 0xfffe, 2, 0x86); // mutation: another word'),
]
