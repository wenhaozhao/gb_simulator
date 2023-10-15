package rust.cpu.lr35902.ins;

import rust.cpu.lr35902.Opcode;

public enum UnprefixedIns implements Ins {
    ADC() {
        @Override
        public String fnExec(Opcode opcode) {
            return new ADC().fnExec(opcode);
        }
    },
    ADD() {
        @Override
        public String fnExec(Opcode opcode) {
            return new ADD().fnExec(opcode);
        }
    },
    AND {
        @Override
        public String fnExec(Opcode opcode) {
            return new AND().fnExec(opcode);
        }
    },
    CALL{
        @Override
        public String fnExec(Opcode opcode) {
            return new CALL().fnExec(opcode);
        }
    },
    CCF {
        @Override
        public String fnExec(Opcode opcode) {
            return STR. """
                fn exec(&self, cpu: &mut LR35902) -> u8 {
                        let flag_c_before = cpu.register.get_flag(Flag::C);
                        cpu.register.set_flag(Flag::C, !flag_c_before);
                        cpu.register.set_flag(Flag::N, false);
                        cpu.register.set_flag(Flag::H, false);
                        self.meta.cycles[0]
                }""" ;
        }
    },
    CP,
    CPL,
    DAA,
    DEC,
    DI(){
        @Override
        public String fnExec(Opcode opcode) {
            return STR."""
                    fn exec(&self, cpu: &mut LR35902) -> u8 {
                            cpu.enable_interrupt = false;
                            self.meta.cycles[0]
                        }""";
        }
    },
    EI(){
        @Override
        public String fnExec(Opcode opcode) {
            return STR."""
                    fn exec(&self, cpu: &mut LR35902) -> u8 {
                            cpu.enable_interrupt = true;
                            self.meta.cycles[0]
                        }""";
        }
    },
    HALT (){
        @Override
        public String fnExec(Opcode opcode) {
            return STR."""
            fn exec(&self, cpu: &mut LR35902) -> u8 {
                    self.meta.cycles[0]
            }""";
        }
    },
    INC,
    JP,
    JR() {
        @Override
        public String fnExec(Opcode opcode) {
            return new JR().fnExec(opcode);
        }
    },
    LD() {
        @Override
        public String fnExec(Opcode opcode) {
            return new LD().fnExec(opcode);
        }
    },
    LDH,
    NOP(){
        @Override
        public String fnExec(Opcode opcode) {
            return STR."""
            fn exec(&self, cpu: &mut LR35902) -> u8 {
                    self.meta.cycles[0]
            }""";
        }
    },
    OR,
    POP,
    PREFIX,
    PUSH,
    RET,
    RETI,
    RLA,
    RLCA,
    RRA,
    RRCA,
    RST,
    SBC,
    SCF(){
        @Override
        public String fnExec(Opcode opcode) {
            return STR."""
            fn exec(&self, cpu: &mut LR35902) -> u8 {
                    cpu.register.set_flag(Flag::C, true);
                    cpu.register.set_flag(Flag::N, false);
                    cpu.register.set_flag(Flag::H, false);
                    self.meta.cycles[0]
            }""";
        }
    },
    STOP(){
        @Override
        public String fnExec(Opcode opcode) {
            return STR."""
            fn exec(&self, cpu: &mut LR35902) -> u8 {
                    self.meta.cycles[0]
            }""";
        }
    },
    SUB,
    XOR;

}
