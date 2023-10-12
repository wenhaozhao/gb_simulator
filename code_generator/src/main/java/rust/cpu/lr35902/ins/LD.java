package rust.cpu.lr35902.ins;

import com.alibaba.fastjson2.JSON;
import org.apache.commons.lang3.StringUtils;
import rust.cpu.lr35902.MetaType;
import rust.cpu.lr35902.Opcode;

public class LD implements Ins {

    @Override
    public String fnExec(Opcode opcode) {
        if (StringUtils.isBlank(opcode.operand1()) || StringUtils.isBlank(opcode.operand2())) {
            throw new IllegalArgumentException(STR. "operand cannot be blank: \{ JSON.toJSONString(opcode) }" );
        }
        var operand1 = opcode.$operand1().orElseThrow(() -> new IllegalArgumentException(opcode.operand1()));
        var operand2 = opcode.$operand2().orElseThrow(() -> new IllegalArgumentException(opcode.operand2()));
        String save;
        if (operand1.metaTypeMatch(MetaType.addr)) {
            save = STR. "cpu.memory.borrow_mut().set_\{ operand2.code(opcode).getRetType() }(left, right);" ;
        } else if (operand1.metaTypeMatch(MetaType.register)) {
            save = STR. "cpu.register.set_\{ operand2.code(opcode).getRetType() }(Register::\{ opcode.operand1() }, right);" ;
        } else {
            throw new IllegalArgumentException(STR. "Unsupported \{ operand1 } on LD" );
        }
        return STR. """
                fn exec(&self, cpu: &mut LR35902) -> u8 {
                    \{ operand1.code(opcode).getCode() }
                    \{ operand2.code(opcode).getCode() }
                    \{ save }
                    self.meta.cycles[0]
                }""" ;
    }

}
