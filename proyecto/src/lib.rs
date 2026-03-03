// Refugio Criollo - Cliente para Solana Playground
// Auto-ejecuta verAnimales() al correr

function getRefugioPDA(fundacion) {
  return web3.PublicKey.findProgramAddressSync(
    [Buffer.from("refugio"), fundacion.toBuffer()],
    pg.PROGRAM_ID
  );
}

async function crearRefugio(nombreRefugio) {
  const [refugioPDA] = getRefugioPDA(pg.wallet.publicKey);

  console.log("Creando refugio...");
  console.log("Fundacion:", pg.wallet.publicKey.toBase58());
  console.log("Refugio PDA:", refugioPDA.toBase58());
  console.log("Nombre:", nombreRefugio);

  const tx = await pg.program.methods
    .crearRefugio(nombreRefugio)
    .accounts({
      fundacion: pg.wallet.publicKey,
      refugio: refugioPDA,
      systemProgram: web3.SystemProgram.programId,
    })
    .rpc();

  console.log("Refugio creado!");
  console.log("Transaction:", tx);
  console.log("Explorer: https://explorer.solana.com/tx/" + tx + "?cluster=devnet");
  
  return tx;
}

async function registrarAnimal(nombreAnimal, edadMeses) {
  const [refugioPDA] = getRefugioPDA(pg.wallet.publicKey);

  console.log("Registrando animal...");
  console.log("Nombre:", nombreAnimal);
  console.log("Edad:", edadMeses, "meses");

  const tx = await pg.program.methods
    .registrarAnimal(nombreAnimal, edadMeses)
    .accounts({
      fundacion: pg.wallet.publicKey,
      refugio: refugioPDA,
    })
    .rpc();

  console.log("Animal registrado!");
  console.log("Transaction:", tx);
  console.log("Explorer: https://explorer.solana.com/tx/" + tx + "?cluster=devnet");
  
  return tx;
}

async function marcarAdoptado(nombreAnimal) {
  const [refugioPDA] = getRefugioPDA(pg.wallet.publicKey);

  console.log("Marcando animal como adoptado...");
  console.log("Animal:", nombreAnimal);

  const tx = await pg.program.methods
    .marcarAdoptado(nombreAnimal)
    .accounts({
      fundacion: pg.wallet.publicKey,
      refugio: refugioPDA,
    })
    .rpc();

  console.log("Animal adoptado!");
  console.log("Transaction:", tx);
  console.log("Explorer: https://explorer.solana.com/tx/" + tx + "?cluster=devnet");
  
  return tx;
}

async function verAnimales() {
  const [refugioPDA] = getRefugioPDA(pg.wallet.publicKey);

  console.log("Consultando animales del refugio...");
  console.log("");

  try {
    const refugioAccount = await pg.program.account.refugio.fetch(refugioPDA);
    
    console.log("========================================");
    console.log("ESTADO DEL REFUGIO");
    console.log("========================================");
    console.log("Fundacion:", refugioAccount.fundacion.toBase58());
    console.log("Nombre:", refugioAccount.nombre);
    console.log("Total animales:", refugioAccount.animales.length);
    console.log("");
    console.log("ANIMALES:");
    
    if (refugioAccount.animales.length === 0) {
      console.log("  (No hay animales registrados)");
    } else {
      refugioAccount.animales.forEach((animal, index) => {
        const estado = animal.disponible ? "Disponible" : "No disponible";
        console.log("");
        console.log("  " + (index + 1) + ". " + animal.nombre);
        console.log("     Edad: " + animal.edadMeses + " meses");
        console.log("     Estado: " + estado);
      });
    }
    
    console.log("");
    console.log("========================================");

    return refugioAccount;
  } catch (error) {
    console.log("========================================");
    console.log("El refugio no existe todavia.");
    console.log("========================================");
    console.log("");
    console.log("Para crear uno, abre la consola del navegador (F12)");
    console.log("y ejecuta:");
    console.log("");
    console.log("  await crearRefugio('Mi Refugio')");
    console.log("");
    console.log("O cambia EJECUTAR_AL_INICIO a 'demo' en el codigo");
    console.log("========================================");
  }
}

async function alternarDisponibilidad(nombreAnimal) {
  const [refugioPDA] = getRefugioPDA(pg.wallet.publicKey);

  console.log("Alternando disponibilidad...");
  console.log("Animal:", nombreAnimal);

  const tx = await pg.program.methods
    .alternarDisponibilidad(nombreAnimal)
    .accounts({
      fundacion: pg.wallet.publicKey,
      refugio: refugioPDA,
    })
    .rpc();

  console.log("Disponibilidad actualizada!");
  console.log("Transaction:", tx);
  console.log("Explorer: https://explorer.solana.com/tx/" + tx + "?cluster=devnet");
  
  return tx;
}

function sleep(ms) {
  return new Promise(resolve => setTimeout(resolve, ms));
}

async function demoCompleto() {
  console.log("");
  console.log("========================================");
  console.log("INICIANDO DEMO DE REFUGIO CRIOLLO");
  console.log("========================================");
  console.log("");
  console.log("Wallet:", pg.wallet.publicKey.toBase58());
  console.log("Program ID:", pg.PROGRAM_ID.toBase58());
  console.log("");

  try {
    console.log("PASO 1: Crear Refugio");
    console.log("========================================");
    await crearRefugio("Fundacion Patitas Felices");
    await sleep(3000);
    
    console.log("");
    console.log("PASO 2: Registrar Animales");
    console.log("========================================");
    
    await registrarAnimal("Max", 24);
    await sleep(2000);
    
    await registrarAnimal("Luna", 18);
    await sleep(2000);
    
    await registrarAnimal("Choco", 36);
    await sleep(2000);

    console.log("");
    console.log("PASO 3: Consultar Animales");
    console.log("========================================");
    await verAnimales();

    console.log("");
    console.log("PASO 4: Cambiar Disponibilidad de Max");
    console.log("========================================");
    await alternarDisponibilidad("Max");
    await sleep(2000);

    console.log("");
    console.log("PASO 5: Verificar Cambios");
    console.log("========================================");
    await verAnimales();

    console.log("");
    console.log("PASO 6: Adoptar a Luna");
    console.log("========================================");
    await marcarAdoptado("Luna");
    await sleep(2000);

    console.log("");
    console.log("PASO 7: Estado Final");
    console.log("========================================");
    await verAnimales();

    const EJECUTAR_AL_INICIO = "demo";  // Cambia "ver" por "demo"
    console.log("========================================");
    console.log("DEMO COMPLETADO EXITOSAMENTE!");
    console.log("========================================");
    console.log("");

  } catch (error) {
    console.error("");
    console.error("Error:", error.message);
    console.log("");
    if (error.message && error.message.includes("already in use")) {
      console.log("El refugio ya existe.");
      console.log("Ejecutando testConRefugioExistente...");
      await testConRefugioExistente();
    }
  }
}

async function testConRefugioExistente() {
  console.log("");
  console.log("Test con refugio existente");
  console.log("");

  try {
    await verAnimales();

    console.log("");
    console.log("========================================");
    console.log("Registrando nuevo animal...");
    console.log("========================================");
    const nombreRandom = "Animal" + Math.floor(Math.random() * 1000);
    await registrarAnimal(nombreRandom, 12);
    await sleep(2000);

    await verAnimales();

    console.log("");
    console.log("Test completado!");
  } catch (error) {
    console.error("Error:", error.message);
  }
}

// ============================================================================
// CONFIGURACION: Cambia esto para ejecutar diferente accion al hacer Run
// ============================================================================

const EJECUTAR_AL_INICIO = "ver";  // Opciones: "ver", "demo", "test", "nada"

// ============================================================================
// AUTO-EJECUCION
// ============================================================================

console.log("");
console.log("========================================");
console.log("REFUGIO CRIOLLO - CLIENTE");
console.log("========================================");
console.log("");
console.log("Program ID:", pg.PROGRAM_ID.toBase58());
console.log("Wallet:", pg.wallet.publicKey.toBase58());
console.log("");

(async () => {
  if (EJECUTAR_AL_INICIO === "ver") {
    console.log("Ejecutando: verAnimales()");
    console.log("");
    await verAnimales();
  } else if (EJECUTAR_AL_INICIO === "demo") {
    console.log("Ejecutando: demoCompleto()");
    console.log("");
    await demoCompleto();
  } else if (EJECUTAR_AL_INICIO === "test") {
    console.log("Ejecutando: testConRefugioExistente()");
    console.log("");
    await testConRefugioExistente();
  } else {
    console.log("COMANDOS DISPONIBLES (usa consola del navegador F12):");
    console.log("");
    console.log("await verAnimales()");
    console.log("await demoCompleto()");
    console.log("await crearRefugio('Nombre')");
    console.log("await registrarAnimal('Nombre', edad)");
    console.log("await alternarDisponibilidad('Nombre')");
    console.log("await marcarAdoptado('Nombre')");
    console.log("await testConRefugioExistente()");
    console.log("");
    console.log("========================================");
  }
})();
