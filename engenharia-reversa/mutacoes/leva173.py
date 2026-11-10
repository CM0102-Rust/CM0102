# Mutacoes do match_wrap_up do motor (7). Uso: python mutacoes.py mutacoes\levaN.py --carreira 2002-06-01
MUTACOES = [
    ('r_banco', "            cod2[i] = w(curtos, j);", "            cod2[i] = w(codes, j);"),
    ('r_idade', "            3 => !p.is_null() && read_i8(rule + 1) < (p.year_of_birth() as i8),", "            3 => !p.is_null() && read_i8(rule + 1) <= (p.year_of_birth() as i8),"),
    ('n_posicao', "        fb(0xf) * fb(0xf) - 300", "        fb(0xf) * fb(0xf) - 299"),
    ('n_ponta', "k1a / 400", "k1a / 401"),
    ('n_contrato', "        3 | 4 | 7 => 25,", "        3 | 4 | 7 => 24,"),
    ('n_final', "        (500 - l4c) * l40", "        (501 - l4c) * l40"),
    ('n_veterano', "            Some(0x9689e0)", "            Some(0x968968)"),
    ('n_forma', "        sum += ftol(a / ((read_f64(rdata::F64_1_8) - b * read_f64(rdata::F64_0_0001)) * read_f64(rdata::F64_50)));", "        sum += ftol(a / ((read_f64(rdata::F64_1_8) - b * read_f64(rdata::F64_0_0001)) * read_f64(rdata::F64_50))) + 1;"),
    ('v_folego', "        let e = extra + ((60 - minute) * 150).min(3000);", "        let e = extra + ((60 - minute) * 149).min(3000);"),
    ('v_tatica', "        l4c += fb(0x26) / 7;", "        l4c += fb(0x26) / 8;"),
    ('v_ferias', "        bonus = (base * 200).max(10000) + 40000;", "        bonus = (base * 200).max(10000) + 39999;"),
    ('v_segunda', "        if second { (ebp + position_attribute_adjust(f, pos2) + 2 * edi) / 3 } else { edi }", "        if second { (ebp + position_attribute_adjust(f, pos2) + 2 * edi) / 4 } else { edi }"),
]
