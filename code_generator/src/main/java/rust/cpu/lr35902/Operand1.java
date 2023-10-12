package rust.cpu.lr35902;

public interface Operand1 extends Operand {
    default CodeInfo code(Opcode opcode) {
        return new CodeInfo("", RetType.none);
    }
}
