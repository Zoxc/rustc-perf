use crate::cdsl::inst::InstructionGroup;
use crate::cdsl::isa::TargetIsa;
use crate::cdsl::regs::{IsaRegs, IsaRegsBuilder, RegBankBuilder, RegClassBuilder};
use crate::cdsl::settings::{PredicateNode, SettingGroup, SettingGroupBuilder};
use crate::shared::Definitions as SharedDefinitions;

fn define_settings(shared: &SettingGroup) -> SettingGroup {
    let mut setting = SettingGroupBuilder::new("riscv");

    let supports_m = setting.add_bool(
        "supports_m",
        "CPU supports the 'M' extension (mul/div)",
        false,
    );
    let supports_a = setting.add_bool(
        "supports_a",
        "CPU supports the 'A' extension (atomics)",
        false,
    );
    let supports_f = setting.add_bool(
        "supports_f",
        "CPU supports the 'F' extension (float)",
        false,
    );
    let supports_d = setting.add_bool(
        "supports_d",
        "CPU supports the 'D' extension (double)",
        false,
    );

    let enable_m = setting.add_bool(
        "enable_m",
        "Enable the use of 'M' instructions if available",
        true,
    );

    setting.add_bool(
        "enable_e",
        "Enable the 'RV32E' instruction set with only 16 registers",
        true,
    );

    let shared_enable_atomics = shared.get_bool("enable_atomics");
    let shared_enable_float = shared.get_bool("enable_float");
    let shared_enable_simd = shared.get_bool("enable_simd");

    setting.add_predicate("use_m", predicate!(supports_m && enable_m));
    setting.add_predicate("use_a", predicate!(supports_a && shared_enable_atomics));
    setting.add_predicate("use_f", predicate!(supports_f && shared_enable_float));
    setting.add_predicate("use_d", predicate!(supports_d && shared_enable_float));
    setting.add_predicate(
        "full_float",
        predicate!(shared_enable_simd && supports_f && supports_d),
    );

    setting.finish()
}

fn define_registers() -> IsaRegs {
    let mut regs = IsaRegsBuilder::new();

    let builder = RegBankBuilder::new("IntRegs", "x")
        .units(32)
        .track_pressure(true);
    let int_regs = regs.add_bank(builder);

    let builder = RegBankBuilder::new("FloatRegs", "f")
        .units(32)
        .track_pressure(true);
    let float_regs = regs.add_bank(builder);

    let builder = RegClassBuilder::new_toplevel("GPR", int_regs);
    regs.add_class(builder);

    let builder = RegClassBuilder::new_toplevel("FPR", float_regs);
    regs.add_class(builder);

    regs.finish()
}

pub fn define(shared_defs: &mut SharedDefinitions) -> TargetIsa {
    let settings = define_settings(&shared_defs.settings);
    let regs = define_registers();

    let inst_group = InstructionGroup::new("riscv", "riscv specific instruction set");

    TargetIsa::new("riscv", inst_group, settings, regs)
}
