package rust.cpu.lr35902;

import com.alibaba.fastjson2.JSON;
import com.alibaba.fastjson2.JSONObject;
import com.alibaba.fastjson2.TypeReference;
import org.jetbrains.annotations.NotNull;

import java.io.FileOutputStream;
import java.io.IOException;
import java.net.URI;
import java.nio.charset.StandardCharsets;
import java.nio.file.Path;
import java.util.List;
import java.util.Map;
import java.util.Objects;
import java.util.stream.Collectors;
import java.util.stream.Stream;

public class Opcodes {
    public static void generateRustCodes(String baseRustFilePath) throws Exception {
        var opcodes = Opcodes.getOpcodes();
        var dir = mkdir(baseRustFilePath);
        for (var opcode : opcodes) {
            writeModFile(dir, opcode);
        }

        var modDefFile = Path.of(URI.create(STR. "\{ dir }/mod.rs" )).toFile();
        if (modDefFile.exists()) {
            if (!modDefFile.delete()) {
                throw new IOException(STR. "Delete file: \{ modDefFile.getAbsoluteFile() } error" );
            }
        }
        var modDefContent = STR. """
use crate::cpu::lr35902::LR35902;
use crate::cpu::lr35902::opcode::Opcode;
\{ opcodes.stream().map(it -> STR. "use crate::cpu::lr35902::opcodes::\{ it.modName() }::*;" ).collect(Collectors.joining("\r\n")) }

\{ opcodes.stream().map(it -> STR. "mod \{ it.modName() };" ).collect(Collectors.joining("\r\n")) }

pub static OPCODES: [Option<&'static dyn Opcode>; 0x0200] = [
    \{ getOPCODES(opcodes) }
];

/// ## get_opcode
/// ### addr
/// - instruction set 0x00_XX
/// - ext instruction (prefix cb) 0xCB_XX
pub fn get_opcode(addr: u16) -> Option<&'static dyn Opcode> {
    let opcode_index = if addr & 0xCB00 == 0xCB00 {
        (addr & 0x00FF) | 0x0100
    } else {
        addr
    } as usize;
    OPCODES[opcode_index]
}

#[cfg(test)]
mod tests{
    use crate::cpu::lr35902::opcodes::get_opcode;

    #[test]
    fn test_get_opcode_0x0000(){
        for addr in 0..=0x00FF {
           if let Some(opcode) = get_opcode(addr){
               let meta = opcode.meta();
               assert_ne!(meta.cb_prefixed, true);
               assert_eq!(meta.addr, addr as u8);
           }
        }

        for addr in 0xCB00..=0xCBFF {
            if let Some(opcode) = get_opcode(addr){
                let meta = opcode.meta();
                assert_eq!(meta.cb_prefixed, true);
                assert_eq!(meta.addr, addr as u8);
            }
        }

    }

}

                """ ;
        try (var os = new FileOutputStream(modDefFile)) {
            os.write(modDefContent.getBytes(StandardCharsets.UTF_8));
        }
    }

    @NotNull
    private static String getOPCODES(List<Opcode> codes) {
        var map = codes.stream().collect(Collectors.toMap(Opcode::actualAddrVal, it -> it));
        var sb = new StringBuilder();
        for (var i = 0x00; i < 0x0CC00; ++i) {
            if (i <= 0xFF || 0xCB00 <= i) {
                if (i > 0 && i % 0x10 == 0) {
                    sb.append("\r\n").append("    ");
                }
                var opcode = map.get(i);
                if (opcode == null) {
                    sb.append("None, ");
                } else {
                    sb.append(STR. "Some(&\{ opcode.staticInstanceDef() }), " );
                }
            }
        }
        return sb.toString();
    }

    private static void writeModFile(String dir, Opcode opcode) throws IOException {
        var rustMod = opcode.modName();
        var rustFile = Path.of(URI.create(STR. "\{ dir }/\{ rustMod }.rs" )).toFile();
        if (rustFile.exists()) {
            if (!rustFile.delete()) {
                throw new IOException(STR. "Delete file: \{ rustFile.getAbsoluteFile() } error" );
            }
        }
        try (var os = new FileOutputStream(rustFile)) {
            String content = opcode.rustFileContent();
            os.write(content.getBytes(StandardCharsets.UTF_8));
        }
    }

    private static String mkdir(String RUST_PATH) throws IOException {
        var dir = Path.of(URI.create(RUST_PATH)).toFile();
        if (!dir.exists()) {
            if (!dir.mkdirs()) {
                throw new IOException();
            }
        }
        return RUST_PATH;
    }

    @NotNull
    public static List<Opcode> getOpcodes() throws IOException {
        try (var inputStream = Opcodes.class.getClassLoader().getResourceAsStream("lr35902_opcodes.json")) {
            var opcodesJson = JSON.parseObject(new String(Objects.requireNonNull(inputStream).readAllBytes(), StandardCharsets.UTF_8));
            return Stream.concat(
                            opcodesJson.<Map<String, JSONObject>>getObject("unprefixed", new TypeReference<Map<String, JSONObject>>() {
                            }).values().stream().peek(it -> it.put("cbprefixed", false)).map(json -> JSON.parseObject(json.toJSONString(), Opcode.class)),
                            opcodesJson.<Map<String, JSONObject>>getObject("cbprefixed", new TypeReference<Map<String, JSONObject>>() {
                            }).values().stream().peek(it -> it.put("cbprefixed", true)).map(json -> JSON.parseObject(json.toJSONString(), Opcode.class))
                            )
                    .sorted()
                    .collect(Collectors.toList());
        }
    }
}
