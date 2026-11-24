# Mutações (sx11_5): a leva das telas não roda mutação pela carreira. O comparador da game_init_from_command_line monta
# o plano com os auxiliares da cópia, então numa sessão do Lucas com JOGAR_COMPARANDO.bat (que passa "-windowed"):
# - o teste do "nomemcheck" consertado (a checagem fica ligada): o plano diverge da original na partida do programa.
# O turno dos humanos e o passo das partidas de fundo só rodam a original no comparador (telas e partidas): a cópia só
# se prova nas sessões sem comparação e nas carreiras Rust.
MUTACOES = [
    ('sx11_5_memcheck',
     '    read_u8(line) != 0 && exe::crt_strstr(line, game_start_fns::TEXT_NOMEMCHECK) == 0',
     '    read_u8(line) != 0 && exe::crt_strstr(line, game_start_fns::TEXT_NOMEMCHECK) != 0 // mutation'),
]
