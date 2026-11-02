# A Liga dos Campeões: a tabela das rodadas das fases de grupos (european_cup.cpp):
# - 0x583c50 (EuropeanCup::create_round_table, 1776 bytes até 0x584340, método +0x3c da Liga dos Campeões, quadro SEH
#   pelas datas locais da segunda fase, sem destrutor no caminho normal): nas fases de grupos a tabela de 6 rodadas
#   (as datas de setembro e outubro da primeira fase, metade A..D e metade E..H, e de novembro a março da segunda,
#   metade A..B e C..D) com o jogo 0 de cada rodada vazio; as preliminares (−1) e o mata-mata (12 em diante, com as
#   rotinas da copa 0x521e60/0x521eb0/0x521f10) ficam com a original (a cópia a chama pelo trampolim). Roda 8 vezes no
#   sorteio da primeira fase (agosto de 2001, 0x585290: cqg_4), 4 no da segunda (6/11/2001, 0x585ab0: cqg_6) e pelas
#   posições previstas (0x586900: cqg_5, a fase 0xc, mata-mata, com a original), na carreira automática até 2002-06-01.
# Igual no cm0102-1080.exe (inteira). Chama originais não portadas: 0x944e46 (malloc) e a caixa; o resto já em Rust
# (0x68a160, 0x68a1c0, 0x549f70, 0x54c8e0, 0x417c10).
# Convenções conferidas nos pontos de chamada da original (lição da 258ª):
# - quem chama a 0x583c50: 0x585852 (push 0; push &id; push &rodadas; push g; mov ecx, edi; call [eax+0x3c]), 0x5860b0
#   (o mesmo com h + 8) e 0x586a08 (push 0; push &id; push &rodadas; push 0xc; mov ecx, edi) -> thiscall, ret 0x10;
#   devolve a tabela; a fase é lida como byte (mov al, byte ptr [esp + 0x10] na entrada).
# - dentro da 0x583c50: 0x944e46 push 0x186; add esp, 4 -> cdecl; 0x68a160 9 push (a tabela, i, dia, mês, anos, semana,
#   2, o ano, 0); add esp (acumulado) -> cdecl (o ano no dx/ax/cx com lixo nos bytes altos; a 0x68a160 só usa os 16 bits
#   de baixo de ano + base: a cópia passa o ano limpo); 0x68a1c0 push 0; push −1 ×3; push 0; push r; push tabela; add
#   esp, 0x1c -> cdecl; 0x549f70 push 2; push ano + 1; push 2; push 1; lea ecx, data -> thiscall, ret 0x10 (o ano + 1
#   no ax: lido como word); 0x54c8e0 push 1; push &saída; lea ecx, data -> thiscall, ret 8; 0x417c10 push fonte; push
#   &saída; lea ecx, data -> thiscall, ret 8; a caixa (aviso_em).
# Comparador: lado a lado nas fases de grupos (a original com as saídas de quem chama, a cópia com saídas próprias; as
# saídas e os bytes gravados das duas tabelas iguais, a da cópia liberada; o resto da memória alocada não é escrito por
# nenhuma das duas; a data estática da 0x54c8e0 fica com o mesmo valor nas duas); as outras fases só a original.
# Determinismo: as datas locais começam zeradas na cópia e são escritas pelo construtor antes de lidas; nada lê pilha,
# heap ou endereço como valor.
SAVE = False
ROTINAS = [('campeoes_rodadas_da_fase', 0x583c50, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00])]
DOCS = ('a Liga dos Campeões: a tabela das rodadas das fases de grupos (as datas das duas fases, por metade)',
        'the Champions League: the round schedule of the group stages (the dates of both phases, by half)',
        [('campeoes_rodadas_da_fase', 'EuropeanCup::create_round_table', '0x583c50', 'european_cup.cpp', 'thiscall, ret 0x10',
          'nas fases de grupos, a tabela de 6 rodadas (as datas da primeira fase em setembro e outubro e da segunda de novembro a março, por metade) com o jogo 0 de cada rodada vazio; as preliminares e o mata-mata ficam com a original',
          'in the group stages, the 6-round schedule (phase 1 dates in September and October and phase 2 from November to March, by half) with fixture 0 of each round empty; the qualifying rounds and the knockout stay with the original')])
CAMPOS = [('liga dos campeões +0x40 ano; valor; lê (campeoes_rodadas_da_fase)', 'champions league +0x40 year; value; reads (campeoes_rodadas_da_fase)'),
          ('tabela das rodadas (0x41 bytes por rodada): +0 dia do ano, +2 ano − base, +4 código, +0x3d valor; jogo 0 em +5 (+5 valor, +9 a, +0xa b, +0xb c); valor; grava pela 0x68a160 e 0x68a1c0 (campeoes_rodadas_da_fase)',
           'round schedule (0x41 bytes per round): +0 day of year, +2 year − base, +4 code, +0x3d value; fixture 0 at +5 (+5 value, +9 a, +0xa b, +0xb c); value; writes through 0x68a160 and 0x68a1c0 (campeoes_rodadas_da_fase)')]
INTERESSA = ['CMRTE']
