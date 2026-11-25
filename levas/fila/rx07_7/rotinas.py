# As notícias de contrato para um técnico humano: a proposta e o teste cancelados por falta de vaga e a volta de um
# empréstimo (contract_manager.cpp; leitor rx07, rodada 6, 25/11/2026). Simulação, carreira ENG (sem LIGAS), no molde
# das notícias do cpw_1. Contagem 0 na carreira automática e no mundo: só saem para um clube com técnico humano; a
# prova é a comparação numa carreira do Lucas (o comparador roda só a original). Iguais no cm0102-1080.exe.
# Convenções conferidas na desmontagem:
# - 0x4d64e0 CONTRACT_MANAGER::post_offer_cancelled_no_space_news thiscall(pessoa, clube) ret 8 e 0x4dade0
#   CONTRACT_MANAGER::post_trial_cancelled_news thiscall(pessoa, clube) ret 8, chamadas pelo sign_contract 0x4cf720
#   (Rust, pelo endereço; push clube; push pessoa; mov ecx, ebp). Iguais fora do tipo (0xbc4 e 0xbbc): News 0x7875e0
#   club_has_human_manager thiscall([0xb7431c], clube) ret 4; NewsItem::NewsItem 0x77bbe0 thiscall(tipo, 0) ret 8 num
#   local de 0xe0 bytes (quadro SEH sem destrutor); set_slot 0x785790 thiscall(k, valor) ret 8 (0 a pessoa, 1 a 3 o id
#   +0x33 dos registros dos nomes, 4 o clube); send_to_club_manager 0x7861e0 thiscall([0xb7431c], notícia, clube) ret 8.
#   Prólogo mov eax, fs:[0] (6).
# - 0x4db0b0 CONTRACT_MANAGER::post_loan_recall_news thiscall(pessoa, clube) ret 8, chamada pelo fim dos empréstimos
#   0x4ccca0 (Rust): o contrato principal pela busca que grava em [0xae1080] (três vezes, como a original), a notícia
#   0xfb8 com 0 a 3 a pessoa, 4 o clube onde joga (staff +0x39), 5 o clube do contrato principal, 6 o clube; vai ao
#   clube do contrato principal quando o clube é o onde ele joga, senão ao onde joga. Prólogo mov eax, fs:[0] (6).
# Comparadores: só a original (a notícia vai à caixa do técnico, aloca).
SAVE = False
ROTINAS = [('post_offer_cancelled_no_space_news', 0x4d64e0, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('post_trial_cancelled_news', 0x4dade0, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('post_loan_recall_news', 0x4db0b0, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00])]
_F = 'contract_manager.cpp'
DOCS = ('as notícias de contrato para um técnico humano: proposta e teste cancelados, empréstimo encerrado',
        "the contract news for a human manager: offer and trial cancelled, loan recalled",
        [('post_offer_cancelled_no_space_news', 'CONTRACT_MANAGER::post_offer_cancelled_no_space_news', '0x4d64e0', _F, 'thiscall, ret 8',
          'a proposta de contrato de um técnico humano a um jogador cai porque o elenco está cheio: a notícia 0xbc4 ao técnico do clube (só um clube com técnico humano), com a pessoa, os nomes e o clube',
          "a human manager's contract offer to a player falls because the squad is full: news item 0xbc4 to the club's manager (only a club with a human manager), with the person, his names and the club"),
         ('post_trial_cancelled_news', 'CONTRACT_MANAGER::post_trial_cancelled_news', '0x4dade0', _F, 'thiscall, ret 8',
          'o teste de um jogador num clube de técnico humano é cancelado porque o elenco está cheio: a notícia 0xbbc, com a pessoa, os nomes e o clube',
          "a player's trial at a human manager's club is cancelled because the squad is full: news item 0xbbc, with the person, his names and the club"),
         ('post_loan_recall_news', 'CONTRACT_MANAGER::post_loan_recall_news', '0x4db0b0', _F, 'thiscall, ret 8',
          'um empréstimo é encerrado por um clube: a notícia 0xfb8 com a pessoa, os nomes, o clube onde joga, o do contrato principal e o que encerrou, ao clube do outro lado (sem contrato principal, nada)',
          'a loan is ended by a club: news item 0xfb8 with the person, his names, the club he plays for, his main contract\'s club and the one that ended it, to the club on the other side (without a main contract, nothing)')])
CAMPOS = []
INTERESSA = []
