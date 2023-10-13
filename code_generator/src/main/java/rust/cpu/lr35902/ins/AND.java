package rust.cpu.lr35902.ins;

import rust.cpu.lr35902.MetaType;
import rust.cpu.lr35902.Opcode;
import rust.cpu.lr35902.OptType;

public class AND implements Ins {

    @Override
    public String fnExec(Opcode opcode) {

        var operand1 = opcode.$operand1().orElseThrow(() -> new IllegalArgumentException(opcode.operand1()));
        var left = operand1.code(opcode, OptType._3).getCode();
        if (operand1.metaTypeMatch(MetaType.addr)) {
            left = STR. """
                    \{ operand1.code(opcode, OptType._3).getCode() }
                            let left = cpu.memory.borrow_mut().get(left);""" ;
        }
        return STR. """
                fn exec(&self, cpu: &mut LR35902) -> u8 {
                        let reg_a = cpu.register.get_u8(Register::A);
                        \{ left }
                        let res = reg_a & left;
                        cpu.register.set_flag(Flag::Z, res == 0);
                        cpu.register.set_flag(Flag::N, false);
                        cpu.register.set_flag(Flag::H, true);
                        cpu.register.set_flag(Flag::C, false);
                        self.meta.cycles[0]
                }""" ;
    }
}
