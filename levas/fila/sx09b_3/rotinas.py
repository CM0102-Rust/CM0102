# O texto do ambito de uma suspensao (Discipline::get_scope_text 0x55ce40, discipline.cpp, 547 bytes ate o ret 0x14 de
# 0x55d05d, mais as tabelas do salto em 0x55d060 (5 dwords) e 0x55d074 (41 bytes), ate 0x55d09c; leitor sx09b, rodada 5,
# 23/11/2026). Chamada pelo texto da noticia de suspensao (write_ban_news_text 0x5576e0, ja Rust, pelo endereco, 4 vezes
# por noticia), pelo da suspensao a mais (0x55b8b0, original) e pela describe_ban (0x55cc20, telas). Roda na carreira
# quando o texto da noticia de suspensao de um jogador do tecnico humano e pedido (desde ago/2001).
# Convencoes conferidas na desmontagem:
# - a rotina: thiscall(Discipline, &4 regras, saida, tamanho, "vale em todas", forma), ret 0x14; devolve a forma em al
#   (eax alto: lixo; os chamadores leem al);
# - tamanho < 0x46 (com sinal): caixa "Error" da discipline.cpp linha 0x1475 e 0xff;
# - os textos por 0x66f4e0 langlib_sprintf cdecl(saida, modelo): "global" 0x9c3478, "world-wide" 0x9c346c, "continental"
#   0x9c3460, "domestic" 0x9c3454;
# - o texto da propria regra: o metodo +0x10 da vtable da regra thiscall(regra, saida, tamanho, forma) (3 argumentos na
#   pilha; o retorno nao e lido);
# - a forma: pelo byte +4 da regra (a vaga) 1 -> 9, 3 -> 1, 0 -> 8 se o byte +0x20 da regra nao e 0xff, senao 0; as
#   outras pelo indice (+5): a tabela do salto da 1 -> 2, 2 -> 6, 35 -> 7, 41 -> 3, o resto 0xff.
# Prologo: mov eax,[esp+0xc]; sub esp,0x200 (10 bytes, sem salto).
# Defeito do original (registrado, a copia faz o mesmo): sem nenhuma regra (as quatro 0xff) a rotina chama o texto da
# regra -1 ([[this] - 4]); os chamadores nunca passam uma suspensao vazia.
# Comparador: lado a lado sobre a saida e a forma devolvida (o texto da regra pela vtable roda nos dois lados).
SAVE = False
ROTINAS = [('get_scope_text', 0x55ce40, [0x8b, 0x44, 0x24, 0x0c, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('o texto do ambito de uma suspensao',
        'the scope text of a ban',
        [('get_scope_text', 'Discipline::get_scope_text', '0x55ce40', 'discipline.cpp', 'thiscall, ret 0x14',
          'escreve por quais entidades a suspensao vale e devolve a forma gramatical do texto: com a vaga da FIFA, "global" se a suspensao vale em todas as competicoes (forma 0xff), "world-wide" quando as quatro vagas estao ocupadas, senao o texto da primeira regra presente; sem ela, "continental" com as vagas 0 a 2 ocupadas, "domestic" com 0 e 1 ocupadas e a 2 livre, senao o texto da primeira regra presente. A forma vem da regra que fala: pela vaga dela (1 -> 9, 3 -> 1, 0 -> 8 ou 0) ou pelo indice (1 -> 2, 2 -> 6, 35 -> 7, 41 -> 3, outros 0xff). Saida com menos de 70 bytes: a caixa de erro e 0xff',
          "writes which bodies the ban covers and returns the grammatical form of the text: with the FIFA slot, \"global\" if the ban is valid in every competition (form 0xff), \"world-wide\" when all four slots are taken, otherwise the first present rule's own text; without it, \"continental\" with slots 0 to 2 taken, \"domestic\" with 0 and 1 taken and 2 free, otherwise the first present rule's own text. The form comes from the speaking rule: by its slot (1 -> 9, 3 -> 1, 0 -> 8 or 0) or by its index (1 -> 2, 2 -> 6, 35 -> 7, 41 -> 3, others 0xff). An output under 70 bytes: the error box and 0xff")])
