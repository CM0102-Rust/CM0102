# Os destrutores do premio de equipe, do premio do ano e do AWOL (award/team_award.cpp, year_award.cpp, awol.cpp;
# leitor sx05b, rodada 5, 23/11/2026). Usa team_award_obj, year_award_obj (sx05b_5) e AWOL_LISTS (sx05b_8): integrar
# depois deles.
# Rodam so quando o jogo e desfeito (o AWARD_MANAGER apaga os premios pela virtual 0 = o destrutor que apaga; o AWOL
# em 0x830134, na limpeza do jogo do Setup.cpp): a carreira automatica nao chega la (o processo termina sem desfazer
# o jogo) e a prova do save abre o jogo ja com -load. Item TELAS: a prova e o Lucas, com um jogo em andamento, carregar
# outro jogo pelo menu (Arquivo -> Carregar) ou voltar ao menu principal e comecar outro, jogando comparando.
# Convencoes conferidas na desmontagem:
# - 0x8af650 thiscall(this): [this] = 0x9703f0 e jmp AWARD::~AWARD 0x412ef0 ([this] = 0x967864); o prologo de 6 bytes
#   (mov dword [ecx], imm32) e seguido do jmp: fica so o mov no prologo;
# - 0x8af630 e 0x92ff60 thiscall(this, bandeiras) ret 4, devolvem this: chamam o destrutor (call rel32 no prologo:
#   RELATIVAS (0x8af630, 3) e (0x92ff60, 3) em rust/src/detours.rs) e, com o bit 0, operator delete 0x944c94;
# - 0x92ff80 thiscall(this): [this] = 0x9713b8; com lista curta (+0x1f), 0x58e330 (FILE_LLIST_DESTRUCT: free das
#   entradas) e operator delete, sem zerar o +0x1f; AWARD::~AWARD;
# - 0x4185f0 thiscall(this): free 0x9452ca da tabela (+0xc) e LLIST::~LLIST 0x68d040 + operator delete das tres
#   listas, cada ponteiro zerado.
# Mutacoes: a carreira nao roda tela nem desfaz o jogo; as do mutacoes.py sao para o comparador numa sessao do Lucas.
TELAS = True
SAVE = False
ROTINAS = [('team_award_destruct', 0x8af650, [0xc7, 0x01, 0xf0, 0x03, 0x97, 0x00]),
           ('team_award_deleting_destruct', 0x8af630, [0x56, 0x8b, 0xf1, 0xe8, 0x18, 0x00, 0x00, 0x00]),
           ('year_award_destruct', 0x92ff80, [0x6a, 0xff, 0x68, 0x98, 0x60, 0x96, 0x00]),
           ('year_award_deleting_destruct', 0x92ff60, [0x56, 0x8b, 0xf1, 0xe8, 0x18, 0x00, 0x00, 0x00]),
           ('awol_destruct', 0x4185f0, [0x56, 0x8b, 0xf1, 0x57, 0x8b, 0x46, 0x0c])]
DOCS = ('os destrutores dos prêmios de equipe e do ano e do AWOL',
        'the team award, year award and AWOL destructors',
        [('team_award_destruct', 'TEAM_AWARD::~TEAM_AWARD', '0x8af650', 'award/team_award.cpp', 'thiscall',
          'desfaz o prêmio de equipe: a tabela virtual da classe de volta e o destrutor da base do prêmio',
          "destroys a team award: the class's vtable back in place and the award base destructor"),
         ('team_award_deleting_destruct', "TEAM_AWARD::`scalar deleting destructor'", '0x8af630', 'award/team_award.cpp', 'thiscall, ret 4',
          'o destrutor que apaga: desfaz o prêmio e, com o bit 0 das bandeiras, libera o objeto; devolve o objeto',
          'the deleting destructor: destroys the award and, with bit 0 of the flags, frees the object; returns the object'),
         ('year_award_destruct', 'YEAR_AWARD::~YEAR_AWARD', '0x92ff80', 'award/year_award.cpp', 'thiscall',
          'desfaz o prêmio do ano: a tabela virtual da classe de volta, a lista curta (quando há) desfeita e liberada (o ponteiro fica) e o destrutor da base',
          "destroys a year award: the class's vtable back in place, the shortlist (when there is one) destroyed and freed (the pointer stays) and the base destructor"),
         ('year_award_deleting_destruct', "YEAR_AWARD::`scalar deleting destructor'", '0x92ff60', 'award/year_award.cpp', 'thiscall, ret 4',
          'o destrutor que apaga do prêmio do ano: desfaz e, com o bit 0, libera; devolve o objeto',
          'the year award deleting destructor: destroys and, with bit 0, frees; returns the object'),
         ('awol_destruct', 'AWOL::~AWOL', '0x4185f0', 'awol.cpp', 'thiscall',
          'desfaz o estado do AWOL: a tabela de marcas liberada e as três listas desfeitas e liberadas, cada ponteiro zerado',
          'destroys the AWOL state: the flag table freed and the three lists destroyed and freed, each pointer cleared')])
CAMPOS = []
INTERESSA = []
