# As ordens da lista de candidatos de um prêmio (award_shortlist.cpp; leitor tel44, rodada 5, 23/11/2026): os três
# comparadores do qsort (0x945333) da AWARD_SHORTLIST, passados por endereço: por nome na AWARD_SHORTLIST::announce
# (0x417a20, já em Rust, push 0x418340 em 0x417a40), por nota e por votos na AWARD_SHORTLIST::pick_winners (0x417c50,
# já em Rust, 0x417c7d e 0x417f77). Simulação (prêmios), sem LIGAS.
# Convenções conferidas na desmontagem: os três cdecl(a, b) com ponteiros para as entradas de 14 bytes, retorno em eax.
# - por nome: o id do jogador em +0 (−1 = sem jogador, nome vazio) -> pessoa (tabela 0xae23bc, 0x6e bytes);
#   0x53dba0 cdecl(pessoa, buf, 2000, 4) e 0x90c1d0 cdecl(nome a, nome b, 1, 0);
# - por nota: fld [a+8]; fcomp [b+8]; test ah, 0x41 -> −1 se a > b; depois b > a -> 1; senão 0 (também sem ordem);
# - por votos: movsx [b+0xc] − movsx [a+0xc].
# Prólogos: 0x418340 mov eax,[esp+4]; mov edx,[0xae23bc] (10 bytes, endereço absoluto); 0x4183f0 e 0x418420 os dois
# mov dos argumentos (8 bytes). Sem RELATIVAS. Corpos iguais no cm0102.exe e no cm0102-1080.exe.
# Comparadores: compare_order (o valor lado a lado; as três só leem).
# Defeitos do original: nenhum.
ROTINAS = [('award_shortlist_compare_by_name', 0x418340, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x15, 0xbc, 0x23, 0xae, 0x00]),
           ('award_shortlist_compare_by_score', 0x4183f0, [0x8b, 0x4c, 0x24, 0x04, 0x8b, 0x54, 0x24, 0x08]),
           ('award_shortlist_compare_by_votes', 0x418420, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x54, 0x24, 0x08])]
DOCS = ('as ordens da lista de candidatos de um prêmio',
        "the sort orders of an award shortlist",
        [('award_shortlist_compare_by_name', 'award_shortlist_compare_by_name', '0x418340', 'award_shortlist.cpp', 'cdecl',
          'a ordem dos candidatos pelo nome do jogador (o nome no estilo 4; sem jogador, vazio), na comparação de textos do jogo',
          "the order of the candidates by the player's name (the name in style 4; without a player, empty), by the game's string comparison"),
         ('award_shortlist_compare_by_score', 'award_shortlist_compare_by_score', '0x4183f0', 'award_shortlist.cpp', 'cdecl',
          'a ordem dos candidatos pela nota, a maior primeiro (iguais ou sem ordem: 0)',
          'the order of the candidates by score, the highest first (equal or unordered: 0)'),
         ('award_shortlist_compare_by_votes', 'award_shortlist_compare_by_votes', '0x418420', 'award_shortlist.cpp', 'cdecl',
          'a ordem dos candidatos pelos votos, o mais votado primeiro',
          'the order of the candidates by votes, the most voted first')])
