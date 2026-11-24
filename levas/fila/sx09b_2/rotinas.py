# A ordem do indice por nacao da disciplina e a contagem de multas de cada clube (leitor sx09b, rodada 5, 23/11/2026):
# - compare_nation_index_entries (0x55e5f0, discipline.cpp, 45 bytes ate o ret de 0x55e61c): o comparador do qsort e
#   do bsearch do indice por nacao (cdecl, dois ponteiros, devolve int). Roda pelo add_ban e release_ban (qsort) e pelo
#   nation_index_range e find_in_nation_index (bsearch), todos ja Rust, que passam o endereco 0x55e5f0;
# - Fine::halve_club_fine_count (0x5a5930, Fine.cpp, 46 bytes ate o ret 4 de 0x5a595b): thiscall(Fine, clube) ret 4;
#   [0x968968] = 0.5, 0x945640 = _ftol. Chamada pela set_season_expectations (0x6a3770, ja Rust) a cada clube na
#   temporada nova;
# - Fine::clear_club_fine_count (0x5a5960, Fine.cpp, 20 bytes ate o ret 4 de 0x5a5971): thiscall(Fine, clube) ret 4.
#   Chamada pela remove_club_manager (0x69b790, ja Rust) quando um tecnico sai.
# A tabela por clube (o ponteiro em Fine+8) e indexada pelo id do clube (o dword +0 do registro), um byte com sinal.
# Prologos sem salto: 0x55e5f0 mov edx,[esp+4]; push esi (5); 0x5a5930/0x5a5960 mov eax,[esp+4]; test eax,eax (6).
# Comparadores: o compare_order (os dois devolvem o mesmo int) e lado a lado sobre o byte do clube.
SAVE = False
ROTINAS = [('compare_nation_index_entries', 0x55e5f0, [0x8b, 0x54, 0x24, 0x04, 0x56]),
           ('halve_club_fine_count', 0x5a5930, [0x8b, 0x44, 0x24, 0x04, 0x85, 0xc0]),
           ('clear_club_fine_count', 0x5a5960, [0x8b, 0x44, 0x24, 0x04, 0x85, 0xc0])]
DOCS = ('a ordem do indice por nacao da disciplina e a contagem de multas de cada clube',
        "the order of the discipline nation index and each club's fine count",
        [('compare_nation_index_entries', 'compare_nation_index_entries', '0x55e5f0', 'discipline.cpp', 'cdecl',
          'ordena dois itens do indice por nacao das suspensoes (pessoa, nacao): pela nacao (a diferenca, quando diferem), depois pela pessoa; uma pessoa -1 de qualquer lado (a chave da busca por nacao) casa com qualquer pessoa da nacao (0)',
          'orders two items of the ban nation index (person, nation): by the nation (the difference, when they differ), then by the person; a person of -1 on either side (the key of the search by nation) matches any person of the nation (0)'),
         ('halve_club_fine_count', 'Fine::halve_club_fine_count', '0x5a5930', 'Fine.cpp', 'thiscall, ret 4',
          'na temporada nova, a contagem de multas de jogadores do clube cai a metade (o byte com sinal vezes 0,5, truncado); sem clube, nada',
          "at the new season, the club's count of player fines drops to half (the signed byte times 0.5, truncated); without a club, nothing"),
         ('clear_club_fine_count', 'Fine::clear_club_fine_count', '0x5a5960', 'Fine.cpp', 'thiscall, ret 4',
          'quando o tecnico sai, a contagem de multas de jogadores do clube volta a zero; sem clube, nada',
          "when the manager leaves, the club's count of player fines goes back to zero; without a club, nothing")])
