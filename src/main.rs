mod r#enum;

use r#enum::ffopts::Type;

use console::style;
use std::{thread, time::Duration}; 
use std::fmt::{Display, Formatter};

use inquire::{error::InquireResult, Select};

fn main() -> InquireResult<()> {
    println!("Welcome to {}! This is a simple CLI tool to help you configurate your fastfetch settings.", style("effc").magenta());
    // i might uncomment this in the future idk if ppl will like it
    // thread::sleep(Duration::from_millis(1000));

    let opts: Type = Select::new("What would you like to edit?:", Type::VARIANTS.to_vec()).prompt()?;

    match opts {
        Type::Logo => {
            edit_type(Type::Logo);
        }
        Type::Modules => {
            edit_type(Type::Modules);
        } 
    }

    Ok(())
}

fn edit_type(module: Type) -> InquireResult<()> {
    match module {
        Type::Logo => {
            let logo_opts: LogoOptions = Select::new("Logo Options:", LogoOptions::VARIANTS.to_vec()).prompt()?;

            match logo_opts {
                LogoOptions::Image | LogoOptions::Color | LogoOptions::Size | LogoOptions::Style => {
                    println!("a");
                }
            }

            Ok(())
        }
        Type::Modules => {
            let mod_opts: ModuleOptions = Select::new("Module Options:", ModuleOptions::VARIANTS.to_vec()).prompt()?;

            match mod_opts {
                ModuleOptions::Colors | ModuleOptions::Icons => {
                    println!("b");
                }
            }

            Ok(())
        }
    }
}

impl Type {
    const VARIANTS: &'static [Type] = &[
        Self::Logo,
        Self::Modules,
    ];
}

impl Display for Type {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{self:?}")
    }
}

// logo and module options
#[derive(Debug, Copy, Clone)]
#[allow(clippy::upper_case_acronyms)]
enum LogoOptions {
    Image,
    Size,
    Color,
    Style,
    // TODO: add more options
}

#[derive(Debug, Copy, Clone)]
#[allow(clippy::upper_case_acronyms)]
enum ModuleOptions {
    Colors,
    Icons,
    // TODO: add more options
}

impl LogoOptions {
    const VARIANTS: &'static [LogoOptions] = &[
        Self::Image,
        Self::Size,
        Self::Color,
        Self::Style,
    ];
}

impl ModuleOptions {
    const VARIANTS: &'static [ModuleOptions] = &[
        Self::Colors,
        Self::Icons,
    ];
}

impl Display for LogoOptions {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{self:?}")
    }
}

impl Display for ModuleOptions {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{self:?}")
    }
}