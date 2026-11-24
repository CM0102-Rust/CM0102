# As disputas: a construcao, a liberacao e a queixa a Comissao do Estatuto do Jogador (DISPUTE::DISPUTE 0x5697e0,
# DISPUTE::~DISPUTE 0x5698d0, DISPUTE::file_complaint 0x56a100, dispute.cpp; leitor sx09c, rodada 5, 23/11/2026). As tres
# rotinas inteiras: 0x5697e0..0x5698cc (ret 4), 0x5698d0..0x569909 (ret) e 0x56a100..0x56a3a2 (ret 0x10).
# Chamadas: o construtor pela criacao do jogo (0x82921a, sem save) e pela carga (0x82c251, com o save); o destrutor pela
# limpeza do jogo (0x830154); a queixa pela multa (0x5a692b, Fine::fine_player) e pela midia (0x741ebe).
# Convencoes conferidas na desmontagem:
# - 0x5697e0 thiscall(disputa, save) ret 4, devolve o objeto; quadro SEH; 0x569910 DISPUTE::load thiscall ret 4 (al;
#   sx09c_3, pelo endereco); 0x9452ca free; 0x68d040 LLIST::~LLIST e 0x944c94 operator delete; 0x945501 calloc
#   cdecl([0xae2364], 1); 0x944cf1 operator new(0xe) e 0x68d020 LLIST::LLIST;
# - 0x5698d0 thiscall(disputa), sem argumentos;
# - 0x56a100 thiscall(disputa, pessoa, clube, a, b) ret 0x10; quadro SEH; 0x418f60 AWOL::is_player_awol
#   thiscall([0xad9c6c], pessoa, 0) ret 8 (al); [0xae233c] as regras da FIFA; 0x90cfc0 utils_random cdecl; as tres
#   reputacoes por fild/fmul [0x967898] (0.01)/fsubr [0x967890] (1.0)/fmulp e 0x945640 _ftol; 0x547af0
#   staff_add_disliked_staff cdecl(quem, de quem, 1); 0x600a50 human_manager_is_human_club cdecl(clube, 1, 1, 0, 0);
#   0x417c10 Date::operator=; 0x54c5b0 Date::add_days thiscall(data, &saida, dias) ret 8; 0x68d0d0 LLIST::add_tail
#   thiscall([disputa], &registro, 0x11) ret 8; 0x56a3b0 dispute_post_news thiscall(disputa, pessoa, clube, 0, a, b)
#   ret 0x14 (sx09b_6, pelo endereco). O id do clube e lido (mov edx,[edi]) antes de conferir se o clube e nulo.
# Prologos sem salto: 0x5697e0 mov eax,fs:[0]; push -1 (8); 0x5698d0 push esi; mov esi,ecx; push edi; mov eax,[esi+4]
# (7); 0x56a100 push -1; push 0x9580d0 (7).
# Comparadores: so a original; a queixa confere depois a marca da pessoa e a lista (uma disputa a mais quando segue).
# A queixa provavelmente nao acontece na carreira de ferias ENG ate 2002-06-01 (so pela multa contestada pela midia ou
# pela multa que o jogador acha injusta, 0x5a692b): roda o construtor (criacao) e so.
SAVE = False
ROTINAS = [('new_dispute', 0x5697e0, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00, 0x6a, 0xff]),
           ('drop_dispute', 0x5698d0, [0x56, 0x8b, 0xf1, 0x57, 0x8b, 0x46, 0x04]),
           ('file_complaint', 0x56a100, [0x6a, 0xff, 0x68, 0xd0, 0x80, 0x95, 0x00])]
DOCS = ('as disputas: construcao, liberacao e a queixa a Comissao do Estatuto do Jogador',
        'the disputes: construction, release and the complaint to the Player Status Committee',
        [('new_dispute', 'DISPUTE::DISPUTE', '0x5697e0', 'dispute.cpp', 'thiscall, ret 4',
          'monta o objeto das disputas: com o save, le o dispute.dat e, dando certo, mais nada (falhando, libera o que a carga deixou); senao a tabela das marcas, um byte zerado por pessoa (calloc), e uma lista vazia; devolve o objeto',
          'builds the dispute object: with a save, reads dispute.dat and, when it succeeds, nothing else (failing, frees what the load left); otherwise the flag table, one zeroed byte per person (calloc), and an empty list; returns the object'),
         ('drop_dispute', 'DISPUTE::~DISPUTE', '0x5698d0', 'dispute.cpp', 'thiscall',
          'libera a tabela das marcas e destroi a lista, zerando os ponteiros',
          'frees the flag table and destroys the list, zeroing the pointers'),
         ('file_complaint', 'DISPUTE::file_complaint', '0x56a100', 'dispute.cpp', 'thiscall, ret 0x10',
          'abre a queixa do jogador (com ficha, sem disputa, nao ausente sem licenca, regras da FIFA em vigor): marca a disputa; as reputacoes atual, local e mundial caem ate 20% (random(21) por cento cada); com clube, ele e o tecnico se desgostam (o tecnico so num clube da IA), sem o byte b cada colega com random(lealdade) acima de 5 e random(profissionalismo) 0 passa a nao gostar dele, e o presidente tambem; a disputa (pessoa, clube, audiencia daqui a 2 a 21 dias, fica no clube = b salvo 1/10) vai para a lista e sai a noticia',
          'opens the player\'s complaint (with a player record, no dispute, not absent without leave, the FIFA rules in force): marks the dispute; his current, home and world reputations drop up to 20% (random(21) percent each); with a club, he and the manager fall out (the manager only at a computer club), without the byte b each squad mate with random(loyalty) above 5 and random(professionalism) 0 dislikes him, and so does the chairman; the dispute (person, club, hearing 2 to 21 days ahead, stays at the club = b except 1 in 10) joins the list and the news is posted')])
