# Os destrutores do premio do mes e dos premios de uma nacao e a ordem da equipe da semana (month_award.cpp,
# nation_awards.cpp; leitor sx04b, rodada 5, 23/11/2026). Rodam so nas telas: os destrutores quando o jogo e desfeito
# (o AWARD_MANAGER apaga cada nacao e cada nacao apaga os seus premios pela virtual 0; a carreira automatica termina sem
# desfazer o jogo e a prova do save abre com -load), e a ordem 0x76a460 so pelo qsort da equipe da semana (0x769c70,
# chamada pela virtual update_team_of_the_week, a partir da tela dos premios; a cobertura do mundo da 0 chamadas na
# carreira). Item TELAS. Caminho para o Lucas, jogando comparando: (1) com um jogo em andamento, abrir os premios da
# Inglaterra e escolher a equipe da semana (a ordem); (2) depois carregar outro jogo pelo menu (Arquivo -> Carregar) ou
# voltar ao menu principal e comecar outro (os destrutores).
# Convencoes conferidas na desmontagem:
# - 0x766f40 thiscall(this): [this] = 0x96da00 e jmp AWARD::~AWARD 0x412ef0; o prologo de 6 bytes (mov dword [ecx],
#   imm32) e seguido do jmp: fica so o mov no prologo (como o 0x8af650 do sx05b_10);
# - 0x766f20 thiscall(this, bandeiras) ret 4, devolve this: chama 0x766f40 (call rel32 no prologo: RELATIVAS
#   (0x766f20, 3) em rust/src/detours.rs) e, com o bit 0, operator delete 0x944c94;
# - 0x7682b0 thiscall(this): [this] = 0x96da10; cada premio pela virtual 0 thiscall(1) (a contagem relida); free
#   0x9452ca da tabela; com notas do mes, 0x58e330 (free do bloco em +0) e operator delete; os ponteiros nao sao
#   zerados;
# - 0x76a460 cdecl(a, b) do qsort: byte b+8 - byte a+8 (sem sinal).
# Prologos: 0x766f20 com o call relativo (RELATIVAS); os outros sem; iguais no cm0102-1080.exe.
# Depende do sx05b_10 (award_destruct_base, award_deleting_destruct), do sx05b_5 (nation_awards_fields) e do sx04b_2
# (MONTH_AWARD_VTABLE, NATION_AWARDS_BASE_VTABLE): integrar depois deles.
# Fica de fora deste item a propria equipe da semana (0x769c70, 1552 bytes com SEH, o vetor de 0x625 x 2 registros e a
# busca das partidas de cada semana): proximo leitor.
# Comparadores: 0x766f40 lado a lado na vtable; os que liberam so a original; a ordem por compare_order!.
# Mutacoes: a carreira nao roda tela nem desfaz o jogo; as do mutacoes.py sao para o comparador numa sessao do Lucas.
TELAS = True
SAVE = False
ROTINAS = [('month_award_destruct', 0x766f40, [0xc7, 0x01, 0x00, 0xda, 0x96, 0x00]),
           ('month_award_deleting_destruct', 0x766f20, [0x56, 0x8b, 0xf1, 0xe8, 0x18, 0x00, 0x00, 0x00]),
           ('nation_awards_destruct', 0x7682b0, [0x56, 0x57, 0x8b, 0xf1, 0x33, 0xff]),
           ('nation_awards_compare_byte_8_descending', 0x76a460, [0x8b, 0x4c, 0x24, 0x04, 0x33, 0xc0])]
DOCS = ('os destrutores do prêmio do mês e dos prêmios de uma nação e a ordem da equipe da semana',
        "the month award's and a nation's awards' destructors and the team of the week order",
        [('month_award_destruct', 'MONTH_AWARD::~MONTH_AWARD', '0x766f40', 'month_award.cpp', 'thiscall',
          'regra: devolve a tabela virtual do prêmio do mês e roda o destrutor da base AWARD',
          "rule: puts the month award's vtable back and runs the AWARD base destructor"),
         ('month_award_deleting_destruct', "MONTH_AWARD::`scalar deleting destructor'", '0x766f20', 'month_award.cpp', 'thiscall, ret 4',
          'regra: o destrutor do prêmio do mês e, com o bit 0 das bandeiras, o objeto liberado',
          'rule: the month award destructor and, with bit 0 of the flags, the object freed'),
         ('nation_awards_destruct', 'NationAwards::~NationAwards', '0x7682b0', 'nation_awards.cpp', 'thiscall',
          'regra: apaga cada prêmio da nação pelo seu destrutor, libera a tabela e, se houver, as notas do mês',
          "rule: deletes each of the nation's awards through its destructor, frees the table and, if any, the month ratings"),
         ('nation_awards_compare_byte_8_descending', 'nation_awards_compare_byte_8_descending', '0x76a460', 'nation_awards.cpp', 'cdecl',
          'regra: a ordem dos candidatos da equipe da semana pela nota da partida, a maior primeiro',
          'rule: the team of the week candidates by match rating, the highest first')])
CAMPOS = []
INTERESSA = []
