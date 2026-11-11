# A escolha do alvo de uma vaga da lista de compras da IA (shortlist_manager.cpp), o mapa do mercado pedido pelo
# Rust Tweaks (levas/reorganizacao/mercado, leitor B, secao_b §3.2, 11/11/2026):
# - 0x83f3a0 (Shortlist_Manager::fill_shortlist_slot, 6624 bytes até a fronteira, 2048 instruções, recursiva, quadro
#   SEH só com o STAFF_CONTRACT local, sem destrutor): chamada pela allocate_transfer_budget 0x83b6e0 (Rust, cpy_3;
#   0x83bab7, excluído 0) para cada vaga aberta que recebe verba, pela update_staff_status 0x843e10 (Rust, cpy_2;
#   0x844108) quando a vaga perde o alvo e é preenchida de novo, e por ela mesma (0x840d46); a 0x83b6e0 roda na
#   update_club_shortlist 0x83a730 (o ritmo da lista: 30 + random(70) dias por clube, laço do dia 0x5cdcee..0x5cdd15).
#   Decide quem entra na lista de compras de cada clube da IA, então quem recebe lance e quem é emprestado.
# Igual no cm0102-1080.exe (bytes 0x83f3a0..0x840d80 conferidos em 11/11/2026).
# Convenções conferidas nos pontos de chamada da original (desmontagem 0x83f3a0..0x840d7d):
# - quem chama: thiscall(this, vaga, clube, excluído), ret 0xc; a recursão em 0x840d46 chama 0x83f3a0 (pelo desvio).
# - 0x85e410 lea ecx, local -> thiscall, ret 0; 0x669340 push nação; add esp, 4 -> cdecl (i8);
#   0x54c8e0 push 0x1c (ou 200); push &saída; mov ecx, data -> thiscall, ret 8; 0x544410 push 6 (5); push np; add esp
#   (cdecl, i8); 0x600a50 5 push; add esp, 0x14 -> cdecl; 0x600e90 / 0x53ca90 / 0x53b900 cdecl de 1;
#   0x5fd210 push gerente; mov ecx, [0xb63c98] -> thiscall, ret 4; 0x5fea50 push gerente; push 4 -> thiscall, ret 8;
#   0x84b230 push status; push posição; push clube; mov ecx, this -> ret 0xc (a Rust é stdcall com o this ignorado);
#   0x5a0590 push clube; mov ecx, [0xae2a38] -> thiscall, ret 4; 0x5986b0 mov ecx, finanças -> thiscall (u8);
#   0x6e6340 push substituto do gerente; push -1; push clube; push 0x1902|0x18c2|0x8118c2|0x19c2; push posição; push
#   pessoa; add esp, 0x18 -> cdecl; 0x4cf5f0 push clube; push pessoa; mov ecx, [0xae1050] -> thiscall, ret 8;
#   0x85fa00 mov ecx, contrato -> thiscall (i16); 0x83bd00 push posição; mov ecx, this -> ret 4; 0x946464 bsearch
#   cdecl (chave, base, n, 8, 0x757ea0); 0x843e10 push !jogador; push 3; push clube; push excluído; mov ecx, this ->
#   thiscall, ret 0x10; 0x69fc60 push pessoa; mov ecx, [0xb67a44] -> thiscall, ret 4; 0x7f7490 push gerente; push
#   pessoa; mov ecx, [0xb74448] -> thiscall, ret 8; 0x544590 / 0x544520 push 0; push pessoa -> cdecl; 0x4cf5a0 push
#   pessoa; mov ecx, [0xae1050] -> thiscall, ret 4 (às vezes guardado em 0xae1080, como a cópia faz); 0x84b180 push
#   clube; push clube atual; push pessoa; mov ecx, this -> ret 0xc; 0x8c7560 push 1; push clube; push pessoa; mov ecx,
#   [0xde1f54] -> thiscall, ret 0xc; 0x8d0040 push 0; push pessoa; push clube; mov ecx, [0xde1f54] -> ret 0xc; 0x8cca10
#   push 0; push pessoa -> thiscall, ret 8; 0x53d980 push 1; push nação; push pessoa -> cdecl; 0x5407e0 push !jogador;
#   push gerente; push pessoa -> cdecl (i16); 0x90cfc0 cdecl de 1; 0x840d80 push 1; push melhor; push &nota; push vaga;
#   push pessoa; push clube; mov ecx, this -> thiscall, ret 0x18; 0x69d1b0 push cargo; push gerente; push clube; push
#   pessoa; mov ecx, [0xb67a44] -> thiscall, ret 0x10; 0x4d0c60 push 0; push -1; push 0; push vendedor; push pessoa;
#   mov ecx, [0xae1050] -> thiscall, ret 0x14; 0x84b6f0 push &taxa; push clube; push contrato; push vaga; mov ecx,
#   this -> ret 0x10; 0x4d42d0 push pessoa; push clube; push vendedor -> thiscall, ret 0xc; 0x4d1600 push 1; push 1;
#   push 0; push pessoa; push clube; push vendedor -> thiscall, ret 0x18; 0x84af00 8 dwords (vendedor, pessoa, oferta
#   i64, clube, pedido i64, empréstimo) -> stdcall, ret 0x20; 0x62ed70 push pessoa; mov ecx, [0xb63d28] -> thiscall,
#   ret 4; 0x8ce990 push 0; push 0; push clube; push pessoa; mov ecx, [0xde1f54] -> thiscall, ret 0x10; 0x846920
#   push 0; push status; push empréstimo; push 1 (0); push clube; push pessoa; mov ecx, this -> thiscall, ret 0x18
#   (f64); 0x844780 push 0; push cargo; push 1; push clube; push pessoa; mov ecx, this -> ret 0x14 (f64); 0x754740 push
#   f64 (2 dwords); push valor; push clube; push pessoa; mov ecx, [0xb6b7c8] -> thiscall, ret 0x14; 0x864550 push 0;
#   push 0xb; push tipo; push 0; push clube; push pessoa; lea ecx, local -> thiscall, ret 0x18 (f64, guardado como
#   f32); 0x5968d0 push tipo; push pessoa; mov ecx, finanças -> thiscall, ret 8; 0x7875e0 push clube; mov ecx,
#   [0xb7431c] -> thiscall, ret 4; 0x587220 push clube; push pessoa; add esp, 8 -> cdecl; 0x8423f0 push clube; push
#   pessoa; mov ecx, this -> thiscall, ret 8.
# Fidelidade: o tipo de contrato (1, 2, 3 ou o status) vai no byte baixo do local do substituto do gerente, e a
# original empurra o dword inteiro para 0x864550 e 0x5968d0: a cópia passa os mesmos 24 bits de cima. As contas em
# f64 (o x87 do jogo roda com precisão de 53 bits); o salário desejado em f32, como a original guarda.
# Comparador: só a original (sorteia, mexe na lista de compras e na lista de interesse da mídia, pode escrever notícia
# e recorre; não roda duas vezes).
SAVE = False
ROTINAS = [('fill_shortlist_slot', 0x83f3a0, [0x55, 0x8b, 0xec, 0x83, 0xe4, 0xf8, 0x6a, 0xff])]
DOCS = ('a escolha do alvo de uma vaga da lista de compras da IA',
        "the AI choosing the target of a shortlist slot",
        [('fill_shortlist_slot', 'Shortlist_Manager::fill_shortlist_slot', '0x83f3a0', 'shortlist_manager.cpp', 'thiscall, ret 0xc',
          'o clube da IA percorre as listas de posição da vaga (as ordenadas por chave numa vaga de empréstimo) e pula: o excluído, quem tem cargo 5, candidato a vaga de técnico, aposentadoria anunciada, saudita, mais de 23 anos numa vaga de status 5, preço × 1,3 acima do teto da vaga (compra) ou não listado para empréstimo, quem já é do clube, quem está em 10 listas ou já na deste clube, quem chegou há menos de 200 dias sem status 7, queixas ou pedido de saída, a regra de estrangeiros e o visto, o empréstimo impossível, a transferência futura; o clube só conhece um em k (k = 100, ou (6025 − reputação) × 4 abaixo de 3500, ou 50 na Bélgica, Holanda, Suécia, Dinamarca e Noruega, mais 5 ou 10 por vaga de olheiro vazia e 30 − 2a − b por olheiro, dividido por 10, no mínimo 2; defeito provável do original: a e b vêm de 0x544410 com as constantes 6 e 5, então só a habilidade atual do olheiro conta, nunca o julgamento dele), além do estrangeiro sem convocação visto com reputação ≥ 2750 (status 5/6) ou 3250 + sorteio(2000) e do jovem da mesma nação com (id + clube) % (25 − jovens do técnico) = 0; nota = valor para o clube (0x840d80) ou a do cargo; abaixo da avaliação da vaga sai; compra: preço pedido (compensação, humano, IA), um extra até 2,5 milhões nas vagas 4 e 5, a previsão do vendedor 2 sai, preço acima do teto sai, nota × a margem (1 − (preço − valor)/(teto × fator de reputação), entre 0,01 e 3) menos a lesão; empréstimo: a previsão do dono e a lesão; nota × (92 + sorteio(17))%; acima do melhor: o interesse (0x846920) com notícia de interesse por sorteio, interesse ≥ 3,15 e o salário desejado × max(3,5/interesse, 0,9) ≤ 1,2 × o teto salarial: vira o melhor; o melhor vira o alvo e entra na lista (0x8423f0) com notícia por sorteio; ninguém: na temporada, uma vaga de compra de status 2 ou 3 vira de empréstimo e recomeça (recursão) se (reputação² × 4e-9 + 0,6) × avaliação < preço, teto < 50000 ou Itália com sorteio, e a liga carregada ou sorteio(reputação) > sorteio(20000); senão o alvo fica −1',
          "the AI club walks the slot's position lists (the ones sorted by key in a loan slot) and skips: the excluded person, job 5, a manager vacancy candidate, retirement announced, Saudi, over 23 in a status-5 slot, price × 1.3 above the slot's ceiling (purchase) or not loan listed, someone already at the club, on 10 shortlists or already on this club's, arrived less than 200 days ago without status 7, grievances or a request to leave, the foreigner and work permit rules, an impossible loan, a future transfer; the club only knows one in k (k = 100, or (6025 − reputation) × 4 below 3500, or 50 in Belgium, Holland, Sweden, Denmark and Norway, plus 5 or 10 per empty scout place and 30 − 2a − b per scout, divided by 10, at least 2; probable defect of the original: a and b come from 0x544410 with the constants 6 and 5, so only the scout's current ability counts, never his judging), plus the uncapped foreigner seen with reputation ≥ 2750 (status 5/6) or 3250 + random(2000) and the youngster of the same nation with (id + club) % (25 − the manager's youngsters) = 0; score = value for the club (0x840d80) or the job's score; below the slot's valuation he is out; purchase: the asking price (compensation, human, AI), an extra up to 2.5 million in status 4 and 5 slots, the seller's prediction 2 is out, a price above the ceiling is out, score × the margin (1 − (price − value)/(ceiling × reputation factor), between 0.01 and 3) minus the injury; loan: the owner's prediction and the injury; score × (92 + random(17))%; above the best: the interest (0x846920) with interest news by draw, interest ≥ 3.15 and the desired wage × max(3.5/interest, 0.9) ≤ 1.2 × the wage cap: the new best; the best becomes the target and enters the shortlist (0x8423f0) with news by draw; nobody: in season, a purchase slot of status 2 or 3 becomes a loan slot and starts again (recursion) if (reputation² × 4e-9 + 0.6) × valuation < price, ceiling < 50000 or Italy with a draw, and the league loaded or random(reputation) > random(20000); otherwise the target stays −1")])
CAMPOS = [('vaga da lista (37 bytes): +0 cargo, +1 posição, +3 avaliação, +7 teto, +0xb preço, +0xf alvo (gravado), +0x13 substituído, +0x17 oferta, +0x24 bandeiras (bit 0 empréstimo gravado na recursão, bits 1-4 status); lê e grava (fill_shortlist_slot)',
           'shortlist slot (37 bytes): +0 job, +1 position, +3 valuation, +7 ceiling, +0xb price, +0xf target (written), +0x13 replaced, +0x17 offer, +0x24 flags (bit 0 loan, written before the recursion, bits 1-4 status); reads and writes (fill_shortlist_slot)'),
          ('Shortlist_Manager +0 e +0x10 pares (pessoa, clube) ordenados por pessoa, +0x18 listas de posição (índice + 16 = as ordenadas por chave), +0x98 contagens; entrada de 12 bytes: +0 id da pessoa, +8 preço; lê (fill_shortlist_slot)',
           'Shortlist_Manager +0 and +0x10 (person, club) pairs sorted by person, +0x18 position lists (index + 16 = the ones sorted by key), +0x98 counts; 12-byte entry: +0 person id, +8 price; reads (fill_shortlist_slot)'),
          ('clube +0x1b3 olheiros (só a habilidade atual +4 do non_player conta), +0xcf técnico (+0x22 jovens), +0x80 reputação, +0x53 nação (+0x75 região, +0x85 padrão da liga, +0x11c liga carregada); finanças +0x165 crise do vendedor; pessoa +0x22 convocações, +0x24/+0x28 seleção, +0x3e chegada ao clube; lê (fill_shortlist_slot)',
           'club +0x1b3 scouts (only the non_player current ability +4 counts), +0xcf manager (+0x22 youngsters), +0x80 reputation, +0x53 nation (+0x75 region, +0x85 league standard, +0x11c league loaded); finance +0x165 seller crisis; person +0x22 caps, +0x24/+0x28 national team, +0x3e date joined club; reads (fill_shortlist_slot)')]
INTERESSA = ['Tweaks', 'CMRTE']
