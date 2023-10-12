package rust.cpu.lr35902.ins;

import com.alibaba.fastjson2.JSON;
import org.apache.commons.lang3.StringUtils;
import rust.cpu.lr35902.MetaType;
import rust.cpu.lr35902.Opcode;

public class LD implements Ins {

    @Override
    public String fnExecContent(Opcode opcode) {
        if (StringUtils.isBlank(opcode.operand1()) || StringUtils.isBlank(opcode.operand2())) {
            throw new IllegalArgumentException(STR. "operand cannot be blank: \{ JSON.toJSONString(opcode) }" );
        }
        var operand1 = opcode.$operand1().orElseThrow(() -> new IllegalArgumentException(opcode.operand1()));
        var operand2 = opcode.$operand2().orElseThrow(() -> new IllegalArgumentException(opcode.operand2()));
        var flags = "";
        var flagEffects = opcode.flagEffects();
        for (var i = 0; i < flagEffects.length; ++i) {
            var flagEffect = flagEffects[i];
            var flagsItem = flagEffect.getType().effect(opcode, flagEffect);
            if (StringUtils.isNotBlank(flagsItem)) {
                flags = STR. """
                    \{ flags }
                    \{ flagsItem } """ ;
            }
        }
        String save;
        if (operand1.metaTypeMatch(MetaType.addr)) {
            save = STR. "cpu.memory.borrow_mut().set_\{ operand2.code(opcode).retType() }(left, right);" ;
        } else if (operand1.metaTypeMatch(MetaType.register)) {
            save = STR. "cpu.register.set_\{ operand2.code(opcode).retType() }(Register::\{ opcode.operand1() }, right);" ;
        } else {
            throw new IllegalArgumentException(STR. "Unsupported \{ operand1 } on LD" );
        }
        return STR. """
                \{ operand1.code(opcode).code() }
                \{ operand2.code(opcode).code() }
                \{ StringUtils.isBlank(flags) ? "// no flag effect" : flags }
                \{ save }
                self.meta.cycles[0]""" ;
    }
}
