# As regras da partida e o amistoso: o alcance das regras de escalação da partida (TRANSFER_MANAGER::
# get_fixture_rules_scope 0x8d2fa0, pelas 0x8cf5e0 e 0x8cf6b0; 0x669470 original, 0x5c6710 já em Rust), o motivo de um
# jogador não poder jogar (TRANSFER_MANAGER::get_ineligibility_code 0x8cf5e0, pela escalação automática 0x6dbc50 e a
# da partida 0x725f70 já em Rust, telas do clube, olheiros; 0x53b900 e 0x418f60 originais; as 28 regras de +0x8ac
# respondem pela virtual +0x18) e a vaga do amistoso marcado (Friendly::find_scheduled_friendly_slot 0x5c6740, pelas
# 0x5c5f40/0x5c6570/0x5c87b0 dos amistosos; quadro SEH sem destrutor: a Data local 0x549ef0 só guarda dia e ano).
# Simulação: as mutações pela carreira e a confirmação no jogo real comparando. Sem caixa, sem sorteio, sem leitura de
# lixo. Comparadores: o alcance lado a lado pelas três saídas; o motivo e a vaga pelo valor (o motivo só pela original
# quando o jogador tem a marca de ausente, que a 0x418f60 pode tirar).
SAVE = False
ROTINAS = [('escopo_das_regras_da_partida', 0x8d2fa0, [0x8b, 0x15, 0x84, 0xf6, 0x9c, 0x00]),
           ('codigo_de_inelegibilidade', 0x8cf5e0, [0x83, 0xec, 0x08, 0x53, 0x55]),
           ('vaga_do_amistoso', 0x5c6740, [0x6a, 0xff, 0x68, 0xf8, 0xa0, 0x95, 0x00])]
DOCS = ('as regras da partida e o amistoso: o alcance das regras de escalação da partida, o motivo de um jogador não poder jogar e a vaga do amistoso marcado',
        'the match rules and the friendly: the scope of the match selection rules, the reason a player cannot play and the slot of a scheduled friendly',
        [('escopo_das_regras_da_partida', 'TRANSFER_MANAGER::get_fixture_rules_scope', '0x8d2fa0', 'transfer_manager.cpp', 'thiscall, ret 0x10',
          'reservas: a nação do mandante e tipo 1; amistoso: 0xff; senão a nação-chave da competição (0x669470), o continente (+0x59) e tipo 4 (0, 1), 2 (3 a 7) ou 1',
          'reserves: the home nation and type 1; friendly: 0xff; otherwise the key nation of the competition (0x669470), the continent (+0x59) and type 4 (0, 1), 2 (3 to 7) or 1'),
         ('codigo_de_inelegibilidade', 'TRANSFER_MANAGER::get_ineligibility_code', '0x8cf5e0', 'transfer_manager.cpp', 'thiscall, ret 8',
          '9 para o ausente sem licença (fora das seleções); senão a primeira das 28 regras (+0x8ac) do continente ou da nação com o mesmo tipo responde (virtual +0x18); nenhuma: 0',
          '9 for an AWOL player (outside national teams); otherwise the first of the 28 rules (+0x8ac) of the continent or nation with the same type answers (virtual +0x18); none: 0'),
         ('vaga_do_amistoso', 'Friendly::find_scheduled_friendly_slot', '0x5c6740', 'friendly.cpp', 'thiscall, ret 0x10',
          'na lista dos amistosos (+0x34cc), a vaga de 25 bytes do clube com o dia, o ano e o adversário pedidos, ou 0',
          'in the friendlies list (+0x34cc), the 25-byte slot of the club with the requested day, year and opponent, or 0')])
CAMPOS = [('partida +0x14 competição, +0x1c mandante; competição +0 id, +0x59 continente; clube +0x53 nação; nação +0x71 continente; [0x9cf684] competição de reservas; tabela 0xb63d60 nações-chave (72 bytes); gerenciador de transferências +0x8ac 28 regras (+0xa nação, +0xe continente, +0x12 tipo); amistosos +0x34cc lista (+0 clube, +8 vagas, vagas de 25 bytes: +0 dia, +2 ano, +0x10 adversário); lê (escopo_das_regras_da_partida, codigo_de_inelegibilidade, vaga_do_amistoso)',
           'fixture +0x14 competition, +0x1c home club; competition +0 id, +0x59 continent; club +0x53 nation; nation +0x71 continent; [0x9cf684] reserve competition; table 0xb63d60 key nations (72 bytes); transfer manager +0x8ac 28 rules (+0xa nation, +0xe continent, +0x12 type); friendlies +0x34cc list (+0 club, +8 slots, 25-byte slots: +0 day, +2 year, +0x10 opponent); reads (escopo_das_regras_da_partida, codigo_de_inelegibilidade, vaga_do_amistoso)')]
INTERESSA = ['CMRTE']
