# Os jogos da temporada da MLS (usa_mls.cpp; leitor cqy, 12/11/2026). Roda 2 vezes na carreira de 8 ligas até
# 2002-06-01 (docs/cobertura_competicoes.txt): na criação da carreira e na virada da MLS. Monta os jogos da primeira
# divisão (League::create_fixtures), as fases da segunda e da terceira (já em Rust: create_second_division_stage e
# create_third_division_stage, cqs_1) e os jogos entre as divisões, pela tabela da fase −2: as rodadas de linhas (2, 5,
# 8, 12, 15 e 18: a linha i de uma divisão contra a linha i de outra, as duplas pela contagem mod 3) e as comuns (seis
# jogos: dois da primeira contra a segunda, dois da primeira contra a terceira, dois da segunda contra a terceira, as
# linhas por uma conta mod 4 sobre a contagem das rodadas comuns); o mando troca pela paridade da contagem.
# - 0x9086e0 UsaMls::create_season_fixtures (usa_mls.cpp), igual no cm0102-1080.exe (inteira).
# Convenções conferidas no ponto de chamada e dentro:
# - a rotina: mov ecx, liga; call -> thiscall, ret 0 (devolve 1; quadro SEH sem destrutor no caminho normal);
# - a virtual +0x3c (push 0; push &+0x3a; push &+0xa9; push fase; mov ecx, esi -> thiscall, ret 0x10), com −3, −2 e −1;
# - League::create_fixtures 0x6835c0, 0x908fc0 e 0x9091e0 (mov ecx, esi -> thiscall, ret 0); free 0x9452ca (push;
#   add esp, 4 -> cdecl); Date::Date 0x549ef0 (push ano; push dia; lea ecx, local -> thiscall, ret 8);
#   Stadium::set_fixture_venue 0x85c260 (push +0x24; push &jogo; push [+0x3d da rodada]; mov ecx, [0xdd7ef4] ->
#   thiscall, ret 0xc); FIXTURE_MANAGER::add_fixture 0x5aa680 (push 1; push &jogo; mov ecx, [0xae2a58] -> thiscall,
#   ret 8).
# Os textos "00_", "0_0" e "_00" (0xaa2094/98/9c) dizem quais das três tabelas (liga, fase 0, fase 1) jogam; viraram
# as constantes MLS_PAIR_*. O registro do jogo (0x4f bytes na pilha) da original guarda lixo da pilha nos bytes que ela
# não grava (+4..+7, +0x18, +0x2e, +0x2f, +0x39, +0x3c e os bits 11..15 de +0x4d, lidos e regravados pelo and/or); a
# cópia põe 0 (o mesmo do create_fixtures, cqd_5); +4 e +0x18 o set_fixture_venue grava.
# Comparador: só a original (cria as fases e põe os jogos no calendário: não roda duas vezes), com o plano: devolve 1,
# +0x3a = 0x413, a tabela da fase −1 e as duas fases existem. A prova do que se joga é a carreira (as mutações).
# Determinismo: nada de sorteio; só lixo de pilha no registro (acima).
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
SAVE = False
ROTINAS = [('usa_mls_create_season_fixtures', 0x9086e0, [0x6a, 0xff, 0x68, 0xb8, 0x53, 0x96, 0x00])]
DOCS = ('os jogos da temporada da MLS: os da primeira divisão, as fases da segunda e da terceira e os jogos entre as divisões',
        "the MLS season fixtures: the first division's own, the second and third division stages and the cross-division fixtures",
        [('usa_mls_create_season_fixtures', 'UsaMls::create_season_fixtures', '0x9086e0', 'usa_mls.cpp', 'thiscall, ret 0',
          'regra: a MLS monta os jogos da primeira divisão (turno e returno), cria a segunda e a terceira divisões como fases e marca os jogos entre as divisões pela tabela da fase −2 (categoria 2; 5 nas três últimas rodadas): nas rodadas 2, 5, 8, 12, 15 e 18, cada linha (1ª a 4ª) de uma divisão contra a mesma linha de outra (primeira × segunda e segunda × terceira, depois primeira × terceira e primeira × segunda, depois segunda × terceira e primeira × terceira, em ciclo); nas outras, seis jogos (dois de cada par de divisões), as linhas por uma conta mod 4 sobre quantas rodadas comuns já houve; o mando troca a cada rodada; no fim, a tabela da própria fase (−1) e o código 0x413',
          "rule: the MLS builds the first division's fixtures (home and away), creates the second and third divisions as stages and schedules the cross-division matches on the stage −2 table (category 2; 5 in the last three rounds): in rounds 2, 5, 8, 12, 15 and 18, each row (1st to 4th) of one division against the same row of another (first × second and second × third, then first × third and first × second, then second × third and first × third, in a cycle); in the others, six matches (two of each pair of divisions), the rows by a mod-4 count over how many ordinary rounds have gone; home and away swap every round; at the end, the table of its own stage (−1) and the code 0x413")])
CAMPOS = [('liga +0xc fases (as divisões 2 e 3), +0xb1 tabela (linhas de 0x3b, +0 o clube), +0xba tabela de rodadas, +0xa9 rodadas, +0x3a código da fase; ponteiro/valor; lê e grava (+0xba, +0x3a)',
           'league +0xc stages (divisions 2 and 3), +0xb1 table (0x3b-byte rows, +0 the club), +0xba round table, +0xa9 rounds, +0x3a stage code; pointer/value; reads and writes (+0xba, +0x3a)')]
INTERESSA = []
