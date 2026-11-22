# A base da interface (5): a carreira automatica nao abre telas, entao a cadeia nao roda mutacoes nesta leva. Estas
# provam o comparador numa sessao do Lucas com o JOGAR_COMPARANDO.bat:
# - o retangulo de um widget com o subcomando errado (o pacote conferido; o set_widget_rect roda ao mover widgets);
# - o tocador "responde" ao contrario (is_sound_playing pelo valor; so com o som ligado, num jogo ao vivo).
MUTACOES = [
    ('tel38_5_retangulo_como_estilo',
     "scrman_widget_packet(sm, widget, 1); // tel38_5: the rectangle sub-command",
     "scrman_widget_packet(sm, widget, 2); // mutation: the style sub-command"),
    ('tel38_5_som_ao_contrario',
     "playing(player, slot) // tel38_5: the sound player's answer",
     "playing(player, slot) ^ 1 // mutation: the answer inverted"),
]
