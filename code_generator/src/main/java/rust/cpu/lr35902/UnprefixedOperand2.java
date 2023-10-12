package rust.cpu.lr35902;

import org.apache.commons.lang3.StringUtils;

import java.util.Optional;
import java.util.regex.Pattern;
import java.util.stream.Stream;

public enum UnprefixedOperand2 implements Operand2 {
    num(new MetaType[]{MetaType.num}, new String[]{"d8", "r8", "a16", "d16"}) {
        // d8 立即数 , r8 符号立即数
        @Override
        public CodeInfo code(Opcode opcode) {
            return switch (opcode.operand2()) {
                case "d8" -> new CodeInfo(STR."let right = cpu.imm_u8();", RetType.u8);
                case "r8" -> new CodeInfo(STR."let right = cpu.imm_u8() as i8;", RetType.i8);
                case "a16", "d16" -> new CodeInfo("let right = cpu.imm_u16();", RetType.u16);
                default -> super.code(opcode);
            };
        }
    },
    adr_num(new MetaType[]{MetaType.num, MetaType.addr}, new String[]{"(a8)", "(a16)"}) {
        @Override
        public CodeInfo code(Opcode opcode) {
            return switch (opcode.operand2()) {
                case "(a8)" -> new CodeInfo(STR."""
                        let right = cpu.imm_u8() as u16;
                        let right = cpu.memory.borrow().get(0xFF00 | right);""", RetType.u8);
                case "(a16)" -> new CodeInfo(STR."""
                        let right = cpu.imm_u16();
                        let right = cpu.memory.borrow().get(right);""", RetType.u8);
                default -> super.code(opcode);
            };
        }
    },
    reg(new MetaType[]{MetaType.register}, new String[]{"AF", "BC", "DE", "HL", "A", "B", "C", "D", "E", "H", "L", "Z", "SP", "SP+r8"}) {
        @Override
        public CodeInfo code(Opcode opcode) {
            var register = opcode.operand2();
            if (register.equals("SP+r8")) {
                return new CodeInfo(
                        STR."""
                        let v1 = cpu.register.get_u16(Register::SP);
                        let v2 = cpu.imm_u8() as i8 as i16 as u16;
                        let right = v1.wrapping_add(v2);
                        cpu.register.set_flag(Flag::Z, false);
                        cpu.register.set_flag(Flag::N, false);
                        cpu.register.set_flag(Flag::H, (v1 & 0x000F) + (v2 & 0x000F) > 0x000F);
                        cpu.register.set_flag(Flag::C, (v1 & 0x00FF) + (v2 & 0x00FF) > 0x00FF);""",
                        RetType.u16
                );
            } else {
                return switch (register.length()) {
                    case 1 ->
                            new CodeInfo(STR. "let right = cpu.register.get_u8(Register::\{ register });" , RetType.u8);
                    case 2 ->
                            new CodeInfo(STR. "let right = cpu.register.get_u16(Register::\{ register });" , RetType.u16);
                    default -> super.code(opcode);
                };
            }
        }
    },
    reg_flag(new MetaType[]{MetaType.register, MetaType.flag, MetaType.condition}, new String[]{"Z", "N", "H", "C", "NC", "NZ"}), //todo
    reg_sp$r8(new MetaType[]{MetaType.register}, new String[]{"SP+r8"}),//todo
    adr_reg(new MetaType[]{MetaType.register, MetaType.addr}, new String[]{"(C)", "(BC)", "(DE)", "(HL)", "(HL+)", "(HL-)"}) {
        final Pattern pattern = Pattern.compile("^\\((\\w{1,2})([+-]?)\\)$");

        @Override
        public CodeInfo code(Opcode opcode) {
            var matches = pattern.matcher(opcode.operand2());
            if (!matches.find()) {
                return super.code(opcode);
            }
            var register = matches.group(1);
            var ext = matches.group(2);
            var code = switch (ext) {
                case "+" -> STR. "cpu.register.get_and_incr_u16(Register::\{ register })" ;
                case "-" -> STR. "cpu.register.get_and_decr_u16(Register::\{ register })" ;
                default -> STR. "cpu.register.get_u16(Register::\{ register })" ;
            };
            if (register.length() == 1) {
                code = STR. "0xFF00 | (\{ code })" ;
            }
            return new CodeInfo(STR. "let right = cpu.memory.borrow().get(\{ code });" , RetType.u8);
        }
    },
    adr_reg_16_ext(new MetaType[]{MetaType.register, MetaType.addr}, new String[]{"(HL+)", "(HL-)"}),//todo

    None(new MetaType[]{}, new String[0]);
    private final MetaType[] metaTypes;
    private final String[] items;

    public static Optional<UnprefixedOperand2> parse(String val) {
        return Stream.of(UnprefixedOperand2.values())
                .filter(it -> Stream.of(it.items).anyMatch(iit -> StringUtils.equalsIgnoreCase(iit, val)))
                .findFirst();
    }

    UnprefixedOperand2(MetaType[] metaTypes, String[] items) {
        this.metaTypes = metaTypes;
        this.items = items;
    }

    @Override
    public MetaType[] getMetaTypes() {
        return metaTypes;
    }
}
