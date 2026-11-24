# A gravacao do hall_of_fame.bin (hall_of_fame.cpp; leitor sx12b, rodada 5, 23/11/2026).
# HallOfFame::save 0x5f00c0 roda no fim de save_game_data 0x82e430 (0x82f5a6: cada jogo salvo), na tela do hall da fama
# (0x825cc0, depois de um erro de importacao) e no construtor 0x5ef0b0 quando a carga falha (sem o arquivo).
# Convencoes conferidas na desmontagem: thiscall(this), ret 0, eax 1/0. fopen 0x945b00 cdecl("hall_of_fame.bin", "wb");
# sem arquivo: langlib_sprintf 0x66f4e0 de "Error saving to disk" (0x9ce4e4) e de 0x9ce464, gui_show_message_box
# 0x5e8290 cdecl(titulo, texto, 0) e 0 (sem a marca do mouse). fwrite 0x945ce6 cdecl(buffer, tamanho, quantidade,
# arquivo); o cabecalho e o dword 0x3abfb2 e o byte 1 num local (5 bytes). this nulo: as caixas 0x375 e 0x29c. A soma e
# dos bytes antes do ou-exclusivo; i mod 480 sem sinal (div); a troca por chave mod 0x52080 com sinal (cdq/idiv), de i
# = 0 para cima (a inversa do hall_of_fame_unscramble_and_check 0x5f0680, chamado pelo exe no fim). fclose 0x945a09.
# Tipo SAVE (gravacao, sem CARGA): roda a cada jogo salvo. A prova do save compara o .qst e nao ve este arquivo: a prova
# desta rotina e o comparador (o arquivo escrito pela original e reescrito pela copia, comparados byte a byte, e a tabela
# em memoria), quando o jogo e salvo comparando.
# Defeito do original: com erro ao gravar a tabela ou a soma, a tabela fica embaralhada na memoria.
SAVE = True
ROTINAS = [('hall_of_fame_save', 0x5f00c0, [0x81, 0xec, 0x68, 0x03, 0x00, 0x00])]
DOCS = ('a gravação do hall da fama',
        'saving the hall of fame',
        [('hall_of_fame_save', 'HallOfFame::save', '0x5f00c0', 'hall_of_fame.cpp', 'thiscall',
          'grava o hall da fama no hall_of_fame.bin: o cabeçalho (assinatura 0x3abfb2 e versão 1), as 28 tabelas embaralhadas (ou-exclusivo com a chave de 480 números e trocas de bytes pelas posições da chave) e a soma dos bytes; depois desembaralha e confere de novo; sem o arquivo, a caixa "Error saving to disk"; cada erro de gravação mostra a caixa, fecha o arquivo e devolve 0; senão 1',
          'writes the hall of fame to hall_of_fame.bin: the header (magic 0x3abfb2 and version 1), the 28 tables scrambled (XOR with the 480-number key and byte swaps by the key positions) and the byte sum; then unscrambles and checks again; with no file, the "Error saving to disk" box; each write error shows the box, closes the file and returns 0; otherwise 1')])
