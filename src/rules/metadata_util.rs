use cargo_metadata::Package;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum TargetType {
    Binary,
    Libaray,
    Other,
}

pub fn get_target_type(package: &Package) -> TargetType {
    let is_bin = package.targets.iter().any(|t| t.is_bin());
    let is_lib = package.targets.iter().any(|t| t.is_lib());
    if is_bin {
        TargetType::Binary
    } else if is_lib {
        TargetType::Libaray
    } else {
        TargetType::Other
    }
}
