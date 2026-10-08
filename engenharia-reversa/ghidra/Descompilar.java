// Descompila as funcoes nos enderecos dados (args: saida.txt endereco...) e grava o pseudocodigo.
import ghidra.app.script.GhidraScript;
import ghidra.app.decompiler.*;
import ghidra.program.model.address.Address;
import ghidra.program.model.listing.Function;
import java.io.PrintWriter;

public class Descompilar extends GhidraScript {
    @Override
    public void run() throws Exception {
        String[] a = getScriptArgs();
        DecompInterface d = new DecompInterface();
        d.openProgram(currentProgram);
        try (PrintWriter out = new PrintWriter(a[0], "UTF-8")) {
            for (int i = 1; i < a.length; i++) {
                Address ad = toAddr(a[i]);
                disassemble(ad);
                Function f = getFunctionAt(ad);
                if (f == null) {
                    f = createFunction(ad, null);
                }
                DecompileResults r = d.decompileFunction(f, 120, monitor);
                out.println("// ===== " + a[i]);
                out.println(r.decompileCompleted() ? r.getDecompiledFunction().getC() : "falhou: " + r.getErrorMessage());
            }
        }
    }
}
