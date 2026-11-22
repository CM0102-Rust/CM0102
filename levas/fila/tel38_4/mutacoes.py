# A base da interface (4): a carreira automatica nao abre telas, entao a cadeia nao roda mutacoes nesta leva. Estas
# provam o comparador numa sessao do Lucas com o JOGAR_COMPARANDO.bat (o plano do pacote acusa):
# - as marcas de um widget com o subcomando do estilo (set_widget_flags: telas que habilitam e desabilitam botoes, ex.
#   a tela de taticas);
# - o tamanho do texto sem o terminador no pacote (set_widget_text: qualquer tela que troca um texto na hora, ex. o relogio do
#   jogo ao vivo).
MUTACOES = [
    ('tel38_4_marcas_como_estilo', 'scrman_set_widget(sm, widget, 0, flags, 4, 0x8d, 0xa2e, false); // tel38_4: the flags sub-command', 'scrman_set_widget(sm, widget, 2, flags, 4, 0x8d, 0xa2e, false); // mutation: the style sub-command'),
    ('tel38_4_tamanho_sem_terminador', '    scrman_widget_packet(sm, widget, 8);\n    let len = c_string_len(text) + 1;\n    scrman_net_put(sm, len as u32, 2, 0x86);', '    scrman_widget_packet(sm, widget, 8);\n    let len = c_string_len(text) + 1;\n    scrman_net_put(sm, (len - 1) as u32, 2, 0x86);'),
]
