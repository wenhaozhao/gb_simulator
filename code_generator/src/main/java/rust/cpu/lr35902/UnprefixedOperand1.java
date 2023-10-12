package rust.cpu.lr35902;

import org.apache.commons.lang3.StringUtils;

import java.util.Optional;
import java.util.regex.Pattern;
import java.util.stream.Stream;

public enum UnprefixedOperand1 implements Operand1 {
    num(new MetaType[]{MetaType.num}, new String[]{"d8", "r8", "a16"}) {
        // d8 立即数 , r8 符号立即数
        @Override
        public CodeInfo code(Opcode opcode, OptType optType) {
            return switch (opcode.operand1()) {
                case "d8" -> new CodeInfo(STR."let left = cpu.imm_u8();", RetType.u8);
                case "r8" -> new CodeInfo(STR."let left = cpu.imm_u8() as i8;", RetType.i8);
                case "a16" -> new CodeInfo(STR."let left = cpu.imm_u16();", RetType.u16);
                default -> super.code(opcode, optType);
            };
        }
    },

    adr_num(new MetaType[]{MetaType.num, MetaType.addr}, new String[]{"(a8)", "(a16)"}) {
        @Override
        public CodeInfo code(Opcode opcode, OptType optType) {
            return switch (opcode.operand1()) {
                case "(a8)" -> new CodeInfo(STR."let left = 0xFF00 | (cpu.imm_u8() as u16);", RetType.u16);
                case "(a16)" -> new CodeInfo(STR."let left = cpu.imm_u16();", RetType.u16);
                default -> super.code(opcode, optType);
            };
        }
    },
    /**
     * H/C可能是flag需要看具体的指令
     */
    reg(new MetaType[]{MetaType.register}, new String[]{"AF", "BC", "DE", "HL", "A", "B", "C", "D", "E", "H", "L", "SP"}){
        @Override
        public CodeInfo code(Opcode opcode, OptType optType) {
            var register = opcode.operand1();
            return switch (optType) {
                case _3 ->switch (register.length()) {
                    case 1 -> new CodeInfo(STR. "let left = cpu.register.get_u8(Register::\{ register });" , RetType.u8);
                    case 2 ->
                            new CodeInfo(STR. "let left = cpu.register.get_u16(Register::\{ register });" , RetType.u16);
                    default -> super.code(opcode, optType);
                };
                default -> super.code(opcode,optType);
            };
        }
    },//todo
    reg_flag(new MetaType[]{MetaType.register, MetaType.flag, MetaType.condition}, new String[]{"ZZ", "NZ", "CC", "NC"}) {

        final Pattern pattern = Pattern.compile("^(\\w)(\\w)$");

        @Override
        public CodeInfo code(Opcode opcode, OptType optType) {
            var matches = pattern.matcher(opcode.operand1());
            if (!matches.find()) {
                return super.code(opcode, optType);
            }
            var neg = matches.group(1).equals("N");
            var flagReg = matches.group(2);
            return new CodeInfo(STR. "let left = \{ neg ? "!" : "" }cpu.register.get_flag(Flag::\{ flagReg });" , RetType.bool);
        }
    },
    adr_reg(new MetaType[]{MetaType.register, MetaType.addr}, new String[]{"(C)", "(BC)", "(DE)", "(HL)", "(HL+)", "(HL-)"}) {
        final Pattern pattern = Pattern.compile("^\\((\\w{1,2})([+-]?)\\)$");

        @Override
        public CodeInfo code(Opcode opcode, OptType optType) {
            var matches = pattern.matcher(opcode.operand1());
            if (!matches.find()) {
                return super.code(opcode, optType);
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
            return new CodeInfo(STR. "let left = \{ code };" , RetType.u16);
        }
    },
    // 0xc7 - v
    rst(new MetaType[]{MetaType.addr}, new String[]{"00H", "08H", "10H", "18H", "20H", "28H", "30H", "38H"}) {
        @Override
        public CodeInfo code(Opcode opcode, OptType optType) {
            return new CodeInfo(STR. "let left = \{ opcode.operand1() };" , RetType.u16);
        }
    },
    None(new MetaType[]{}, new String[0]) {
        @Override
        public CodeInfo code(Opcode opcode, OptType optType) {
            return super.code(opcode, optType);
        }
    };

    private final MetaType[] metaTypes;
    private final String[] items;

    public static Optional<UnprefixedOperand1> parse(String val) {
        return Stream.of(UnprefixedOperand1.values())
                .filter(it -> Stream.of(it.items).anyMatch(iit -> StringUtils.equalsIgnoreCase(iit, val)))
                .findFirst();
    }

    UnprefixedOperand1(MetaType[] metaTypes, String[] items) {
        this.metaTypes = metaTypes;
        this.items = items;
    }

    @Override
    public MetaType[] getMetaTypes() {
        return metaTypes;
    }
}
