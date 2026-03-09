🛠️ REFAX — Refaccionaria Descentralizada en Solana
Una refaccionaria minimalista y completamente on-chain para gestionar inventario de autopartes utilizando Program Derived Addresses (PDAs) sobre Solana.

🔧 Descripción General
REFAX es un smart contract desarrollado con Anchor sobre la blockchain de Solana que implementa un sistema CRUD descentralizado para administrar refacciones directamente en la red.

Cada refacción se almacena en su propia cuenta y la refaccionaria mantiene un índice de PDAs, permitiendo gestionar inventario de manera transparente y verificable.

Las refacciones se modelan como cuentas PDA individuales, lo que garantiza:

Integridad y trazabilidad de los datos en cadena
​

Propiedad verificable ligada a la wallet del dueño
​

Direcciones determinísticas basadas en seeds (semillas)
​

Uso eficiente del almacenamiento al poder cerrar cuentas y recuperar renta
​

🧩 Características Principales
🏪 Registro de Refaccionaria
El programa permite crear una refaccionaria personalizada por wallet, identificada por un nombre:

Nombre de la refaccionaria (string limitado a 60 caracteres)

Propietario (wallet creadora)

Lista de PDAs de refacciones asociadas

Cada refaccionaria se almacena en una PDA derivada de:

rust
["refaccionaria", nombre_refaccionaria, owner_pubkey]
Estas PDAs se derivan de seeds y el program_id, lo que permite recrear las direcciones desde el cliente sin necesidad de claves privadas.
​

🔩 Gestión de Refacciones
Cada refacción se almacena como una cuenta independiente con la siguiente información:

nombre de la refacción

automovil (vehículo al que pertenece)

marca

modelo

año (u16)

precio (u64, en lamports)

disponible (bool)

refaccionaria (nombre de la refaccionaria a la que pertenece)

El programa permite:

Agregar nuevas refacciones al inventario de una refaccionaria

Alternar el estado de disponibilidad (por ejemplo, en existencia / agotado)

Eliminar refacciones, liberando su cuenta on-chain

Cada refacción se almacena en una PDA derivada de:

rust
["refaccion", nombre_refaccion, owner_pubkey]
🔄 Operaciones Seguras
Todas las operaciones críticas validan que el owner de la refaccionaria sea quien firma la transacción.

De esta forma, solo el dueño puede:

Agregar refacciones a su refaccionaria

Modificar el estado de disponibilidad

Eliminar refacciones existentes

Las validaciones se realizan con macros de Anchor (require! y enums de error personalizados), siguiendo buenas prácticas de seguridad en programas Solana.
​

🏛️ Arquitectura del Programa
📦 Modelo de Datos
El contrato define dos cuentas principales:

rust
Refaccionaria {
    owner: Pubkey,
    n_refaccionaria: String,
    refacciones: Vec<Pubkey>,
}

Refaccion {
    refaccionaria: String,
    nombre: String,
    automovil: String,
    marca: String,
    modelo: String,
    año: u16,
    precio: u64,
    disponible: bool,
}
La refaccionaria actúa como índice de refacciones (vector de PDAs), mientras que cada refacción almacena su información detallada.
​

🧭 Contextos
El programa utiliza contextos Anchor para encapsular las cuentas requeridas en cada instrucción:

rust
NuevaRefaccionaria
NuevaRefaccion
ModificarRefaccion
EliminarRefaccion
NuevaRefaccionaria: inicializa una nueva cuenta PDA de refaccionaria.

NuevaRefaccion: crea una nueva refacción y la vincula a una refaccionaria.

ModificarRefaccion: alterna el estado de disponibilidad.

EliminarRefaccion: remueve la refacción de la lista y cierra su cuenta, devolviendo la renta al owner.

🧱 Diseño Basado en PDAs
El sistema aprovecha Program Derived Addresses para modelar refaccionarias y refacciones:
​

Direcciones determinísticas a partir de seeds ("refaccionaria", "refaccion", nombre, owner_pubkey)

No requieren clave privada; solo el programa puede firmar sobre ellas mediante invoke_signed
​

Evitan el uso de índices globales centralizados

Facilitan el escalado del inventario, ya que cada refacción vive en su propia cuenta

Esta arquitectura se alinea con los patrones recomendados para dApps CRUD en Solana usando Anchor.

🔐 Seguridad y Buenas Prácticas
El contrato implementa varias medidas:

Validación de propietario en todas las operaciones de escritura sobre el inventario

Verificación de pertenencia: una refacción solo puede eliminarse desde su refaccionaria correspondiente

Errores explícitos mediante #[error_code] para mejorar la depuración en el cliente Anchor
​

Cierre de cuentas (close = owner) para recuperar renta y liberar espacio en la red cuando se eliminan refacciones
​

⚡ Eficiencia y Costos
Al almacenar cada refacción en su propia PDA y mantener solo un vector de direcciones en la refaccionaria, el sistema evita estructuras globales pesadas.
​
Esto permite:

Menor uso de almacenamiento a nivel de cada cuenta

Lecturas específicas de refacciones desde el cliente sin recorrer listas globales

Recuperación de renta al eliminar refacciones, haciendo el sistema más rent-efficient
​

🎯 Objetivo del Proyecto
REFAX busca demostrar cómo construir una refaccionaria descentralizada limpia, segura y escalable en Solana utilizando Anchor y PDAs.

El proyecto funciona como:

Ejemplo educativo de CRUD on-chain aplicado a inventarios de autopartes

Base para sistemas de gestión de refaccionarias y stocks en Web3

Demostración práctica del uso de PDAs e índices on-chain para controlar inventarios distribuidos en Solana
