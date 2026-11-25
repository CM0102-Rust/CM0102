# As cláusulas de não acesso e o fim das cláusulas de divisão de um clube (contract_manager.cpp; leitor rx07, rodada 6,
# 25/11/2026). Simulação: só rodam no fim das ligas que usam o League::promote_and_relegate (0x689c80, Rust) e o
# League::move_club_to_division (0x6830b0, Rust): Itália e Espanha antes de 2002-08-01 (592 e 151 chamadas no mundo).
# Iguais no cm0102-1080.exe.
# Convenções conferidas na desmontagem:
# - 0x4cced0 CONTRACT_MANAGER::trigger_non_promotion_clauses thiscall(clube, liga) ret 8 (quem chama: 0x689c80 e
#   0x405ef0, push liga; push clube; mov ecx, [0xae1050]); prólogo sub esp, 0x20; mov eax, [esp+0x24] (7).
# - 0x4cd120 CONTRACT_MANAGER::reset_division_clauses thiscall(clube, zera_numeros) ret 8 (quem chama: 0x6830b0);
#   prólogo push ecx; push ebx; push ebp; mov ebp, [esp+0x10] (7). O eax da volta não é lido.
# - 0x540a50 partner_club cdecl(clube, &é_A, 1) (Rust); o é_A da 0x4cced0 é um local sem início, mas a 0x540a50 o
#   escreve sempre que devolve um parceiro (o club_and_b_team).
# - 0x54c5b0 Date::operator+ thiscall(data, saída, dias) ret 8; 0x8e3db0
#   TRANSFER_MANAGER::post_release_clause_activated_news thiscall([0xde1f54], pessoa, liga, 0, contrato) ret 0x10;
#   0x85a850 SQUAD_MANAGER::set_squad_number thiscall([0xdd7ef0], pessoa, clube, 0) ret 0xc.
# Comparadores: o plano antes (o byte de cada cláusula de cada contrato principal com o clube e o time B), só a
# original (a notícia e os números de camisa não rodam duas vezes) e os bytes conferidos.
SAVE = False
LIGAS = 'ENG KOR ARG ITA FRA ESP AUS'
ATE = '2002-08-01'
ROTINAS = [('trigger_non_promotion_clauses', 0x4cced0, [0x83, 0xec, 0x20, 0x8b, 0x44, 0x24, 0x24]),
           ('reset_division_clauses', 0x4cd120, [0x51, 0x53, 0x55, 0x8b, 0x6c, 0x24, 0x10])]
_F = 'contract_manager.cpp'
DOCS = ('as cláusulas de não acesso e o fim das cláusulas de divisão',
        'the non-promotion clauses and the division clauses reset',
        [('trigger_non_promotion_clauses', 'CONTRACT_MANAGER::trigger_non_promotion_clauses', '0x4cced0', _F, 'thiscall, ret 8',
          'dispara as cláusulas de não acesso de um clube que não subiu: no clube e no seu time B (um time B não faz nada: quem cuida é o time A), cada pessoa do elenco (50 vagas) cujo contrato principal é com o clube, com a cláusula de não acesso ainda por disparar e começado há pelo menos 45 dias, tem a cláusula disparada (2) e recebe a notícia da cláusula de rescisão ativada (tipo 0, com a liga)',
          'triggers the non-promotion clauses of a club that did not go up: at the club and at its B team (a B team does nothing: its A team handles it), every squad member (50 slots) whose main contract is with the club, with the non-promotion clause still to trigger and started at least 45 days ago, gets the clause triggered (2) and the release clause activated news is posted (type 0, with the league)'),
         ('reset_division_clauses', 'CONTRACT_MANAGER::reset_division_clauses', '0x4cd120', _F, 'thiscall, ret 8',
          'quando o clube muda de divisão: com a opção ligada, cada pessoa do elenco perde o número da camisa; depois, no clube e no seu time B (um time B para aí), cada contrato principal com o clube perde a cláusula de rebaixamento e a de não acesso que ainda estavam por disparar',
          'when the club changes division: with the option set, every squad member loses his squad number; then, at the club and at its B team (a B team stops there), every main contract with the club drops the relegation and non-promotion clauses still to trigger')])
CAMPOS = []
INTERESSA = []
