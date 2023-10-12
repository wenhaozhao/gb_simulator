package rust.cpu.lr35902.ins;

import org.apache.commons.lang3.StringUtils;
import rust.cpu.lr35902.Opcode;

/**
 * 相对跳转
 */
public class JR implements Ins{

    @Override
    public String fnExecContent(Opcode opcode) {
// 单操作数
        var operand1 = opcode.$operand1().orElseThrow(() -> new IllegalArgumentException(opcode.operand1()));
        if (StringUtils.isBlank(opcode.operand2())) {
            return STR."""
            \{ operand1.code(opcode).getT1() }
            cpu.register.pc_incr_by_\{operand1.code(opcode).getT2()}(left);
            self.meta.cycles[0]""";
        }else {
            var operand2 = opcode.$operand2().orElseThrow(() -> new IllegalArgumentException(opcode.operand2()));
            return STR."""
                \{ operand1.code(opcode).getT1() }
                if left {
                    \{ operand2.code(opcode).getT1() }
                    cpu.register.pc_incr_by_\{operand2.code(opcode).getT2()}(right);
                    return self.meta.cycles[0];
                }
                self.meta.cycles[1]""";
        }
    }

}
