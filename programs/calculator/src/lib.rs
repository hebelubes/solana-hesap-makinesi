// programs/calculator/src/lib.rs
use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS"); // Benzersiz program ID'si

#[program]
pub mod calculator {
    use super::*;

    // Hesap makinesi veri yapısı
    #[account]
    pub struct CalculatorAccount {
        pub result: u64, // Sonucu saklar
    }

    // Toplama işlemi
    pub fn add(ctx: Context<Calculate>, a: u64, b: u64) -> Result<()> {
        let result = a + b;
        ctx.accounts.calculator.result = result;
        msg!("Toplama: {} + {} = {}", a, b, result);
        Ok(())
    }

    // Çıkarma işlemi
    pub fn subtract(ctx: Context<Calculate>, a: u64, b: u64) -> Result<()> {
        require!(a >= b, CalculatorError::NegativeResult);
        let result = a - b;
        ctx.accounts.calculator.result = result;
        msg!("Çıkarma: {} - {} = {}", a, b, result);
        Ok(())
    }

    // Çarpma işlemi
    pub fn multiply(ctx: Context<Calculate>, a: u64, b: u64) -> Result<()> {
        let result = a * b;
        ctx.accounts.calculator.result = result;
        msg!("Çarpma: {} * {} = {}", a, b, result);
        Ok(())
    }

    // Bölme işlemi
    pub fn divide(ctx: Context<Calculate>, a: u64, b: u64) -> Result<()> {
        require!(b > 0, CalculatorError::DivisionByZero);
        let result = a / b;
        ctx.accounts.calculator.result = result;
        msg!("Bölme: {} / {} = {}", a, b, result);
        Ok(())
    }
}

// Hesap makinesi için context
#[derive(Accounts)]
pub struct Calculate<'info> {
    #[account(mut)]
    pub calculator: Account<'info, CalculatorAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

// Hata kodları
#[error_code]
pub enum CalculatorError {
    #[msg("Sonuç negatif olamaz")]
    NegativeResult,
    #[msg("Sıfıra bölme hatası")]
    DivisionByZero,
}
