# langlib_sprintf, o formatador de textos do jogo (Langlib.cpp; leitor rx04, rodada 6, 25/11/2026). Todo texto de
# notícia e de tela passa por ele (~5.500 chamadas no exe).
# - 0x66f4e0 langlib_sprintf (1.744 bytes; cdecl variádica: saída, formato, depois um dword por "{"/"[" do texto inglês
#   e os argumentos do vsprintf; devolve a saída depois dos espaços iniciais do formato). Lida instrução a instrução e
#   partida em pedaços com nome: os gêneros (0x66f5a2), os pedaços (0x66f612: vsprintf 0x9470ce e o corte nos "~"), o
#   modelo traduzido (0x66f681), o texto inglês limpo (0x66f7d9) e a 2a passada das marcas (0x66f96e). O quadro da 2a
#   passada fica na ordem da pilha da original (LanglibMarksFrame): uma marca no começo lê o byte antes do texto.
#   Chama a 0x66fbb0 (rx04_1), a 0x6701f0 langlib_apply_grammar_modifier (cdecl, 5 dwords; só lê o byte dos gêneros;
#   devolve 0xb64804, 0xad9c64 ou 0 — o 0 derruba a original, e a cópia do mesmo jeito), strstr 0x945a60 e o operator
#   delete das 3 strings da entrada.
# - As chamadas passam até 26 dwords (0x748782: add esp, 0x68): LanglibArgs tem 24 depois do formato; as 348 chamadas
#   que limpam a pilha só depois de outra chamada passam, pelo que se vê, no máximo 76 bytes [inferência].
# Iguais no cm0102-1080.exe (inteira). Prólogo: push -1; push 0x95c8db (7 bytes, sem relativo).
# Estado entre chamadas: nenhum (a linha 0xb6619c, os pedaços 0xb66174 e as marcas 0xb669c4/0xb671ec são refeitos a cada
# chamada; a única global da 0x6701f0 é a saída 0xb64804, zerada na entrada).
# Conferido no language.ldb (9.807 textos, 10 línguas): no máximo 9 "<" numa chave (o par 9, dentro dos 20 bytes), 9
# pedaços 0x08 por tradução, números de pedaço até 8, até 10 pedaços no formato.
# Defeito do original: marca sem "}" (17 traduções: ESP, ITA, HOL, NOR, SUE, ALE; nenhuma em português) faz a leitura
# seguir depois do fim do texto; a cópia lê igual, mas o que há depois da string é o do bloco dela.
# Comparador: lado a lado, a cópia antes num buffer próprio (um argumento pode ser a própria saída), depois a original
# na saída de quem chama; o texto, a posição devolvida e a linha, os pedaços e as marcas que cada uma deixa, iguais.
SAVE = False
ROTINAS = [('langlib_sprintf', 0x66f4e0, [0x6a, 0xff, 0x68, 0xdb, 0xc8, 0x95, 0x00])]
DOCS = ('o formatador de textos do jogo, com a tradução e as marcas de gênero',
        "the game's text formatter, with the translation and the gender marks",
        [('langlib_sprintf', 'langlib_sprintf', '0x66f4e0', 'Langlib.cpp', 'cdecl, variádica',
          'regra: escreve uma mensagem como o sprintf, traduzida: os espaços do começo passam iguais; com uma língua que não o inglês e uma tradução não vazia, os argumentos viram pedaços (o formato de argumentos da tradução, cortado nos "~") e cada 0x08 da tradução recebe o pedaço do seu número, passando pelo modificador de gramática quando há gênero ("{upper}" antes, "{s}" depois e os dois bytes de gênero que o chamador deu para cada "{" e "[" do texto inglês); senão, o texto inglês sem as notas dos tradutores ("<...>"), sem "{}" e "[]", formatado com os argumentos depois dos gêneros, e as marcas que sobraram ("palavra{s}", "{upper}palavra") aplicadas à palavra que tocam; "%%" vira "%"',
          'rule: writes a message like sprintf, translated: the leading spaces pass as they are; with a language other than English and a non-empty translation, the arguments become pieces (the translation\'s argument format, cut at the "~") and each 0x08 of the translation gets the piece of its number, through the grammar modifier when it has a gender ("{upper}" before, "{s}" after and the two gender bytes the caller gave for each "{" and "[" of the English text); otherwise the English text without the translators\' notes ("<...>"), without "{}" and "[]", formatted with the arguments after the genders, and the marks left ("word{s}", "{upper}word") applied to the word they touch; "%%" becomes "%"')])
INTERESSA = []
