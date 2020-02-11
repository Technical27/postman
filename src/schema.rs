table! {
    guilds (id) {
        id -> BigInt,
    }
}

table! {
    guildusers (guild_id) {
        guild_id -> BigInt,
        user_id -> BigInt,
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

joinable!(guildusers -> users (user_id));

allow_tables_to_appear_in_same_query!(
    guilds,
    guildusers,
    messages,
    users,
);
