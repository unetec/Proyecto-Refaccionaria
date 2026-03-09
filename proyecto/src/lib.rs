use anchor_lang::prelude::*;

declare_id!("72QMStJ3YzjufKXN5DKEEUSAKYawXpmiBQgLgXxfQm4E");

#[program]
pub mod refaccionaria {
    use super::*;

    pub fn crear_refaccionaria(context: Context<NuevaRefaccionaria>, n_refaccionaria: String) -> Result<()> {
        let owner_id = context.accounts.owner.key();

        let refacciones = Vec::<Pubkey>::new();

        context.accounts.refaccionaria.set_inner(Refaccionaria { 
            owner: owner_id,
            n_refaccionaria: n_refaccionaria.clone(),
            refacciones,
        });

        msg!("Refaccionaria {}, creada exitosamente!. Owner id: {}", n_refaccionaria.clone(), owner_id);

        Ok(())
    }

    pub fn agregar_refaccion(context: Context<NuevaRefaccion>, nombre: String, automovil: String, marca: String, modelo: String, año: u16, precio: u64) -> Result<()> {
        
        require!(
            context.accounts.refaccionaria.owner == context.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        let refaccion = Refaccion {
            refaccionaria: context.accounts.refaccionaria.n_refaccionaria.clone(),
            nombre: nombre.clone(),
            automovil,
            marca,
            modelo,
            año,
            precio,
            disponible: true,
        };

        context.accounts.refaccion.set_inner(refaccion);

        context
            .accounts
            .refaccionaria
            .refacciones
            .push(context.accounts.refaccion.key());

        msg!("Refaccion {}, creada exitosamente, en la refaccionaria {}!. Owner id: {}", nombre.clone(),  context.accounts.refaccionaria.n_refaccionaria, context.accounts.owner.key());
    
        Ok(())
    }

    pub fn eliminar_refaccion(context: Context<EliminarRefaccion>, nombre: String) -> Result<()> {
        require!(
            context.accounts.refaccionaria.owner == context.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        let refaccionaria = &mut context.accounts.refaccionaria;
        let refacciones = &refaccionaria.refacciones;

        require!(
            context.accounts.refaccion.refaccionaria == refaccionaria.n_refaccionaria,
            Errores::RefaccionNoPertenece
        );

        require!(refaccionaria.refacciones.contains(&context.accounts.refaccion.key()), Errores::RefaccionNoExiste);

        let mut pos = 0;

        for i in 0..refacciones.len() {
            if refacciones[i] == context.accounts.refaccion.key() {
                pos = i;
                break
            }
        }

        refaccionaria.refacciones.remove(pos);

        msg!("Refaccion '{}' eliminada exitosamente de la refaccionaria {}!. Owner id: {}", nombre, refaccionaria.n_refaccionaria, context.accounts.owner.key());
            
        Ok(())
    }

    pub fn alternar_estado(context: Context<ModificarRefaccion>, nombre: String) -> Result<()> {
        require!(
            context.accounts.refaccionaria.owner == context.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        let refaccion = &mut context.accounts.refaccion;
        let estado = refaccion.disponible;
        let nuevo_estado = !estado;
        refaccion.disponible = nuevo_estado;
        
        msg!(
            "La refaccion: {} ahora tiene un valor de disponibilidad: {}",
            nombre,
            nuevo_estado
        );

        Ok(())
    }
}

#[error_code]
pub enum Errores {
    #[msg("Error, no eres el propietario de la refaccionaria que deseas modificar")]
    NoEresElOwner,
    #[msg("Error, la refaccion con la que deseas interactuar no existe")]
    RefaccionNoExiste,
    #[msg("Error, la refaccion no pertenece a esta refaccionaria")]
    RefaccionNoPertenece,
}

#[account]
#[derive(InitSpace)]
pub struct Refaccionaria {
    pub owner: Pubkey,

    #[max_len(60)]
    pub n_refaccionaria: String,

    #[max_len(10)]
    pub refacciones: Vec<Pubkey>,
}

#[account]
#[derive(InitSpace, PartialEq, Debug)]
pub struct Refaccion {
    #[max_len(60)]
    pub refaccionaria: String,

    #[max_len(60)]
    pub nombre: String,

    #[max_len(30)]
    pub automovil: String,

    #[max_len(30)]
    pub marca: String,

    #[max_len(30)]
    pub modelo: String,

    pub año: u16,

    pub precio: u64,

    pub disponible: bool,
}

#[derive(Accounts)]
#[instruction(n_refaccionaria:String)]
pub struct NuevaRefaccionaria<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        init,
        payer = owner, 
        space = 8 + Refaccionaria::INIT_SPACE, 
        seeds = [b"refaccionaria", n_refaccionaria.as_bytes(), owner.key().as_ref()],
        bump
    )]
    pub refaccionaria: Account<'info, Refaccionaria>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(nombre:String)]
pub struct NuevaRefaccion<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        init,
        payer = owner, 
        space = 8 + Refaccion::INIT_SPACE,
        seeds = [b"refaccion", nombre.as_bytes(), owner.key().as_ref()],
        bump
    )]
    pub refaccion: Account<'info, Refaccion>,

    #[account(mut)]
    pub refaccionaria: Account<'info, Refaccionaria>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ModificarRefaccion<'info> {
    pub owner: Signer<'info>,

    #[account(mut)]
    pub refaccion: Account<'info, Refaccion>,

    #[account(mut)]
    pub refaccionaria: Account<'info, Refaccionaria>,
}

#[derive(Accounts)]
pub struct EliminarRefaccion<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        mut,
        close = owner,
        constraint = refaccion.refaccionaria == refaccionaria.n_refaccionaria @ Errores::RefaccionNoPertenece
    )]
    pub refaccion: Account<'info, Refaccion>,

    #[account(mut)]
    pub refaccionaria: Account<'info, Refaccionaria>,
}
