# Os motivos de um jogador recusar a mudança (transfer_manager.cpp; leitor rxs1, rodada 6, 25/11/2026). A rotina
# inteira (0x8e4ae0..0x8e4f1a), igual no cm0102-1080.exe; contagem 0 na carreira ENG e no mundo até 2002-08-01: roda nos
# textos das notícias (write_player_rejects_move_news_text, rxs1_3, e a 0x8e08e0 do rx05_5) e nas telas da proposta
# recusada (contract_screens.rs, transfer_manager.rs, já em Rust, pelo exe::transfer_manager_build_refusal_reasons_text).
# Convenção conferida na desmontagem e nos pontos de chamada: stdcall(clube vendedor, clube comprador, dois sem uso,
# motivos, interesse (double), buffer) ret 0x20, sem quadro SEH; devolve a quantidade em al. Os cinco buffers de 250
# bytes zerados no começo; 2.75 é o rdata::F64_2_75 (0x9690a8), comparado com fcomp (NaN não passa).
# Comparador: lado a lado sobre a quantidade e o texto (a cópia num buffer local que começa com o texto do chamador;
# sem motivo, nenhum dos dois escreve). Não usa o gerador nem grava no jogo.
SAVE = False
ROTINAS = [('transfer_manager_build_refusal_reasons_text', 0x8e4ae0, [0x81, 0xec, 0xe8, 0x04, 0x00, 0x00])]
DOCS = ('os motivos de um jogador recusar a mudança',
        'the reasons a player refuses a move',
        [('transfer_manager_build_refusal_reasons_text', 'transfer_manager_build_refusal_reasons_text', '0x8e4ae0', 'transfer_manager.cpp', 'stdcall, ret 0x20',
          'até dois motivos das marcas, unidos por "e": esperar o fim da temporada (sozinho); muito difícil deixar o clube, não sair para um rival, feliz no clube, feliz com o contrato, chegou há pouco, ou ainda não decidiu (com interesse ≥ 2,75); não perder a competição continental do clube; não ir jogar nos reservas no exterior, não descer de divisão sem garantia de titular ou só com garantia de titular; não ir para a nação ou o continente do comprador, ou a língua e o modo de vida; não tem certeza de que a mudança é boa a longo prazo; devolve quantos escreveu',
          'up to two reasons from the flags, joined with "and": wait for the end of the season (alone); very difficult to leave the club, not to a rival, happy at the club, happy with his contract, only recently joined, or not ready to decide (with interest ≥ 2.75); not to lose the club\'s continental competition; not to play in the reserves abroad, not to drop a division without first team football or only with it; not to the buyer\'s nation or continent, or the language and lifestyle; unsure the move helps in the long term; returns how many it wrote')])
