package rust.cpu.lr35902;

import com.alibaba.fastjson2.JSON;
import org.apache.commons.lang3.StringUtils;
import org.jetbrains.annotations.NotNull;

import java.util.Optional;
import java.util.function.Supplier;
import java.util.stream.Collectors;
import java.util.stream.IntStream;

public record Opcode(
        String mnemonic,
        int length,
        int[] cycles,
        String[] flags,
        String addr,
        String group,
        String operand1,
        String operand2,
        boolean cbprefixed
) implements Comparable<Opcode> {

    public String mnemonic() {
        return this.mnemonic.toUpperCase();
    }

    public String addr() {
        return String.format("0x%02X", this.addrVal());
    }

    public int addrVal() {
        return Integer.parseInt(this.addr.substring(2), 16);
    }

    public int actualAddrVal() {
        int addr = this.addrVal();
        if (this.cbprefixed) {
            addr = addr | 0xCB00;
        }
        return addr;
    }

    @NotNull
    private String cyclesDesc() {
        var cycles = this.cycles();
        var desc = IntStream.of(cycles).mapToObj(String::valueOf).collect(Collectors.joining("/"));
        if (StringUtils.isBlank(desc)) {
            desc = "0";
        }
        return desc;
    }

    @NotNull
    private String cyclesRust() {
        var cycles = this.cycles();
        assert cycles.length == 2;
        return STR. "[\{ cycles.length > 0 ? cycles[0] : 0 }, \{ cycles.length > 1 ? cycles[1] : 0 }]" ;
    }

    public FlagEffect[] flagEffects() {
        var flags = this.flags();
        assert flags.length == 4;
        return new FlagEffect[]{
                new FlagEffect(flags, 0),
                new FlagEffect(flags, 1),
                new FlagEffect(flags, 2),
                new FlagEffect(flags, 3)

        };
    }

    @NotNull
    private String flagsRust() {
        var flags = this.flags();
        var flagEffects = this.flagEffects();
        assert flags.length == 4;
        return STR. "[\{ flagEffects[0] }, \{ flagEffects[1] }, \{ flagEffects[2] }, \{ flagEffects[3] }]" ;
    }

    @NotNull
    private String parametersDesc() {
        if (StringUtils.isBlank(this.operand1())) {
            return "";
        }
        if (StringUtils.isBlank(this.operand2())) {
            return this.operand1();
        }
        return STR. "\{ this.operand1() },\{ this.operand2() }" ;
    }

    @NotNull
    private String parametersRust() {
        // [Some("BC"), None]
        if (StringUtils.isBlank(this.operand1())) {
            return "[None, None]";
        }
        if (StringUtils.isBlank(this.operand2())) {
            return STR. """
            [Some("\{ this.operand1() }"), None]""" ;
        } else {
            return STR. """
            [Some("\{ this.operand1() }"), Some("\{ this.operand2() }")]""" ;
        }
    }

    public String structDef() {
        return STR. "_\{ String.format("0x%04X", this.actualAddrVal()) }" ;
    }

    public String staticInstanceDef() {
        return STR. "\{ this.structDef() }_" ;
    }

    public String modName() {
        return STR. "\{ this.structDef() }_\{ this.mnemonic() }" .toLowerCase();
    }

    @NotNull
    public String rustFileContent() {
        return STR. """
use crate::cpu::lr35902::LR35902;
use crate::cpu::lr35902::opcode::{FlagEffect, Opcode, OpcodeMeta};
use crate::cpu::lr35902::registers::{Flag, Register};

static META: OpcodeMeta = OpcodeMeta {
    mnemonic: "\{ this.mnemonic() }",
    length: \{ this.length() },
    cycles: \{ this.cyclesRust() },
    flags: \{ this.flagsRust() },
    addr: \{ this.addr() },
    group: "\{ this.group() }",
    parameters: \{ this.parametersRust() },
    cb_prefixed: \{ this.cbprefixed() },
};

/// Instruction | Parameters | Opcode | Cycles
/// ----------- | ---------- | ------ | ------
/// \{ this.mnemonic() } | \{ this.parametersDesc() } | \{ this.addr() } | \{ this.cyclesDesc() }
pub struct \{ this.structDef() } {
    meta: &'static OpcodeMeta,
}

pub static \{ this.staticInstanceDef() }: \{ this.structDef() } = \{ this.structDef() } {
    meta: &META,
};

impl Opcode for \{ this.structDef() } {

    fn meta(&self) -> &'static OpcodeMeta {
        self.meta
    }

    fn exec(&self, cpu: &mut LR35902) -> u8 {
        \{ this.fnExec() }
    }
}
                                """ ;
    }

    private String fnExec() {
        var content = switch (this.mnemonic().toUpperCase()) {
            case "LD" -> ld();
            case "JR" -> jr();
            default -> null;
        };
        if (StringUtils.isBlank(content)) {
            content = "todo!()";
        }
        return content;
    }

    /**
     * 相对跳转
     */
    private String jr(){
        // 单操作数
        var operand1 = this.$operand1().orElseThrow(() -> new IllegalArgumentException(this.operand1()));
        if (StringUtils.isBlank(this.operand2())) {
            return STR."""
            \{ operand1.code(this).getT1() }
            cpu.register.pc_incr_by_\{operand1.code(this).getT2()}(left);
            self.meta.cycles[0]""";
        }else {
            var operand2 = this.$operand2().orElseThrow(() -> new IllegalArgumentException(this.operand2()));
            return STR."""
                \{ operand1.code(this).getT1() }
                if left {
                    \{ operand2.code(this).getT1() }
                    cpu.register.pc_incr_by_\{operand2.code(this).getT2()}(right);
                    return self.meta.cycles[0];
                }
                self.meta.cycles[1]""";
        }
    }

    private String ld() {
        if (StringUtils.isBlank(this.operand1()) || StringUtils.isBlank(this.operand2())) {
            throw new IllegalArgumentException(STR. "operand cannot be blank: \{ JSON.toJSONString(this) }" );
        }
        var operand1 = this.$operand1().orElseThrow(() -> new IllegalArgumentException(this.operand1()));
        var operand2 = this.$operand2().orElseThrow(() -> new IllegalArgumentException(this.operand2()));
        var flagEffectCode = "";
        var flagEffects = this.flagEffects();
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
                return STR. "cpu.memory.borrow_mut().set_\{ operand2.code(this).getT2() }(left, right);" ;
            }
            if (operand1.metaTypeMatch(OperandType.MetaType.register)) {
                return STR. "cpu.register.set_\{ operand2.code(this).getT2() }(Register::\{ this.operand1() }, right);" ;
            }
            // flags
            throw new IllegalArgumentException(STR. "Unsupported \{ operand1 } on LD" );
        };
        return STR. """
                \{ operand1.code(this).getT1() }
                \{ operand2.code(this).getT1() }
                \{ StringUtils.isBlank(flagEffectCode)?"// no flag effect":flagEffectCode }
                \{ supplier.get() }
                self.meta.cycles[0]""" ;
    }

    public Optional<? extends OperandType.Operand1> $operand1() {
        Optional<? extends OperandType.Operand1> $operand1;
        if (this.cbprefixed()) {
            $operand1 = OperandType.CBPrefixedOperand1.parse(this.operand1());
        } else {
            $operand1 = OperandType.UnprefixedOperand1.parse(this.operand1());
        }
        return $operand1;
    }

    public Optional<? extends OperandType.Operand2> $operand2() {
        Optional<? extends OperandType.Operand2> $operand2;
        if (this.cbprefixed()) {
            $operand2 = OperandType.CBPrefixedOperand2.parse(this.operand2());
        } else {
            $operand2 = OperandType.UnprefixedOperand2.parse(this.operand2());
        }
        return $operand2;
    }

    @Override
    public int compareTo(@NotNull Opcode that) {
        return Integer.compare(this.addrVal(), that.addrVal());
    }

    public static void main(String[] args) throws Exception {
        var json = """
                {
                      "mnemonic": "INC",
                      "length": 1,
                      "cycles": [
                        4
                      ],
                      "flags": [
                        "Z",
                        "0",
                        "H",
                        "-"
                      ],
                      "addr": "0x0c",
                      "group": "x8/alu",
                      "operand1": "C",
                      "cbprefixed": true
                    }
                """;
        var jsonObject = JSON.parseObject(json);
        var opcode = JSON.parseObject(jsonObject.toJSONString(), Opcode.class);
        System.out.println(opcode.rustFileContent());

    }
}