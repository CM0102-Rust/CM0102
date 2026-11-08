# Os amistosos da pré-temporada da IA (friendly.cpp, o que restava fora da fila): Friendly::plan_club_friendlies
# (0x5c4770, 0x3ea bytes, pelo plano da pré-temporada 0x5c27a0, que rodou 2 vezes na base: "cpt_11 planejou a
# pré-temporada"; cada vez marca dezenas de amistosos: "cps_12 planned friendly"), Friendly::add_tournament (0x5c6e00,
# 0xfd bytes, pelo torneio da IA 0x5c3be0 na mesma pré-temporada e pelas telas do clube) e, passageira,
# Friendly::find_human_club_open_invitation (0x5c68b0, 0x61 bytes, pelas telas dos amistosos do humano). Sem SEH. Iguais
# no cm0102-1080.exe (corpo inteiro das três).
# Convenções conferidas nos pontos de chamada da original (lição da 258ª):
# - 0x9452ca (free): push plano; call; add esp, 4 -> cdecl. 0x944e46 (malloc): push 0x7d0; call; add esp, 4 -> cdecl.
# - 0x54cab0 (data menos dias): lea ecx, [liga + 0x39]; push 0x34; push &saída; sem add esp -> thiscall, ret 8.
# - 0x946464 (bsearch): 5 pushes (0x5c5290, 4, conta, ordenados, &chave); add esp, 0x14 -> cdecl.
# - 0x600a50 (clube_humano): 5 pushes (0, 0, 1, 1, clube); add esp, 0x14 -> cdecl(clube, 1, 1, 0, 0).
# - 0x90cfc0 (sorteio): push n; call; add esp, 4 -> cdecl (3, 4, 3).
# - 0x5c4cb0 (Friendly::find_friendly_opponent): 5 pushes (0, faixa, conta, entradas, candidato); mov ecx, this; sem
#   add esp -> thiscall, ret 0x14.
# - 0x5c4380 (Friendly::plan_friendly_on_free_date): 3 pushes; mov ecx, this; sem add esp -> thiscall(this, anfitrião,
#   casa, fora), ret 0xc: (candidato, adversário, candidato) com a faixa do candidato maior, senão (candidato, candidato,
#   adversário).
# - 0x945640 (_ftol) -> `ftol`. 0x945333 (qsort): 4 pushes (0x778e90, 0x14, conta, plano); add esp, 0x10 -> cdecl.
# - 0x417c10 (Date::copy): lea ecx, [torneio + 4]; push início; push &local; sem add esp -> thiscall, ret 8.
# - 0x68d1b0 (próximo da lista): mov ecx, [this + 0x34c8]; sem push -> thiscall.
# Comparadores: a 0x5c4770 só a original (libera e aloca o plano, e a 0x5c4380 aloca as listas dos candidatos: não roda
# duas vezes), conferindo o plano que ela deixa (alocado, capacidade não menor que a conta, +0x34dc zerado, em ordem de
# data pela 0x778e90); a cópia leva os contadores para o modo Rust. A 0x5c6e00 lado a lado no torneio livre e na
# devolução; a 0x5c68b0 lado a lado no cursor da lista e na devolução.
SAVE = False
ROTINAS = [('amistosos_dos_clubes', 0x5c4770, [0x81, 0xec, 0x50, 0x02, 0x00, 0x00]),
           ('acrescentar_torneio', 0x5c6e00, [0x83, 0xec, 0x08, 0x53, 0x56, 0x57]),
           ('convite_aberto_do_clube_humano', 0x5c68b0, [0x56, 0x8b, 0xf1, 0x57, 0x8b, 0x86, 0xc8, 0x34, 0x00, 0x00])]
DOCS = ('os amistosos da pré-temporada da IA: os amistosos dos clubes, o torneio acrescentado e o convite aberto do clube humano',
        'the AI pre-season friendlies: the club friendlies, the added tournament and the human club open invitation',
        [('amistosos_dos_clubes', 'Friendly::plan_club_friendlies', '0x5c4770', 'friendly.cpp', 'thiscall, ret 0xc',
          'refaz o plano de amistosos (+0x34d8); para cada liga ativa na pré-temporada, os clubes da IA da nação (pela busca nos candidatos ordenados) ganham uma faixa pela reputação e um sorteio, e até sorteio(3) + 4 amistosos com adversários achados (0x5c4cb0) em datas livres (0x5c4380), a faixa caindo 20% a cada um; no fim, o plano em ordem de data',
          'rebuilds the friendly plan (+0x34d8); for each league in pre-season, the AI clubs of the nation (by searching the sorted candidates) get a band by reputation and a draw, and up to draw(3) + 4 friendlies with opponents found (0x5c4cb0) on free dates (0x5c4380), the band falling 20% each time; at the end, the plan sorted by date'),
         ('acrescentar_torneio', 'Friendly::add_tournament', '0x5c6e00', 'friendly.cpp', 'thiscall, ret 0x28',
          'preenche o primeiro dos 127 torneios livres com o anfitrião, o tipo, a data de início, os bytes e palavras das regras, o nome e os 4 convidados (marcados), ou 0 sem vaga',
          'fills the first of the 127 free tournaments with the host, the type, the start date, the rule bytes and words, the name and the 4 invited clubs (flagged), or 0 with no free slot'),
         ('convite_aberto_do_clube_humano', 'Friendly::find_human_club_open_invitation', '0x5c68b0', 'friendly.cpp', 'thiscall',
          'o amistoso da lista (+0x34c8) que o clube do técnico humano da vez pediu e que ainda espera adversário, ou 0',
          'the friendly in the list (+0x34c8) that the current human manager club requested and that still waits for an opponent, or 0')])
CAMPOS = [('Friendly +0x34d8 / +0x34dc / +0x34e0 / +0x34e4: o plano dos amistosos (20 bytes), zerado, a conta e a capacidade; ponteiro e valor; libera, aloca e grava (amistosos_dos_clubes)',
           'Friendly +0x34d8 / +0x34dc / +0x34e0 / +0x34e4: the friendly plan (20 bytes), zeroed, the count and the capacity; pointer and value; frees, allocates and writes (amistosos_dos_clubes)'),
          ('[0xb63d60] as 35 ligas ativas (0x48 bytes: +0 a nação, +0x2a o início da temporada, +0x39 a data da pré-temporada); ponteiro e valor; lê (amistosos_dos_clubes)',
           '[0xb63d60] the 35 active leagues (0x48 bytes: +0 the nation, +0x2a the season start, +0x39 the pre-season date); pointer and value; reads (amistosos_dos_clubes)'),
          ('candidato do amistoso +0 o id do clube, +4 a conta de amistosos (byte), +5 a faixa, +6 a nação; valor; lê (amistosos_dos_clubes)',
           'friendly candidate +0 the club id, +4 the friendly count (byte), +5 the band, +6 the nation; value; reads (amistosos_dos_clubes)'),
          ('clube +0x53 a nação, +0x57 a liga, +0x80 a reputação, +0x1db o id comparado com [0x9cf6f8]; ponteiro e valor; lê (amistosos_dos_clubes)',
           'club +0x53 the nation, +0x57 the league, +0x80 the reputation, +0x1db the id compared with [0x9cf6f8]; pointer and value; reads (amistosos_dos_clubes)'),
          ('Friendly +0xb1: os 127 torneios (0x69 bytes: +0 anfitrião, +4 a data, +0xc tipo, +0xd/+0xe bytes, +0xf/+0x11/+0x13 palavras, +0x15 os 4 convidados, +0x25 as marcas, +0x29/+0x2d/+0x65 zerados, +0x31 o nome); ponteiro e valor; grava (acrescentar_torneio)',
           'Friendly +0xb1: the 127 tournaments (0x69 bytes: +0 host, +4 the date, +0xc type, +0xd/+0xe bytes, +0xf/+0x11/+0x13 words, +0x15 the 4 invited clubs, +0x25 the flags, +0x29/+0x2d/+0x65 zeroed, +0x31 the name); pointer and value; writes (acrescentar_torneio)'),
          ('Friendly +0x34c8: a lista dos amistosos (+2 o começo, +0xa o cursor; amistoso +8 ainda sem data, +0xc o clube); ponteiro; lê e grava o cursor (convite_aberto_do_clube_humano)',
           'Friendly +0x34c8: the friendly list (+2 the head, +0xa the cursor; friendly +8 still without date, +0xc the club); pointer; reads and writes the cursor (convite_aberto_do_clube_humano)')]
INTERESSA = []
