package rust.cpu.lr35902;

public record CodeInfo(String code, RetType retType) {

    static CodeInfo of(String code, RetType retType) {
        return new CodeInfo(code, retType);
    }

}
