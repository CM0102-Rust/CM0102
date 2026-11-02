# A comissão de treino do técnico humano (training_manager.cpp), pedido de 02/11 (agente C, família das pessoas):
# - 0x8b2880 (TRAINING_MANAGER::update_coach_assignments, 864 bytes, x87 no fim): as seis vagas da comissão do humano
#   (0x8b4cd0): tira quem saiu, põe o auxiliar e os treinadores novos com as cinco categorias e refaz a nota de cada
#   categoria marcada (a chance 0x544410 dos atributos da ficha de não jogador, dividida em float pela quantidade de
#   categorias, 1..20). 18 chamadas na carreira automática até 2002-06-01 (cobertura de 02/11, ENG, técnico de férias):
#   contratações e saídas de treinadores do clube humano. A nota vai para o fator do treinador (0x8b41d0, já em Rust),
#   que pesa o progresso do treino dos jogadores do clube humano (0x8b1750, "cqd_8 programa do humano"). As mutações
#   estão nela.
# - 0x8b2470 (TRAINING_MANAGER::reassign_squad_schedules, 1040 bytes): os programas dos dois elencos quando o técnico
#   humano troca de clube (2 chamadas na carreira: a escolha do clube no começo).
# - 0x8b2230 (os nomes padrão dos 167 programas, 32 bytes): passageira (1 chamada, na criação dos programas).
# Iguais no cm0102-1080.exe (conferido byte a byte, as três inteiras).
# Convenções conferidas nos pontos de chamada da original (lição da 258ª):
#   0x8b2880: thiscall (mov ecx, [0xde1f44]; push 0; push ebx em 0x8ca1c7; mov ecx, ebp; push; push em 0x8b2861), 2 na
#   pilha (ret 8: clube, técnico). Dentro: 0x600a50 cdecl (add esp, 0x14), 5 (clube, 1, 1, 0, 0); 0x53b900 cdecl (add
#   esp, 4), 1; 0x8b4cd0 thiscall (mov ecx, ebx = this), 1 (ret 4); 0x53cb80 cdecl, 1, e 0x53cc30 cdecl, 2 (os três
#   dwords saem juntos no add esp, 0xc); 0x544410 cdecl (add esp, 8), 2 (a ficha e o byte do atributo empurrado num
#   registrador com lixo em cima: a chamada lê o byte); 0x945640 _ftol. A tabela de saltos 0x8b2bc0 (categoria 0 ->
#   0x8b2b31, 1 -> 0x8b2add, 2 -> 0x8b2aac, 3 -> 0x8b2b14, 4 -> 0x8b2b51) conferida.
#   0x8b2470: thiscall (mov ecx, [0xde1f44]; push ebx; push edi; push esi em 0x690aec), 3 (ret 0xc: técnico, antigo,
#   novo). Dentro: 0x541f90 cdecl (add esp, 0xc), 3; 0x62c660 thiscall (mov ecx, [0xb63d28]), 2 (pessoa, carga);
#   0x8b4f60 thiscall (mov ecx, eax = o programa), 0; 0x8b2880 thiscall (mov ecx, ebp), 2 (o clube novo, o técnico);
#   a caixa de erro (0x944cff, 0x90d130, 0x944c9f, 0x5e8290: aviso_em).
#   0x8b2230: thiscall (mov ecx, esi em 0x8b41b3, sem push; ret). Dentro: 0x8b50f0 thiscall (mov ecx, esi), 0.
# Leitura de pilha: o divisor da 0x8b2880 é lido em [esp + quantidade × 4 + 0x18], uma casa abaixo da tabela de floats
#   montada no começo; a quantidade é de 1 a 5 sempre que a conta roda (a categoria marcada entra na contagem), então
#   a casa 0 (sem valor) nunca é lida.
# Comparadores: os três lado a lado. 0x8b2880 na comissão (54 bytes) e na marca +0x244 do clube; 0x8b2470 nas
# entradas do treino e nas cargas de forma dos dois elencos, na comissão e nas marcas dos clubes (só a original com a
# caixa); 0x8b2230 nos 167 programas. Sem sorteio.
SAVE = False
ROTINAS = [('distribuir_os_treinadores', 0x8b2880, [0x83, 0xec, 0x34, 0x53, 0x56]),
           ('trocar_os_programas_do_elenco', 0x8b2470, [0x81, 0xec, 0xd0, 0x04, 0x00, 0x00]),
           ('nomes_padrao_dos_programas', 0x8b2230, [0x56, 0x57, 0x8b, 0xf1, 0xbf, 0xa7, 0x00, 0x00, 0x00])]
DOCS = ('a comissão de treino do técnico humano: as notas dos treinadores por categoria, os programas do elenco na troca de clube e os nomes padrão dos programas',
        'the human manager training staff: the coach ratings per category, the squad schedules on a club change and the default schedule names',
        [('distribuir_os_treinadores', 'TRAINING_MANAGER::update_coach_assignments', '0x8b2880', 'training_manager.cpp', 'thiscall',
          'as seis vagas da comissão de treino do técnico humano: tira quem saiu do clube, põe o auxiliar e os treinadores novos com as cinco categorias e refaz a nota de cada categoria (a chance pelos atributos de treino, dividida pela quantidade de categorias do treinador, 1 a 20)',
          'the six training staff slots of the human manager: drops who left the club, adds the new assistant and coaches with all five categories and recomputes each category rating (the chance from the coaching attributes, divided by the coach category count, 1 to 20)'),
         ('trocar_os_programas_do_elenco', 'TRAINING_MANAGER::reassign_squad_schedules', '0x8b2470', 'training_manager.cpp', 'thiscall',
          'na troca de clube do técnico humano: o elenco antigo volta aos programas padrão (goleiro ou geral) com carga 50; o novo vai para o programa de goleiros ou o geral do técnico, com a carga do programa; e a comissão do clube novo',
          'on the human manager club change: the old squad goes back to the default schedules (goalkeeper or general) with load 50; the new one goes to the manager goalkeeper or general schedule, with the schedule load; and the new club training staff'),
         ('nomes_padrao_dos_programas', '(training_manager.cpp, 0x8b2230)', '0x8b2230', 'training_manager.cpp', 'thiscall',
          'os nomes padrão dos 167 programas de treino',
          'the default names of the 167 training schedules')])
CAMPOS = [('treino (TRAINING_MANAGER [0xde1f44]) +0x652d + 54 × ([0xae2364] − id − 1) a comissão do humano: seis vagas de 9 bytes (+0 id do treinador, +4..+8 nota por categoria, 0 = não treina); id/valor; lê e grava (distribuir_os_treinadores)',
           'training (TRAINING_MANAGER [0xde1f44]) +0x652d + 54 × ([0xae2364] − id − 1) the human training staff: six 9-byte slots (+0 coach id, +4..+8 rating per category, 0 = not coaching); id/value; reads and writes (distribuir_os_treinadores)'),
          ('ficha de não jogador (pessoa +0x69) +0xe, +0x10, +0x11, +0x12, +0x21 atributos de treinador [inferência: os que a chance usa por categoria]; valor; lê',
           'non-player record (person +0x69) +0xe, +0x10, +0x11, +0x12, +0x21 coaching attributes [inference: the ones the chance uses per category]; value; reads'),
          ('treino +0x651d entrada de 6 bytes por ficha de jogador, +0 a vaga do programa; valor; grava (trocar_os_programas_do_elenco); + 0x9b × (vaga + 7 + 10 × humano) os programas do humano, +0x15 tipo (1 geral, 6 goleiros); valor; lê',
           'training +0x651d 6-byte entry per player record, +0 the schedule slot; value; writes (trocar_os_programas_do_elenco); + 0x9b × (slot + 7 + 10 × human) the human schedules, +0x15 type (1 general, 6 goalkeepers); value; reads'),
          ('ficha do jogador (pessoa +0x61) +0xf goleiro; valor; lê; [[0xb63d28]] + 31 × id + 0x1e carga de treino; valor; grava',
           'player record (person +0x61) +0xf goalkeeper; value; reads; [[0xb63d28]] + 31 × id + 0x1e training load; value; writes'),
          ('clube +0xcf técnico, +0x244 marca do parceiro (zerada pela 0x540a50); ponteiro/valor; lê',
           'club +0xcf manager, +0x244 partner flag (cleared by 0x540a50); pointer/value; reads')]
INTERESSA = ['CMRTE', 'SGE']
