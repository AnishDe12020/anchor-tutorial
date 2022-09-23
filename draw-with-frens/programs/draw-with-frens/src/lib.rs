use anchor_lang::prelude::*;

declare_id!("5NUFiWz9TS3aNuJSoRBvDHznxRckqjakeCqypS4Dbo3V");

#[program]
pub mod draw_with_frens {
    use super::*;

    const MIN_POS: u8 = 0;
    const MAX_POS: u8 = 99;
    const MIN_COL: u8 = 0;
    const MAX_COL: u8 = 255;

    pub fn create_pixel(
        ctx: Context<CreatePixel>,
        pos_x: u8,
        pos_y: u8,
        init_col_r: u8,
        init_col_g: u8,
        init_col_b: u8,
    ) -> Result<()> {
        if pos_x < MIN_POS || pos_x > MAX_POS {
            return Err(error!(ErrorCode::InvalidXCoordinate));
        }

        if pos_y < MIN_POS || pos_y > MAX_POS {
            return Err(error!(ErrorCode::InvalidYCoordinate));
        }

        if init_col_r < MIN_COL || init_col_r > MAX_COL {
            return Err(error!(ErrorCode::InvalidRColor));
        }

        if init_col_b < MIN_COL || init_col_b > MAX_COL {
            return Err(error!(ErrorCode::InvalidBColor));
        }

        if init_col_g < MIN_COL || init_col_g > MAX_COL {
            return Err(error!(ErrorCode::InvalidGColor));
        }

        let pixel = &mut ctx.accounts.pixel;
        pixel.pos_x = pos_x;
        pixel.pos_y = pos_y;
        pixel.col_r = init_col_r;
        pixel.col_g = init_col_g;
        pixel.col_b = init_col_b;
        pixel.bump = *ctx.bumps.get("pixel").unwrap();

        emit!(PixelChanged {
            pos_x,
            pos_y,
            col_r: init_col_r,
            col_g: init_col_g,
            col_b: init_col_b,
        });

        Ok(())
    }

    pub fn update_pixel(
        ctx: Context<UpdatePixel>,
        new_col_r: u8,
        new_col_g: u8,
        new_col_b: u8,
    ) -> Result<()> {
        // Validation
        if new_col_r < MIN_COL || new_col_r > MAX_COL {
            return Err(error!(ErrorCode::InvalidRColor));
        }

        if new_col_g < MIN_COL || new_col_g > MAX_COL {
            return Err(error!(ErrorCode::InvalidBColor));
        }

        if new_col_b < MIN_COL || new_col_b > MAX_COL {
            return Err(error!(ErrorCode::InvalidGColor));
        }

        // Set values
        let pixel = &mut ctx.accounts.pixel;
        pixel.col_r = new_col_r;
        pixel.col_g = new_col_g;
        pixel.col_b = new_col_b;

        emit!(PixelChanged {
            pos_x: pixel.pos_x,
            pos_y: pixel.pos_y,
            col_r: new_col_r,
            col_g: new_col_g,
            col_b: new_col_b,
        });

        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(pos_x: u8, pos_y: u8)]
pub struct CreatePixel<'info> {
    #[account(
        init,
        payer=user,
        space=Pixel::LEN,
        seeds=[b"pixel".as_ref(),[pos_x, pos_y].as_ref()],
        bump
    )]
    pub pixel: Account<'info, Pixel>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdatePixel<'info> {
    #[account(
        mut,
        seeds = [b"pixel".as_ref(), [pixel.pos_x, pixel.pos_y].as_ref()],
         bump = pixel.bump,
    )]
    pub pixel: Account<'info, Pixel>,
}

#[account]
pub struct Pixel {
    pub pos_x: u8,
    pub pos_y: u8,
    pub col_r: u8,
    pub col_g: u8,
    pub col_b: u8,
    pub bump: u8,
}

const DISCRIMINATOR_LENGTH: usize = 8;
const POS_LENGTH: usize = 1;
const COL_LENGTH: usize = 1;
const BUMP_LENGTH: usize = 1;

impl Pixel {
    const LEN: usize = DISCRIMINATOR_LENGTH + (POS_LENGTH * 2) + (COL_LENGTH * 3) + BUMP_LENGTH;
}

#[error_code]
pub enum ErrorCode {
    #[msg("The given x coordinate is not between 0-99")]
    InvalidXCoordinate,
    #[msg("The given y coordinate is not between 0-99")]
    InvalidYCoordinate,
    #[msg("The given Red color is not between 0-255")]
    InvalidRColor,
    #[msg("The given Green color is not between 0-255")]
    InvalidGColor,
    #[msg("The given Blue color is not between 0-255")]
    InvalidBColor,
}

#[event]
pub struct PixelChanged {
    pub pos_x: u8,
    pub pos_y: u8,
    pub col_r: u8,
    pub col_g: u8,
    pub col_b: u8,
}
