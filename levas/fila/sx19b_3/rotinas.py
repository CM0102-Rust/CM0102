# Um orgao disciplinar no save: a carga e a gravacao (ruling_body.cpp; leitor sx19b, rodada 5, 24/11/2026):
# RulingBody::load (0x7e78a0..0x7e7b42 ret 4, mais o erro ate 0x7e7b79) e RulingBody::save (0x7e7b80..0x7e7d49, ret 4).
# Chamadas: a carga pelo construtor da base com o save (0x7e77d0, construct_ruling_body, ja Rust: chama pelo endereco
# FN_RULING_BODY_LOAD, que passa pelo desvio); a gravacao pelo Discipline::save (0x555f41, as 43 regras).
# Convencoes conferidas na desmontagem:
# - 0x7e78a0 thiscall(orgao, item do save) ret 4 -> 1/0 (neg/sbb do retorno da vtable +0x18); quadro SEH so para as duas
#   datas do no local (0x54a110 Date::Date em no+4 e no+0x14, sobrescritas pela leitura);
# - 0x7e7b80 thiscall(orgao, item do save) ret 4 -> o eax da vtable +0x1c (no erro, o da caixa: o chamador ignora);
# - 0x932c30 zread / 0x932e10 zwrite cdecl(buf, tamanho, quantos, [item]); 0x944e46 malloc; 0x68d0d0 LLIST::add_tail
#   thiscall([+7], no, 0x70) ret 8; 0x68d1b0 LLIST::get_next thiscall([+7]); a caixa: _splitpath 0x944cff, 0x90d130,
#   sprintf "v%s %s.%s %d", 0x5e8290("Error", texto, 0), [0xb67a34] = 0 (show_error_at).
# O no da suspensao pendente (0x70 bytes) leva a partida em +0x20; na carga os ponteiros dela (competicao +0x34,
# estadio +0x38, mandante +0x3c, visitante +0x40) voltam dos ids (+0x20, +0x24, +0x2c, +0x30) pelas tabelas 0xae23d0
# (0x6b bytes), 0xae23b0 (0x4e) e 0xae23b4 (0x245).
# Comparadores: so a original (leem e gravam o arquivo); a carga confere depois capacidade = contagem.
SAVE = True
CARGA = True  # a carga so roda ao carregar: a prova do save carrega o save de ferias com e sem a leva
ROTINAS = [('ruling_body_load', 0x7e78a0, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('ruling_body_save', 0x7e7b80, [0x81, 0xec, 0x04, 0x02, 0x00, 0x00])]
DOCS = ('um órgão disciplinar no save: a carga e a gravação',
        'a ruling body in the saved game: loading and saving',
        [('ruling_body_load', 'RulingBody::load', '0x7e78a0', 'ruling_body/ruling_body.cpp', 'thiscall, ret 4',
          'lê do save o total de entradas e, com entradas, o vetor delas (14 bytes cada; a capacidade fica igual ao total); depois o número de suspensões pendentes e cada nó de 0x70 bytes, cuja partida volta a apontar para a competição, o estádio e os dois clubes pelos ids antes de ir para o fim da lista do órgão; por fim os dados próprios do órgão (vtable +0x18): 1 quando respondem. Leitura curta ou sem memória: a caixa "Error" do ruling_body.cpp com a linha do campo e 0',
          'reads from the save the entry count and, with entries, their array (14 bytes each; the capacity equals the count); then the number of pending bans and each 0x70-byte node, whose fixture points again to the competition, the stadium and both clubs from their ids before it goes to the end of the body\'s list; last the body\'s own data (vtable +0x18): 1 when they answer. A short read or no memory: the ruling_body.cpp "Error" box with the field line and 0'),
         ('ruling_body_save', 'RulingBody::save', '0x7e7b80', 'ruling_body/ruling_body.cpp', 'thiscall, ret 4',
          'grava no save o total de entradas e o vetor delas, o número de suspensões pendentes (0 sem lista) e cada nó da lista desde o começo, e por fim os dados próprios do órgão (vtable +0x1c). Gravação curta: a caixa "Error" com a linha do campo',
          'writes to the save the entry count and their array, the number of pending bans (0 without a list) and each node of the list from the head, and last the body\'s own data (vtable +0x1c). A short write: the "Error" box with the field line')])
CAMPOS = [('suspensão pendente (nó de 0x70 bytes) +0x20: a partida (Fixture, ids e ponteiros religados na carga)',
           'pending ban (0x70-byte node) +0x20: the fixture (Fixture, ids and pointers relinked on load)')]
INTERESSA = []
