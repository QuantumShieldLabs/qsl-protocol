pub mod establish;
pub mod init;
pub mod recv;
pub mod register;
pub mod relay;
pub mod rotate;
pub mod send;
pub mod status;

pub fn stub(name: &str) -> Result<(), String> {
    Err(format!(
        "{name} is not implemented yet (see NA-0015 PR2 for sessionful Suite-2 flows)."
    ))
}
