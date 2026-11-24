# O motivo de uma multa em texto (fine_reason_text 0x5a73a0, Fine.cpp, 233 bytes ate o ret 0x10 de 0x5a7488, mais as
# duas tabelas do salto em 0x5a748c e 0x5a749c (4 dwords cada), ate 0x5a74ac; leitor sx09b, rodada 5, 23/11/2026).
# Chamada pela tela de disciplina do jogador (draw_discipline_player_screen 0x5a83e0, ja Rust, de 0x5a8839: os cinco
# motivos, em maiusculas, saida 0xde1f64 de 0x7d0 bytes) e pelo texto da noticia da reacao a multa (0x5a76e0, original,
# de 0x5a77de).
# Convencoes conferidas na desmontagem: thiscall(Fine, motivo byte, saida, tamanho word, minusculas byte), ret 0x10; o
# ecx nao e lido; saida nula: nada; tamanho (word com sinal) < 0x32: so o 0 no primeiro byte; os textos por 0x66f4e0
# langlib_sprintf cdecl(saida, modelo); o motivo 1..4 pela tabela, o resto "No Reason"/"no reason".
# Prologo: mov eax,[esp+8]; test eax,eax (6 bytes, sem salto).
# Comparador: lado a lado sobre a saida.
# Como chegar no jogo (para o Lucas provar): o perfil de um jogador do proprio clube, menu da acao "Discipline"
# (Disciplina): a tela lista os cinco motivos da multa.
TELAS = True
SAVE = False
ROTINAS = [('fine_reason_text', 0x5a73a0, [0x8b, 0x44, 0x24, 0x08, 0x85, 0xc0])]
DOCS = ('o motivo de uma multa em texto',
        'the reason of a fine as text',
        [('fine_reason_text', 'fine_reason_text', '0x5a73a0', 'Fine.cpp', 'thiscall, ret 0x10',
          'escreve o motivo da multa: 1 comportamento antiprofissional, 2 expulsao no jogo anterior, 3 comportamento violento, 4 mau desempenho, outro "sem motivo"; em minusculas quando pedido; saida com menos de 50 bytes: so a cadeia vazia',
          'writes the reason of the fine: 1 unprofessional behaviour, 2 dismissal in the prior match, 3 violent behaviour, 4 poor performance, any other "no reason"; in lower case when asked; an output under 50 bytes: only the empty string')])
