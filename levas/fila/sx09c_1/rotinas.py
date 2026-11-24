# O gerente das multas: o passe diario, a construcao e a liberacao (Fine::daily_update 0x5a5820, Fine::Fine 0x5a4fc0,
# Fine::free_data 0x5a5120, Fine.cpp; leitor sx09c, rodada 5, 23/11/2026). As tres rotinas inteiras: 0x5a5820..0x5a592a
# (ret; o resto ate 0x5a5930 e enchimento), 0x5a4fc0..0x5a5116 (ret 4), 0x5a5120..0x5a5177 (ret).
# Chamadas: o passe diario pelo dia do jogo (0x5cdb05, game.cpp, ecx = [0xae2a3c]); o construtor pela criacao do jogo
# (0x829124, sem save) e pela carga (0x82c15d, com o save); o free_data pela limpeza do jogo (0x830114).
# Convencoes conferidas na desmontagem:
# - 0x5a5820 thiscall(fine), sem argumentos; 0x68d1b0 LLIST::get_next (ecx = lista); 0x456740 Date::operator>
#   thiscall(0xae2c90, dword data, dword bissexto) ret 8; 0x68d1f0 LLIST::remove thiscall(lista, dado) ret 4 (compara
#   o dado byte a byte com o tamanho de cada no); 0x54cdb0 Date::days_since thiscall(0xae2c90, dword, dword) ret 8;
#   0x62c400 Injury::get_player_injury_status thiscall([0xb63d28], pessoa) ret 4 (al); 0x5a7f20
#   post_coach_fine_feedback_news stdcall(primeiro, segundo, resultado) ret 0xc (ja Rust, fine.rs; chamada direta);
# - 0x5a4fc0 thiscall(fine, save) ret 4, devolve o objeto; quadro SEH (sem destrutor que mude o resultado); 0x5a5180
#   Fine::load thiscall(fine, save) ret 4 (al; sx09c_2, chamada pelo endereco: os itens vao em levas de tipos diferentes);
#   0x68d040 LLIST::~LLIST thiscall; 0x944c94 operator delete cdecl; 0x944cf1 operator new cdecl(0xe); 0x68d020
#   LLIST::LLIST thiscall; 0x945501 calloc cdecl(quantidade de clubes [0xae235c], 1);
# - 0x5a5120 thiscall(fine), sem argumentos; 0x9452ca free cdecl.
# O registro da multa (14 bytes, lista em +0): +0 id do jogador, +4 motivo, +5 aceita (resposta da is_fine_justified),
# +6 data em que a multa sai da lista. O do retorno do tecnico (17 bytes, lista em +4): +0 id do auxiliar tecnico do
# clube (club +0xd3) na hora da multa, +4 id do jogador multado, +8 resultado (1 ou 2), +9 data do retorno.
# Prologos sem salto: 0x5a5820 push ecx; push ebx; push esi; push edi; mov edi,ecx (6); 0x5a4fc0 mov eax,fs:[0];
# push -1 (8); 0x5a5120 push esi; mov esi,ecx; push edi; mov edi,[esi] (6).
# Defeito do original (registrado, a copia faz o mesmo): o retorno do tecnico sai da lista pelo LLIST::remove com o
# slot de pilha que ainda guarda a ultima multa lida no primeiro passeio (ja liberada quando era a vencida), ou o
# proprio objeto Fine quando a lista de multas esta vazia, em vez do registro do retorno: o registro fica e a noticia
# volta todo dia enquanto as condicoes valem. As condicoes (mais de 30 dias no clube, lesao) olham a pessoa de +0, o
# auxiliar tecnico, que e quem da o retorno (slot 0 da noticia: "<Coach Name> has indicated...").
# Comparadores: so a original (as tres liberam ou alocam); o passe diario confere depois o plano (a lista de multas
# perde um no so quando ha multa vencida; a de retornos fica igual quando nenhum vence).
SAVE = False
ROTINAS = [('fine_daily_update', 0x5a5820, [0x51, 0x53, 0x56, 0x57, 0x8b, 0xf9]),
           ('new_fine', 0x5a4fc0, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00, 0x6a, 0xff]),
           ('fine_free_data', 0x5a5120, [0x56, 0x8b, 0xf1, 0x57, 0x8b, 0x3e])]
DOCS = ('o gerente das multas: o passe diario, a construcao e a liberacao',
        'the Fine manager: the daily pass, its construction and its release',
        [('fine_daily_update', 'Fine::daily_update', '0x5a5820', 'Fine.cpp', 'thiscall',
          'todo dia: tira da lista a primeira multa vencida (data +6 ja passada; o passeio que fica sem no encerra tudo); depois, o primeiro retorno do tecnico vencido (data +9): se a pessoa de +0 (o auxiliar tecnico) esta no clube ha mais de 30 dias e a lesao dela nao e 2, posta a noticia do retorno (post_coach_fine_feedback_news com +0, o jogador de +4 e o resultado +8); e manda tirar da lista o no igual ao slot velho da pilha (defeito do original: o registro do retorno fica e a noticia volta todo dia)',
          'every day: removes the first expired fine from the list (date +6 past; a walk that runs out of nodes ends everything); then the first due coach feedback (date +9): when the person of +0 (the assistant manager) joined the club more than 30 days ago and his injury status is not 2, posts the feedback news (post_coach_fine_feedback_news with +0, the player of +4 and the outcome +8); and asks the list to remove the node equal to the stale stack slot (defect of the original: the feedback record stays and the news comes back every day)'),
         ('new_fine', 'Fine::Fine', '0x5a4fc0', 'Fine.cpp', 'thiscall, ret 4',
          'monta o gerente das multas: com o save, le o fine.dat (Fine::load) e, se der certo, mais nada; senao (ou sem save) as listas que a carga deixou sao destruidas e ficam duas listas novas vazias (multas e retornos do tecnico) e a tabela das multas por clube, um byte zerado por clube (calloc); devolve o objeto',
          'builds the Fine manager: with a save, reads fine.dat (Fine::load) and, when it succeeds, nothing else; otherwise (or without a save) the lists the load left are destroyed and two new empty lists (fines and coach feedback) and the per-club fine count table, one zeroed byte per club (calloc), are made; returns the object'),
         ('fine_free_data', 'Fine::free_data', '0x5a5120', 'Fine.cpp', 'thiscall',
          'libera as duas listas (destrutor e operator delete) e a tabela das multas por clube (free), zerando cada ponteiro',
          'frees both lists (destructor and operator delete) and the per-club fine count table (free), zeroing each pointer')])
