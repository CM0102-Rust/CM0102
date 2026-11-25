# The Club World Championship's entrants (world_club_champ.cpp; reader rxf, round 6, 25/11/2026). Simulation with the
# 8 leagues and ATE 2003-08-15, as rx10_11..rx10_14: the draw is on 5 Feb 2003 (world_club_champ_update_for_date, in
# Rust, calls it by address).
# - 0x92a8b0 WorldClubChamp::pick_entrants(): thiscall, ret 0, returns 1 (0 with the malloc box). Prologue
#   `sub esp, 0x208` (6 bytes, no relative call).
# - Calls: malloc 0x944e46; HostCountry::find_hosts 0x5fa730 thiscall on [0xb5e134](comp id, year word, &first,
#   &second, 1) ret 0x14 (al); utils_random 0x90cfc0 cdecl([0xae2350]); WorldClubChamp::pick_entrant 0x92ae60
#   thiscall(continent, nation, list, slot, club comp) ret 0x14 (still in the exe, rx10_15/FORA.md); the non-fatal
#   "Error" box (lines 0x3fb, 0x402).
# - The pushes, conferred: slots 0,1 [0x9cfa08] Africa; 2,3 [0x9cfa0c] Asia; 4 [0x9cfa10] Europe; 5 Europe with
#   [0x9cf6f0] (the UEFA Cup's id) as the last argument; 6,7 [0x9cfa14] North America; 8 [0x9cfa18] Oceania; 9,10
#   [0x9cfa1c] South America; 11 (-1, host). The nation argument of slots 0..10 is -1, the 4th the slot.
# Same bytes in cm0102-1080. Comparer: only the original, with the plan (size 12 and the list).
SAVE = False
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
ATE = '2003-08-15'
ROTINAS = [('world_club_champ_pick_entrants', 0x92a8b0, [0x81, 0xec, 0x08, 0x02, 0x00, 0x00])]
DOCS = ('os participantes do Mundial de Clubes',
        'the Club World Championship entrants',
        [('world_club_champ_pick_entrants', 'WorldClubChamp::pick_entrants', '0x92a8b0', 'world_club_champ.cpp', 'thiscall, ret 0',
          'regra: o Mundial de Clubes tem 12 participantes: dois clubes da África, dois da Ásia, dois da Europa (o segundo pela Copa da UEFA), dois da América do Norte, um da Oceania, dois da América do Sul e um da nação-sede; sem sede achada, uma nação sorteada',
          "rule: the Club World Championship has 12 entrants: two clubs from Africa, two from Asia, two from Europe (the second through the UEFA Cup), two from North America, one from Oceania, two from South America and one from the host nation; with no host found, a random nation")])
