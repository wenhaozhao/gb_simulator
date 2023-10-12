package rust.cpu.lr35902;

public interface Operand2 extends Operand {
    default CodeInfo code(Opcode opcode) {
        return CodeInfo.of("", RetType.none);
    }
}
