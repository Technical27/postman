table! {
    guilds (id) {
        id -> Int8,
    }
}

table! {
    guildusers (user_id, guild_id) {
        guild_id -> Int8,
        user_id -> Int8,
    }
}

table! {
    messages (msg_id) {
        msg_id -> Int8,
        cmd_msg_id -> Int8,
    }
}

table! {
    users (id) {
        id -> Int8,
        rank -> Int8,
    }
}

joinable!(guildusers -> guilds (guild_id));
joinable!(guildusers -> users (user_id));

allow_tables_to_appear_in_same_query!(
    guilds,
    guildusers,
    messages,
    users,
);
