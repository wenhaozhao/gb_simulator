package rust.cpu.lr35902.ins;

import org.apache.commons.lang3.StringUtils;
import rust.cpu.lr35902.Opcode;

import java.util.stream.Stream;

/**
 * var flags = "";
 * var flagEffects = opcode.flagEffects();
 * for (var i = 0; i < flagEffects.length; ++i) {
 * var flagEffect = flagEffects[i];
 * String item;
 * if (StringUtils.isNotBlank(item = flagEffect.getType().effect(opcode, flagEffect))) {
 * flags = STR. """
 * \{ flags }
 * \{ item } """ ;
 * }
 * }
 */
public class ADD implements Ins {
    @Override
    public String fnExec(Opcode opcode) {
        var operand1 = opcode.$operand1().orElseThrow(() -> new IllegalArgumentException(opcode.operand1()));
        var operand2 = opcode.$operand2().orElseThrow(() -> new IllegalArgumentException(opcode.operand2()));
        var body = switch (operand2.code(opcode).getRetType()) {
            // ADD HL, r16
            case u16 -> STR."""
                    let res = left.wrapping_add(right);
                    cpu.register.set_flag(Flag::N, false);
                    cpu.register.set_flag(Flag::H, (right & 0x0FFF) + (left & 0x0FFF) > 0x0FFF);
                    cpu.register.set_flag(Flag::C, right > (0xFFFF - left));
                    cpu.register.set_u16(Register::HL, res);""";
            // ADD A, r8/d8
            case u8 -> STR."""
                    let res = left.wrapping_add(right);
                    cpu.register.set_flag(Flag::Z, res == 0x00);
                    cpu.register.set_flag(Flag::N, false);
                    cpu.register.set_flag(Flag::H, (left & 0x0F) + (right & 0x0F) > 0x0F);
                    cpu.register.set_flag(Flag::C, ((left as u16) & 0x00FF) + ((right as u16) & 0x00FF) > 0x00FF);
                    cpu.register.set_u8(Register::A, res);""";
            // ADD SP, r8
            case i8 -> STR."""
                    let right = right as i16 as u16;
                    let res = left.wrapping_add(right);
                    cpu.register.set_flag(Flag::Z, false);
                    cpu.register.set_flag(Flag::N, false);
                    cpu.register.set_flag(Flag::H, (left & 0x000F) + (right & 0x000F) > 0x000F);
                    cpu.register.set_flag(Flag::C, (left & 0x00FF) + (right & 0x00FF) > 0x00FF);
                    cpu.register.set_u16(Register::SP, res);""";
            default -> throw new IllegalArgumentException(STR. "Unsupported operand2 \{ opcode.operand2() }" );
        };
        return STR. """
                fn exec(&self, cpu: &mut LR35902) -> u8 {
                    \{ operand1.code(opcode).getCode() }
                    \{ operand2.code(opcode).getCode() }
                    \{ body }
                    self.meta.cycles[0]
                }""" ;
    }
}
