use anchor_lang::prelude::*;

declare_id!("7FdoTRNSdgya71wk4h3ZSf4dtuiW7ZJozSG3RBR8sxQR");

#[program]
pub mod anchor_todo_list_project {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
