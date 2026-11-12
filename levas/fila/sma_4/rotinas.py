# As tabelas das seleções num jogo novo (NationalTeams::init_tables 0x76a840, national_teams.cpp; leitor sma,
# 12/11/2026). Chamada só pela construtora NationalTeams::NationalTeams (0x76a4e0, original) quando não há save.
# Convenções conferidas:
# - a rotina: thiscall(this), ret 0 (o Ghidra a lê como fastcall: só o ecx);
# - 0x945501 (calloc) e 0x944e46 (malloc) da CRT: cdecl;
# - 0x53b620 (cargo de jogador): push byte -> cdecl; 0x53c190 (sair da comissão): push 0; push 1; push 0; push pessoa;
#   push seleção -> cdecl;
# - 0x76abf0 (as reservas, Rust): ecx = this -> thiscall; 0x76c120 (escolher o elenco, Rust): push 0; push 0; push
#   0xae2c90; push tipo; push nação; ecx = this -> thiscall, ret 0x14;
# - a caixa de erro (linhas 0xd9, 0x12c, 0x137): show_error_at.
# Comparador: as tabelas são alocadas uma vez por jogo; só a original roda e os tamanhos que ela deixa são conferidos
# contra a regra (as reservas por nação, os inícios e os totais).
SAVE = False
ROTINAS = [('nationalteams_init_tables', 0x76a840, [0x81, 0xec, 0x08, 0x02, 0x00, 0x00])]
DOCS = ('as tabelas das seleções num jogo novo',
        'the national team tables of a new game',
        [('nationalteams_init_tables', 'NationalTeams::init_tables', '0x76a840', 'national_teams.cpp', 'thiscall',
          'monta a tabela por nação de um jogo novo: conta as pessoas de cada nação (quem tem cargo de jogador numa seleção sai da comissão dela), sem competição atual nem na fila; a reserva principal começa no total acumulado e tem a contagem entre 10 e 100 (com 100, o 1º nível 10 e os outros três 30), a sub-21 entre 10 e 50 (com 50, o 1º nível 5 e os outros 15, senão o tamanho da principal); aloca as duas reservas (as duas do tamanho do total principal), monta-as e, para cada nação a partir da segunda com continente, escolhe os elencos principal e sub-21, limpa as marcas de convocação e põe 24 como maior elenco B',
          'builds the per-nation table of a new game: counts each nation\'s people (anyone in a player job at a national team leaves its staff), no current or queued competition; the senior pool starts at the running total and takes the count within 10..100 (with 100, the first level 10 and the other three 30), the under-21 within 10..50 (with 50, the first level 5 and the others 15, otherwise the senior size); allocates both pools (both of the senior total\'s size), builds them and, for each nation from the second one with a continent, picks the senior and under-21 squads, clears the call-up marks and sets 24 as the largest B squad')])
