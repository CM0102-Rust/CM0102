# A partida de um orgao disciplinar e o destrutor dele (ruling_body.cpp; leitor sx19b, rodada 5, 24/11/2026):
# RulingBody::process_match (0x7e7ff0..0x7e82dd, ret 0x14) e RulingBody::~RulingBody (0x7e7860..0x7e7893, ret).
# Chamadas: process_match pelo Discipline::process_match (0x55ca3c, 0x55ca9d, 0x55cad5: cada orgao que vale para a
# partida, com as bandeiras dos lados que ele cobre); roda na carreira ENG (59.166 vezes no mundo ate 2002-08-01). O
# destrutor pela limpeza da disciplina (0x5557f6, Discipline::~Discipline, e o desenrolar 0x95ff53).
# Convencoes conferidas na desmontagem:
# - 0x7e7ff0 thiscall(orgao, partida, registro da partida, casa, fora, cumprir) ret 0x14, sem retorno usado;
#   0x5c6710 Friendly::is_friendly_competition thiscall([0xae2c74], competicao) ret 4; 0x53b900
#   database_is_national_team cdecl(clube); 0x5409b0 club_is_national_team cdecl(clube); 0x557120
#   Discipline::serve_international_bans thiscall([0xae28f0], clube) ret 4 (ja Rust); 0x4cf5a0 get_contract
#   thiscall([0xae1050], pessoa) ret 4; 0x556e90 update_ban_progress thiscall([0xae28f0], id, regra +5, 1) ret 0xc;
#   0x55e300 handle_referee_assault thiscall([0xae28f0], linha, partida, orgao) ret 0xc (sx09c_5, pelo endereco);
#   vtable +8 process_match_cards thiscall(orgao, linha, partida) ret 8 (sx18_*/sx19_*, pela tabela).
# - 0x7e7860 thiscall(orgao) ret: vtable 0x96f104; 0x68d040 LLIST::~LLIST e 0x944c94 operator delete na lista +7;
#   0x9452ca free nas entradas +0xb.
# Os dois blocos de time do registro: +0x190 + 0x625 x lado, 20 linhas de 0x49 bytes (a conta do original e
# registro + lado + 0x625 x lado... = registro + 0x625 x lado + 0x190 + 0x49 x k).
# Comparadores: process_match so a original (suspensoes, noticias e jogos cumpridos nao se repetem); o destrutor so a
# original, com a vtable conferida depois.
SAVE = False
ROTINAS = [('ruling_body_process_match', 0x7e7ff0, [0x83, 0xec, 0x10, 0x53, 0x55]),
           ('ruling_body_destructor', 0x7e7860, [0x56, 0x8b, 0xf1, 0x57, 0x8b, 0x7e, 0x07])]
DOCS = ('a partida de um órgão disciplinar e o destrutor dele',
        'a ruling body\'s match and its destructor',
        [('ruling_body_process_match', 'RulingBody::process_match', '0x7e7ff0', 'ruling_body/ruling_body.cpp', 'thiscall, ret 0x14',
          'depois de uma partida, para cada lado que o órgão cobre: com "cumprir", cada um dos 50 do elenco (abaixo do total de pessoas, fora do contrato de tipo 5) cumpre um jogo das suspensões na regra do órgão; uma seleção (que não é time de clube) com a marca do lado cumpre de uma vez as suspensões de seleção. Depois, cada uma das 20 linhas do registro do lado (reserva só se entrou) com pessoa e jogador passa pela agressão ao árbitro e, sem ela e fora do contrato de tipo 5 (salvo quando o mandante é seleção), vai para as regras dos cartões do órgão; em amistoso ou na competição reserva só vale, e só na reserva, o vermelho com menos de 2 amarelos e código de falta 10 ou mais',
          'after a match, for each side the body covers: with "serve", each of the 50 squad players (below the person count, not on a type 5 contract) serves one match of his bans under the body\'s rule set; a national side (not a club side) with the side mark serves its national bans at once. Then each of the side\'s 20 record rows (a substitute only if he came on) with a person and a player goes through the referee assault and, without one and off a type 5 contract (unless the home side is a national side), to the body\'s card rules; in a friendly or the reserve competition only a red card with fewer than 2 bookings and an offence code of 10 or more counts, and only in the reserve competition'),
         ('ruling_body_destructor', 'RulingBody::~RulingBody', '0x7e7860', 'ruling_body/ruling_body.cpp', 'thiscall',
          'desfaz o órgão: a vtable da base, a lista de suspensões pendentes destruída e liberada e o vetor de entradas liberado',
          'destroys the body: the base vtable, the pending ban list destroyed and deleted and the entry array freed')])
CAMPOS = []
INTERESSA = []
