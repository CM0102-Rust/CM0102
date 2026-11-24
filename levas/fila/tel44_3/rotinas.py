# Esvaziar a lista de candidatos de um prêmio (AWARD_SHORTLIST::clear 0x418320, award_shortlist.cpp; leitor tel44,
# rodada 5, 23/11/2026). Chamada pelos prêmios do ano das nações (nation_awards.cpp: 0x7683c0 em 0x76850f, antes de
# encher a lista pelas notas da temporada, e 0x768b20 em 0x768bf1, depois de premiar os três primeiros). Simulação
# (prêmios), sem LIGAS.
# Convenção conferida na desmontagem: thiscall(lista), sem argumentos na pilha; grava a palavra +4 = 0 e 8 dwords −1
# a partir de +8 (os 16 votos, palavras); deixa −1 em eax, que os dois chamadores não leem.
# Prólogo: mov edx, ecx; push edi; mov ecx, 8 (8 bytes). Sem RELATIVAS. Corpo igual no cm0102.exe e no 1080.
# Comparador: lado a lado sobre +4..+0x28.
# Defeitos do original: nenhum.
ROTINAS = [('award_shortlist_clear', 0x418320, [0x8b, 0xd1, 0x57, 0xb9, 0x08, 0x00, 0x00, 0x00])]
DOCS = ('esvaziar a lista de candidatos de um prêmio',
        'emptying an award shortlist',
        [('award_shortlist_clear', 'AWARD_SHORTLIST::clear', '0x418320', 'award_shortlist.cpp', 'thiscall',
          'esvazia a lista de candidatos: nenhuma entrada e os 16 votos dos treinadores em −1',
          "empties the shortlist: no entries and the 16 managers' votes at -1")])
