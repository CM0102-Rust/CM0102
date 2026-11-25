# Os contratos de todas as pessoas na criação do jogo (CONTRACT_MANAGER::initialise_all_staff_contracts 0x4c7560,
# contract_manager.cpp, 1.008 bytes; leitor rx07, rodada 6, 25/11/2026). Simulação, carreira ENG (sem LIGAS): roda uma
# vez, na criação do jogo (Setup 0x828110, Rust), e a carreira de teste cria o jogo do zero. Igual no cm0102-1080.exe.
# A rotina vai de 0x4c7560 ao ret de 0x4c7944 (enchimento até 0x4c7950).
# Convenções conferidas na desmontagem:
# - a rotina: thiscall(gerente de contratos) ret 0, quadro SEH (só para a Date e o STAFF_CONTRACT locais); prólogo
#   push -1; push 0x956c71 (7), sem relativas.
# - 0x85e410 STAFF_CONTRACT::STAFF_CONTRACT num local que nada lê (a cópia não o monta); malloc 0x944e46 (cnt × 80);
#   a caixa "Memory Allocation Error" da linha 0xd7 (0x944cff, 0x90d130, sprintf 0x944c9f com os argumentos do
#   0x90d130 que ficaram na pilha, 0x66f4e0 ×2, 0x5e8290), e volta sem sair do jogo (o award_load_error_box).
# - 0x4d0c60 contract_manager_calc_player_value thiscall(this, pessoa, clube, 1, -1, 0) ret 0x14 -> staff +0x52;
#   0x549ef0 Date(31, 1900) é só a data vazia comparada (ano, depois dia); 0x54a110 Date::Date; 0x669bd0
#   key_nation_club_in_season cdecl(clube, 0, &data); 0x54c5b0 +10 dias; 0x54cfb0 get_month; 0x90cfc0 utils_random
#   cdecl(6) e depois (0x1c) (o -1 empurrado antes fica como 4º argumento do Date::Date); 0x549f70 Date::Date
#   thiscall(dia, mês, ano, -1) ret 0x10 (só os bytes baixos contam); 0x417c10 Date::operator= thiscall(dst, &tmp, src)
#   ret 8; 0x54c770 Date::add_weeks thiscall(data, &saída, n) ret 8 (G_GAME_START_DATE + 8 semanas; o fim + 52).
# - 0x4c79a0 get_free_staff_contract_space thiscall(this, 1) ret 4; 0x85e960 STAFF_CONTRACT::generate_initial_contract
#   thiscall(contrato, pessoa, valor) ret 8.
# Defeito do original: sem contrato livre, o número da camisa do jogador vai para o endereço 0x3a (contrato nulo).
# Comparador: só a original (aloca e sorteia os fins de contrato); depois, toda pessoa cujo cargo no clube leva
# contrato tem um contrato principal com o id dela, e o número de contratos não passa da capacidade.
SAVE = False
ROTINAS = [('initialise_all_staff_contracts', 0x4c7560, [0x6a, 0xff, 0x68, 0x71, 0x6c, 0x95, 0x00])]
_F = 'contract_manager.cpp'
DOCS = ('os contratos de todas as pessoas na criação do jogo',
        "every person's contract when the game is created",
        [('initialise_all_staff_contracts', 'CONTRACT_MANAGER::initialise_all_staff_contracts', '0x4c7560', _F, 'thiscall, ret 0',
          'monta os contratos do jogo novo: um vetor de 80 bytes por pessoa (sem memória, a caixa "Memory Allocation Error" e nada mais), nenhum em uso; para cada pessoa, apaga o bit 7 das marcas do registro de 79 bytes e calcula o valor; com clube, o fim do contrato vazio (31/1900) vira um dia de 1 a 28 do mês da data da temporada do clube mais 10 dias, de 1 a 6 anos depois do ano atual, e todo fim que não passa do início do jogo mais 8 semanas anda 52 semanas até passar; com seleção, o fim vazio do contrato da seleção vira 10 de junho, de 1 a 5 anos depois; com clube e um cargo que leva contrato (0, 6 a 13 e 15), um contrato livre vira o principal e é gerado com o valor e, para um jogador, guarda o número da camisa da ficha (de 1 a 50) negativo',
          "builds the new game's contracts: an 80-byte array per person (out of memory, the \"Memory Allocation Error\" box and nothing else), none in use; for each person, clears bit 7 of the 79-byte record's flags and works out his value; with a club, an unset contract end (31/1900) becomes day 1 to 28 of the month of the club's season date plus 10 days, 1 to 6 years after the current year, and every end not past the game start plus 8 weeks moves on 52 weeks until it is; with a national team, an unset national contract end becomes 10 June, 1 to 5 years later; with a club and a job that takes a contract (0, 6 to 13 and 15), a free contract becomes his main one and is generated with the value and, for a player, keeps the sheet's squad number (1 to 50) negated")])
CAMPOS = []
INTERESSA = []
