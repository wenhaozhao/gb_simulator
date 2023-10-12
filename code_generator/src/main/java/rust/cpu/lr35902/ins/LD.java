package rust.cpu.lr35902.ins;

import com.alibaba.fastjson2.JSON;
import org.apache.commons.lang3.StringUtils;
import rust.cpu.lr35902.Opcode;
import rust.cpu.lr35902.OperandType;

import java.util.function.Supplier;

public class LD implements Ins {

    @Override
    public String fnExecContent(Opcode opcode) {
        if (StringUtils.isBlank(opcode.operand1()) || StringUtils.isBlank(opcode.operand2())) {
            throw new IllegalArgumentException(STR. "operand cannot be blank: \{ JSON.toJSONString(opcode) }" );
        }
        var operand1 = opcode.$operand1().orElseThrow(() -> new IllegalArgumentException(opcode.operand1()));
        var operand2 = opcode.$operand2().orElseThrow(() -> new IllegalArgumentException(opcode.operand2()));
        var flagEffectCode = "";
        var flagEffects = opcode.flagEffects();
        for (var i = 0; i < flagEffects.length; ++i) {
            var flagEffect = flagEffects[i];
            var flagEffectCodeItem = switch (flagEffect.getType()) {
                case Reset -> STR. "self.meta.flags[\{ i }].effect(cpu, 0, 0);" ;
                case Set -> STR. "self.meta.flags[\{ i }].effect(cpu, 0, 0);" ;
                case Fun -> STR. "self.meta.flags[\{ i }].effect(cpu, flag_effect_l, flag_effect_r);" ;
                default -> "";
            };
            if (StringUtils.isNotBlank(flagEffectCodeItem)) {
                flagEffectCode = STR. """
                    \{ flagEffectCode }
                    \{ flagEffectCodeItem } """ ;
            }
        }
        Supplier<String> supplier = () -> {
            if (operand1.metaTypeMatch(OperandType.MetaType.addr)) {
                return STR. "cpu.memory.borrow_mut().set_\{ operand2.code(opcode).getT2() }(left, right);" ;
            }
            if (operand1.metaTypeMatch(OperandType.MetaType.register)) {
                return STR. "cpu.register.set_\{ operand2.code(opcode).getT2() }(Register::\{ opcode.operand1() }, right);" ;
            }
            // flags
            throw new IllegalArgumentException(STR. "Unsupported \{ operand1 } on LD" );
        };
        return STR. """
                \{ operand1.code(opcode).getT1() }
                \{ operand2.code(opcode).getT1() }
                \{ StringUtils.isBlank(flagEffectCode) ? "// no flag effect" : flagEffectCode }
                \{ supplier.get() }
                self.meta.cycles[0]""" ;
    }
}
