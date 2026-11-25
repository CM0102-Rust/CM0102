# Uma linha do historico de uma pessoa na tela de historico (staff_records.cpp; leitor rx11b, rodada 6, 25/11/2026).
# Telas: chamada pela tela do historico da pessoa (0x885820, staff_screens.cpp, ja Rust) para cada linha.
# Caminho no jogo: o perfil de uma pessoa -> Historico (tecnico com contratacoes, vendas e emprestimos; jogador com
# premios, primeira convocacao, titulos, acessos e rebaixamentos). Os tipos 0x96/0x97 aparecem no historico de um
# tecnico (contratado, demitido) e 0x64..0x6a nas transferencias que ele fez.
# Convencoes conferidas na desmontagem: thiscall(no, Text) ret 8, this nao lido, quadro SEH de 0x4e8 bytes (o estado so
# marca o objeto Cash local); uma cadeia de if por tipo (+0xc), sem tabela de salto; 0x66f4e0 langlib_sprintf no
# buffer 0xde1f64 e 0x944c9f sprintf da CRT para as temporadas ("%d/%02d", "%d"); 0x4ae660 Text::set_string;
# 0x4ae8a0 Text::add_link thiscall(Text, s, tipo, alvo) ret 0xc; 0x53e140 cdecl(3 ids, buf, tamanho, 1, id); 0x53e990
# cdecl(pessoa, 3 ids, 0); 0x43f460 Cash::get_display_amount thiscall(&valor, 5) ret 4 -> edx:eax (o segundo 5
# empurrado fica na pilha como o ultimo argumento do 0x441c10 cash_format_money cdecl(buf, 0x64, valor64, 5));
# 0x53ad00 cdecl(cargo (byte; os 3 bytes de cima do dword sao lixo da pilha), 0, 0, &genero); 0x54d3b0 cdecl(mes);
# 0x4bfa30 cdecl(comp, fase, rodada, forma, buf, tamanho) e 0x4bff70 cdecl(comp, fase, rodada, forma) -> al (as
# palavras empurradas com os bytes de cima de ebx/esi do momento: as duas leem so a palavra); 0x90c310 cdecl(pos, -1)
# (o buffer do rotulo, empurrado antes, fica como argumento do sprintf).
# Comparador: lado a lado (compare_text_writer).
TELAS = True
ROTINAS = [('format_staff_history_entry', 0x86e0b0, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00])]
DOCS = ('uma linha do histórico de uma pessoa',
        "one line of a person's history",
        [('format_staff_history_entry', 'Staff_Records::format_staff_history_entry', '0x86e0b0', 'staff_records.cpp', 'thiscall, ret 8',
          'regra: pelo tipo da linha: contratado, demitido ou que se demitiu como técnico; técnico contratado ou demitido pelo clube; campeão, vice, terceiro de uma copa ou liga; prêmios do ano (a temporada "ano/ano") e do mês; jogador do ano da torcida; primeira convocação contra uma seleção e a idade; classificado, eliminado, não classificado numa fase; acesso, rebaixamento, a posição final numa liga; contratações e saídas (com valor, em troca, de graça, como outro cargo), empréstimos e dispensas; cada nome com vínculo',
          "rule: by the line's kind: hired, sacked or resigned as manager; a manager hired or sacked by the club; winners, runners up, third of a cup or league; awards of the year (the season \"year/year\") and of the month; supporters' player of the year; first international cap against a nation and the age; qualified, knocked out, failed to qualify in a round; promotion, relegation, the final position in a league; signings and departures (for a fee, in part exchange, free, as another job), loans and releases; each name with a link")])
CAMPOS = []
INTERESSA = []
