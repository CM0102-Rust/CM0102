# O construtor do TRANSFER_MANAGER e as regras de transferencia das nacoes (transfer_manager.cpp; leitor sx02b, rodada
# 5, 23/11/2026). Cada uma roda 1 vez na carreira ENG (o construtor no create_game, LoadStep 0x22 do setup.rs, e as
# regras dentro dele).
# Convencoes conferidas:
# - 0x8bba30: thiscall(this, stream), ret 4, devolve this; o eh vector constructor iterator 0x9463af e stdcall(this+0x18,
#   0xe, 0x96, LLIST::LLIST, LLIST::~LLIST); 0x549ef0 (Date::Date) thiscall(&tmp, 0, 0); 0x417c10 (Date::operator=)
#   thiscall(+0x84d, &tmp2, &tmp); calloc(G_STAFF_COUNT, 8) e calloc(G_CLUB_COUNT, 0x10) cdecl; 0x8bc020 (load_from_disk)
#   thiscall(this, stream) ret 4; operator new(0xe0) e 0x839a80 (Shortlist_Manager::Shortlist_Manager) thiscall(sm, 0)
#   ret 4; 0x8c6e60 (init_work_permits, ja em Rust) thiscall(this); as caixas de memoria pelo show_out_of_memory_box
#   (linhas 0xa3, 0xab, 0xc0); a checagem de CD com GetLogicalDriveStringsA/GetDriveTypeA pela IAT (o exe sem CD,
#   cm0102-1080, troca o jne em 0x8bbaf2 por nops e o test em 0x8bbb34 por xor: a copia le esses bytes).
# - 0x8d26b0: thiscall(this), ret 0; operator new(0x19) por objeto; o europeu 0x583280 thiscall(obj, 0, -1,
#   [KEY_CONTINENT_EUROPE], 2, 0) (push 0; push 2; push continente; push -1; push 0) e os nacionais thiscall(obj, id,
#   [KEY_NATION_..], -1, 1, 0) (push 0; push 1; push -1; push nacao; push id), ret 0x14; as vagas 0x10 e 0x11 construidas
#   na ordem 0x11 (Noruega, +0x8f0) e depois 0x10 (Irlanda do Norte, +0x8ec); malloc(4 x conta) e qsort(tabela, conta,
#   4, 0x8e5590) cdecl.
# Comparadores: so a original nos dois (alocam objetos que nao se constroem duas vezes); o das regras reconstroi a tabela
# das datas das janelas a partir dos objetos da original (mesmo qsort) e confere a nacao de cada objeto nacional.
SAVE = False
ROTINAS = [('construct_transfer_manager', 0x8bba30, [0x6a, 0xff, 0x68, 0x88, 0x39, 0x96, 0x00]),
           ('setup_transfer_rules', 0x8d26b0, [0x6a, 0xff, 0x68, 0x98, 0x42, 0x96, 0x00])]
DOCS = ('o construtor do gerente de transferências e as regras de transferência das nações',
        "the transfer manager's constructor and the nations' transfer rules",
        [('construct_transfer_manager', 'TRANSFER_MANAGER::TRANSFER_MANAGER', '0x8bba30', 'transfer_manager.cpp', 'thiscall, ret 4',
          'regra: monta os 150 dias da agenda de ofertas, a data e as listas de parcelas e de transferências livres; confere o CD; zera ofertas, negociações, transferências passadas e futuras, percentuais de revenda, bônus por jogos, permissões de trabalho e negociações de empréstimo; a data atual vazia e a fatia 0; as regras de transferência; os registros por pessoa (8 bytes) e por clube (16 bytes) zerados (sem memória: a caixa e para); a vaga livre −1; com o arquivo do jogo salvo, lê dele e para; senão um gerente de listas de observação novo (sem memória: a caixa e para) e as permissões de trabalho iniciais',
          "rule: builds the 150 days of the offer schedule, the date and the instalment and free transfer lists; checks the CD; zeroes the offers, negotiations, past and future transfers, sell-on percentages, appearance money, work permits and loan negotiations; the current date empty and slice 0; the transfer rules; the per-person (8 bytes) and per-club (16 bytes) records zeroed (out of memory: the box and stop); the free slot −1; with the saved game, reads from it and stops; otherwise a new shortlist manager (out of memory: the box and stop) and the initial work permits"),
         ('setup_transfer_rules', 'TRANSFER_MANAGER::setup_transfer_rules', '0x8d26b0', 'transfer_manager.cpp', 'thiscall, ret 0',
          'regra: 28 objetos de regras: o europeu (continente Europa) e 27 nacionais (Argentina, Austrália, Bélgica, Brasil, Croácia, Dinamarca, Inglaterra, Finlândia, França, Alemanha, Grécia, Holanda, Irlanda, Itália, Japão, Noruega, Irlanda do Norte, Polônia, Portugal, Rússia, Escócia, Espanha, Suécia, Turquia, Estados Unidos, País de Gales, Coreia do Sul); depois a tabela com a data de abertura e a de fechamento de cada janela de todos eles, ordenada por mês e dia',
          "rule: 28 rules objects: the European one (continent Europe) and 27 national ones (Argentina, Australia, Belgium, Brazil, Croatia, Denmark, England, Finland, France, Germany, Greece, Holland, Ireland, Italy, Japan, Norway, Northern Ireland, Poland, Portugal, Russia, Scotland, Spain, Sweden, Turkey, United States, Wales, South Korea); then the table with the opening and closing date of every window of them all, sorted by month and day")])
CAMPOS = []
INTERESSA = []
