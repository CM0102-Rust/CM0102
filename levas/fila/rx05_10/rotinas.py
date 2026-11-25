# O texto do motivo de recusa de uma oferta (transfer_manager_get_rejection_reason_text 0x8c5690,
# transfer_manager.cpp, 368 bytes, 0x8c5690..0x8c57ee; leitor rx05, rodada 6, 25/11/2026). Igual no cm0102-1080.exe.
# Só a tela de recusa de oferta a chama (transfer_screens_build_reject_bid_dialog 0x8efda0, em Rust, pelo
# TS_FN_REJECTION_REASON_TEXT): item de tela.
# Caminho no jogo para o Lucas provar: com uma oferta recebida por um jogador do seu clube (caixa de entrada ou o
# perfil do jogador, "Ofertas"), escolher "Recusar": o diálogo lista os motivos (nenhum, indispensável, importante,
# e os que valem: promessa, mesma divisão, inaceitável, rival), cada um escrito por esta rotina.
# Convenções conferidas na desmontagem: thiscall(this, motivo, buffer, tamanho) ret 0xc, o this não é lido; o motivo
# comparado como word com sinal (movsx); tamanho < 0x32: a caixa "Error" do transfer_manager.cpp linha 0x163d e
# [0xb67a34] = 0 (show_error_box), sem escrever; os textos pelo langlib_sprintf (0x66f4e0, cdecl); motivo fora da
# lista não escreve nada.
# Comparador: lado a lado sobre o texto (a cópia num buffer local do mesmo tamanho, até 0x400); tamanho < 50 só a
# original (a caixa de erro).
TELAS = True
SAVE = False
ROTINAS = [('get_rejection_reason_text', 0x8c5690, [0x8b, 0x44, 0x24, 0x0c, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('o texto do motivo de recusa de uma oferta',
        'the text of a bid rejection reason',
        [('get_rejection_reason_text', 'transfer_manager_get_rejection_reason_text', '0x8c5690', 'transfer_manager.cpp', 'thiscall, ret 0xc',
          'escreve o texto do motivo (nenhum, inaceitável 0xa, indispensável 0x200, importante 0x400, rival 0x800, mesma divisão 0x1000, promessa 0x2000); outro motivo não escreve; com o buffer menor que 50 bytes, a caixa de erro e nada',
          'writes the reason text (none, unacceptable 0xa, indispensable 0x200, important 0x400, rival 0x800, same division 0x1000, prospect 0x2000); another reason writes nothing; with a buffer under 50 bytes, the error box and nothing')])
