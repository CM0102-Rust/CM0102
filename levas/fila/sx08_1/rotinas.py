# Database.cpp (leitor sx08, rodada 5, 23/11/2026): a ordem de pessoas por data de nascimento (qsort) e o
# inicializador estatico da data da mudanca das regras de transferencia.
# - 0x540130: cdecl(a, b) -> eax, comparador do crt_qsort (SQUAD_MANAGER::allocate_squad_number 0x85a470 e a lista de
#   numeros do squad_manager.cpp, ja em Rust, passam 0x540130 ao qsort); quadro SEH so por causa das duas Date locais
#   (0x54a110: dia 40 de 1972); ano comparado sem sinal (ja/jb), dia com sinal (setg/setl); empate:
#   0x53dba0(pessoa, buf, 0x7d0, 4) e 0x90c1d0(a, b, 1, 0). O mapa diz "mais novo primeiro"; a desmontagem poe o mais
#   velho primeiro (b nascido depois -> -1).
# - 0x524cd0: cdecl(), inicializador estatico do CRT (roda no _initterm, depois do DllMain, com os desvios ja postos
#   no nosso exe; no exe com patch de terceiros a instalacao e adiada e ele roda original). Date::Date 0x549f70 e
#   thiscall(data, 31, 7, 2001, -1), ret 0x10; atexit 0x94627a cdecl(0x524d00).
# Fora do item: 0x524d00 (destrutor vazio, um ret: corpo vazio, sem desvio).
# Tipo: simulacao (a carreira ENG ate 2002-06-01 roda as duas; a 0x524cd0 uma vez, na partida do jogo).
SAVE = False
ROTINAS = [('database_compare_staff_by_birth_date', 0x540130, [0x6a, 0xff, 0x68, 0x46, 0x79, 0x95, 0x00]),
           ('init_transfer_rule_change_date', 0x524cd0, [0x6a, 0xff, 0x68, 0xd1, 0x07, 0x00, 0x00])]
DOCS = ('a ordem das pessoas por data de nascimento e a data das novas regras de transferência',
        'the order of people by date of birth and the date of the new transfer rules',
        [('database_compare_staff_by_birth_date', 'database_compare_staff_by_birth_date', '0x540130', 'Database.cpp', 'cdecl',
          'compara duas pessoas (ponteiros, para o qsort) pela data de nascimento, a mais velha primeiro: o ano (sem sinal) decide, depois o dia do ano (com sinal); na mesma data, o nome "sobrenome, nome" sem diferenciar maiúsculas. Pessoa nula vale a data padrão (dia 40 de 1972) e nome vazio. O mapa da comunidade diz "mais nova primeiro", mas a desmontagem põe a mais velha primeiro',
          'compares two people (pointers, for qsort) by date of birth, oldest first: the year (unsigned) decides, then the day of the year (signed); on the same date, the name "surname, first name" case-insensitively. A null person counts as the default date (day 40 of 1972) and the empty name. The community map says "youngest first", but the disassembly puts the oldest first'),
         ('init_transfer_rule_change_date', 'init_transfer_rule_change_date', '0x524cd0', 'Database.cpp', 'cdecl',
          'inicializador estático do exe (antes do WinMain): põe em G_TRANSFER_RULE_CHANGE_DATE a data em que as novas regras de transferência entram em vigor, 31 de agosto de 2001, e registra o destrutor (vazio) no atexit. A data decide a notícia do novo sistema de transferências e os contratos assinados antes da mudança',
          'static initialiser of the exe (before WinMain): sets G_TRANSFER_RULE_CHANGE_DATE to the date the new transfer rules come into force, 31 August 2001, and registers the (empty) destructor with atexit. The date decides the news of the new transfer system and the contracts signed before the change')])
CAMPOS = []
INTERESSA = []
