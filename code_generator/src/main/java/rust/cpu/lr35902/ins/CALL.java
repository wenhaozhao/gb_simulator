package rust.cpu.lr35902.ins;

import rust.cpu.lr35902.Opcode;

public class CALL implements Ins {

    // CALL IF or CALL
    @Override
    public String fnExec(Opcode opcode) {
        var operand1 = opcode.$operand1().orElseThrow(() -> new IllegalArgumentException(opcode.operand1()));
        var operand2 = opcode.$operand2().orElseThrow(() -> new IllegalArgumentException(opcode.operand2()));
        return STR."""
                fn exec(&self, cpu: &mut LR35902) -> u8 {
                    let left = !cpu.register.get_flag(Flag::Z);
                    if left {
                        let right = cpu.imm_u16();
                        cpu.call(right);
                        return self.meta.cycles[0];
                    }
                    self.meta.cycles[1]
                }""";

    }
}
