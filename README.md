## Programa de Administración de Eventos en Solana

### Introducción

El objetivo de este proyecto es desarrollar un ADMINISTRADOR DE EVENTOS DESCENTRALIZADO basado en la blockchain de Solana. Este administrador permitirá a los usuarios crear eventos, participar como colaboradores, vender entradas y distribuir las ganancias obtenidas al finalizar el evento.

Estos eventos dependerán de la colaboración de los usuarios para llevarse a cabo, ya que los fondos necesarios para su organización, se obtendrán de la venta de Tokens del Evento que los usuarios adquieran a manera de colaboradores. Aquellos usuarios colaboradores del evento recibirán parte de las ganancias que se genere con la venta de entradas. Estos tokens tendrán un valor 1:1 de una moneda específica asignada al momento de crear el evento, que actuará como Moneda Aceptada en todas las transacciones. Las ganancias obtenidas de los Tokens del Evento se depositarán en una Bóveda de Tesorería. El organizador podrá retirar fondos de la Bóveda de Tesorería para cubrir los gastos referentes al evento.

Cada evento pondrá a la venta una cantidad de entradas con un valor definido al momento de crear el evento. Las ganancias obtenidas de la venta de las entradas se depositarán en una Bóveda de Ganancias. Al finalizar el evento, cada colaborador podrá retirar el monto que le corresponde de la Bóveda de Ganancias, valor que se calcula de forma proporcional a la cantidad de Tokens del Evento adquiridos por cada colaborador.

### Estructura de un Evento

La estructura del evento estará definida por los datos básicos del evento y las cuentas necesarias para realizar las transacciones:

```plaintext
Event {
    // data
    name: String,
    ticket_price: u64,
    active: bool,
    // accounts
    authority: Pubkey,
    accepted_mint: Pubkey,
    ...
}
```

### Descripción del Programa

El administrador de eventos está compuesto por seis (6) instrucciones principales que describen el flujo de trabajo del sistema:

1.  **create\_event:** Crea un nuevo evento en la blockchain.
2.  **buy\_tokens:** Transfiere el valor del precio del Token del Evento a la Bóveda de Tesorería y hace mint de los Tokens del Evento a la cuenta del colaborador.
3.  **buy\_tickets**: Transfiere el valor del precio de la entrada del evento a la Bóveda de Ganancias.
4.  **withdraw\_funds:** Transfiere fondos desde la Bóveda de Tesorería a la cuenta del organizador del evento.
5.  **close\_event:** Actualiza los datos del evento para indicar que ya no está activo.
6.  **withdraw\_earnings:** Transfiere fondos desde la Bóveda de Ganancias a la cuenta del colaborador que solicita el retiro.
