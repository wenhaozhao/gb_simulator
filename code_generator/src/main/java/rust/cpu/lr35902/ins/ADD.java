package rust.cpu.lr35902.ins;

import rust.cpu.lr35902.Opcode;

public class ADD implements Ins {
    @Override
    public String fnExec(Opcode opcode) {
        return STR. """
                fn exec(&self, cpu: &mut LR35902) -> u8 {
                    \{ this.fnExecContent(opcode) }
                }""" ;
    }
}
