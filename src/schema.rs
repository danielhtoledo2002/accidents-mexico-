// @generated automatically by Diesel CLI.

diesel::table! {
    atms (name) {
        name -> Varchar,
        address -> Varchar,
        bank_id -> Unsigned<Integer>,
        money -> Unsigned<Double>,
    }
}

diesel::table! {
    cards (number) {
        number -> Varchar,
        bank_id -> Unsigned<Integer>,
        cvv -> Unsigned<Integer>,
        nip -> Integer,
        expiration_date -> Date,
        balance -> Unsigned<Double>,
        #[sql_name = "type"]
        type_ -> Varchar,
        expired -> Bool,
        tryall -> Unsigned<Integer>,
    }
}

diesel::table! {
    deposits (id) {
        id -> Unsigned<Integer>,
        amount -> Unsigned<Double>,
        date -> Date,
        card_number -> Varchar,
        atm_name -> Varchar,
    }
}

diesel::table! {
    deudas (id) {
        id -> Unsigned<Integer>,
        number -> Varchar,
        #[sql_name = "type"]
        type_ -> Varchar,
        deuda -> Double,
    }
}

diesel::table! {
    transfers (id) {
        id -> Unsigned<Integer>,
        date -> Date,
        amount -> Unsigned<Double>,
        sent_money -> Varchar,
        received_money -> Varchar,
    }
}

diesel::table! {
    withdrawals (id) {
        id -> Unsigned<Integer>,
        amount -> Double,
        date -> Date,
        atm_name -> Varchar,
        card_number -> Varchar,
    }
}

diesel::joinable!(deposits -> atms (atm_name));
diesel::joinable!(deposits -> cards (card_number));
diesel::joinable!(deudas -> cards (number));
diesel::joinable!(withdrawals -> atms (atm_name));
diesel::joinable!(withdrawals -> cards (card_number));

diesel::allow_tables_to_appear_in_same_query!(
    atms,
    cards,
    deposits,
    deudas,
    transfers,
    withdrawals,
);
