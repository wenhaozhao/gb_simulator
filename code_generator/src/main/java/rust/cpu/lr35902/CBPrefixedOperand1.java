package rust.cpu.lr35902;

import org.apache.commons.lang3.StringUtils;

import java.util.Optional;
import java.util.stream.Stream;

public enum CBPrefixedOperand1 implements Operand1 {
    None(new MetaType[]{}, new String[0]);

    private final MetaType[] metaTypes;
    private final String[] items;

    public static Optional<CBPrefixedOperand1> parse(String val) {
        return Stream.of(CBPrefixedOperand1.values())
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
