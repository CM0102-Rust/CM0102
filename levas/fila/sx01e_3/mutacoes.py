# Mutacoes (sx01e_3). O texto nao volta para a simulacao: acusadas pela comparacao do escritor (o texto, os vinculos
# e o gerador), com o modo CM_TEXTO_NOTICIAS na carreira automatica ou abrindo a noticia jogando comparando. Cada uma
# num trecho que roda em toda manchete ou todo corpo do seu escritor:
# - desculpas: a manchete par vira "pede desculpas" (a paridade trocada);
# - pedido de apoio: o vinculo do nome do jogador com o tipo de clube;
# - resposta as criticas: o vinculo do nome curto do clube perdido;
# - tecnico contra a FA: o vinculo do nome longo do clube com o tipo de pessoa.
MUTACOES = [
    ('sx01e_desculpas_paridade',
     '    } else if variant % 2 != 0 {\n        0xa17c54 // "... apologises"',
     '    } else if variant % 2 == 0 { // mutation\n        0xa17c54 // "... apologises"',
     'sx01e_3 apology text headline'),
    ('sx01e_apoio_vinculo',
     '        t.link(self.ptr(), 0x7d7, staff_at(self.id as i32 as isize).addr());',
     '        t.link(self.ptr(), 0x7d5, staff_at(self.id as i32 as isize).addr()); // mutation',
     'sx01e_3 apology text headline'),
    ('sx01e_criticas_sem_clube',
     '    }\n    t.link_club_short(club);\n}\n\n/// Writes the headline of news item 0xbdc',
     '    }\n    // mutation: no club link\n}\n\n/// Writes the headline of news item 0xbdc',
     'sx01e_3 criticism reaction text body'),
    ('sx01e_fa_vinculo_longo',
     '    t.link(long, 0x7d5, club.addr());',
     '    t.link(long, 0x7d7, club.addr()); // mutation',
     'sx01e_3 FA complaint text body'),
]
