package rust.cpu.lr35902;

public class CodeInfo {

    final String code;
    final RetType retType;

    public CodeInfo(String code, RetType retType) {
        this.code = code;
        this.retType = retType;
    }

    public String getCode() {
        return code;
    }

    public RetType getRetType() {
        return retType;
    }
}
