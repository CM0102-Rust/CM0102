# Imprimir as linhas da lista de impressão (simulated_stats.cpp; leitor rx13, rodada 6, 25/11/2026). Item de TELAS (a
# rede: os comandos 21 e 22 do jogo em rede, Display 0x567833/0x56786c, e o command_simple do network.rs, que a chama
# por endereço). Caminho no jogo para o Lucas: num jogo em rede, mandar imprimir (a lista de impressão); abre a caixa de
# impressão do Windows (pode cancelar: cancelado, nada mais roda). Não dá para provar sem uma impressora ou o "Microsoft
# Print to PDF".
# - 0x84c830 print_text_lines stdcall(linhas, n, caracteres) ret 0xc (o ecx 0xdd7ee8 dos chamadores não é lido):
#   PRINTDLGA de 0x42 bytes na pilha (dono [0xb67a2c], Flags 0x100, páginas 1..1, 1 cópia; o resto 0); PrintDlgA (a
#   0x944c16 salta para [0x967260]) 0: fim. StartDocA([0x967040]) com DOCINFOA (0x14, 0xa827dc, 0, 0, 0) <= 0: fim. Sem
#   linhas: EndDoc. CreateFontA(0, GetDeviceCaps(HORZRES 8) / palavra com sinal, 0 x 11, "Courier" 0xa827d4); nula:
#   EndDoc. GetTextMetricsA; falha ou n <= 0: DeleteObject e EndDoc. Para cada linha: a primeira, uma nula antes da
#   última ou uma cujo fundo (y + tmHeight - GetDeviceCaps(0x71) - 1) alcança GetDeviceCaps(VERTRES 0xa) abre página
#   (EndPage da aberta, StartPage > 0 senão DeleteObject e EndDoc, SelectObject da fonte, y = GetDeviceCaps(0x71)); a
#   não nula: TextOutA(x = GetDeviceCaps(0x70), y, linha, strlen) e y += tmHeight. No fim EndPage da aberta,
#   DeleteObject, EndDoc. Todas as chamadas do Windows pela tabela de importação do jogo. DEFEITO: o DC da impressora e
#   os hDevMode/hDevNames da caixa nunca são liberados. Prólogo sub esp, 0x9c (6 bytes).
# Igual no cm0102-1080.exe. Comparador: só a original (imprime).
SAVE = False
TELAS = True
ROTINAS = [('print_text_lines', 0x84c830, [0x81, 0xec, 0x9c, 0x00, 0x00, 0x00])]
DOCS = ('imprimir as linhas da lista de impressão',
        "printing the print list's lines",
        [('print_text_lines', 'print_text_lines', '0x84c830', 'simulated_stats.cpp', 'stdcall (ecx não lido), ret 0xc',
          'regra: imprime uma lista de linhas de texto: a caixa de impressão do Windows, depois as linhas em Courier, com tantos caracteres na largura da página quanto pedidos, uma embaixo da outra a partir do topo imprimível; uma página nova começa quando a próxima linha passaria da altura da página, e uma linha nula (que não seja a última) começa uma página nova',
          'rule: printing a list of text lines: the Windows print dialog, then the lines in Courier, as many characters across the page as asked, one under the other from the printable top; a page is started when the next line would pass the page height, and a null line (not the last) starts a new page')])
CAMPOS = []
INTERESSA = []
