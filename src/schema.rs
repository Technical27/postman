table! {
    guilds (id) {
        id -> BigInt,
    }
}

table! {
    messages (msg_id) {
        msg_id -> BigInt,
        cmd_msg_id -> BigInt,
    }
}

table! {
    users (id) {
        id -> BigInt,
        guild_id -> BigInt,
    }
}

allow_tables_to_appear_in_same_query!(
    guilds,
    messages,
    users,
);
