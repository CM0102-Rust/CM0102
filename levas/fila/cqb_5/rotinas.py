# A lesão nova, a notícia dela e a ausência sem licença (injury.cpp, awol.cpp): Injury::add_injury (0x62d250, ~2,6 KB:
# o mapa dá 842 bytes, mas a rotina vai até a 0x62dc92; pelos efeitos físicos da partida 0x62c080, pelo dia das lesões
# 0x62b7b0, pelos registros iniciais 0x62cf70, pela recaída 0x62e500 e pelos surtos 0x62e9e0, todos já em Rust: roda
# a cada lesão da carreira, "lesao registrada" 16384 e "cpv_8 batida vira lesao" 1024 na base da 261ª),
# injury_post_injury_news (0x62dca0: a notícia 2000 da lesão, chamada só pela 0x62d250: passageira) e
# AWOL::is_player_awol (0x418f60: a marca de ausente sem licença, lida pela seleção, pelo treino, pelo preço pedido,
# pelo dia da comissão e pelas transferências, quase sempre sem a marca: passageira). Iguais no cm0102-1080.exe. A
# 0x62d250 sem SEH (quadro de 0x220 bytes só para as caixas de erro) e com x87 em dupla precisão (fild, fmul qword
# [0x967898] = 0,01, fimul, _ftol 0x945640); a 0x62dca0 com quadro SEH sem destrutor (a notícia local). A
# injury_get_type_name_gender (0x6306e0) ficou de fora: começa com call (E8) e não cabe o desvio.
# Convenções conferidas nos pontos de chamada da original (lição da 258ª):
# - quem chama a 0x62d250: 0x62bc89 (dia das lesões), 0x62c28a, 0x62d0df, 0x62e698, 0x62eb6e: oito push e mov ecx,
#   Injury -> thiscall(this, tipo, retroativa, pessoa, partida, surto, arg, recorrência, só_recorrência), ret 0x20.
# - quem chama a 0x62dca0: 0x62d8d3 push arg; push lesão; push pessoa; mov ecx, esi -> thiscall(this sem uso, pessoa,
#   lesão, arg), ret 0xc (a original troca o ecx por [0xde1f54] na entrada; o mapa a dá como stdcall).
# - quem chama a 0x418f60: 0x8c15f0, 0x8cf60d, 0x62b90e, 0x5d6b3b (push só_ler; push pessoa; mov ecx, [0xad9c6c])
#   -> thiscall(awol, pessoa, só_ler), ret 8.
# - dentro da 0x62d250: 0x62c680 push pessoa; mov ecx, esi -> thiscall(this, pessoa), ret 4; 0x8b1750 mov ecx,
#   [0xde1f44]; push 0; push pessoa -> thiscall(treino, pessoa, 0), ret 8; 0x90cfc0 push n; add esp, 4 -> cdecl(n);
#   0x58e340 push lesão; push reg+0x18; push reg+0x14; mov ecx, [esi+0xc] -> thiscall(arquivo, a, b, lesão), ret 0xc;
#   0x945667 push tam; push vetor; add esp, 8 -> cdecl(ptr, tam) (realloc); 0x944e46 push tam; add esp, 4 -> cdecl
#   (malloc); a caixa (aviso_em: 0x944cff, 0x90d130, 0x944c9f, 0x5e8290) com o fonte 0x9e7f28 e a linha 0x909;
#   0x7e4750 push lesão+1; push clube; push partida; add esp, 0xc -> cdecl(partida, clube, saída); 0x544410 push
#   byte; push np; add esp, 8 -> cdecl(np, nível); 0x600a50 push 0; push 0; push 1; push 0; push clube; add esp, 0x14
#   -> cdecl(clube, 0, 1, 0, 0); 0x5444a0 push 0; push pessoa; add esp, 8 -> cdecl(pessoa, 0); 0x842b00 mov ecx,
#   [[0xde1f54]+0x856]; push 0; push pessoa -> thiscall(listas, pessoa, 0), ret 8; 0x54c8e0 mov ecx, 0xae2c90; push n;
#   push saída -> thiscall(data, saída, n), ret 8; 0x417c10 push origem; push saída; lea ecx, lesão+0x20 ->
#   thiscall(destino, saída, origem), ret 8; 0x62dca0 como acima; 0x775cd0 mov ecx, [0xb74318]; push dias; push
#   pessoa -> thiscall(seleções, pessoa, dias), ret 8; 0x62e500 push lesão; push registro; mov ecx, esi ->
#   thiscall(this, registro, lesão), ret 8; 0x7a7f20 mov ecx, [0xb7434c]; push dias; push pessoa -> thiscall(regen,
#   pessoa, dias), ret 8; 0x62e820 push 0; push id do clube; push tipo (o dword inteiro do argumento); push 0xae2c90;
#   mov ecx, esi -> thiscall(this, data, tipo, clube, 0), ret 0x10.
# - dentro da 0x62dca0: 0x8d1da0 mov ecx, [0xde1f54]; push 0x1e; push vetor; push 0x5a; push pessoa ->
#   thiscall(gerência, pessoa, 0x5a, vetor, 0x1e), ret 0x10; 0x77bbe0 push 0; push 0x7d0; lea ecx, notícia ->
#   thiscall(notícia, tipo, 0), ret 8; 0x785790 push v; push k; lea ecx, notícia -> thiscall, ret 8; 0x7e4de0 push
#   &byte; push notícia; push lesão+1; add esp, 0xc -> cdecl(registro, notícia, &marca); 0x786780 mov ecx, [0xb7431c];
#   push notícia -> thiscall(news, notícia), ret 4; 0x7861e0 push clube; push notícia; mov ecx, [0xb7431c] ->
#   thiscall(news, notícia, clube), ret 8; 0x7875e0 mov ecx, [0xb7431c]; push clube -> thiscall(news, clube), ret 4;
#   0x4cf5a0 mov ecx, [0xae1050]; push pessoa -> thiscall(gerência, pessoa), ret 4.
# - dentro da 0x418f60: 0x4cf5a0 como acima; 0x4196a0 push pessoa; mov ecx, edi -> thiscall(this, pessoa), ret 4.
# Comparadores: a 0x62d250 só a original (aloca, entrega a notícia, agenda aposentadoria, e o treino 0x8b1750 e a
# recaída 0x62e500, originais ou em Rust, sorteiam no meio e a recaída pode lesionar de novo), conferindo depois o que
# a lesão garante: −1 nos argumentos inválidos e na só recorrência, −1 de resto só com a pessoa ainda lesionada (ou
# sem vetor); com a vaga, o tipo na vaga e em +0x1d, +0x1c = 0, +0x2c = +0x28, +0x2e = +0x2a e +0x2a = a volta ao
# treino calculada pela cópia a partir da duração (o x87); a 0x62dca0 só a original (notícia entregue); a 0x418f60
# lado a lado em [0xae1080] e no al (sem alocar: é quente), só a original quando a ausência acaba (a 0x4196a0 libera
# o nó).
SAVE = False
ROTINAS = [('lesao_nova', 0x62d250, [0x81, 0xec, 0x20, 0x02, 0x00, 0x00]),
           ('noticia_da_lesao', 0x62dca0, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('ausente_sem_licenca', 0x418f60, [0x56, 0x8b, 0x74, 0x24, 0x08])]
DOCS = ('a lesão nova (a vaga, a duração pelo fisioterapeuta, pela idade e pela propensão, a volta ao treino, a recaída e a aposentadoria), a notícia da lesão e a marca de ausente sem licença',
        'the new injury (the slot, the length by the physio, age and proneness, the return to training, the recurrence and the retirement), the injury news and the AWOL flag',
        [('lesao_nova', 'Injury::add_injury', '0x62d250', 'injury.cpp', 'thiscall, ret 0x20',
          'registra a lesão: troca a anterior só nos tipos 0x4d/0x4e/0x4f/0x51, acha ou cria a vaga, grava a partida, a duração (base + variáveis × (35 − melhor fisioterapeuta) / 20 com sorteios e os extras da idade e da propensão), a volta ao treino (percentual do tipo, até 60 dias antes do fim), a data, a notícia, a recorrência, a seleção, a recaída, a aposentadoria e o surto',
          'records the injury: replaces the previous one only for types 0x4d/0x4e/0x4f/0x51, finds or grows the slot, records the match, the length (base + variable × (35 − best physio) / 20 with draws and the age and proneness extras), the return to training (type percentage, at most 60 days before the end), the date, the news, the recurrence, the national team, the relapse, the retirement and the outbreak'),
         ('noticia_da_lesao', 'injury_post_injury_news', '0x62dca0', 'injury.cpp', 'thiscall (this sem uso), ret 0xc',
          'a notícia 2000 da lesão: a pessoa, o tipo, a data, os dias, o nascimento e a partida; de 100 dias, a todos com potencial acima de 8000 ou aos clubes interessados; até 100, ao clube humano, ao clube do contrato e à seleção',
          'injury news 2000: the person, the type, the date, the days, the birth date and the match; over 100 days, to everyone with potential above 8000 or to the interested clubs; up to 100, to the human club, the contract club and the national team'),
         ('ausente_sem_licenca', 'AWOL::is_player_awol', '0x418f60', 'awol.cpp', 'thiscall, ret 8',
          'o bit 0 da marca da pessoa; sem só_ler e sem contrato ou clube, a ausência acaba antes (0x4196a0)',
          "bit 0 of the person's flag; without read-only and without a contract or club, the absence ends first (0x4196a0)")])
CAMPOS = [('registro físico (31 bytes, [Injury+0]) +0xe / +0x12 / +0x13 / +0x14 / +0x18 / +0x1c / +0x1d: a lesão atual (vaga, −1 = nenhuma), o tipo e a conta da recorrência, o histórico, a marca e o último tipo; id e valor; lê e grava (lesao_nova)',
           'fitness record (31 bytes, [Injury+0]) +0xe / +0x12 / +0x13 / +0x14 / +0x18 / +0x1c / +0x1d: the current injury (slot, −1 = none), the recurrence type and count, the history, the flag and the last type; id and value; reads and writes (lesao_nova)'),
          ('lesão (0x30 bytes, [Injury+4], conta [Injury+8]) +0 tipo (0xff = vaga livre), +1 registro da partida (+0x1a = 0xff sem partida), +0x20 data, +0x28 dias, +0x2a volta ao treino, +0x2c/+0x2e os dois contadores; valor; grava (lesao_nova), lê (noticia_da_lesao)',
           'injury (0x30 bytes, [Injury+4], count [Injury+8]) +0 type (0xff = free slot), +1 match record (+0x1a = 0xff without a match), +0x20 date, +0x28 days, +0x2a return to training, +0x2c/+0x2e the two counters; value; writes (lesao_nova), reads (noticia_da_lesao)'),
          ('tabela dos tipos de lesão (13 bytes por tipo em 0x9e7adb): +0 dias de base, +2 dias variáveis, +4 percentual da volta ao treino, +5 pode voltar; valor; lê (lesao_nova)',
           'injury type table (13 bytes per type at 0x9e7adb): +0 base days, +2 variable days, +4 return-to-training percentage, +5 can recur; value; reads (lesao_nova)'),
          ('ficha do jogador (+0x61) +0x2d propensão a lesões, +0x34 forma natural, +0xd potencial; valor; lê (lesao_nova, noticia_da_lesao)',
           'player profile (+0x61) +0x2d injury proneness, +0x34 natural fitness, +0xd potential; value; reads (lesao_nova, noticia_da_lesao)'),
          ('registro de 79 bytes ([0xae2d0c]) +0xa: sobe com a lesão até 100; valor; lê e grava (lesao_nova)',
           '79-byte record ([0xae2d0c]) +0xa: rises with the injury up to 100; value; reads and writes (lesao_nova)'),
          ('clube +0x1cf: os três fisioterapeutas (com +0x69 a ficha de não jogador); ponteiro; lê (lesao_nova)',
           'club +0x1cf: the three physios (with +0x69 the non-player profile); pointer; reads (lesao_nova)'),
          ('lista de observação [[0xde1f54]+0x856] +0xdc + id do clube × 9 + 8: a marca 100 da lesão longa; valor; grava (lesao_nova)',
           'shortlist [[0xde1f54]+0x856] +0xdc + club id × 9 + 8: the long-injury mark 100; value; writes (lesao_nova)'),
          ('AWOL ([0xad9c6c]) +0xc + id: bit 0 ausente sem licença; valor; lê (ausente_sem_licenca)',
           'AWOL ([0xad9c6c]) +0xc + id: bit 0 absent without leave; value; reads (ausente_sem_licenca)'),
          ('[0xae1080]: o último contrato consultado; ponteiro; grava (noticia_da_lesao, ausente_sem_licenca)',
           '[0xae1080]: the last contract looked up; pointer; writes (noticia_da_lesao, ausente_sem_licenca)')]
INTERESSA = ['CMRTE', 'GPF2']
