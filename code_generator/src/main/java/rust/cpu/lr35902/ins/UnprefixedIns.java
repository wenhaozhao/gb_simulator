package rust.cpu.lr35902.ins;

import rust.cpu.lr35902.Opcode;

public enum UnprefixedIns implements Ins {
    ADC(){
        @Override
        public String fnExec(Opcode opcode) {
            return super.fnExec(opcode);
        }
    },
    ADD,
    AND,
    CALL,
    CCF,
    CP,
    CPL,
    DAA,
    DEC,
    DI,
    EI,
    HALT,
    INC,
    JP,
    JR(){
        @Override
        public String fnExec(Opcode opcode) {
            return new JR().fnExec(opcode);
        }
    },
    LD(){
        @Override
        public String fnExec(Opcode opcode) {
            return new LD().fnExec(opcode);
        }
    },
    LDH,
    NOP,
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
    SCF,
    STOP,
    SUB,
    XOR;

}