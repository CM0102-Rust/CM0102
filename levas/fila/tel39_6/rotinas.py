# A classe Text (1): trocar e acrescentar o texto, acrescentar um vinculo (comp_text.cpp; leitor tel39, rodada 2 das
# telas, 22/11/2026).
# Convencoes conferidas na desmontagem: Text::set_string thiscall(this, origem) ret 4 -> eax 0/1; Text::append_string
# thiscall(this, origem) ret 4 -> eax 0/1; Text::add_link thiscall(this, frase, tipo, dado) ret 0xc -> eax 0/1 (quadro
# SEH so pelo construtor da lista). Chamadas: operator new 0x944cf1 e delete 0x944c94 (cdecl), sprintf 0x944c9f,
# LLIST::LLIST 0x68d020, get_next 0x68d1b0, add_tail 0x68d0d0 (thiscall); a caixa "Error" do comp_text.cpp (linhas
# 139, 151, 177, 192, 227, 235, 247) como a tel26_error_box faz.
# Quando rodam: em todo texto de noticia, de tela e de midia (centenas de chamadores: 0x4ae770 em 0x4d7ff4...,
# 0x4ae8a0 pelas noticias): na carreira o tempo todo.
# Comparadores: a copia roda primeiro num Text local (copia do texto e, no add_link, da lista de vinculos), depois a
# original no Text; texto, vinculos e retornos iguais (os bytes do vinculo depois do terminador nao entram: lixo da
# pilha na original). Origem nula ou frase longa (caixa de erro) so a original.
# Defeitos do original: o append_string sem memoria perde o texto antigo (vazamento) e deixa o Text vazio; o
# add_link copia para a lista os bytes do vinculo depois do terminador sem inicia-los (lixo da pilha; aqui zero).
# Depende de: list_rewind (comp/fix_man.rs), text_contents (compare/media.rs), tel26_error_box (motor.rs).
ROTINAS = [('text_set_string', 0x4ae660, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('text_append_string', 0x4ae770, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('text_add_link', 0x4ae8a0, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00])]
DOCS = ('a classe Text (1): trocar e acrescentar o texto, acrescentar um vínculo',
        'the Text class (1): setting and appending the string, adding a link',
        [('text_set_string', 'Text::set_string', '0x4ae660', 'comp_text.cpp', 'thiscall',
          'troca o texto de um Text por uma cópia nova da origem (libera o antigo); origem nula ou falta de memória: a caixa de erro e 0',
          "replaces a Text's string with a new copy of the source (freeing the old one); a null source or no memory: the error box and 0"),
         ('text_append_string', 'Text::append_string', '0x4ae770', 'comp_text.cpp', 'thiscall',
          'acrescenta a origem ao texto de um Text (sem texto, troca); origem nula ou falta de memória: a caixa de erro e 0',
          "appends the source to a Text's string (with none, sets it); a null source or no memory: the error box and 0"),
         ('text_add_link', 'Text::add_link', '0x4ae8a0', 'comp_text.cpp', 'thiscall',
          'acrescenta um vínculo (frase do texto, tipo e dado) à lista do Text, criando a lista; a mesma frase já ligada não entra de novo (1 se ao mesmo objeto, 0 se a outro); frase nula ou de 200 bytes ou mais: a caixa de erro e 0',
          "adds a link (a phrase of the text, a kind and its data) to the Text's list, making the list; the same phrase already linked is not added again (1 when to the same object, 0 when to another); a null phrase or one of 200 bytes or more: the error box and 0")])
