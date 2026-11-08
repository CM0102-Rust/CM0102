# A pessoa liberada sem custo, as ferias do tecnico humano e a noticia da permissao para discutir o contrato
# (transfer_manager.cpp, human_manager.cpp).
# - 0x8c1af0 (free_transfer_staff, 7 chamadas ate 2002-06-01 na cobertura de 17/10; "cps_3 released" e
#   "cps_3 compensation" 8 na base): a transferencia passada, a compensacao paga ao clube, as listas de observacao, as
#   escalacoes e o elenco do clube atual, as ofertas e o contrato, as clausulas e a permissao de trabalho.
# - 0x5fd2f0 (set_on_holiday, 1 chamada: a carreira automatica de ferias): a marca de ferias e a conta dos humanos.
# - 0x8d7cd0 (post_contract_talks_news, 7 chamadas): a noticia 0xfac.
# Convencoes conferidas nos pontos de chamada da original (licao da 258a):
#   0x4cf6d0 ficha_de_contrato / 0x4cf5a0 contrato_de: thiscall (mov ecx, [0xae1050]), 1 arg. 0x944e46 malloc /
#   0x945667 realloc: cdecl (add esp 4/8), pelo cpl3_crescer (caixa pelo cpl3_sem_memoria, linha 0xce9). 0x417c10
#   Date::operator=: thiscall (lea ecx, destino), 2 args (resultado, origem). 0x7c5fe0 historico: thiscall (mov ecx,
#   [0xb74350]), 6 args. 0x85fa00 dias ate o fim: thiscall (mov ecx, edi = contrato), 0 args, devolve ax. 0x5a0590
#   financas: thiscall (mov ecx, [0xae2a38]), 1 arg. 0x599920 pagar: thiscall (mov ecx, eax = financas), 1 arg (o push
#   edx vem antes da 0x5a0590). 0x843e10 listas: thiscall (mov ecx, [ebp+0x856]), 4 args. 0x896c60 escalacao: thiscall
#   (mov ecx, [0xde1d08]), 4 args. 0x53c190 elenco: cdecl (add esp, 0x14), 5 args. 0x4cca70 ofertas de contrato:
#   thiscall (mov ecx, [0xae1050]), 2 args. 0x4d0b50 contrato principal: thiscall (mov ecx, [0xae1050]), 1 arg.
#   0x8cb4f0 / 0x8cb590 / 0x8cb630: thiscall (mov ecx, ebp = this), 1 arg. 0x8c7e20 permissao: thiscall (mov ecx,
#   ebp), 2 args. 0x4196a0: thiscall (mov ecx, [0xad9c6c]), 1 arg. 0x7874b0 responder: thiscall (mov ecx, [0xb7431c]),
#   2 args. 0x6b5040 interesse do dia: thiscall (mov ecx, [0xb67a50]), 0 args. 0x944cff / 0x90d130 / 0x944c9f /
#   0x5e8290: a caixa "Error" pelo aviso_em (linhas 0x41c e 0x428 do human_manager.cpp 0x9cf11c). 0x7875e0 humano: a
#   original poe ecx = [0xb7431c] e empurra 1 arg; a rotina e stdcall (ret 4) e nao le ecx: declarada thiscall, mesma
#   pilha. 0x77bbe0 construir / 0x785790 gravar: thiscall (lea ecx, local), 2 args. 0x7861e0 entregar: thiscall (mov
#   ecx, [0xb7431c]), 2 args. 0x68d1b0 proximo: thiscall (mov ecx, [esi+4] = a troca), 0 args.
# Comparadores: a liberacao so a original, pelo plano (a transferencia passada, o contrato tirado, +0x52 e +0x6d); as
# ferias so a original, pelo plano (a marca e a conta); a noticia so a original (aloca). Iguais no cm0102-1080.exe.
SAVE = False
ROTINAS = [('liberar_sem_custo', 0x8c1af0, [0x81, 0xec, 0x08, 0x02, 0x00, 0x00]),
           ('ferias_do_humano', 0x5fd2f0, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('noticia_conversa_de_contrato', 0x8d7cd0, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00])]
DOCS = ('a pessoa liberada sem custo, as férias do técnico humano e a notícia da permissão para discutir o contrato',
        'the person released on a free transfer, the human manager\'s holiday and the news of permission to discuss a contract',
        [('liberar_sem_custo', 'free_transfer_staff', '0x8c1af0', 'transfer_manager.cpp', 'thiscall',
          'registra a transferência livre e, fora do só registro, paga a compensação ao clube, tira a pessoa das listas, das escalações, do elenco, das ofertas e do contrato, das cláusulas e da permissão de trabalho',
          'records the free transfer and, unless record only, pays the club\'s compensation, removes the person from shortlists, selections, squad, offers and contract, clauses and work permit'),
         ('ferias_do_humano', 'set_on_holiday', '0x5fd2f0', 'human_manager.cpp', 'thiscall',
          'liga ou desliga as férias do técnico humano, com a conta dos humanos de férias; ao ligar, responde as notícias pendentes',
          'turns the human manager\'s holiday on or off, keeping the count of humans on holiday; when turning on, answers pending news'),
         ('noticia_conversa_de_contrato', 'post_contract_talks_news', '0x8d7cd0', 'transfer_manager.cpp', 'thiscall',
          'a notícia 0xfac da permissão para discutir o contrato ao comprador humano e, por jogador da troca, ao vendedor humano',
          'the 0xfac news of permission to discuss a contract to the human buyer and, per exchange player, to the human seller')])
CAMPOS = [('gerência de transferências +0x862 transferências passadas (0x25 bytes: +0 pessoa, +4..+0xc chaves, +0x10, +0x14 clube, +0x18, +0x1c tipo, +0x1d data), +0x866 capacidade, +0x86a conta; ponteiro/valor; grava (liberar_sem_custo)',
           'transfer manager +0x862 past transfers (0x25 bytes: +0 person, +4..+0xc keys, +0x10, +0x14 club, +0x18, +0x1c type, +0x1d date), +0x866 capacity, +0x86a count; pointer/value; writes (liberar_sem_custo)'),
          ('pessoa +0x39 clube atual, +0x3e data, +0x52 valor, +0x6d, +0x61 ficha; ponteiro/valor; lê e grava (liberar_sem_custo)',
           'person +0x39 current club, +0x3e date, +0x52 value, +0x6d, +0x61 player record; pointer/value; reads and writes (liberar_sem_custo)'),
          ('contrato +0x35 & 0x3f tipo, +0xc salário, +4 clube; valor; lê', 'contract +0x35 & 0x3f type, +0xc wage, +4 club; value; reads'),
          ('humanos (0xa6ef bytes em [this]) +0x9a01 férias; [this+4] quantos, [this+0xc] de férias; valor; grava (ferias_do_humano)',
           'humans (0xa6ef bytes at [this]) +0x9a01 holiday; [this+4] count, [this+0xc] on holiday; value; writes (ferias_do_humano)'),
          ('[0xb74c26 + 0x300 × [0xb77c7a]] técnico humano da vez; ponteiro; lê', '[0xb74c26 + 0x300 × [0xb77c7a]] current human manager; pointer; reads'),
          ('oferta +4 troca, +8 comprador, +0xc pessoa, +0x20, +0x24; valor; lê (noticia_conversa_de_contrato)', 'offer +4 exchange, +8 buyer, +0xc person, +0x20, +0x24; value; reads (noticia_conversa_de_contrato)'),
          ('[0xae1080] último contrato consultado; ponteiro; grava', '[0xae1080] last contract looked up; pointer; writes')]
INTERESSA = ['CMRTE', 'SGE']
