# Mutacoes (cx03_19), carreira de 8 ligas ate 2002-06-01. Nenhuma abre caixa. Os comparadores so rodam a original
# (conferem o plano dela): a copia so a carreira acusa.
# - as duas copas com a importancia 20 em vez de 15: a importancia dos jogos muda a escalacao dos clubes brasileiros;
# - a Libertadores sem a nova temporada do sorteio da Conmebol: os classificados mudam;
# - a Mercosul sem premio: o dinheiro das rodadas muda.
MUTACOES = [('cx03_19_importancia_vinte', '    this.set_importance(0xf);\n    if exe::league_load_from_temp_file(this, 1) != 0 {\n        return this.addr();\n    }\n    covered!(branch);', '    this.set_importance(0x14); // mutation\n    if exe::league_load_from_temp_file(this, 1) != 0 {\n        return this.addr();\n    }\n    covered!(branch);'), ('cx03_19_libertadores_sem_sorteio_novo', '    vtable: 0x968bf4, stages: 8, seeding: true,', '    vtable: 0x968bf4, stages: 8, seeding: false, /* mutation */'), ('cx03_19_mercosul_sem_premio', '    day: 0xf, month: 5, week: 0xffff_ffff, prize: Some(0x1e8480),', '    day: 0xf, month: 5, week: 0xffff_ffff, prize: Some(0), // mutation')]
