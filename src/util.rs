#[macro_export]
macro_rules! register_commands {
    ( $( $command:tt )+ ) => {
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

#[macro_export]
macro_rules! prelude {
    ( $( $package:tt )* ) => {
        $(
            mod $package;
            pub use $package::*;
        )*
    };
}
