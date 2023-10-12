package rust.cpu.lr35902;


import org.apache.commons.lang3.StringUtils;
import reactor.util.function.Tuple2;
import reactor.util.function.Tuples;

import java.util.Optional;
import java.util.regex.Pattern;
import java.util.stream.Stream;


public class OperandType {

    public enum RetType {
        none, u8, i8, u16, bool;
    }

    public enum MetaType {
        num, register, flag, addr, condition, not_condition,
    }

    public enum UnprefixedOperand1 implements Operand1 {
        num(new MetaType[]{MetaType.num}, new String[]{"d8", "r8", "a16"}) {
            // d8 立即数 , r8 符号立即数
            @Override
            public Tuple2<String, RetType> code(Opcode opcode) {
                return switch (opcode.operand1()) {
                    case "d8" -> Tuples.of(STR."let left = cpu.imm_u8();", RetType.u8);
                    case "r8" -> Tuples.of(STR."let left = cpu.imm_u8() as i8;", RetType.i8);
                    case "a16" -> Tuples.of(STR."let left = cpu.imm_u16();", RetType.u16);
                    default -> super.code(opcode);
                };
            }
        },

        adr_num(new MetaType[]{MetaType.num, MetaType.addr}, new String[]{"(a8)", "(a16)"}) {
            @Override
            public Tuple2<String, RetType> code(Opcode opcode) {
                return switch (opcode.operand1()) {
                    case "(a8)" -> Tuples.of(STR."let left = 0xFF00 | (cpu.imm_u8() as u16);", RetType.u16);
                    case "(a16)" -> Tuples.of(STR."let left = cpu.imm_u16();", RetType.u16);
                    default -> super.code(opcode);
                };
            }
        },
        /**
         * H/C可能是flag需要看具体的指令
         */
        reg(new MetaType[]{MetaType.register}, new String[]{"AF", "BC", "DE", "HL", "A", "B", "C", "D", "E", "H", "L", "SP"}),//todo
        reg_flag(new MetaType[]{MetaType.register, MetaType.flag, MetaType.condition}, new String[]{"ZZ", "NZ", "CC", "NC"}) {

            final Pattern pattern = Pattern.compile("^(\\w)(\\w)$");

            @Override
            public Tuple2<String, RetType> code(Opcode opcode) {
                var matches = pattern.matcher(opcode.operand1());
                if (!matches.find()) {
                    return super.code(opcode);
                }
                var neg = matches.group(1).equals("N");
                var flagReg = matches.group(2);
                return Tuples.of(STR. "let left = \{ neg ? "!" : "" }cpu.register.get_flag(Flag::\{ flagReg });" , RetType.bool);
            }
        },
        adr_reg(new MetaType[]{MetaType.register, MetaType.addr}, new String[]{"(C)", "(BC)", "(DE)", "(HL)", "(HL+)", "(HL-)"}) {
            final Pattern pattern = Pattern.compile("^\\((\\w{1,2})([+-]?)\\)$");

            @Override
            public Tuple2<String, RetType> code(Opcode opcode) {
                var matches = pattern.matcher(opcode.operand1());
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
                return Tuples.of(STR. "let left = \{ code };" , RetType.u16);
            }
        },
        // 0xc7 - v
        rst(new MetaType[]{MetaType.addr}, new String[]{"00H", "08H", "10H", "18H", "20H", "28H", "30H", "38H"}) {
            @Override
            public Tuple2<String, RetType> code(Opcode opcode) {
                return Tuples.of(STR. "let left = \{ opcode.operand1() };" , RetType.u16);
            }
        },
        None(new MetaType[]{}, new String[0]) {
            @Override
            public Tuple2<String, RetType> code(Opcode opcode) {
                return super.code(opcode);
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

    public enum UnprefixedOperand2 implements Operand2 {
        num(new MetaType[]{MetaType.num}, new String[]{"d8", "r8", "a16", "d16"}) {
            // d8 立即数 , r8 符号立即数
            @Override
            public Tuple2<String, RetType> code(Opcode opcode) {
                return switch (opcode.operand2()) {
                    case "d8" -> Tuples.of(STR."let right = cpu.imm_u8();", RetType.u8);
                    case "r8" -> Tuples.of(STR."let right = cpu.imm_u8() as i8;", RetType.i8);
                    case "a16", "d16" -> Tuples.of("let right = cpu.imm_u16();", RetType.u16);
                    default -> super.code(opcode);
                };
            }
        },
        adr_num(new MetaType[]{MetaType.num, MetaType.addr}, new String[]{"(a8)", "(a16)"}) {
            @Override
            public Tuple2<String, RetType> code(Opcode opcode) {
                return switch (opcode.operand2()) {
                    case "(a8)" -> Tuples.of(STR."""
                            let right = cpu.imm_u8() as u16;
                            let right = cpu.memory.borrow().get(0xFF00 | right);""", RetType.u8);
                    case "(a16)" -> Tuples.of(STR."""
                            let right = cpu.imm_u16();
                            let right = cpu.memory.borrow().get(right);""", RetType.u8);
                    default -> super.code(opcode);
                };
            }
        },
        reg(new MetaType[]{MetaType.register}, new String[]{"AF", "BC", "DE", "HL", "A", "B", "C", "D", "E", "H", "L", "Z", "SP", "SP+r8"}) {
            @Override
            public Tuple2<String, RetType> code(Opcode opcode) {
                var register = opcode.operand2();
                if (register.equals("SP+r8")) {
                    return Tuples.of(
                            STR."""
                                    let flag_effect_l = cpu.register.get_u16(Register::SP);
                                    let flag_effect_r = cpu.imm_u8() as i8 as i16 as u16;
                                    let right = flag_effect_l.wrapping_add(flag_effect_r);
                                    cpu.register.set_u16(Register::HL, right);
                                    """,
                            RetType.u16
                    );
                } else {
                    return switch (register.length()) {
                        case 1 ->
                                Tuples.of(STR. "let right = cpu.register.get_u8(Register::\{ register });" , RetType.u8);
                        case 2 ->
                                Tuples.of(STR. "let right = cpu.register.get_u16(Register::\{ register });" , RetType.u16);
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
            public Tuple2<String, RetType> code(Opcode opcode) {
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
                return Tuples.of(STR. "let right = cpu.memory.borrow().get(\{ code });" , RetType.u8);
            }
        },
        adr_reg_16_ext(new MetaType[]{MetaType.register, MetaType.addr}, new String[]{"(HL+)", "(HL-)"}),//todo

        None(new MetaType[]{}, new String[0]);
        private final MetaType[] metaTypes;
        private final String[] items;

        public static Optional<UnprefixedOperand2> parse(String val) {
            return Stream.of(OperandType.UnprefixedOperand2.values())
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

    public enum CBPrefixedOperand1 implements Operand1 {
        None(new MetaType[]{}, new String[0]);

        private final MetaType[] metaTypes;
        private final String[] items;

        public static Optional<CBPrefixedOperand1> parse(String val) {
            return Stream.of(OperandType.CBPrefixedOperand1.values())
                    .filter(it -> Stream.of(it.items).anyMatch(iit -> StringUtils.equalsIgnoreCase(iit, val)))
                    .findFirst();
        }

        CBPrefixedOperand1(MetaType[] metaTypes, String[] items) {
            this.metaTypes = metaTypes;
            this.items = items;
        }

        @Override
        public MetaType[] getMetaTypes() {
            return metaTypes;
        }

    }

    public enum CBPrefixedOperand2 implements Operand2 {
        None(new MetaType[]{}, new String[0]);


        private final MetaType[] metaTypes;
        private final String[] items;

        public static Optional<CBPrefixedOperand2> parse(String val) {
            return Stream.of(OperandType.CBPrefixedOperand2.values())
                    .filter(it -> Stream.of(it.items).anyMatch(iit -> StringUtils.equalsIgnoreCase(iit, val)))
                    .findFirst();
        }

        CBPrefixedOperand2(MetaType[] metaTypes, String[] items) {

            this.metaTypes = metaTypes;
            this.items = items;
        }

        @Override
        public MetaType[] getMetaTypes() {
            return metaTypes;
        }

    }

    public interface Operand {

        MetaType[] getMetaTypes();

        default boolean metaTypeMatch(MetaType type, MetaType... types) {
            return Stream.of(getMetaTypes()).anyMatch(m -> Stream.concat(Stream.of(type), Stream.of(types)).anyMatch(it -> it == m));
        }
    }

    public interface Operand1 extends Operand {
        default Tuple2<String, RetType> code(Opcode opcode) {
            return Tuples.of("", RetType.none);
        }
    }

    public interface Operand2 extends Operand {
        default Tuple2<String, RetType> code(Opcode opcode) {
            return Tuples.of("", RetType.none);
        }
    }
}


