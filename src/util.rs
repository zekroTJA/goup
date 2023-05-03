#[macro_export]
macro_rules! register_commands {
    ( $( $command:tt ),+ $(,)* ) => {
        #[derive(Subcommand)]
        enum Commands {
            $(
                $command($command),
            )*
        }

        impl Deref for Commands {
            type Target = dyn $crate::commands::Command;

            fn deref(&self) -> &Self::Target {
                match &self {
                    $(
                        Self::$command(c) => c,
                    )*
                }
            }
        }
    };
}
