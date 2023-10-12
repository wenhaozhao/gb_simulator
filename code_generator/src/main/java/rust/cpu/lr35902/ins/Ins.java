package rust.cpu.lr35902.ins;

import org.apache.commons.lang3.StringUtils;
import rust.cpu.lr35902.Opcode;

import java.util.stream.Stream;

public interface Ins {

    default String fnExec(Opcode opcode) {
        return STR. """
                fn exec(&self, cpu: &mut LR35902) -> u8 {
                    \{ this.fnExecContent(opcode) }
                }""" ;
    }

    default String fnExecContent(Opcode opcode) {
        return STR."todo!()";
    }

    class Util {
        public static Ins parse(Opcode opcode) {
            Stream<? extends Ins> stream;
            if (opcode.cbprefixed()) {
                stream = Stream.of(CBPrefixedIns.values());
            } else {
                stream = Stream.of(UnprefixedIns.values());
            }
            return stream.filter(it -> StringUtils.equalsIgnoreCase(((Enum<?>) it).name(), opcode.mnemonic()))
                    .findFirst()
                    .orElseThrow();
        }
    }
}
