# A disciplina: a construcao, a liberacao e a suspensao por agressao ao arbitro (Discipline::Discipline 0x5554a0,
# Discipline::~Discipline 0x555790, Discipline::handle_referee_assault 0x55e300, discipline.cpp; leitor sx09c, rodada
# 5, 23/11/2026). As tres rotinas inteiras: 0x5554a0..0x555781 (ret 4), 0x555790..0x555852 (ret) e 0x55e300..0x55e5ed
# (ret 0xc; o resto ate 0x55e5f0 e enchimento).
# Chamadas: o construtor pela criacao do jogo (0x82902b, sem save) e pela carga (0x82c066, com o save); o destrutor pela
# limpeza do jogo (0x830074); a agressao pelo lance de jogador da regra (0x7e8244, RulingBody, ecx = [0xae28f0]).
# Convencoes conferidas na desmontagem:
# - 0x5554a0 thiscall(disciplina, save) ret 4, devolve o objeto; quadro SEH; 0x68d020 LLIST::LLIST em +0x1c; a busca
#   do CD (GetLogicalDriveStringsA [0x9670f8] (0xc8, buf); GetDriveTypeA [0x967100]; sprintf 0x944c9f "%s%c..."
#   cm0102.exe; 0x4c0aa0 file_find_first cdecl(nome, achado, 0); 0x4c0c10 file_find_close; a caixa "Insert CD" pelo
#   0x66f4e0 langlib_sprintf e 0x5e8290 com 1, e de novo); 0x944cf1 operator new(8), 0x90bde0 utils_build_file_path
#   cdecl("CM3_TEMP", "ban_history.tmp"), 0x58e280 FILE_LLIST::FILE_LLIST thiscall(arquivo, caminho, 0x14) ret 8
#   (club_history_new_file_list); 0x555860 Discipline::load thiscall(disciplina, save) ret 4 (eax; sx09c_4, pelo
#   endereco); 0x944e46 malloc; 0x55d420 Discipline::create_rule_sets thiscall(disciplina, 0) ret 4 (ja Rust). No
#   cm0102-1080.exe o jne do tipo da unidade (0x55550f) virou nop e o test do achado (0x555551) virou xor: a copia le
#   esses bytes do exe que roda (require_game_cd), e o assinaturas.py guarda o hash dos dois exes;
# - 0x555790 thiscall(disciplina), sem argumentos; quadro SEH; 0x9452ca free; 0x7e7860 RulingBody::~RulingBody
#   thiscall (pelo endereco) e 0x944c94 operator delete nas 43 regras; 0x58e330 FILE_LLIST::~FILE_LLIST
#   (FILE_LLIST_DESTRUCT, comp_screens.rs); 0x68d040 LLIST::~LLIST em +0x1c;
# - 0x55e300 thiscall(disciplina, linha da partida, partida, nao usado) ret 0xc -> eax (0 ou 1); quadro SEH com tres
#   datas locais (0x54a110); 0x7e82e0 RulingBody::find_or_add_discipline_entry thiscall(regra, pessoa, 0/1) ret 8;
#   0x556d00 Discipline::release_ban thiscall(disciplina, pessoa, indice) ret 8 (o indice em ax, a parte alta de eax e
#   a do endereco da entrada; a release_ban so le a palavra); 0x669bd0 key_nation_club_in_season cdecl(clube, &inicio,
#   &fim); 0x54cdb0 Date::days_since thiscall(&fim, dword hoje, dword bissexto) ret 8; 0x417c10 Date::operator=;
#   0x556940 Discipline::add_ban thiscall([0xae28f0], pessoa, &suspensao) ret 8 (ja Rust); 0x557280
#   Discipline::post_ban_news thiscall([0xae28f0], pessoa, &suspensao, partida) ret 0xc (sx09b_1, pelo endereco);
#   0x600a50 human_manager_is_human_club cdecl(clube, 0, 1, 0, 0); 0x5487b0 club_nation_is_not_argentina cdecl (ja
#   Rust; so al); 0x90cfc0 utils_random cdecl; 0x758de0 Media::queue_manager_fa_complaint_story thiscall([0xb6b7c8],
#   clube, pessoa, tipo, -1, valor, -1) ret 0x18 (ja Rust).
# Prologos sem salto: 0x5554a0 push -1; push 0x957c8c (7); 0x555790 push -1; push 0x957cab (7); 0x55e300 push -1;
# push 0x957ff8 (7).
# Byte de pilha que a original nao inicia: o byte de marcas (+0x13) da suspensao local da agressao; os bits 3..7 ficam
# com o lixo da pilha, o bit 0 e posto pela add_ban, e so os bits 0..2 passam ao vetor (BanRecord::operator=) e a
# noticia le so o bit 1: sem efeito; a copia comeca o byte em 0 (registrado no DEFEITOS).
# Comparadores: so a original (alocam, liberam, sorteiam, postam noticia); a agressao confere depois a suspensao nova
# (por dias, motivo 3, inicio hoje) na entrada da FIFA da pessoa.
SAVE = False
ROTINAS = [('new_discipline', 0x5554a0, [0x6a, 0xff, 0x68, 0x8c, 0x7c, 0x95, 0x00]),
           ('drop_discipline', 0x555790, [0x6a, 0xff, 0x68, 0xab, 0x7c, 0x95, 0x00]),
           ('handle_referee_assault', 0x55e300, [0x6a, 0xff, 0x68, 0xf8, 0x7f, 0x95, 0x00])]
DOCS = ('a disciplina: construcao, liberacao e a suspensao por agressao ao arbitro',
        'the discipline: construction, release and the ban for assaulting a referee',
        [('new_discipline', 'Discipline::Discipline', '0x5554a0', 'discipline.cpp', 'thiscall, ret 4',
          'monta a disciplina: a lista das suspensoes guardadas, a exigencia do CD do jogo (com a caixa "Insert CD" ate achar; o exe sem CD e respeitado), o vetor das suspensoes e o indice por nacao vazios e o arquivo do historico (ban_history.tmp, registros de 20 bytes; falhando, a caixa "Error" e mais nada). Com o save, le o discipline.dat e, dando certo, mais nada; senao os registros de 29 bytes por pessoa (todos 0xff, sem guardadas; falhando, a caixa) e as regras novas',
          'builds the discipline: the held-ban list, the game CD requirement (with the "Insert CD" box until it is found; a no-CD exe is honoured), the empty ban array and nation index, and the history file (ban_history.tmp, 20-byte records; failing, the "Error" box and nothing else). With a save, reads discipline.dat and, when it succeeds, nothing else; otherwise the 29-byte per-person records (all 0xff, no held bans; failing, the box) and the new rule sets'),
         ('drop_discipline', 'Discipline::~Discipline', '0x555790', 'discipline.cpp', 'thiscall',
          'libera o vetor das suspensoes, o indice por nacao e os registros por pessoa, destroi as 43 regras e a tabela delas, o arquivo do historico e a lista das guardadas',
          'frees the ban array, the nation index and the per-person records, destroys the 43 rule sets and their table, the history file and the held-ban list'),
         ('handle_referee_assault', 'Discipline::handle_referee_assault', '0x55e300', 'discipline.cpp', 'thiscall, ret 0xc',
          'com um cartao de codigo 12 (agressao ao arbitro): solta as suspensoes atuais da pessoa e cria uma da FIFA, por tempo, de 90 dias (180 quando faltam menos de 80 dias para o fim da temporada do clube), motivo 3 (conduta), comecando hoje, e posta a noticia; depois, num clube da IA, nao argentino, com reputacao acima de 5250, tecnico, estado abaixo de -3 e confianca da diretoria abaixo de 4250, sorteia a reclamacao do tecnico a federacao (tipo 1 com 1/20; tipo 3 pela pressao e profissionalismo; tipo 2 com pressao e ambicao acima de 10 e 1/20). Sem o codigo 12, 0; senao 1',
          'with a card of offence 12 (assaulting the referee): releases the person\'s current bans and creates a FIFA ban by days, 90 (180 when the club\'s season ends within 80 days), reason 3 (misconduct), starting today, and posts the news; then, at a computer club, not Argentinian, with a reputation above 5250, a manager, a state below -3 and board confidence below 4250, draws the manager\'s complaint to the FA (kind 1 with 1/20; kind 3 by pressure and professionalism; kind 2 with pressure and ambition above 10 and 1/20). Without offence 12, 0; otherwise 1')])
