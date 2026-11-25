# O gênero gramatical do nome de um tipo de lesão (injury.cpp; leitor rx13, rodada 6, 25/11/2026). Simulação: os textos
# das notícias de lesão (news 0x780813.., injury 0x6318aa/0x631d1a) e o relatório do fisioterapeuta (0x7a209e..) a
# chamam; a carreira ENG até 2002-06-01 a roda nas notícias de lesão.
# - 0x6306e0 Injury::get_type_name_gender thiscall(tipo) ret 4 (o this não é lido): langlib_get_language 0x66f4d0 até
#   três vezes; língua 9, 3 ou 5: movsx do tipo, acima de 0x4c (sem sinal) 1, senão a tabela de saltos da língua
#   (0x63074c, 0x630880, 0x6309b4; os 77 destinos lidos do exe: 0, 1, 2, 3 ou 4); outra língua 0. Retorno em al.
#   O prólogo começa com call rel32 (call 0x66f4d0): entra em RELATIVAS com (0x6306e0, 0) (INTEGRAR_ANTES.txt).
# Igual no cm0102-1080.exe. Comparador: o byte lado a lado (a função não tem efeito).
SAVE = False
ROTINAS = [('injury_get_type_name_gender', 0x6306e0, [0xe8, 0xeb, 0xed, 0x03, 0x00])]
DOCS = ('o gênero gramatical do nome de um tipo de lesão',
        "the grammatical gender of an injury type's name",
        [('injury_get_type_name_gender', 'Injury::get_type_name_gender', '0x6306e0', 'injury.cpp', 'thiscall, ret 4',
          'regra: a classe de gênero do nome da lesão, que escolhe o artigo do texto da notícia: nas línguas 9, 3 e 5 cada tipo de lesão tem a sua (um tipo fora da tabela, 1); nas outras línguas, 0',
          "rule: the gender class of an injury's name, which picks the article of the news text: in languages 9, 3 and 5 each injury type has its own (a type out of the table, 1); in any other language, 0")])
CAMPOS = []
INTERESSA = []
