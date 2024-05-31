use crate::CreateBrand;
use anchor_lang::prelude::*;

pub fn create_brand(ctx: Context<CreateBrand>, brand_name: String) -> Result<()> {
    let brand = &mut ctx.accounts.brand;
    brand.name = brand_name;
    brand.owner = *ctx.accounts.admin.key;
    Ok(())
}
