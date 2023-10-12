package rust.cpu.lr35902;

import java.util.stream.Stream;

public interface Operand {

    MetaType[] getMetaTypes();

    default boolean metaTypeMatch(MetaType type, MetaType... types) {
        return Stream.of(getMetaTypes()).anyMatch(m -> Stream.concat(Stream.of(type), Stream.of(types)).anyMatch(it -> it == m));
    }
}
