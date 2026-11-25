# Os textos das notícias do contrato sem a permissão revista, da renovação da permissão de trabalho e da
# transferência que cai por falta de verba (transfer_manager.cpp; leitor rx05, rodada 6, 25/11/2026). As três inteiras
# (0x8e4460..0x8e460b, 0x8e3b80..0x8e3da2, 0x8dd830..0x8dda88), iguais no cm0102-1080.exe; contagem 0 na carreira ENG
# e no mundo até 2002-08-01 (os textos só são escritos quando alguém abre a notícia).
# Convenções conferidas na desmontagem: as três thiscall(this, notícia, corpo, Text) ret 0xc, sem quadro SEH; os
# mesmos auxiliares de texto do mke_5 (0x785830, 0x53e140, 0x66f4e0, 0x4ae660, 0x4ae8a0, 0x53e990); 0x53b3d0
# database_get_national_team cdecl(nação). Na 0x8dd830 as vagas são lidas na ordem 1..5 e a 0 só no corpo.
# Defeito do original (0x8dd830, a cópia mantém): o corpo põe os nomes longos dos clubes (+4) com o gênero dos nomes
# curtos (+0x52).
# Comparadores: lado a lado do texto e dos vínculos (compare_text_writer).
SAVE = False
ROTINAS = [('write_contract_permit_refused_news_text', 0x8e4460, [0x81, 0xec, 0x30, 0x01, 0x00, 0x00]),
           ('write_work_permit_renewal_news_text', 0x8e3b80, [0x81, 0xec, 0x34, 0x01, 0x00, 0x00]),
           ('write_funds_refused_news_text', 0x8dd830, [0x81, 0xec, 0x38, 0x01, 0x00, 0x00])]
DOCS = ('os textos das notícias do contrato sem a permissão revista, da renovação da permissão de trabalho e da transferência que cai por falta de verba',
        'the news texts of the contract without the revised permit, of the work permit renewal and of the transfer that collapses for lack of funds',
        [('write_contract_permit_refused_news_text', 'TRANSFER_MANAGER::write_contract_permit_refused_news_text', '0x8e4460', 'transfer_manager.cpp', 'thiscall, ret 0xc',
          'o título "proposta de contrato cancelada" ou o corpo "o contrato de <nome> com <clube> não pode ser renovado agora porque a permissão revista não sai"; os vínculos à pessoa e ao clube',
          'the headline "contract offer cancelled" or the body "<name>\'s contract with <club> may not be extended as a revised work permit cannot be issued"; the links to the person and the club'),
         ('write_work_permit_renewal_news_text', 'TRANSFER_MANAGER::write_work_permit_renewal_news_text', '0x8e3b80', 'transfer_manager.cpp', 'thiscall, ret 0xc',
          'o título "permissão renovada" ou "permissão expira", ou o corpo: renovada por mais um ano, ou vencida por poucos jogos na nação; os vínculos à pessoa e à seleção da nação',
          'the headline "work permit renewed" or "work permit expires", or the body: extended for another year, or expired for too few games in the nation; the links to the person and to the nation\'s national team'),
         ('write_funds_refused_news_text', 'TRANSFER_MANAGER::write_funds_refused_news_text', '0x8dd830', 'transfer_manager.cpp', 'thiscall, ret 0xc',
          'o título "transferência para <comprador> não sai" ou o corpo "a transferência de <nome> de <vendedor> para <comprador> caiu porque <comprador> não libera a verba"; os vínculos à pessoa e aos clubes',
          'the headline "transfer to <buyer> falls through" or the body "<name>\'s proposed transfer from <seller> to <buyer> has collapsed as <buyer> will not release the funds"; the links to the person and the clubs')])
