# Do Database.cpp, os atributos vistos por quem olha (a névoa, fog of war, [0x9cc270]): se o atributo do jogador
# aparece para quem vê (0x543280: telas de busca e filtros, perfil e comparação de jogadores, tela do clube), o valor
# dele para quem vê (0x543440: comparação de jogadores e perfil da pessoa) e o nome do atributo de técnico (0x543b10:
# telas de busca e perfil). Simulação (SAVE = False), rotinas só de tela. Sem SEH, sem sorteio; as locais de 0x200
# só servem à caixa (pessoa nula, atributo fora da faixa, texto curto), que fica com a original. As chamadas
# (0x5afde0, 0x5b02f0, 0x543280, 0x543540, 0x66f4e0) são pelo endereço do jogo; a 0x5b02f0 guarda o último clube
# achado em [0xae1080] (o mesmo valor nas duas).
SAVE = False
ROTINAS = [('atributo_visivel', 0x543280, [0x8b, 0x44, 0x24, 0x04, 0x56]),
           ('atributo_para_quem_ve', 0x543440, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('nome_do_atributo_de_tecnico', 0x543b10, [0x8a, 0x4c, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('os atributos vistos por quem olha: se o atributo do jogador aparece, o valor dele para quem olha e o nome do atributo de técnico',
        'attributes as the viewer sees them: whether a player attribute is visible, its value for the viewer and the non-player attribute name',
        [('atributo_visivel', 'staff_is_player_attribute_visible', '0x543280', 'Database.cpp', 'cdecl',
          '1 se quem vê enxerga o atributo da pessoa: sem névoa ou conhecimento 3, sempre; conhecimento 1 ou 2, pelo atributo e o resto do id por 4 (divisível por 4 ou 2, ou por 3); em `al`',
          '1 when the viewer sees the attribute: no fog or knowledge 3, always; knowledge 1 or 2, by the attribute and the id modulo 4 (divisible by 4 or 2, or by 3); in `al`'),
         ('atributo_para_quem_ve', 'staff_get_player_attribute_for_viewer', '0x543440', 'Database.cpp', 'cdecl',
          'o atributo do jogador (0x543540) ou 0xf9 (desconhecido) quando a névoa o esconde de quem vê; em `al`',
          'the player attribute (0x543540) or 0xf9 (unknown) when the fog hides it from the viewer; in `al`'),
         ('nome_do_atributo_de_tecnico', 'database_get_non_player_attribute_name', '0x543b10', 'Database.cpp', 'cdecl',
          'o nome traduzido (0x66f4e0) dos 12 atributos de técnico, abreviado ou inteiro; devolve 1',
          'the translated name (0x66f4e0) of the 12 non-player attributes, short or full; returns 1')])
CAMPOS = [('TStaff +0: id; inteiro; lê (atributo_visivel: o resto por 4); +0x61: jogador; ponteiro; lê (atributo_para_quem_ve)',
           'TStaff +0: id; integer; reads (atributo_visivel: modulo 4); +0x61: player; pointer; reads (atributo_para_quem_ve)'),
          ('[0x9cc270]: névoa ligada; byte; [0xae2a60]: tabela da névoa (fog_of_war); lê',
           '[0x9cc270]: fog of war on; byte; [0xae2a60]: fog of war table; reads')]
INTERESSA = ['SGE', 'CMRTE']
