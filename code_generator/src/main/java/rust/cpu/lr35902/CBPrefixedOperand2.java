package rust.cpu.lr35902;

import org.apache.commons.lang3.StringUtils;

import java.util.Optional;
import java.util.stream.Stream;

public enum CBPrefixedOperand2 implements Operand2 {
    None(new MetaType[]{}, new String[0]);


    private final MetaType[] metaTypes;
    private final String[] items;

    public static Optional<CBPrefixedOperand2> parse(String val) {
        return Stream.of(CBPrefixedOperand2.values())
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
