package rust.cpu.lr35902;

public class Main {

    private static final String RUST_PATH = "file:///Users/zhaowenhao/Documents/my/gb_simulator/src/cpu/lr35902/opcodes";

    public static void main(String[] args) throws Exception {
        Opcodes.generateRustCodes(RUST_PATH);
    }


}
