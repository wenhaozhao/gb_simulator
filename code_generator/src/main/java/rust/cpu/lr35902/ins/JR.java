package rust.cpu.lr35902.ins;

import org.apache.commons.lang3.StringUtils;
import rust.cpu.lr35902.Opcode;
import rust.cpu.lr35902.OptType;

/**
 * 相对跳转
 */
public class JR implements Ins {

    @Override
    public String fnExec(Opcode opcode) {
        // 单操作数
        var operand1 = opcode.$operand1().orElseThrow(() -> new IllegalArgumentException(opcode.operand1()));
        String body;
        if (StringUtils.isBlank(opcode.operand2())) {
            body = STR. """
            \{ operand1.code(opcode, OptType._2).getCode() }
            cpu.register.pc_incr_by_\{ operand1.code(opcode, OptType._2).getRetType() }(left);
            self.meta.cycles[0]""" ;
        } else {
            var operand2 = opcode.$operand2().orElseThrow(() -> new IllegalArgumentException(opcode.operand2()));
            body = STR. """
                \{ operand1.code(opcode, OptType._2).getCode() }
                if left {
                    \{ operand2.code(opcode).getCode() }
                    cpu.register.pc_incr_by_\{ operand2.code(opcode).getRetType() }(right);
                    return self.meta.cycles[0];
                }
                self.meta.cycles[1]""" ;
        }
        return STR. """
                fn exec(&self, cpu: &mut LR35902) -> u8 {
                    \{ body }
                }""" ;
    }

}
