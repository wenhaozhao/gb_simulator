package rust.cpu.lr35902;

public class Register {

    public static void main(String[] args) {
        var regName = "bc";
        var code = STR."""

            impl Registers {

                #[inline]
                pub fn get_\{regName}(&self) -> u16 {
                    self.\{regName}
                }

                #[inline]
                pub fn set_\{regName}(&mut self, val: u16) {
                    self.\{regName} = val;
                }
               \s
                #[inline]
                pub fn \{regName}_incr_by(&mut self, by: u16) {
                    self.\{regName} += by;
                }

                #[inline]
                pub fn \{regName}_get_and_incr_by(&mut self, by: u16) -> u16 {
                    let before = self.\{regName};
                    self.\{regName}_incr_by(by);
                    before
                }

                #[inline]
                pub fn \{regName}_incr(&mut self) {
                    self.\{regName}_incr_by(0x0001u16);
                }

                #[inline]
                pub fn \{regName}_get_and_incr(&mut self) -> u16 {
                    let before = self.\{regName};
                    self.\{regName}_incr();
                    before
                }

                #[inline]
                pub fn \{regName}_incr_by_and_get(&mut self, by: u16) -> u16 {
                    self.\{regName}_incr_by(by);
                    self.\{regName}
                }

                #[inline]
                pub fn \{regName}_incr_and_get(&mut self) -> u16 {
                    self.\{regName}_incr_by_and_get(0x0001u16);
                    self.\{regName}
                }

                #[inline]
                pub fn \{regName}_decr_by(&mut self, by: u16) {
                    self.\{regName} -= by;
                }

                #[inline]
                pub fn \{regName}_get_and_decr_by(&mut self, by: u16) -> u16 {
                    let before = self.\{regName};
                    self.\{regName}_decr_by(by);
                    before
                }

                #[inline]
                pub fn \{regName}_decr(&mut self) {
                    self.\{regName}_decr_by(0x0001u16);
                }

                #[inline]
                pub fn \{regName}_get_and_decr(&mut self) -> u16 {
                    let before = self.\{regName};
                    self.\{regName}_decr();
                    before
                }

                #[inline]
                pub fn \{regName}_decr_by_and_get(&mut self, by: u16) -> u16 {
                    self.\{regName}_decr_by(by);
                    self.\{regName}
                }

                #[inline]
                pub fn \{regName}_decr_and_get(&mut self) -> u16 {
                    self.\{regName}_decr_by_and_get(0x0001u16);
                    self.\{regName}
                }
            }
            """;
        System.out.println(code);
    }

}

