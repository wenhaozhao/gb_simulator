package rust.cpu.lr35902;

public class FlagEffect {

    public static final String Z = "Z";
    public static final String N = "N";
    public static final String H = "H";
    public static final String C = "C";
    private static final String[] FLAGS = {Z, N, H, C};
    private final String flag;
    private final Type type;

    public FlagEffect(String[] flags, int i) {
        assert flags.length == 4;
        this.flag = FLAGS[i];
        this.type = flagEffect(flags, i);
    }

    private static Type flagEffect(String[] flags, int i) {
        var flag = flags[i].trim();
        return switch (flag) {
            case "-" -> Type.None;
            case "0" -> Type.Reset;
            case "1" -> Type.Set;
            default -> Type.Fun;
        };
    }

    public String getFlag() {
        return flag;
    }

    public String getFullFlag() {
        return STR. "Flag::\{ this.getFlag() }" ;
    }

    public Type getType() {
        return type;
    }

    public enum Type {
        /// 无影响
        None() {
            @Override
            public String toString(FlagEffect flagEffect) {
                return STR."FlagEffect::None";
            }
        },
        /// 指令之后它被重置
        Reset() {
            @Override
            public String toString(FlagEffect flagEffect) {
                return STR. "FlagEffect::Reset(\{ flagEffect.getFullFlag() })" ;
            }

            @Override
            public String effect(Opcode opcode, FlagEffect flagEffect) {
                return STR. "cpu.register.set_flag(\{ flagEffect.getFullFlag() }, false);" ;
            }
        },
        /// 已设置
        Set() {
            @Override
            public String toString(FlagEffect flagEffect) {
                return STR. "FlagEffect::Set(\{ flagEffect.getFullFlag() })" ;
            }

            @Override
            public String effect(Opcode opcode, FlagEffect flagEffect) {
                return STR. "cpu.register.set_flag(\{ flagEffect.getFullFlag() }, true);" ;
            }
        },
        /// 相应的标志将受到其功能的预期影响
        Fun() {
            @Override
            public String toString(FlagEffect flagEffect) {
                return STR. "FlagEffect::Fun(\{ flagEffect.getFullFlag() })" ;
            }

        };

        public abstract String toString(FlagEffect flagEffect);

        public String effect(Opcode opcode, FlagEffect flagEffect) {
            return "";
        }
    }

    @Override
    public String toString() {
        return this.getType().toString(this);
    }
}
