//////////////////// Imports ////////////////////
import { PublicKey } from "@solana/web3.js";

////////////////// Constantes ////////////////////
const n_refaccionaria = "MiRefa"; // Nombre de la refaccionaria
const owner = pg.wallet.publicKey; // Wallet

console.log("Mi dirección:", owner.toString());
const balance = await pg.connection.getBalance(owner);
console.log(`Mi balance: ${balance / web3.LAMPORTS_PER_SOL} SOL`);

//////////////////// FUNCIONES ////////////////////

//////////////////// OBTENER PDAs ////////////////////

//////////////////// Refaccionaria ////////////////////
function pdaRefaccionaria(n_refaccionaria: string) {
  return PublicKey.findProgramAddressSync(
    [
      Buffer.from("refaccionaria"), // Semilla 1: b"refaccionaria"
      Buffer.from(n_refaccionaria), // Semilla 2: nombre de la refaccionaria -> String
      owner.toBuffer(), // Semilla 3: wallet -> Pubkey
    ],
    pg.PROGRAM_ID
  );
}

//////////////////// Refaccion ////////////////////
function pdaRefaccion(n_refaccion: string) {
  return PublicKey.findProgramAddressSync(
    [
      Buffer.from("refaccion"), // Semilla 1: b"refaccion"
      Buffer.from(n_refaccion), // Semilla 2: nombre de la refaccion -> String
      owner.toBuffer(), // Semilla 3: wallet -> Pubkey
    ],
    pg.PROGRAM_ID
  );
}

//////////////////// Crear Refaccionaria ////////////////////
// Para crear la refaccionaria solo es necesario el nombre que tendrá
async function crearRefaccionaria(n_refaccionaria: string) {
  const [pda_refaccionaria] = pdaRefaccionaria(n_refaccionaria);

  const txHash = await pg.program.methods
    .crearRefaccionaria(n_refaccionaria) // crear_refaccionaria
    .accounts({
      owner: owner,
      refaccionaria: pda_refaccionaria,
    })
    .rpc();

  console.log("txHash: ", txHash);
}

//////////////////// Agregar Refacción ////////////////////
// Para crear una refacción se pasan todos los campos del struct Refaccion
async function agregarRefaccion(
  nombre: string,
  automovil: string,
  marca: string,
  modelo: string,
  año: number,
  precio: number
) {
  const [pda_refaccion] = pdaRefaccion(nombre);
  const [pda_refaccionaria] = pdaRefaccionaria(n_refaccionaria);

  const txHash = await pg.program.methods
    .agregarRefaccion(nombre, automovil, marca, modelo, año, new BN(precio))
    .accounts({
      owner: owner,
      refaccion: pda_refaccion,
      refaccionaria: pda_refaccionaria,
    })
    .rpc();

  console.log("txHash: ", txHash);
}

//////////////////// Alternar estado ////////////////////
// Cambia disponible true/false de una refacción por nombre
async function cambiarEstado(nombre: string) {
  const [pda_refaccion] = pdaRefaccion(nombre);
  const [pda_refaccionaria] = pdaRefaccionaria(n_refaccionaria);

  const txHash = await pg.program.methods
    .alternarEstado(nombre)
    .accounts({
      owner: owner,
      refaccion: pda_refaccion,
      refaccionaria: pda_refaccionaria,
    })
    .rpc();

  console.log("txHash: ", txHash);
}

//////////////////// Eliminar Refacción ////////////////////
// Elimina una refacción por nombre
async function eliminarRefaccion(nombre: string) {
  const [pda_refaccion] = pdaRefaccion(nombre);
  const [pda_refaccionaria] = pdaRefaccionaria(n_refaccionaria);

  const txHash = await pg.program.methods
    .eliminarRefaccion(nombre)
    .accounts({
      owner: owner,
      refaccion: pda_refaccion,
      refaccionaria: pda_refaccionaria,
    })
    .rpc();

  console.log("txHash: ", txHash);
}

//////////////////// Ver Refacciones ////////////////////
// Lee las cuentas de la refaccionaria y cada refacción asociada
async function verRefacciones(n_refaccionaria: string) {
  const [pda_refaccionaria] = pdaRefaccionaria(n_refaccionaria);

  try {
    const refaccionariaAccount = await pg.program.account.refaccionaria.fetch(
      pda_refaccionaria
    );

    const numero_refacciones = refaccionariaAccount.refacciones.length;

    if (!refaccionariaAccount.refacciones || numero_refacciones === 0) {
      console.log("Refaccionaria vacía");
      return;
    }

    console.log("Cantidad de refacciones:", numero_refacciones);

    for (let i = 0; i < numero_refacciones; i++) {
      const refaccionKey = refaccionariaAccount.refacciones[i];

      const refaccionAccount = await pg.program.account.refaccion.fetch(
        refaccionKey
      );

      console.log(
        `Refacción #${i + 1}: \n` +
          ` * Nombre: ${refaccionAccount.nombre} \n` +
          ` * Automóvil: ${refaccionAccount.automovil} \n` +
          ` * Marca: ${refaccionAccount.marca} \n` +
          ` * Modelo: ${refaccionAccount.modelo} \n` +
          ` * Año: ${refaccionAccount.año} \n` +
          ` * Precio: ${refaccionAccount.precio.toString()} \n` +
          ` * Refaccionaria: ${refaccionAccount.refaccionaria} \n` +
          ` * Disponible: ${refaccionAccount.disponible} \n` +
          ` * Dirección(PDA): ${refaccionKey.toBase58()}`
      );
    }
  } catch (error: any) {
    console.error("Error viendo refacciones:", error);

    if (error.message) {
      console.error("Mensaje de error:", error.message);
    }
    if (error.logs) {
      console.error("Logs del programa:", error.logs);
    }
  }
}

// Ejemplos de uso:
//crearRefaccionaria(n_refaccionaria);
// agregarRefaccion("Parabrisas", "Mustang", "Ford", "GT500", 1967, 6800);
// eliminarRefaccion("Filtro de aceite");
// cambiarEstado("Filtro de aceite");
verRefacciones(n_refaccionaria);


// solana confirm -v <txHash>
