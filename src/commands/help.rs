use serenity::client::Context;
use serenity::framework::standard::{macros::help, Args, CommandGroup, CommandResult, HelpOptions};
use serenity::model::prelude::Message;

#[help]
pub fn help(
    ctx: &mut Context,
    msg: &Message,
    mut args: Args,
    _: &'static HelpOptions,
    groups: &[&'static CommandGroup],
) -> CommandResult {
    match args.single::<String>() {
        Ok(arg) => {
            for group in groups {
                for cmd in group.options.commands {
                    if cmd.options.names.iter().any(|x| x == &arg.as_str()) {
                        if !cmd.options.help_available {
                            return Ok(());
                        }
                        msg.channel_id.send_message(&ctx.http, |m| {
                            m.embed(|e| {
                                e.title(cmd.options.names[0]);

                                e.field("description", cmd.options.desc.unwrap_or("none"), false);

                                if cmd.options.names.len() > 1 {
                                    e.field("aliases", cmd.options.names.join(", "), false);
                                } else {
                                    e.field("aliases", "none", false);
                                }

                                e.field("usage", cmd.options.usage.unwrap_or("none"), false);

                                if cmd.options.examples.len() > 0 {
                                    e.field("examples", cmd.options.examples.join("\n"), false);
                                } else {
                                    e.field("examples", "none", false);
                                }

                                e
                            })
                        })?;
                    }
                }
            }
        }
        Err(_) => {}
    }
    Ok(())
}
